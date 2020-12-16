const RECONNECT_DELAY_SECONDS: u64 = 5;
const GRPC_COMPLETION_QUEUE_SIZE: usize = 1;
const MAX_SEND_ATTEMPTS: usize = 5;

mod protos;

pub use protos::MurmurRPC::*;
pub use protos::MurmurRPC_grpc::V1Client;
use futures::{StreamExt, SinkExt, join, executor::block_on, future::{join_all, Future}};
use std::{thread::{self, JoinHandle}, time};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::pin::Pin;
use grpcio::{ChannelBuilder, Environment, WriteFlags};
pub use protobuf::*;

pub type FutureValue<O> = Pin<Box<(dyn Future<Output = O>)>>;

pub type Handler<T> = fn(t: Arc<Mutex<T>>, c: V1Client, event: Server_Event) -> FutureValue<bool>;

pub type ChatFilter<T> = fn(t: Arc<Mutex<T>>, c: V1Client, filter: TextMessage_Filter) -> FutureValue<(bool, TextMessage_Filter)>;

pub type Authenticator<T> = fn(t: Arc<Mutex<T>>, c: V1Client, response: Authenticator_Response, request: Authenticator_Request) -> FutureValue<(bool, Authenticator_Response)>;

pub type ContextActionHandler<T> = fn(t: Arc<Mutex<T>>, c: V1Client, action: ContextAction) -> FutureValue<bool>;

pub type ConnectHandler<T> = fn(t: Arc<Mutex<T>>, c: V1Client) -> bool;

pub type DisconnectHandler<T> = fn(t: Arc<Mutex<T>>) -> bool;


#[derive(Clone)]
pub struct MurmurInterface<T> 
where T: Send + Clone,
{
    pub t: Arc<Mutex<T>>,
    pub addr: String,
    pub server_id: u32,
    pub auto_reconnect: bool,
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
    pub server_connected: Vec<ConnectHandler<T>>,
    pub server_disconnected: Vec<DisconnectHandler<T>>
}

impl<T> MurmurInterface<T> 
where T: Send + Clone,
{
    pub fn new(t: T, addr: String) -> Self {
        Self {
            t: Arc::new(Mutex::new(t)),
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

pub struct MurmurInterfaceBuilder<T>
where T: Send + Clone,
{
    pub i: MurmurInterface<T>
}

impl<T> MurmurInterfaceBuilder<T>
where T: Send + Clone,
{
    pub fn new<S>(t: T, addr: S) -> Self 
    where S: Into<String>
    {
        Self {
            i: MurmurInterface::new(t, addr.into())
        }
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
    pub fn build(&self) -> MurmurInterface<T> {
        self.i.clone()
    }
}


pub struct ClientManager {
    pub clients: HashMap<String, V1Client>
}

impl ClientManager {
    
    pub fn new() -> Self {
        Self {
            clients: HashMap::new()
        }
    }

    pub fn start_connection<T>(&mut self, i: MurmurInterface<T>) -> JoinHandle<()> 
    where T: Send + Clone + 'static,
    {
        let addr = i.addr.clone();
        let c = if let Some(c) = self.clients.get(&addr) {
            println!("Using existing client for {}", &i.addr);
            c.to_owned()
        } else {
            let env = Environment::new(GRPC_COMPLETION_QUEUE_SIZE);
            let builder = ChannelBuilder::new(Arc::new(env));
            let channel = builder.connect(i.addr.as_ref());
            println!("Connecting to {}", &i.addr);
            let c = V1Client::new(channel);
            self.clients.insert(addr, c.clone());
            c
        };
        start_connection(i, c)
    }
}


fn start_connection<T>(i: MurmurInterface<T>, c: V1Client) -> JoinHandle<()>
where T: Send + Clone + 'static,
{
    thread::spawn(move || {
        loop {
            let i_clone = i.clone();
            start_single(i_clone, c.clone());
            println!("Connection to server at {} with id {} closed",
                     &i.addr, &i.server_id);
            if !i.auto_reconnect { break; }
            thread::sleep(time::Duration::from_secs(RECONNECT_DELAY_SECONDS));
        }
    })
}


fn start_single<T>(i: MurmurInterface<T>, c: V1Client)
where T: Send + Clone + 'static,
{
    let t = i.t;
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

    for connect_handler in server_connected.iter() {
        if !(connect_handler)(t.clone(), c.clone()) {
            break;
        }
    }

    let mut server = Server::new();
    server.set_id(server_id);
    let server = c.server_get(&server)
        .expect("getting current server");


    // SERVER EVENTS
    let server_event_fut = {
        let c = c.clone();
        let t = t.clone();
        let server = server.clone();

        async move {
            let mut event_stream = c.server_events(&server)
                .expect("Connecting to the event stream");
            while let Some(Ok(event)) = event_stream.next().await {
                match event.get_field_type() {
                    Server_Event_Type::UserConnected => handle_event(
                        t.clone(), c.clone(), &user_connected, &event),
                    Server_Event_Type::UserDisconnected => handle_event(
                        t.clone(), c.clone(), &user_disconnected, &event),
                    Server_Event_Type::UserStateChanged => handle_event(
                        t.clone(), c.clone(), &user_state_changed, &event),
                    Server_Event_Type::UserTextMessage => handle_event(
                        t.clone(), c.clone(), &user_text_message, &event),
                    Server_Event_Type::ChannelCreated => handle_event(
                        t.clone(), c.clone(), &channel_created, &event),
                    Server_Event_Type::ChannelRemoved => handle_event(
                        t.clone(), c.clone(), &channel_removed, &event),
                    Server_Event_Type::ChannelStateChanged => handle_event
                        (t.clone(), c.clone(), &channel_state_changed, &event),
                }.await
            }
        }
    };


    // CHAT FILTER
    let chat_filter_fut = {
        let c = c.clone();
        let t = t.clone();

        async move {
            if !chat_filters.is_empty() {
                let (mut filter_sender, mut filter_receiver) = c.text_message_filter()
                    .expect("Connecting to filter stream");
                let mut initial_filter = TextMessage_Filter::new();
                initial_filter.set_server(server.clone());
                filter_sender.send((initial_filter, WriteFlags::default())).await
                    .expect("Sending inital filter to open filter stream");
                while let Some(Ok(mut filter)) = filter_receiver.next().await {
                    if filter.get_server().get_id() != server_id { break; }
                    for chat_filter in chat_filters.iter() {
                        let (cont, new_filter) = (chat_filter)(
                            t.clone(), c.clone(), filter).await;
                        filter = new_filter;
                        if !cont { break; }
                    }
                    if !try_send(filter, &mut filter_sender).await { break; }
                }
            }
        }
    };


    // AUTHENTICATOR
    let authenticator_fut = {
        let c = c.clone();
        let t = t.clone();

        async move {
            if !authenticators.is_empty() {
                let (mut auth_sender, mut auth_receiver) = c.authenticator_stream()
                    .expect("Connecting to authenticator stream");
                while let Some(Ok(request)) = auth_receiver.next().await {
                    let mut response = Authenticator_Response::new();
                    for authenticator in authenticators.iter() {
                        let (cont, new_response) = (authenticator)(
                            t.clone(), c.clone(), response, request.clone()).await;
                        response = new_response;
                        if !cont { break; }
                    }
                    if !try_send(response, &mut auth_sender).await { break; }
                }
            }
        }
    };


    // CONTEXT MENU ACTIONS
    let context_action_fut = {
        let c = c.clone();
        let t = t.clone();
        join_all(context_actions.into_iter().map(|(action, handlers)| {
            let c = c.clone();
            let t = t.clone();
            async move {
                while c.context_action_add(&action).is_err() {}
                if !handlers.is_empty() {
                    let mut context_action_stream = c.context_action_events(&action)
                        .expect("Connecting to context action stream");
                    while let Some(Ok(context_action)) = context_action_stream.next().await {
                        if context_action.get_server().get_id() != server_id { break; }
                        for handler in handlers.iter() {
                            if !(handler)(t.clone(), c.clone(), context_action.clone()).await {
                                break;
                            }
                        }
                    }
                }
            }
        }))
    };


    // join all the tasks into a single future
    block_on(async move {
        join!(server_event_fut, chat_filter_fut, authenticator_fut, context_action_fut);
    });

    for disconnect_handler in server_disconnected.iter() {
        if !(disconnect_handler)(t.clone()) {
            break;
        }
    }
}

pub fn from_option<T>(option: Option<T>) -> SingularPtrField<T> {
    SingularPtrField::from_option(option)
}

pub fn future<F, O>(f: F) -> Pin<Box<dyn Future<Output = O>>>
where F: Future<Output = O> + 'static
{
    Box::pin(f)
}

async fn try_send<T, S>(message: T, mut sink: S) -> bool
where T: Clone,
      S: SinkExt<(T, WriteFlags)> + Unpin
{
    for _ in 0..MAX_SEND_ATTEMPTS {
        if sink.send((message.clone(), WriteFlags::default().buffer_hint(false))).await.is_ok() {
            return sink.flush().await.is_ok();
        }
    }
    false
}


async fn handle_event<T>(t: Arc<Mutex<T>>, c: V1Client, handlers: &Vec<Handler<T>>, event: &Server_Event) 
where T: Send + Clone,
{
    for handler in handlers.iter() {
        if !(handler)(t.clone(), c.clone(), event.to_owned()).await {
            return;
        }
    }
}
