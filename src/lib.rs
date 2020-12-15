// The delay in seconds before trying to reconnect after server connection closes
const RECONNECT_DELAY_SECONDS: u64 = 5;

const GRPC_COMPLETION_QUEUE_SIZE: usize = 1;

mod protos;

pub use protos::MurmurRPC::*;

use futures::{StreamExt, SinkExt, join, executor::block_on, future::join_all};

/// Client is a more specific definiton for [V1Client](murmur_rpc/v1_client/struct.V1Client.html)
/// which owns the methods that communicate with the Mumble server.
pub use protos::MurmurRPC_grpc::V1Client;

use std::{thread::{self, JoinHandle}, time};
use std::sync::{Arc, Mutex};

use grpcio::{ChannelBuilder, Environment, WriteFlags};
use protobuf::SingularPtrField;

/// Function that handles Mumble server events. Returns a boolean which determines whether or not
/// other functions will be allowed to process the event it has handled (similar to cases falling
/// through in a switch statement from other languages).
pub type Handler<T> = fn(t: Arc<Mutex<T>>, c: V1Client, e: &Server_Event) -> bool;

/// Funtion that filters the text chat and determines whether to Block/Reject/Drop messages.
/// Returns a boolean which determines whether or not other functions will be able to process the
/// message it has filtered (similar to cases falling through in a switch statement from other languages). 
/// The function's body should mutate `filter`.
pub type ChatFilter<T> = fn(t: Arc<Mutex<T>>, c: V1Client, filter: &mut TextMessage_Filter) -> bool;

/// Function that handles events on the Mumble server authentication stream. Returns a boolean
/// which determines whether or not other functions will be able to process the authentication event
/// it has handled (similar to cases falling through in a switch statement from other languages).
pub type Authenticator<T> = fn(t: Arc<Mutex<T>>, c: V1Client, response: &mut Authenticator_Response, request: &Authenticator_Request) -> bool;

/// Function that handles events on the Mumble server context action stream. Returns a boolean
/// which determines whether or not other functions will be able to process the authentication
/// event it has handled (similar to cases falling through in a switch statement from other languages).
pub type ContextActionHandler<T> = fn(t: Arc<Mutex<T>>, c: V1Client, action: &ContextAction) -> bool;

/// Functicon that gets called in response to the connection to Murmur either opening.
pub type ConnectHandler<T> = fn(t: Arc<Mutex<T>>, c: V1Client) -> bool;

/// Function that gets called in response to the connection to Murmur closing.
pub type DisconnectHandler<T> = fn(t: Arc<Mutex<T>>) -> bool;


#[derive(Clone)]
pub struct MurmurInterface<T> 
where T: Send + Clone,
{
    /// Mutex that holds data in order to allow a persistent state shared.
    pub t: Arc<Mutex<T>>,
    /// The address to connect to.
    pub addr: String,
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
where T: Send + Clone
{
    /// Create a new MurmurInterfaceBuilder provided a data value and an addr to which the gRPC
    /// connection will be made
    pub fn new(t: T, addr: String) -> Self {
        Self {
            i: MurmurInterface::new(t, addr)
        }
    }
    /// Assign the the value that will be used to represent state
    pub fn data(mut self, t: T) -> Self {
        self.i.t = Arc::new(Mutex::new(t));
        self
    }
    /// Alternative method to provide the value for state if it is already wrapped in a
    /// [DataMutex](struct.DataMutex.html)
    pub fn data_mutex(mut self, t: Arc<Mutex<T>>) -> Self {
        self.i.t = t;
        self
    }
    /// Set the address to which the connection will be made
    pub fn addr(mut self, addr: String) -> Self {
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
    pub fn build(&self) -> MurmurInterface<T> {
        self.i.clone()
    }
}


/// Start a connection to a Mumble server using the given [MurmurInterface](struct.MurmurInterface.html).
/// The function's body spawns a new thread and returns its join handle. I reccommend using
/// [MurmurInterfaceBuilder](struct.MurmurInterfaceBuilder.html) to generate MurmurInterfaces.
pub fn start_connection<T>(i: MurmurInterface<T>) -> JoinHandle<()>
where T: Send + Clone + 'static,
{
    thread::spawn(move || {
        loop {
            let i_clone = i.clone();
            start_single(i_clone);
            // Don't iterate more than once if this interface is not configure to
            // auto-reconnect.
            if !i.auto_reconnect { break; }

            thread::sleep(time::Duration::from_secs(RECONNECT_DELAY_SECONDS));
        }
    })
}

fn start_single<T>(i: MurmurInterface<T>)
where T: Send + Clone + 'static,
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
 
    let env = Environment::new(GRPC_COMPLETION_QUEUE_SIZE);
    let builder = ChannelBuilder::new(Arc::new(env));
    let channel = builder.connect(addr.as_ref());
    let c = V1Client::new(channel);

    for connect_handler in server_connected.iter() {
        if !(connect_handler)(t.clone(), c.clone()) {
            break;
        }
    }

    let mut server = Server::new();
    server.set_id(server_id);
    let server = server;

    // SERVER EVENTS
    let server_event_fut = {
        let c = c.clone();
        let t = t.clone();
        let server = server.clone();
        Box::pin(async move {
            if !user_connected.is_empty() || !user_disconnected.is_empty() || !user_state_changed.is_empty() 
                || !user_text_message.is_empty() || !channel_created.is_empty() || !channel_removed.is_empty() 
                    || !channel_state_changed.is_empty()
            {
                let mut event_stream = c.server_events(&server)
                    .expect("Connecting to the event stream");
                loop {
                    if let Some(Ok(event)) = event_stream.next().await {
                        match event.get_field_type() {
                            Server_Event_Type::UserConnected       => handle_event(t.clone(), c.clone(), &user_connected, &event),
                            Server_Event_Type::UserDisconnected    => handle_event(t.clone(), c.clone(), &user_disconnected, &event),
                            Server_Event_Type::UserStateChanged    => handle_event(t.clone(), c.clone(), &user_state_changed, &event),
                            Server_Event_Type::UserTextMessage     => handle_event(t.clone(), c.clone(), &user_text_message, &event),
                            Server_Event_Type::ChannelCreated      => handle_event(t.clone(), c.clone(), &channel_created, &event),
                            Server_Event_Type::ChannelRemoved      => handle_event(t.clone(), c.clone(), &channel_removed, &event),
                            Server_Event_Type::ChannelStateChanged => handle_event(t.clone(), c.clone(), &channel_state_changed, &event),
                        };
                    }
                }
            }
        })
    };


    // CHAT FILTER
    let chat_filter_fut = {
        let c = c.clone();
        let t = t.clone();

        Box::pin(async move {
            if !chat_filters.is_empty() {
                let (mut filter_sender, mut filter_receiver) = c.text_message_filter()
                    .expect("Connecting to filter stream");
                let mut initial_filter = TextMessage_Filter::new();
                initial_filter.set_server(server.clone());
                filter_sender.send((initial_filter, WriteFlags::default())).await
                    .expect("Sending inital filter to open filter stream");
                loop {
                    if let Some(Ok(mut filter)) = filter_receiver.next().await {
                        for chat_filter in chat_filters.iter() {
                            if !(chat_filter)(t.clone(), c.clone(), &mut filter) { break; }
                        }
                        while filter_sender.send((filter.clone(), WriteFlags::default())).await.is_err() {}
                    }
                }
            }
        })
    };


    // AUTHENTICATOR
    let authenticator_fut = {
        let c = c.clone();
        let t = t.clone();

        Box::pin(async move {
            if !authenticators.is_empty() {
                let (mut auth_sender, mut auth_receiver) = c.authenticator_stream()
                    .expect("Connecting to authenticator stream");
                loop {
                    if let Some(Ok(request)) = auth_receiver.next().await {
                        let mut response = Authenticator_Response::new();
                        for authenticator in authenticators.iter() {
                            if !(authenticator)(t.clone(), c.clone(), &mut response, &request) { break; }
                        }
                        while auth_sender.send((response.clone(), WriteFlags::default())).await.is_err() {}
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
            let c = c.clone();
            let t = t.clone();
            Box::pin(async move {
                while c.context_action_add(&action).is_err() {}
                if !handlers.is_empty() {
                    let mut context_action_stream = c.context_action_events(&action)
                        .expect("Connecting to context action stream");
                    loop {
                        if let Some(Ok(context_action)) = context_action_stream.next().await {
                            for handler in handlers.iter() {
                                if !(handler)(t.clone(), c.clone(), &context_action) {
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


fn handle_event<T>(t: Arc<Mutex<T>>, c: V1Client, handlers: &Vec<Handler<T>>, event: &Server_Event) 
where T: Send + Clone
{
    for handler in handlers.iter() {
        if !(handler)(t.clone(), c.clone(), event) {
            return;
        }
    }
}
