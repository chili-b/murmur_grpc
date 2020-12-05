// The delay in seconds before trying to reconnect after server connection closes
const RECONNECT_DELAY_SECONDS: u64 = 5;

const CHANNEL_BUFFER_SIZE: usize = 300;

mod murmur_rpc {
    tonic::include_proto!("murmur_rpc");
}

use murmur_rpc::v1_client::V1Client;
/// Client is a more specific definiton for [V1Client](murmur_rpc/v1_client/struct.V1Client.html)
/// which owns the methods that communicate with the Mumble server.
pub type Client = V1Client<tonic::transport::Channel>;

/// Gererated from the Murmur Protocol Buffer file
pub use murmur_rpc::*;

use text_message::Filter;
use authenticator::{Request, Response};
use server::event::Type;
use server::Event;

use futures::join;
use futures::future::join_all;
use futures::executor::block_on;
use futures::Future;

use tonic::transport::Endpoint;
use tonic::codegen::StdError;
use tonic::Streaming;

use tokio::sync::{Mutex, MutexGuard, mpsc};
use tokio::runtime::Builder;

use std::convert::TryInto;
use std::sync::Arc;
use std::marker::Send;
use std::pin::Pin;
use std::{thread::{self, JoinHandle}, time};

// https://www.reddit.com/r/rust/comments/f7qrya/defining_an_async_function_type/

/// A future with bool as its output type. Can by produced by either
/// [future_from_bool](fn.future_from_bool.html) or [future_from_async](fn.future_from_async.html)
pub type FutureBool = Pin<Box<dyn Future<Output = bool> + Send>>;

/// Turn a bool into a boxed future.
pub fn future_from_bool(b: bool) -> FutureBool {
    Box::pin(futures::future::ready(b))
}

/// Turn an async block that return a bool into a boxed future.
pub fn future_from_async<F: Future<Output = bool> + Send + 'static>(f: F) -> FutureBool {
    Box::pin(f)
}

async fn message_and_state<T>(stream: &mut Streaming<T>) -> (Option<T>, bool) {
    match stream.message().await {
        Ok(message) => {
            println!("message success")
            (message, true)
        },
        Err(status) => {
            eprintln!("message error: {:?}", status);
            (None, false)
        }
    }
}

/// Function that handles Mumble server events. Returns a boolean which determines whether or not
/// other functions will be allowed to process the event it has handled (similar to cases falling
/// through in a switch statement from other languages).
pub type Handler<T> = fn(t: DataMutex<T>, c: Client, e: &Event) -> FutureBool;

/// Funtion that filters the text chat and determines whether to Block/Reject/Drop messages.
/// Returns a boolean which determines whether or not other functions will be able to process the
/// message it has filtered (similar to cases falling through in a switch statement from other languages). 
/// The function's body should mutate `filter`.
pub type ChatFilter<T> = fn(t: DataMutex<T>, c: Client, filter: &mut Filter) -> FutureBool;

/// Function that handles events on the Mumble server authentication stream. Returns a boolean
/// which determines whether or not other functions will be able to process the authentication event
/// it has handled (similar to cases falling through in a switch statement from other languages).
pub type Authenticator<T> = fn(t: DataMutex<T>, c: Client, response: &mut Response, request: &Request) -> FutureBool;

/// Function that handles events on the Mumble server context action stream. Returns a boolean
/// which determines whether or not other functions will be able to process the authentication
/// event it has handled (similar to cases falling through in a switch statement from other languages).
pub type ContextActionHandler<T> = fn(t: DataMutex<T>, c: Client, action: &ContextAction) -> FutureBool;

/// Functicon that gets called in response to the connection to Murmur either opening.
pub type ConnectHandler<T> = fn(t: DataMutex<T>, c: Client) -> FutureBool;

/// Function that gets called in response to the connection to Murmur closing.
pub type DisconnectHandler<T> = fn(t: DataMutex<T>) -> FutureBool;

/// This struct is a wrapper over an asynchronous [Mutex](../tokio/sync/struct.Mutex.html) that
/// allows the contained value to be accessed inside or outside a tokio runtime asynchronously or
/// not.
#[derive(Clone)]
pub struct DataMutex<T> 
{
    t: Arc<Mutex<T>>
}

impl<T> DataMutex<T> 
{
    pub fn new(t: T) -> Self {
        Self {
            t: Arc::new(Mutex::new(t))
        }
    }

    /// Lock the Mutex synchronously while inside a tokio runtime. Calling this method while
    /// outside of a tokio runtime will cause a panic.
    pub fn lock(&mut self) -> MutexGuard<T> {
        block_on(self.t.lock())
    }

    /// Lock the Mutex asynchronously while inside a tokio runtime.
    pub async fn lock_async(&mut self) -> MutexGuard<'_, T> {
        self.t.lock().await
    }

    /// Lock the Mutex synchronously while outside a tokio runtime. Calling this method inside a
    /// tokio runtime will cause a panic.
    pub fn lock_outside_runtime(&mut self) -> MutexGuard<T> {
        runtime(self.t.lock())
    }
}

#[derive(Clone)]
pub struct MurmurInterface<A, T> 
where A: TryInto<Endpoint> + Send + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone,
{
    /// Mutex that holds data in order to allow a persistent state shared.
    pub t: DataMutex<T>,
    /// The address to connect to.
    pub addr: A,
    /// The virtual server id to connect to.
    pub server_id: u32,
    /// Whether or not to automatically try to reconnect when the server connection closes
    pub auto_reconnect: bool,
    /// Functions to be called when a user connects.
    pub user_connected: Vec<Handler<T>>,
    /// Functions to be called when a user disconnects.
    pub user_disconnected: Vec<Handler<T>>,
    /// Functions to be called when user state changes (mute, unmute, comment update, etc.)
    pub user_state_changed: Vec<Handler<T>>,
    /// Functions to be called when a user sends a text message.
    pub user_text_message: Vec<Handler<T>>,
    /// Functions to be called when a channel is created.
    pub channel_created: Vec<Handler<T>>,
    /// Functions to be called when a channel is removed.
    pub channel_removed: Vec<Handler<T>>,
    /// Functions to be called when a channel's state changes (renamed, linked, etc.)
    pub channel_state_changed: Vec<Handler<T>>,
    /// Functions to be called to filter the text chat.
    pub chat_filters: Vec<ChatFilter<T>>,
    /// Functions to be called to parse the authenticator stream.
    pub authenticators: Vec<Authenticator<T>>,
    /// Context actions to parse along with the functions that will parse them.
    pub context_actions: Vec<(ContextAction, Vec<ContextActionHandler<T>>)>,
    /// Functions to be called when the connection to Murmur opens
    pub server_connected: Vec<ConnectHandler<T>>,
    /// Functions to be called when the connection to Murmur closes
    pub server_disconnected: Vec<DisconnectHandler<T>>
}

impl<A, T> MurmurInterface<A, T> 
where A: TryInto<Endpoint> + Send + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone,
{
    pub fn new(t: T, addr: A) -> Self {
        Self {
            t: DataMutex::new(t),
            addr: addr,
            server_id: 1,
            auto_reconnect: false,
            user_connected: vec![],
            user_disconnected: vec![],
            user_state_changed: vec![],
            user_text_message: vec![],
            channel_created: vec![],
            channel_removed: vec![],
            channel_state_changed: vec![],
            chat_filters: vec![],
            authenticators: vec![],
            context_actions: vec![],
            server_connected: vec![],
            server_disconnected: vec![],
        }
    }
}

pub struct MurmurInterfaceBuilder<A, T>
where A: TryInto<Endpoint> + Send + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone,
{
    pub i: MurmurInterface<A, T>
}

impl<A, T> MurmurInterfaceBuilder<A, T>
where A: TryInto<Endpoint> + Send + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone
{
    /// Create a new MurmurInterfaceBuilder provided a data value and an addr to which the gRPC
    /// connection will be made
    pub fn new(t: T, addr: A) -> Self {
        Self {
            i: MurmurInterface::new(t, addr)
        }
    }
    /// Assign the the value that will be used to represent state
    pub fn data(mut self, t: T) -> Self {
        self.i.t = DataMutex{t: Arc::new(Mutex::new(t))};
        self
    }
    /// Alternative method to provide the value for state if it is already wrapped in a
    /// [DataMutex](struct.DataMutex.html)
    pub fn data_mutex(mut self, t: DataMutex<T>) -> Self {
        self.i.t = t;
        self
    }
    /// Set the address to which the connection will be made
    pub fn addr(mut self, addr: A) -> Self {
        self.i.addr = addr;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will respond to users connecting to the
    /// server
    pub fn user_connected(mut self, user_connected: Vec<Handler<T>>) -> Self {
        self.i.user_connected = user_connected;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will respond to users disconnecting from the
    /// server
    pub fn user_disconnected(mut self, user_disconnected: Vec<Handler<T>>) -> Self {
        self.i.user_disconnected = user_disconnected;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will respond to users' states changing (mute,
    /// unmute, change avatar, etc.)
    pub fn user_state_changed(mut self, user_state_changed: Vec<Handler<T>>) -> Self {
        self.i.user_state_changed = user_state_changed;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will respond to users sending text messages
    pub fn user_text_message(mut self, user_text_message: Vec<Handler<T>>) -> Self {
        self.i.user_text_message = user_text_message;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will repsond to channels being created.
    pub fn channel_created(mut self, channel_created: Vec<Handler<T>>) -> Self {
        self.i.channel_created = channel_created;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will respond to channels being removed.
    pub fn channel_removed(mut self, channel_removed: Vec<Handler<T>>) -> Self {
        self.i.channel_removed = channel_removed;
        self
    }
    /// Provide the [Handler](type.Handler.html)s that will respond to channels' states changing
    /// (renamed, linked, description updated, etc.)
    pub fn channel_state_changed(mut self, channel_state_changed: Vec<Handler<T>>) -> Self {
        self.i.channel_state_changed = channel_state_changed;
        self
    }
    /// Provide the [ChatFilter](type.ChatFilter.html)s that will filter text messages sent by
    /// users.
    pub fn chat_filters(mut self, chat_filters: Vec<ChatFilter<T>>) -> Self {
        self.i.chat_filters = chat_filters;
        self
    }
    /// Provide the [Authenticator](type.Authenticator.html)s that will handle user authentication
    /// events (register, unregister, authenticate, etc.)
    pub fn authenticators(mut self, authenticators: Vec<Authenticator<T>>) -> Self {
        self.i.authenticators = authenticators;
        self
    }
    /// This one is less straightforward. I'm not explaining it right now because I'm lazy.
    pub fn context_actions(mut self, context_actions: Vec<(ContextAction, Vec<ContextActionHandler<T>>)>) -> Self {
        self.i.context_actions = context_actions;
        self
    }
    /// Provide the [ConnectHandler](type.ConnectHandler.html)s that will respond to a connection
    /// to the Mumble server being established.
    pub fn server_connected(mut self, connection_handlers: Vec<ConnectHandler<T>>) -> Self {
        self.i.server_connected = connection_handlers;
        self
    }
    /// Provide the [DisconnectHandler](type.DisconnectHandler.html)s that will respond to the
    /// connection to the Mumble server being closed.
    pub fn server_disconnected(mut self, disconnection_handlers: Vec<DisconnectHandler<T>>) -> Self {
        self.i.server_disconnected = disconnection_handlers;
        self
    }
    /// Return a clone of the contained MurmurInterface
    pub fn build(&self) -> MurmurInterface<A, T> {
        self.i.clone()
    }
}


/// Start a connection to a Mumble server using the given [MurmurInterface](struct.MurmurInterface.html).
/// The function's body spawns a new thread and returns its join handle. I reccommend using
/// [MurmurInterfaceBuilder](struct.MurmurInterfaceBuilder.html) to generate MurmurInterfaces.
pub fn start_connection<A, T>(i: MurmurInterface<A, T>) -> JoinHandle<()>
where T: Send + Clone + 'static,
      A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>
{
    thread::spawn(move || {
        runtime(async move {
            loop {
                let i_clone = i.clone();
                start_single(i_clone).await;
                // Don't iterate more than once if this interface is not configure to
                // auto-reconnect.
                if !i.auto_reconnect { break; }

                thread::sleep(time::Duration::from_secs(RECONNECT_DELAY_SECONDS));
            }
        });
    })
}

async fn start_single<A, T>(i: MurmurInterface<A, T>)
where T: Send + Clone + 'static,
      A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>
{
    let t = i.t;
    let addr = i.addr;
    let server_id = i.server_id;
    let user_connected = i.user_connected;
    let user_disconnected = i.user_disconnected;
    let user_state_changed = i.user_state_changed;
    let user_text_message = i.user_text_message;
    let channel_created = i.channel_created;
    let channel_removed = i.channel_removed;
    let channel_state_changed = i.channel_state_changed;
    let chat_filters = i.chat_filters;
    let authenticators = i.authenticators;
    let context_actions = i.context_actions;
    let server_connected = i.server_connected;
    let server_disconnected = i.server_disconnected;

    let c = if let Ok(c) = V1Client::connect(addr).await {
        c
    } else {
        return;
    };

    for connect_handler in server_connected.iter() {
        if !(connect_handler)(t.clone(), c.clone()).await {
            break;
        }
    }

    let server = Server {
        id: server_id,
        running: Some(true),
        uptime: None,
    };

    // SERVER EVENTS
    let server_event_fut = {
        let mut c = c.clone();
        let t = t.clone();
        let server = server.clone();
        Box::pin(async move {
            if !user_connected.is_empty() || !user_disconnected.is_empty() || !user_state_changed.is_empty() 
                || !user_text_message.is_empty() || !channel_created.is_empty() || !channel_removed.is_empty() 
                    || !channel_state_changed.is_empty()
            {
                let mut event_stream = c.server_events(server).await
                    .expect("Connecting to the event stream")
                    .into_inner();
                loop {
                    if let Ok(Some(event)) = event_stream.message().await {
                        // the generated method name 'type' conflics with a rust keyword so 'r#' is needed
                        match event.r#type() {
                            Type::UserConnected       => handle_event(t.clone(), c.clone(), &user_connected, &event),
                            Type::UserDisconnected    => handle_event(t.clone(), c.clone(), &user_disconnected, &event),
                            Type::UserStateChanged    => handle_event(t.clone(), c.clone(), &user_state_changed, &event),
                            Type::UserTextMessage     => handle_event(t.clone(), c.clone(), &user_text_message, &event),
                            Type::ChannelCreated      => handle_event(t.clone(), c.clone(), &channel_created, &event),
                            Type::ChannelRemoved      => handle_event(t.clone(), c.clone(), &channel_removed, &event),
                            Type::ChannelStateChanged => handle_event(t.clone(), c.clone(), &channel_state_changed, &event),
                        }.await;
                    }
                }
            }
        })
    };

    // CHAT FILTER
    let chat_filter_fut = {
        let mut c = c.clone();
        let t = t.clone();

        let (mut s, r) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        s.send(Filter {server: Some(server.clone()), action: None, message: None}).await
            .expect("Sending initial message over filter stream to activate it");

        Box::pin(async move {
            if !chat_filters.is_empty() {
                let mut filter_stream = c.text_message_filter(r).await
                    .expect("Connecting to filter stream")
                    .into_inner();
                loop {
                    let (message, is_connected) = message_and_state(&mut filter_stream).await;
                    if let Some(mut filter) = message {
                        for chat_filter in chat_filters.iter() {
                            if !(chat_filter)(t.clone(), c.clone(), &mut filter).await { break; }
                        }
                        s.send(filter.clone()).await
                            .expect("Sending filter to stream");
                    }
                }
            }
        })
    };

    // AUTHENTICATOR
    let authenticator_fut = {
        let mut c = c.clone();
        let t = t.clone();
        let (mut s, r) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        Box::pin(async move {
            if !authenticators.is_empty() {
                let mut authenticator_stream = c.authenticator_stream(r).await
                    .expect("Connecting to authenticator stream")
                    .into_inner();
                loop {
                    if let Ok(Some(request)) = authenticator_stream.message().await {
                        let mut response = Response::default();
                        for authenticator in authenticators.iter() {
                            if !(authenticator)(t.clone(), c.clone(), &mut response, &request).await { break; }
                        }
                        s.send(response).await.unwrap();
                    }
                }
            }
        })
    };

    // CONTEXT MENU ACTIONS
    let context_action_fut = {
        let c = c.clone();
        let t = t.clone();
        join_all(context_actions.into_iter().map(|(action, handlers)| {
            let mut c = c.clone();
            let t = t.clone();
            Box::pin(async move {
                if !handlers.is_empty() {
                    let mut context_action_stream = c.context_action_events(action).await
                        .expect("Connecting to context action stream")
                        .into_inner();
                    loop {
                        if let Ok(Some(context_action)) = context_action_stream.message().await {
                            for handler in handlers.iter() {
                                if !(handler)(t.clone(), c.clone(), &context_action).await {
                                    break;
                                }
                            }
                        }
                    }
                }
            })
        }))
    };

    // join all the tasks into a single future
    drop(join!(server_event_fut, authenticator_fut, chat_filter_fut, context_action_fut));

    for disconnect_handler in server_disconnected.iter() {
        if !(disconnect_handler)(t.clone()).await {
            break;
        }
    }
}

async fn handle_event<T>(t: DataMutex<T>, c: Client, handlers: &Vec<Handler<T>>, event: &Event) 
where T: Send + Clone
{
    for handler in handlers.iter() {
        if !(handler)(t.clone(), c.clone(), event).await {
            return;
        }
    }
}

/// Create a runtime in order to execute a single Future outside of a tokio runtime. This function
/// will panic if it is called inside of a tokio runtime.
pub fn runtime<F: Future>(f: F) -> F::Output {
    Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap()
        .block_on(f)
}
