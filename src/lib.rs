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
use text_message::filter::Action;

use futures::join;
use futures::future::join_all;
use futures::executor::block_on;
use futures::Future;

use tonic::transport::Endpoint;
use tonic::codegen::StdError;

use tokio::sync::{Mutex, MutexGuard, mpsc::{self, Sender, Receiver}};
use tokio::runtime;
//use tokio::task::block_in_place;

use std::sync::mpsc as std_mpsc;
type ServerDisconnectSender = std_mpsc::Sender<()>;
type ServerDisconnectReceiver = std_mpsc::Receiver<()>;

pub use rayon::{ThreadPoolBuilder, ThreadPool};

use std::convert::TryInto;
use std::sync::Arc;
use std::marker::Send;
use std::pin::Pin;
use std::{thread, time};

// https://www.reddit.com/r/rust/comments/f7qrya/defining_an_async_function_type/

pub type FutureBool = Pin<Box<dyn Future<Output = bool> + Send>>;

/// Turn a bool into a boxed future.
pub fn future_from_bool(b: bool) -> FutureBool {
    Box::pin(futures::future::ready(b))
}

/// Turn an async block that return a bool into a boxed future.
pub fn future_from_async<F: Future<Output = bool> + Send + 'static>(f: F) -> FutureBool {
    Box::pin(f)
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

impl<T: std::marker::Send> DataMutex<T> 
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

    /*
    /// Lock the Mutex synchronously while outside a tokio runtime. Calling this method inside a
    /// tokio runtime will cause a panic.
    pub fn lock_outside_runtime(&mut self) -> MutexGuard<T> {
        runtime(self.t.lock())
    }
    */
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
    pub fn new(t: T, addr: A) -> Self {
        Self {
            i: MurmurInterface::new(t, addr)
        }
    }
    pub fn data(mut self, t: T) -> Self {
        self.i.t = DataMutex{t: Arc::new(Mutex::new(t))};
        self
    }
    pub fn data_mutex(mut self, t: DataMutex<T>) -> Self {
        self.i.t = t;
        self
    }
    pub fn addr(mut self, addr: A) -> Self {
        self.i.addr = addr;
        self
    }
    pub fn user_connected(mut self, user_connected: Vec<Handler<T>>) -> Self {
        self.i.user_connected = user_connected;
        self
    }
    pub fn user_disconnected(mut self, user_disconnected: Vec<Handler<T>>) -> Self {
        self.i.user_disconnected = user_disconnected;
        self
    }
    pub fn user_state_changed(mut self, user_state_changed: Vec<Handler<T>>) -> Self {
        self.i.user_state_changed = user_state_changed;
        self
    }
    pub fn user_text_message(mut self, user_text_message: Vec<Handler<T>>) -> Self {
        self.i.user_text_message = user_text_message;
        self
    }
    pub fn channel_created(mut self, channel_created: Vec<Handler<T>>) -> Self {
        self.i.channel_created = channel_created;
        self
    }
    pub fn channel_removed(mut self, channel_removed: Vec<Handler<T>>) -> Self {
        self.i.channel_removed = channel_removed;
        self
    }
    pub fn channel_state_changed(mut self, channel_state_changed: Vec<Handler<T>>) -> Self {
        self.i.channel_state_changed = channel_state_changed;
        self
    }
    pub fn chat_filters(mut self, chat_filters: Vec<ChatFilter<T>>) -> Self {
        self.i.chat_filters = chat_filters;
        self
    }
    pub fn authenticators(mut self, authenticators: Vec<Authenticator<T>>) -> Self {
        self.i.authenticators = authenticators;
        self
    }
    pub fn context_actions(mut self, context_actions: Vec<(ContextAction, Vec<ContextActionHandler<T>>)>) -> Self {
        self.i.context_actions = context_actions;
        self
    }
    pub fn server_connected(mut self, connection_handlers: Vec<ConnectHandler<T>>) -> Self {
        self.i.server_connected = connection_handlers;
        self
    }
    pub fn server_disconnected(mut self, disconnection_handlers: Vec<DisconnectHandler<T>>) -> Self {
        self.i.server_disconnected = disconnection_handlers;
        self
    }
    pub fn build(&self) -> MurmurInterface<A, T> {
        self.i.clone()
    }
}

/// Start a connection to a Mumble server for each provided MurmurInterface
/// 
/// # Arguments
///
/// * `num_threads` - The number of threads available to the threadpool on which the connections
/// will run.
///
/// * `interfaces` - The data required to set up each connection
///
/// # Return value
///
/// Returns a Vec of tuples. The first tuple element is the Client needed to communicate with each
/// server. The second tuple element is a DataMutex that contains the persistent data for each
/// server.
///
/// # Example
///
/// This is an example which shows how to print the contents of every text message sent to a server
/// into the console. The `text_message` function will be called every time a text message is sent.
///
/// ```
/// use murmur_grpc::*;
///
/// fn text_message(_t: DataMutex<()>, _c: Client, event: &Event) -> FutureBool {
///     println!("{}", event.message.as_ref().unwrap().text.as_ref().unwrap());
///     future_from_bool(true)
/// }
///
/// fn main() {
///     let i = MurmurInterfaceBuilder::new((), "http://127.0.0.1:50051")
///         .user_text_message(vec![text_message])
///         .build();
///     let server_disconnect_receiver = murmur_grpc::start(1, vec![i])[0].2;
///     // wait for the connection to the server to close.
///     server_disconnect_receiver.recv();
/// }
/// ```
pub fn start<A, T>(num_threads: usize, interfaces: Vec<MurmurInterface<A, T>>) -> Vec<(Client, DataMutex<T>, ServerDisconnectReceiver)>
where A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone + 'static,
{
    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let mut result_vec = vec![];
    for i in interfaces.into_iter() {
        let (s, r) = std_mpsc::channel();
        // for whatever reason, we cannot pass this client into each child thread as trying to use
        // its streams will panic if we do.
        let c = runtime(V1Client::connect(i.addr.clone())).unwrap();
        result_vec.push((c, i.t.clone(), r));
        add_connection_to_thread_pool(&thread_pool, i, Some(s));
    }
    result_vec
}

pub fn add_connection_to_thread_pool<A, T>(thread_pool: &ThreadPool, i: MurmurInterface<A, T>, s: Option<ServerDisconnectSender>)
where T: Send + Clone + 'static,
      A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>
{
    thread_pool.spawn(move || {
        runtime(async move {
            tokio::spawn(async move {
                loop {
                    let i_clone = i.clone();
                    start_single(i_clone).await;
                    // send indication that the server connection has closed.
                    if let Some(s) = &s {
                        s.send(())
                            .expect("Sending indication that connection to server has closed");
                    }
                    // Don't iterate more than once if this interface is not configure to
                    // auto-reconnect.
                    if !i.auto_reconnect {break;}
                    thread::sleep(time::Duration::from_secs(RECONNECT_DELAY_SECONDS));
                }
            }).await.unwrap();
        });
    });
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
        tokio::task::spawn(async move {
            if !user_connected.is_empty() || !user_disconnected.is_empty() || !user_state_changed.is_empty() 
                || !user_text_message.is_empty() || !channel_created.is_empty() || !channel_removed.is_empty() 
                    || !channel_state_changed.is_empty() 
            {
                let mut event_stream = c.server_events(server).await
                    .expect("Connecting to the event stream")
                    .into_inner();
                while let Ok(Some(event)) = event_stream.message().await {
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
        })
    };
    // CHAT FILTER
    let chat_filter_fut = {
        let mut c = c.clone();
        let t = t.clone();
        let (mut s, r): (Sender<Filter>, Receiver<Filter>) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        s.send(Filter {server: Some(server.clone()), action: None, message: None}).await
            .expect("Sending initial message over filter stream to activate it");
        tokio::task::spawn(async move {
            if !chat_filters.is_empty() {
                let mut filter_stream = c.text_message_filter(r).await
                    .expect("Connecting to filter stream")
                    .into_inner();
                while let Ok(Some(mut filter)) = filter_stream.message().await {
                    for chat_filter in chat_filters.iter() {
                        if !(chat_filter)(t.clone(), c.clone(), &mut filter).await || filter.action() != Action::Accept {
                            break;
                        }
                    }
                    s.send(filter).await.unwrap();
                }
            }
        })
    };
    // AUTHENTICATOR
    let authenticator_fut = {
        let mut c = c.clone();
        let t = t.clone();
        let (mut s, r): (Sender<Response>, Receiver<Response>) = mpsc::channel(CHANNEL_BUFFER_SIZE);
        tokio::task::spawn(async move {
            if !authenticators.is_empty() {
                let mut authenticator_stream = c.authenticator_stream(r).await
                    .expect("Connecting to authenticator stream")
                    .into_inner();
                while let Ok(Some(request)) = authenticator_stream.message().await {
                    let mut response = Response::default();
                    for authenticator in authenticators.iter() {
                        if !(authenticator)(t.clone(), c.clone(), &mut response, &request).await {
                            break;
                        }
                    }
                    s.send(response).await.unwrap();
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
            tokio::task::spawn(async move {
                if !handlers.is_empty() {
                    let mut context_action_stream = c.context_action_events(action).await
                        .expect("Connecting to context action stream")
                        .into_inner();
                    while let Ok(Some(context_action)) = context_action_stream.message().await {
                        for handler in handlers.iter() {
                            if !(handler)(t.clone(), c.clone(), &context_action).await {
                                break;
                            }
                        }
                    }
                }
            })
        }))
    };

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
pub fn runtime<F: 'static + Future + std::marker::Send>(f: F) -> F::Output 
where F::Output: std::marker::Send 
{
    block_on(runtime::Builder::new_current_thread().enable_all().build().unwrap().spawn(f)).unwrap()
}
