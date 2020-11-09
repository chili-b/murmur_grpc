pub mod murmur_rpc {
    tonic::include_proto!("murmur_rpc");
}

use murmur_rpc::v1_client::V1Client;
/// Client is a more specific definiton for [V1Client](murmur_rpc/v1_client/struct.V1Client.html)
/// which owns the methods that communicate with the Mumble server.
pub type Client = V1Client<tonic::transport::Channel>;
pub use murmur_rpc::*;

use futures::join;
use futures::future::join_all;
pub use futures::executor::block_on;
use futures::Future;

use tonic::transport::Endpoint;
use tonic::codegen::StdError;

use tokio::sync::{Mutex, MutexGuard, mpsc::{self, Sender, Receiver}};
use tokio::runtime::Runtime;

pub use rayon::{ThreadPoolBuilder, ThreadPool};

use std::convert::TryInto;
use std::sync::Arc;
use std::marker::Send;

/// Function that handles Mumble server events. Returns a boolean which determines whether or not
/// other functions will be allowed to process the event it has handled (similar to cases falling
/// through in a switch statement from other languages).
pub type Handler<T> = fn(t: DataMutex<T>, c: Client, e: &Event) -> bool;

/// Funtion that filters the text chat and determines whether to Block/Reject/Drop messages.
/// Returns a boolean which determines whether or not other functions will be able to process the
/// message it has filtered (similar to cases falling through in a switch statement from other languages). 
/// The function's body should mutate `filter`.
pub type ChatFilter<T> = fn(t: DataMutex<T>, c: Client, filter: &mut Filter) -> bool;

/// Function that handles events on the Mumble server authentication stream. Returns a boolean
/// which determines whether or not other functions will be able to process the authentication event
/// it has handled (similar to cases falling through in a switch statement from other languages).
pub type Authenticator<T> = fn(t: DataMutex<T>, c: Client, response: &mut Response, request: &Request) -> bool;

/// Function that handles events on the Mumble server context action stream. Returns a boolean
/// which determines whether or not other functions will be able to process the authentication
/// event it has handled (similar to cases falling through in a switch statement from other languages).
pub type ContextActionHandler<T> = fn(t: DataMutex<T>, c: Client, action: &ContextAction) -> bool;

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
        Runtime::new().unwrap().block_on(self.t.lock())
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
}

impl<A, T> MurmurInterface<A, T> 
where A: TryInto<Endpoint> + Send + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone,
{
    pub fn new(t: T, addr: A) -> Self {
        Self {
            t: DataMutex{t: Arc::new(Mutex::new(t))},
            addr: addr,
            server_id: 1,
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
/// fn text_message(_t: DataMutex<()>, _c: Client, event: &Event) -> bool {
///     println!("{}", event.message.as_ref().unwrap().text.as_ref().unwrap());
///     true
/// }
///
/// fn main() {
///     let i = MurmurInterfaceBuilder::new((), "http://127.0.0.1:50051")
///         .user_text_message(vec![text_message])
///         .build();
///     murmur_grpc::start(1, vec![i]);
///     std::thread::park();
/// }
/// ```
pub fn start<A, T>(num_threads: usize, interfaces: Vec<MurmurInterface<A, T>>) -> Vec<(Client, DataMutex<T>)>
where A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>,
      T: Send + Clone + 'static,
{
    let thread_pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    let mut result_vec = vec![];
    for i in interfaces.into_iter() {
        // for whatever reason, we cannot pass this client into each child thread as trying to use
        // its streams will panic if we do.
        let c = Runtime::new().unwrap().block_on(V1Client::connect(i.addr.clone())).unwrap();
        result_vec.push((c, i.t.clone()));
        add_connection_to_thread_pool(&thread_pool, i);
    }
    result_vec
}

pub fn add_connection_to_thread_pool<A, T>(thread_pool: &ThreadPool, i: MurmurInterface<A, T>) 
where T: Send + Clone + 'static,
      A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>
{
    thread_pool.spawn(move || {
        runtime(async move {
            tokio::spawn(async move {
                start_single(i).await;
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

    let c = V1Client::connect(addr).await.unwrap();
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
                let mut event_stream = c.server_events(server).await.unwrap().into_inner();
                while let Some(event) = event_stream.message().await.unwrap() {
                    // the generated method name 'type' conflics with a rust keyword so 'r#' is needed
                    match event.r#type() {
                        Type::UserConnected       => handle_event(t.clone(), c.clone(), &user_connected, &event),
                        Type::UserDisconnected    => handle_event(t.clone(), c.clone(), &user_disconnected, &event),
                        Type::UserStateChanged    => handle_event(t.clone(), c.clone(), &user_state_changed, &event),
                        Type::UserTextMessage     => handle_event(t.clone(), c.clone(), &user_text_message, &event),
                        Type::ChannelCreated      => handle_event(t.clone(), c.clone(), &channel_created, &event),
                        Type::ChannelRemoved      => handle_event(t.clone(), c.clone(), &channel_removed, &event),
                        Type::ChannelStateChanged => handle_event(t.clone(), c.clone(), &channel_state_changed, &event),
                    }
                }
            }
        })
    };
    // CHAT FILTER
    let chat_filter_fut = {
        let mut c = c.clone();
        let t = t.clone();
        let (mut s, r): (Sender<Filter>, Receiver<Filter>) = mpsc::channel(1);
        s.send(Filter {server: Some(server.clone()), action: None, message: None}).await.unwrap();
        tokio::task::spawn(async move {
            if !chat_filters.is_empty() {
                let mut filter_stream = c.text_message_filter(r).await.unwrap().into_inner();
                while let Some(mut filter) = filter_stream.message().await.unwrap() {
                    for chat_filter in chat_filters.iter() {
                        if !(chat_filter)(t.clone(), c.clone(), &mut filter) || filter.action() != Action::Accept {
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
        let (mut s, r): (Sender<Response>, Receiver<Response>) = mpsc::channel(1);
        tokio::task::spawn(async move {
            if !authenticators.is_empty() {
                let mut authenticator_stream = c.authenticator_stream(r).await.unwrap().into_inner();
                while let Some(request) = authenticator_stream.message().await.unwrap() {
                    let mut response = Response::default();
                    for authenticator in authenticators.iter() {
                        if !(authenticator)(t.clone(), c.clone(), &mut response, &request) {
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
                    let mut context_action_stream = c.context_action_events(action).await.unwrap().into_inner();
                    while let Some(context_action) = context_action_stream.message().await.unwrap() {
                        for handler in handlers.iter() {
                            if !(handler)(t.clone(), c.clone(), &context_action) {
                                break;
                            }
                        }
                    }
                }
            })
        }))
    };
    drop(join!(server_event_fut, authenticator_fut, chat_filter_fut, context_action_fut));
}

fn handle_event<T>(t: DataMutex<T>, c: Client, handlers: &Vec<Handler<T>>, event: &Event) 
where T: Send + Clone
{
    for handler in handlers.iter() {
        if !(handler)(t.clone(), c.clone(), event) {
            return;
        }
    }
}

/// Create a runtime in order to execute a single Future outside of a tokio runtime. This function
/// will panic if it is called inside of a tokio runtime.
pub fn runtime<F: Future>(f: F) -> F::Output {
    Runtime::new().unwrap().block_on(f)
}
