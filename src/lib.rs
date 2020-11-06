pub mod murmur_rpc {
    tonic::include_proto!("murmur_rpc");
}

use murmur_rpc::v1_client::V1Client;
pub type Client = V1Client<tonic::transport::Channel>;
pub use murmur_rpc::{Server, User, TextMessage, Channel};
use murmur_rpc::server::Event;
use murmur_rpc::server::event::Type;
pub use murmur_rpc::text_message::filter::Action;
pub use murmur_rpc::text_message::Filter;
pub use murmur_rpc::authenticator::{Response, Request};

use futures::executor::block_on;
use futures::future::join_all;

use tonic::transport::Endpoint;
use tonic::codegen::StdError;

use tokio::sync::{Mutex, MutexGuard, mpsc::{self, Sender, Receiver}};
use tokio::runtime::Runtime;

use std::convert::TryInto;
use std::sync::Arc;
use std::thread;
use std::marker::Send;

type Handler<T> = fn(
    t: &mut T,
    c: &mut Client, 
    user: &Option<User>, 
    message: &Option<TextMessage>, 
    channel: &Option<Channel>,
    server: &Option<Server>) -> bool;

type ChatFilter<T> = fn(t: &mut T, c: &mut Client, filter: &mut Filter) -> bool;

type Authenticator<T> = fn(t: &mut T, c: &mut Client, response: &mut Response, request: &Request) -> bool;

// TODO type ContextActionHandler<T> = fn(t: &mut T, c: &mut Client, ) -> bool;

type EventFields<'a> = (&'a Option<User>, &'a Option<TextMessage>, &'a Option<Channel>, &'a Option<Server>);

pub struct ClientDataMutex<T> {
    c: Client,
    t: Arc<Mutex<T>>
}

impl<T> ClientDataMutex<T> {
    pub fn client(&mut self) -> &mut Client {
        &mut self.c
    }

    pub fn lock_data(&self) -> MutexGuard<T> {
        block_on(self.t.lock())
    }

    pub async fn lock_data_async(&self) -> MutexGuard<'_, T> {
        self.t.lock().await
    }
}

pub struct MurmurInterface<A, T> 
where A: TryInto<Endpoint> + Send,
      A::Error: Into<StdError>,
      T: Send,
{
    pub t: T,
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
}

pub fn start<A, T>(interfaces: Vec<MurmurInterface<A, T>>) -> Vec<(Client, Arc<Mutex<T>>)>
where A: TryInto<Endpoint> + Send + 'static + Clone,
      A::Error: Into<StdError>,
      T: Send + 'static,
{
    let mut result_vec = vec![];
    for i in interfaces.into_iter() {
        let t = Arc::new(Mutex::new(i.t));
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
        // for whatever reason, we cannot pass this client into each child thread as trying to use
        // its streams will panic if we do.
        let c = Runtime::new().unwrap().block_on(V1Client::connect(addr.clone())).unwrap();
        println!("client connected");
        result_vec.push((c, t.clone()));
        thread::spawn(move || {
            Runtime::new().unwrap().block_on(async move {
                tokio::spawn(async move {
                    start_single(addr, t, server_id, 
                                 user_connected, 
                                 user_disconnected, 
                                 user_state_changed, 
                                 user_text_message, 
                                 channel_created, 
                                 channel_removed, 
                                 channel_state_changed, 
                                 chat_filters, 
                                 authenticators).await;
                }).await.unwrap();
            });
        });
    }
    result_vec
}

async fn start_single<T, A>(a: A, t: Arc<Mutex<T>>, server_id: u32, 
                            user_connected: Vec<Handler<T>>, 
                            user_disconnected: Vec<Handler<T>>, 
                            user_state_changed: Vec<Handler<T>>, 
                            user_text_message: Vec<Handler<T>>, 
                            channel_created: Vec<Handler<T>>, 
                            channel_removed: Vec<Handler<T>>, 
                            channel_state_changed: Vec<Handler<T>>, 
                            chat_filters: Vec<ChatFilter<T>>, 
                            authenticators: Vec<Authenticator<T>>)
where T: Send + 'static,
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
                    let mut t = t.lock().await;
                    let event_fields = event_fields(&event);
                    // the generated method name 'type' conflics with a rust keyword so 'r#' is needed
                    match event.r#type() {
                        Type::UserConnected       => handle_event(&mut *t, &mut c, &user_connected, event_fields),
                        Type::UserDisconnected    => handle_event(&mut *t, &mut c, &user_disconnected, event_fields),
                        Type::UserStateChanged    => handle_event(&mut *t, &mut c, &user_state_changed, event_fields),
                        Type::UserTextMessage     => handle_event(&mut *t, &mut c, &user_text_message, event_fields),
                        Type::ChannelCreated      => handle_event(&mut *t, &mut c, &channel_created, event_fields),
                        Type::ChannelRemoved      => handle_event(&mut *t, &mut c, &channel_removed, event_fields),
                        Type::ChannelStateChanged => handle_event(&mut *t, &mut c, &channel_state_changed, event_fields),
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
                    let mut t = t.lock().await;
                    for chat_filter in chat_filters.iter() {
                        if !(chat_filter)(&mut t, &mut c, &mut filter) || filter.action() != Action::Accept {
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
                    let mut t = t.lock().await;
                    let mut response = Response::default();
                    for authenticator in authenticators.iter() {
                        if !(authenticator)(&mut t, &mut c, &mut response, &request) {
                            break;
                        }
                    }
                    s.send(response).await.unwrap();
                }
            }
        })
    };
    join_all(vec![server_event_fut, authenticator_fut, chat_filter_fut]).await;
}

fn handle_event<T>(t: &mut T, c: &mut V1Client<tonic::transport::Channel>, handlers: &Vec<Handler<T>>, event_fields: EventFields) 
where T: Send
{
    let (user, message, channel, server) = event_fields;
    for handler in handlers.iter() {
        if !(handler)(t, c, user, message, channel, server) {
            return;
        }
    }
}

fn event_fields(event: &Event) -> (&Option<User>, &Option<TextMessage>, &Option<Channel>, &Option<Server>) {
    (&event.user, &event.message, &event.channel, &event.server)
}
