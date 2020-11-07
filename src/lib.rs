pub mod murmur_rpc {
    tonic::include_proto!("murmur_rpc");
}

use murmur_rpc::v1_client::V1Client;
pub type Client = V1Client<tonic::transport::Channel>;
pub use murmur_rpc::{Server, User, TextMessage, Channel, ContextAction};
pub use murmur_rpc::server::Event;
use murmur_rpc::server::event::Type;
pub use murmur_rpc::text_message::filter::Action;
pub use murmur_rpc::text_message::Filter;
pub use murmur_rpc::authenticator::{Response, Request};

use futures::join;
use futures::future::join_all;
use futures::executor::block_on;

use tonic::transport::Endpoint;
use tonic::codegen::StdError;

use tokio::sync::{Mutex, MutexGuard, mpsc::{self, Sender, Receiver}};
use tokio::runtime::Runtime;

use rayon::ThreadPoolBuilder;

use std::convert::TryInto;
use std::sync::Arc;
use std::marker::Send;

type Handler<T> = fn(t: DataMutex<T>, c: Client, e: &Event) -> bool;

type ChatFilter<T> = fn(t: DataMutex<T>, c: Client, filter: &mut Filter) -> bool;

type Authenticator<T> = fn(t: DataMutex<T>, c: Client, response: &mut Response, request: &Request) -> bool;

type ContextActionHandler<T> = fn(t: DataMutex<T>, c: Client, action: &ContextAction) -> bool;

#[derive(Clone)]
pub struct DataMutex<T> 
where T: Clone 
{
    t: Arc<Mutex<T>>
}

impl<T> DataMutex<T> 
where T: Clone
{
    pub fn lock(&mut self) -> MutexGuard<T> {
        block_on(self.t.lock())
    }

    pub async fn lock_async(&mut self) -> MutexGuard<'_, T> {
        self.t.lock().await
    }

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
    pub t: DataMutex<T>,
    pub addr: A,
    pub server_id: u32,
    pub user_connected: Vec<Handler<T>>,
    pub user_disconnected: Vec<Handler<T>>,
    pub user_state_changed: Vec<Handler<T>>,
    pub user_text_message: Vec<Handler<T>>,
    pub channel_created: Vec<Handler<T>>,
    pub channel_removed: Vec<Handler<T>>,
    pub channel_state_changed: Vec<Handler<T>>,
    pub chat_filters: Vec<ChatFilter<T>>,
    pub authenticators: Vec<Authenticator<T>>,
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

pub fn start<A, T>(num_threads: usize, interfaces: Vec<MurmurInterface<A, T>>) -> Vec<Client>
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
        result_vec.push(c);
        thread_pool.spawn(move || {
            Runtime::new().unwrap().block_on(async move {
                tokio::spawn(async move {
                    start_single(i.addr, i.t, i.server_id, 
                                 i.user_connected, 
                                 i.user_disconnected, 
                                 i.user_state_changed, 
                                 i.user_text_message, 
                                 i.channel_created, 
                                 i.channel_removed, 
                                 i.channel_state_changed, 
                                 i.chat_filters, 
                                 i.authenticators,
                                 i.context_actions).await;
                }).await.unwrap();
            });
        });
    }
    result_vec
}

async fn start_single<T, A>(a: A, t: DataMutex<T>, server_id: u32, 
                            user_connected: Vec<Handler<T>>, 
                            user_disconnected: Vec<Handler<T>>, 
                            user_state_changed: Vec<Handler<T>>, 
                            user_text_message: Vec<Handler<T>>, 
                            channel_created: Vec<Handler<T>>, 
                            channel_removed: Vec<Handler<T>>, 
                            channel_state_changed: Vec<Handler<T>>, 
                            chat_filters: Vec<ChatFilter<T>>, 
                            authenticators: Vec<Authenticator<T>>,
                            context_actions: Vec<(ContextAction, Vec<ContextActionHandler<T>>)>)
where T: Send + Clone + 'static,
      A: TryInto<Endpoint> + Send + 'static,
      A::Error: Into<StdError>
{
    let c = V1Client::connect(a).await.unwrap();
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
                    //let event_fields = event_fields(&event);
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
