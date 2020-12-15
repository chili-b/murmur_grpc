// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_V1_GET_UPTIME: ::grpcio::Method<super::MurmurRPC::Void, super::MurmurRPC::Uptime> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/GetUptime",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_GET_VERSION: ::grpcio::Method<super::MurmurRPC::Void, super::MurmurRPC::Version> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/GetVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_EVENTS: ::grpcio::Method<super::MurmurRPC::Void, super::MurmurRPC::Event> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/MurmurRPC.V1/Events",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_CREATE: ::grpcio::Method<super::MurmurRPC::Void, super::MurmurRPC::Server> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ServerCreate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_QUERY: ::grpcio::Method<super::MurmurRPC::Server_Query, super::MurmurRPC::Server_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ServerQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_GET: ::grpcio::Method<super::MurmurRPC::Server, super::MurmurRPC::Server> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ServerGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_START: ::grpcio::Method<super::MurmurRPC::Server, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ServerStart",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_STOP: ::grpcio::Method<super::MurmurRPC::Server, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ServerStop",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_REMOVE: ::grpcio::Method<super::MurmurRPC::Server, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ServerRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_SERVER_EVENTS: ::grpcio::Method<super::MurmurRPC::Server, super::MurmurRPC::Server_Event> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/MurmurRPC.V1/ServerEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONTEXT_ACTION_ADD: ::grpcio::Method<super::MurmurRPC::ContextAction, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ContextActionAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONTEXT_ACTION_REMOVE: ::grpcio::Method<super::MurmurRPC::ContextAction, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ContextActionRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONTEXT_ACTION_EVENTS: ::grpcio::Method<super::MurmurRPC::ContextAction, super::MurmurRPC::ContextAction> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/MurmurRPC.V1/ContextActionEvents",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_TEXT_MESSAGE_SEND: ::grpcio::Method<super::MurmurRPC::TextMessage, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/TextMessageSend",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_TEXT_MESSAGE_FILTER: ::grpcio::Method<super::MurmurRPC::TextMessage_Filter, super::MurmurRPC::TextMessage_Filter> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/MurmurRPC.V1/TextMessageFilter",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_LOG_QUERY: ::grpcio::Method<super::MurmurRPC::Log_Query, super::MurmurRPC::Log_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/LogQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONFIG_GET: ::grpcio::Method<super::MurmurRPC::Server, super::MurmurRPC::Config> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ConfigGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONFIG_GET_FIELD: ::grpcio::Method<super::MurmurRPC::Config_Field, super::MurmurRPC::Config_Field> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ConfigGetField",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONFIG_SET_FIELD: ::grpcio::Method<super::MurmurRPC::Config_Field, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ConfigSetField",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CONFIG_GET_DEFAULT: ::grpcio::Method<super::MurmurRPC::Void, super::MurmurRPC::Config> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ConfigGetDefault",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CHANNEL_QUERY: ::grpcio::Method<super::MurmurRPC::Channel_Query, super::MurmurRPC::Channel_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ChannelQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CHANNEL_GET: ::grpcio::Method<super::MurmurRPC::Channel, super::MurmurRPC::Channel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ChannelGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CHANNEL_ADD: ::grpcio::Method<super::MurmurRPC::Channel, super::MurmurRPC::Channel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ChannelAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CHANNEL_REMOVE: ::grpcio::Method<super::MurmurRPC::Channel, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ChannelRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_CHANNEL_UPDATE: ::grpcio::Method<super::MurmurRPC::Channel, super::MurmurRPC::Channel> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ChannelUpdate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_USER_QUERY: ::grpcio::Method<super::MurmurRPC::User_Query, super::MurmurRPC::User_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/UserQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_USER_GET: ::grpcio::Method<super::MurmurRPC::User, super::MurmurRPC::User> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/UserGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_USER_UPDATE: ::grpcio::Method<super::MurmurRPC::User, super::MurmurRPC::User> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/UserUpdate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_USER_KICK: ::grpcio::Method<super::MurmurRPC::User_Kick, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/UserKick",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_TREE_QUERY: ::grpcio::Method<super::MurmurRPC::Tree_Query, super::MurmurRPC::Tree> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/TreeQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_BANS_GET: ::grpcio::Method<super::MurmurRPC::Ban_Query, super::MurmurRPC::Ban_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/BansGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_BANS_SET: ::grpcio::Method<super::MurmurRPC::Ban_List, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/BansSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_ACL_GET: ::grpcio::Method<super::MurmurRPC::Channel, super::MurmurRPC::ACL_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ACLGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_ACL_SET: ::grpcio::Method<super::MurmurRPC::ACL_List, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ACLSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_ACL_GET_EFFECTIVE_PERMISSIONS: ::grpcio::Method<super::MurmurRPC::ACL_Query, super::MurmurRPC::ACL> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ACLGetEffectivePermissions",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_ACL_ADD_TEMPORARY_GROUP: ::grpcio::Method<super::MurmurRPC::ACL_TemporaryGroup, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ACLAddTemporaryGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_ACL_REMOVE_TEMPORARY_GROUP: ::grpcio::Method<super::MurmurRPC::ACL_TemporaryGroup, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/ACLRemoveTemporaryGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_AUTHENTICATOR_STREAM: ::grpcio::Method<super::MurmurRPC::Authenticator_Response, super::MurmurRPC::Authenticator_Request> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/MurmurRPC.V1/AuthenticatorStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_DATABASE_USER_QUERY: ::grpcio::Method<super::MurmurRPC::DatabaseUser_Query, super::MurmurRPC::DatabaseUser_List> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/DatabaseUserQuery",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_DATABASE_USER_GET: ::grpcio::Method<super::MurmurRPC::DatabaseUser, super::MurmurRPC::DatabaseUser> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/DatabaseUserGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_DATABASE_USER_UPDATE: ::grpcio::Method<super::MurmurRPC::DatabaseUser, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/DatabaseUserUpdate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_DATABASE_USER_REGISTER: ::grpcio::Method<super::MurmurRPC::DatabaseUser, super::MurmurRPC::DatabaseUser> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/DatabaseUserRegister",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_DATABASE_USER_DEREGISTER: ::grpcio::Method<super::MurmurRPC::DatabaseUser, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/DatabaseUserDeregister",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_DATABASE_USER_VERIFY: ::grpcio::Method<super::MurmurRPC::DatabaseUser_Verify, super::MurmurRPC::DatabaseUser> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/DatabaseUserVerify",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_REDIRECT_WHISPER_GROUP_ADD: ::grpcio::Method<super::MurmurRPC::RedirectWhisperGroup, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/RedirectWhisperGroupAdd",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_V1_REDIRECT_WHISPER_GROUP_REMOVE: ::grpcio::Method<super::MurmurRPC::RedirectWhisperGroup, super::MurmurRPC::Void> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/MurmurRPC.V1/RedirectWhisperGroupRemove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct V1Client {
    client: ::grpcio::Client,
}

impl V1Client {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        V1Client {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_uptime_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Uptime> {
        self.client.unary_call(&METHOD_V1_GET_UPTIME, req, opt)
    }

    pub fn get_uptime(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<super::MurmurRPC::Uptime> {
        self.get_uptime_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_uptime_async_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Uptime>> {
        self.client.unary_call_async(&METHOD_V1_GET_UPTIME, req, opt)
    }

    pub fn get_uptime_async(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Uptime>> {
        self.get_uptime_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_version_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Version> {
        self.client.unary_call(&METHOD_V1_GET_VERSION, req, opt)
    }

    pub fn get_version(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<super::MurmurRPC::Version> {
        self.get_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_version_async_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Version>> {
        self.client.unary_call_async(&METHOD_V1_GET_VERSION, req, opt)
    }

    pub fn get_version_async(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Version>> {
        self.get_version_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn events_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::MurmurRPC::Event>> {
        self.client.server_streaming(&METHOD_V1_EVENTS, req, opt)
    }

    pub fn events(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::MurmurRPC::Event>> {
        self.events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_create_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Server> {
        self.client.unary_call(&METHOD_V1_SERVER_CREATE, req, opt)
    }

    pub fn server_create(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<super::MurmurRPC::Server> {
        self.server_create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_create_async_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Server>> {
        self.client.unary_call_async(&METHOD_V1_SERVER_CREATE, req, opt)
    }

    pub fn server_create_async(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Server>> {
        self.server_create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_query_opt(&self, req: &super::MurmurRPC::Server_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Server_List> {
        self.client.unary_call(&METHOD_V1_SERVER_QUERY, req, opt)
    }

    pub fn server_query(&self, req: &super::MurmurRPC::Server_Query) -> ::grpcio::Result<super::MurmurRPC::Server_List> {
        self.server_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_query_async_opt(&self, req: &super::MurmurRPC::Server_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Server_List>> {
        self.client.unary_call_async(&METHOD_V1_SERVER_QUERY, req, opt)
    }

    pub fn server_query_async(&self, req: &super::MurmurRPC::Server_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Server_List>> {
        self.server_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_get_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Server> {
        self.client.unary_call(&METHOD_V1_SERVER_GET, req, opt)
    }

    pub fn server_get(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<super::MurmurRPC::Server> {
        self.server_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_get_async_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Server>> {
        self.client.unary_call_async(&METHOD_V1_SERVER_GET, req, opt)
    }

    pub fn server_get_async(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Server>> {
        self.server_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_start_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_SERVER_START, req, opt)
    }

    pub fn server_start(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.server_start_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_start_async_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_SERVER_START, req, opt)
    }

    pub fn server_start_async(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.server_start_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_stop_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_SERVER_STOP, req, opt)
    }

    pub fn server_stop(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.server_stop_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_stop_async_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_SERVER_STOP, req, opt)
    }

    pub fn server_stop_async(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.server_stop_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_remove_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_SERVER_REMOVE, req, opt)
    }

    pub fn server_remove(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.server_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_remove_async_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_SERVER_REMOVE, req, opt)
    }

    pub fn server_remove_async(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.server_remove_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn server_events_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::MurmurRPC::Server_Event>> {
        self.client.server_streaming(&METHOD_V1_SERVER_EVENTS, req, opt)
    }

    pub fn server_events(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::MurmurRPC::Server_Event>> {
        self.server_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn context_action_add_opt(&self, req: &super::MurmurRPC::ContextAction, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_CONTEXT_ACTION_ADD, req, opt)
    }

    pub fn context_action_add(&self, req: &super::MurmurRPC::ContextAction) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.context_action_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn context_action_add_async_opt(&self, req: &super::MurmurRPC::ContextAction, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_CONTEXT_ACTION_ADD, req, opt)
    }

    pub fn context_action_add_async(&self, req: &super::MurmurRPC::ContextAction) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.context_action_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn context_action_remove_opt(&self, req: &super::MurmurRPC::ContextAction, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_CONTEXT_ACTION_REMOVE, req, opt)
    }

    pub fn context_action_remove(&self, req: &super::MurmurRPC::ContextAction) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.context_action_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn context_action_remove_async_opt(&self, req: &super::MurmurRPC::ContextAction, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_CONTEXT_ACTION_REMOVE, req, opt)
    }

    pub fn context_action_remove_async(&self, req: &super::MurmurRPC::ContextAction) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.context_action_remove_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn context_action_events_opt(&self, req: &super::MurmurRPC::ContextAction, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::MurmurRPC::ContextAction>> {
        self.client.server_streaming(&METHOD_V1_CONTEXT_ACTION_EVENTS, req, opt)
    }

    pub fn context_action_events(&self, req: &super::MurmurRPC::ContextAction) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::MurmurRPC::ContextAction>> {
        self.context_action_events_opt(req, ::grpcio::CallOption::default())
    }

    pub fn text_message_send_opt(&self, req: &super::MurmurRPC::TextMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_TEXT_MESSAGE_SEND, req, opt)
    }

    pub fn text_message_send(&self, req: &super::MurmurRPC::TextMessage) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.text_message_send_opt(req, ::grpcio::CallOption::default())
    }

    pub fn text_message_send_async_opt(&self, req: &super::MurmurRPC::TextMessage, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_TEXT_MESSAGE_SEND, req, opt)
    }

    pub fn text_message_send_async(&self, req: &super::MurmurRPC::TextMessage) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.text_message_send_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn text_message_filter_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::MurmurRPC::TextMessage_Filter>, ::grpcio::ClientDuplexReceiver<super::MurmurRPC::TextMessage_Filter>)> {
        self.client.duplex_streaming(&METHOD_V1_TEXT_MESSAGE_FILTER, opt)
    }

    pub fn text_message_filter(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::MurmurRPC::TextMessage_Filter>, ::grpcio::ClientDuplexReceiver<super::MurmurRPC::TextMessage_Filter>)> {
        self.text_message_filter_opt(::grpcio::CallOption::default())
    }

    pub fn log_query_opt(&self, req: &super::MurmurRPC::Log_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Log_List> {
        self.client.unary_call(&METHOD_V1_LOG_QUERY, req, opt)
    }

    pub fn log_query(&self, req: &super::MurmurRPC::Log_Query) -> ::grpcio::Result<super::MurmurRPC::Log_List> {
        self.log_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn log_query_async_opt(&self, req: &super::MurmurRPC::Log_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Log_List>> {
        self.client.unary_call_async(&METHOD_V1_LOG_QUERY, req, opt)
    }

    pub fn log_query_async(&self, req: &super::MurmurRPC::Log_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Log_List>> {
        self.log_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_get_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Config> {
        self.client.unary_call(&METHOD_V1_CONFIG_GET, req, opt)
    }

    pub fn config_get(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<super::MurmurRPC::Config> {
        self.config_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_get_async_opt(&self, req: &super::MurmurRPC::Server, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Config>> {
        self.client.unary_call_async(&METHOD_V1_CONFIG_GET, req, opt)
    }

    pub fn config_get_async(&self, req: &super::MurmurRPC::Server) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Config>> {
        self.config_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_get_field_opt(&self, req: &super::MurmurRPC::Config_Field, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Config_Field> {
        self.client.unary_call(&METHOD_V1_CONFIG_GET_FIELD, req, opt)
    }

    pub fn config_get_field(&self, req: &super::MurmurRPC::Config_Field) -> ::grpcio::Result<super::MurmurRPC::Config_Field> {
        self.config_get_field_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_get_field_async_opt(&self, req: &super::MurmurRPC::Config_Field, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Config_Field>> {
        self.client.unary_call_async(&METHOD_V1_CONFIG_GET_FIELD, req, opt)
    }

    pub fn config_get_field_async(&self, req: &super::MurmurRPC::Config_Field) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Config_Field>> {
        self.config_get_field_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_set_field_opt(&self, req: &super::MurmurRPC::Config_Field, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_CONFIG_SET_FIELD, req, opt)
    }

    pub fn config_set_field(&self, req: &super::MurmurRPC::Config_Field) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.config_set_field_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_set_field_async_opt(&self, req: &super::MurmurRPC::Config_Field, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_CONFIG_SET_FIELD, req, opt)
    }

    pub fn config_set_field_async(&self, req: &super::MurmurRPC::Config_Field) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.config_set_field_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_get_default_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Config> {
        self.client.unary_call(&METHOD_V1_CONFIG_GET_DEFAULT, req, opt)
    }

    pub fn config_get_default(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<super::MurmurRPC::Config> {
        self.config_get_default_opt(req, ::grpcio::CallOption::default())
    }

    pub fn config_get_default_async_opt(&self, req: &super::MurmurRPC::Void, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Config>> {
        self.client.unary_call_async(&METHOD_V1_CONFIG_GET_DEFAULT, req, opt)
    }

    pub fn config_get_default_async(&self, req: &super::MurmurRPC::Void) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Config>> {
        self.config_get_default_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_query_opt(&self, req: &super::MurmurRPC::Channel_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Channel_List> {
        self.client.unary_call(&METHOD_V1_CHANNEL_QUERY, req, opt)
    }

    pub fn channel_query(&self, req: &super::MurmurRPC::Channel_Query) -> ::grpcio::Result<super::MurmurRPC::Channel_List> {
        self.channel_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_query_async_opt(&self, req: &super::MurmurRPC::Channel_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel_List>> {
        self.client.unary_call_async(&METHOD_V1_CHANNEL_QUERY, req, opt)
    }

    pub fn channel_query_async(&self, req: &super::MurmurRPC::Channel_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel_List>> {
        self.channel_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_get_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Channel> {
        self.client.unary_call(&METHOD_V1_CHANNEL_GET, req, opt)
    }

    pub fn channel_get(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<super::MurmurRPC::Channel> {
        self.channel_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_get_async_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel>> {
        self.client.unary_call_async(&METHOD_V1_CHANNEL_GET, req, opt)
    }

    pub fn channel_get_async(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel>> {
        self.channel_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_add_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Channel> {
        self.client.unary_call(&METHOD_V1_CHANNEL_ADD, req, opt)
    }

    pub fn channel_add(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<super::MurmurRPC::Channel> {
        self.channel_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_add_async_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel>> {
        self.client.unary_call_async(&METHOD_V1_CHANNEL_ADD, req, opt)
    }

    pub fn channel_add_async(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel>> {
        self.channel_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_remove_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_CHANNEL_REMOVE, req, opt)
    }

    pub fn channel_remove(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.channel_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_remove_async_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_CHANNEL_REMOVE, req, opt)
    }

    pub fn channel_remove_async(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.channel_remove_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_update_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Channel> {
        self.client.unary_call(&METHOD_V1_CHANNEL_UPDATE, req, opt)
    }

    pub fn channel_update(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<super::MurmurRPC::Channel> {
        self.channel_update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn channel_update_async_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel>> {
        self.client.unary_call_async(&METHOD_V1_CHANNEL_UPDATE, req, opt)
    }

    pub fn channel_update_async(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Channel>> {
        self.channel_update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_query_opt(&self, req: &super::MurmurRPC::User_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::User_List> {
        self.client.unary_call(&METHOD_V1_USER_QUERY, req, opt)
    }

    pub fn user_query(&self, req: &super::MurmurRPC::User_Query) -> ::grpcio::Result<super::MurmurRPC::User_List> {
        self.user_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_query_async_opt(&self, req: &super::MurmurRPC::User_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::User_List>> {
        self.client.unary_call_async(&METHOD_V1_USER_QUERY, req, opt)
    }

    pub fn user_query_async(&self, req: &super::MurmurRPC::User_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::User_List>> {
        self.user_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_get_opt(&self, req: &super::MurmurRPC::User, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::User> {
        self.client.unary_call(&METHOD_V1_USER_GET, req, opt)
    }

    pub fn user_get(&self, req: &super::MurmurRPC::User) -> ::grpcio::Result<super::MurmurRPC::User> {
        self.user_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_get_async_opt(&self, req: &super::MurmurRPC::User, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::User>> {
        self.client.unary_call_async(&METHOD_V1_USER_GET, req, opt)
    }

    pub fn user_get_async(&self, req: &super::MurmurRPC::User) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::User>> {
        self.user_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_update_opt(&self, req: &super::MurmurRPC::User, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::User> {
        self.client.unary_call(&METHOD_V1_USER_UPDATE, req, opt)
    }

    pub fn user_update(&self, req: &super::MurmurRPC::User) -> ::grpcio::Result<super::MurmurRPC::User> {
        self.user_update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_update_async_opt(&self, req: &super::MurmurRPC::User, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::User>> {
        self.client.unary_call_async(&METHOD_V1_USER_UPDATE, req, opt)
    }

    pub fn user_update_async(&self, req: &super::MurmurRPC::User) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::User>> {
        self.user_update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_kick_opt(&self, req: &super::MurmurRPC::User_Kick, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_USER_KICK, req, opt)
    }

    pub fn user_kick(&self, req: &super::MurmurRPC::User_Kick) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.user_kick_opt(req, ::grpcio::CallOption::default())
    }

    pub fn user_kick_async_opt(&self, req: &super::MurmurRPC::User_Kick, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_USER_KICK, req, opt)
    }

    pub fn user_kick_async(&self, req: &super::MurmurRPC::User_Kick) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.user_kick_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn tree_query_opt(&self, req: &super::MurmurRPC::Tree_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Tree> {
        self.client.unary_call(&METHOD_V1_TREE_QUERY, req, opt)
    }

    pub fn tree_query(&self, req: &super::MurmurRPC::Tree_Query) -> ::grpcio::Result<super::MurmurRPC::Tree> {
        self.tree_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn tree_query_async_opt(&self, req: &super::MurmurRPC::Tree_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Tree>> {
        self.client.unary_call_async(&METHOD_V1_TREE_QUERY, req, opt)
    }

    pub fn tree_query_async(&self, req: &super::MurmurRPC::Tree_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Tree>> {
        self.tree_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bans_get_opt(&self, req: &super::MurmurRPC::Ban_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Ban_List> {
        self.client.unary_call(&METHOD_V1_BANS_GET, req, opt)
    }

    pub fn bans_get(&self, req: &super::MurmurRPC::Ban_Query) -> ::grpcio::Result<super::MurmurRPC::Ban_List> {
        self.bans_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bans_get_async_opt(&self, req: &super::MurmurRPC::Ban_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Ban_List>> {
        self.client.unary_call_async(&METHOD_V1_BANS_GET, req, opt)
    }

    pub fn bans_get_async(&self, req: &super::MurmurRPC::Ban_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Ban_List>> {
        self.bans_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bans_set_opt(&self, req: &super::MurmurRPC::Ban_List, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_BANS_SET, req, opt)
    }

    pub fn bans_set(&self, req: &super::MurmurRPC::Ban_List) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.bans_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn bans_set_async_opt(&self, req: &super::MurmurRPC::Ban_List, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_BANS_SET, req, opt)
    }

    pub fn bans_set_async(&self, req: &super::MurmurRPC::Ban_List) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.bans_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_get_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::ACL_List> {
        self.client.unary_call(&METHOD_V1_ACL_GET, req, opt)
    }

    pub fn acl_get(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<super::MurmurRPC::ACL_List> {
        self.acl_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_get_async_opt(&self, req: &super::MurmurRPC::Channel, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::ACL_List>> {
        self.client.unary_call_async(&METHOD_V1_ACL_GET, req, opt)
    }

    pub fn acl_get_async(&self, req: &super::MurmurRPC::Channel) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::ACL_List>> {
        self.acl_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_set_opt(&self, req: &super::MurmurRPC::ACL_List, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_ACL_SET, req, opt)
    }

    pub fn acl_set(&self, req: &super::MurmurRPC::ACL_List) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.acl_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_set_async_opt(&self, req: &super::MurmurRPC::ACL_List, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_ACL_SET, req, opt)
    }

    pub fn acl_set_async(&self, req: &super::MurmurRPC::ACL_List) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.acl_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_get_effective_permissions_opt(&self, req: &super::MurmurRPC::ACL_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::ACL> {
        self.client.unary_call(&METHOD_V1_ACL_GET_EFFECTIVE_PERMISSIONS, req, opt)
    }

    pub fn acl_get_effective_permissions(&self, req: &super::MurmurRPC::ACL_Query) -> ::grpcio::Result<super::MurmurRPC::ACL> {
        self.acl_get_effective_permissions_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_get_effective_permissions_async_opt(&self, req: &super::MurmurRPC::ACL_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::ACL>> {
        self.client.unary_call_async(&METHOD_V1_ACL_GET_EFFECTIVE_PERMISSIONS, req, opt)
    }

    pub fn acl_get_effective_permissions_async(&self, req: &super::MurmurRPC::ACL_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::ACL>> {
        self.acl_get_effective_permissions_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_add_temporary_group_opt(&self, req: &super::MurmurRPC::ACL_TemporaryGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_ACL_ADD_TEMPORARY_GROUP, req, opt)
    }

    pub fn acl_add_temporary_group(&self, req: &super::MurmurRPC::ACL_TemporaryGroup) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.acl_add_temporary_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_add_temporary_group_async_opt(&self, req: &super::MurmurRPC::ACL_TemporaryGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_ACL_ADD_TEMPORARY_GROUP, req, opt)
    }

    pub fn acl_add_temporary_group_async(&self, req: &super::MurmurRPC::ACL_TemporaryGroup) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.acl_add_temporary_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_remove_temporary_group_opt(&self, req: &super::MurmurRPC::ACL_TemporaryGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_ACL_REMOVE_TEMPORARY_GROUP, req, opt)
    }

    pub fn acl_remove_temporary_group(&self, req: &super::MurmurRPC::ACL_TemporaryGroup) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.acl_remove_temporary_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acl_remove_temporary_group_async_opt(&self, req: &super::MurmurRPC::ACL_TemporaryGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_ACL_REMOVE_TEMPORARY_GROUP, req, opt)
    }

    pub fn acl_remove_temporary_group_async(&self, req: &super::MurmurRPC::ACL_TemporaryGroup) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.acl_remove_temporary_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn authenticator_stream_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::MurmurRPC::Authenticator_Response>, ::grpcio::ClientDuplexReceiver<super::MurmurRPC::Authenticator_Request>)> {
        self.client.duplex_streaming(&METHOD_V1_AUTHENTICATOR_STREAM, opt)
    }

    pub fn authenticator_stream(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::MurmurRPC::Authenticator_Response>, ::grpcio::ClientDuplexReceiver<super::MurmurRPC::Authenticator_Request>)> {
        self.authenticator_stream_opt(::grpcio::CallOption::default())
    }

    pub fn database_user_query_opt(&self, req: &super::MurmurRPC::DatabaseUser_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser_List> {
        self.client.unary_call(&METHOD_V1_DATABASE_USER_QUERY, req, opt)
    }

    pub fn database_user_query(&self, req: &super::MurmurRPC::DatabaseUser_Query) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser_List> {
        self.database_user_query_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_query_async_opt(&self, req: &super::MurmurRPC::DatabaseUser_Query, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser_List>> {
        self.client.unary_call_async(&METHOD_V1_DATABASE_USER_QUERY, req, opt)
    }

    pub fn database_user_query_async(&self, req: &super::MurmurRPC::DatabaseUser_Query) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser_List>> {
        self.database_user_query_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_get_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser> {
        self.client.unary_call(&METHOD_V1_DATABASE_USER_GET, req, opt)
    }

    pub fn database_user_get(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser> {
        self.database_user_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_get_async_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser>> {
        self.client.unary_call_async(&METHOD_V1_DATABASE_USER_GET, req, opt)
    }

    pub fn database_user_get_async(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser>> {
        self.database_user_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_update_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_DATABASE_USER_UPDATE, req, opt)
    }

    pub fn database_user_update(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.database_user_update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_update_async_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_DATABASE_USER_UPDATE, req, opt)
    }

    pub fn database_user_update_async(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.database_user_update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_register_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser> {
        self.client.unary_call(&METHOD_V1_DATABASE_USER_REGISTER, req, opt)
    }

    pub fn database_user_register(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser> {
        self.database_user_register_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_register_async_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser>> {
        self.client.unary_call_async(&METHOD_V1_DATABASE_USER_REGISTER, req, opt)
    }

    pub fn database_user_register_async(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser>> {
        self.database_user_register_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_deregister_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_DATABASE_USER_DEREGISTER, req, opt)
    }

    pub fn database_user_deregister(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.database_user_deregister_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_deregister_async_opt(&self, req: &super::MurmurRPC::DatabaseUser, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_DATABASE_USER_DEREGISTER, req, opt)
    }

    pub fn database_user_deregister_async(&self, req: &super::MurmurRPC::DatabaseUser) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.database_user_deregister_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_verify_opt(&self, req: &super::MurmurRPC::DatabaseUser_Verify, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser> {
        self.client.unary_call(&METHOD_V1_DATABASE_USER_VERIFY, req, opt)
    }

    pub fn database_user_verify(&self, req: &super::MurmurRPC::DatabaseUser_Verify) -> ::grpcio::Result<super::MurmurRPC::DatabaseUser> {
        self.database_user_verify_opt(req, ::grpcio::CallOption::default())
    }

    pub fn database_user_verify_async_opt(&self, req: &super::MurmurRPC::DatabaseUser_Verify, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser>> {
        self.client.unary_call_async(&METHOD_V1_DATABASE_USER_VERIFY, req, opt)
    }

    pub fn database_user_verify_async(&self, req: &super::MurmurRPC::DatabaseUser_Verify) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::DatabaseUser>> {
        self.database_user_verify_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn redirect_whisper_group_add_opt(&self, req: &super::MurmurRPC::RedirectWhisperGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_REDIRECT_WHISPER_GROUP_ADD, req, opt)
    }

    pub fn redirect_whisper_group_add(&self, req: &super::MurmurRPC::RedirectWhisperGroup) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.redirect_whisper_group_add_opt(req, ::grpcio::CallOption::default())
    }

    pub fn redirect_whisper_group_add_async_opt(&self, req: &super::MurmurRPC::RedirectWhisperGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_REDIRECT_WHISPER_GROUP_ADD, req, opt)
    }

    pub fn redirect_whisper_group_add_async(&self, req: &super::MurmurRPC::RedirectWhisperGroup) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.redirect_whisper_group_add_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn redirect_whisper_group_remove_opt(&self, req: &super::MurmurRPC::RedirectWhisperGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.client.unary_call(&METHOD_V1_REDIRECT_WHISPER_GROUP_REMOVE, req, opt)
    }

    pub fn redirect_whisper_group_remove(&self, req: &super::MurmurRPC::RedirectWhisperGroup) -> ::grpcio::Result<super::MurmurRPC::Void> {
        self.redirect_whisper_group_remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn redirect_whisper_group_remove_async_opt(&self, req: &super::MurmurRPC::RedirectWhisperGroup, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.client.unary_call_async(&METHOD_V1_REDIRECT_WHISPER_GROUP_REMOVE, req, opt)
    }

    pub fn redirect_whisper_group_remove_async(&self, req: &super::MurmurRPC::RedirectWhisperGroup) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::MurmurRPC::Void>> {
        self.redirect_whisper_group_remove_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait V1 {
    fn get_uptime(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Void, sink: ::grpcio::UnarySink<super::MurmurRPC::Uptime>);
    fn get_version(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Void, sink: ::grpcio::UnarySink<super::MurmurRPC::Version>);
    fn events(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Void, sink: ::grpcio::ServerStreamingSink<super::MurmurRPC::Event>);
    fn server_create(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Void, sink: ::grpcio::UnarySink<super::MurmurRPC::Server>);
    fn server_query(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::Server_List>);
    fn server_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server, sink: ::grpcio::UnarySink<super::MurmurRPC::Server>);
    fn server_start(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn server_stop(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn server_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn server_events(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server, sink: ::grpcio::ServerStreamingSink<super::MurmurRPC::Server_Event>);
    fn context_action_add(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ContextAction, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn context_action_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ContextAction, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn context_action_events(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ContextAction, sink: ::grpcio::ServerStreamingSink<super::MurmurRPC::ContextAction>);
    fn text_message_send(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::TextMessage, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn text_message_filter(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::MurmurRPC::TextMessage_Filter>, sink: ::grpcio::DuplexSink<super::MurmurRPC::TextMessage_Filter>);
    fn log_query(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Log_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::Log_List>);
    fn config_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Server, sink: ::grpcio::UnarySink<super::MurmurRPC::Config>);
    fn config_get_field(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Config_Field, sink: ::grpcio::UnarySink<super::MurmurRPC::Config_Field>);
    fn config_set_field(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Config_Field, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn config_get_default(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Void, sink: ::grpcio::UnarySink<super::MurmurRPC::Config>);
    fn channel_query(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Channel_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::Channel_List>);
    fn channel_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Channel, sink: ::grpcio::UnarySink<super::MurmurRPC::Channel>);
    fn channel_add(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Channel, sink: ::grpcio::UnarySink<super::MurmurRPC::Channel>);
    fn channel_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Channel, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn channel_update(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Channel, sink: ::grpcio::UnarySink<super::MurmurRPC::Channel>);
    fn user_query(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::User_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::User_List>);
    fn user_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::User, sink: ::grpcio::UnarySink<super::MurmurRPC::User>);
    fn user_update(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::User, sink: ::grpcio::UnarySink<super::MurmurRPC::User>);
    fn user_kick(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::User_Kick, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn tree_query(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Tree_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::Tree>);
    fn bans_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Ban_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::Ban_List>);
    fn bans_set(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Ban_List, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn acl_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::Channel, sink: ::grpcio::UnarySink<super::MurmurRPC::ACL_List>);
    fn acl_set(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ACL_List, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn acl_get_effective_permissions(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ACL_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::ACL>);
    fn acl_add_temporary_group(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ACL_TemporaryGroup, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn acl_remove_temporary_group(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::ACL_TemporaryGroup, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn authenticator_stream(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::MurmurRPC::Authenticator_Response>, sink: ::grpcio::DuplexSink<super::MurmurRPC::Authenticator_Request>);
    fn database_user_query(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::DatabaseUser_Query, sink: ::grpcio::UnarySink<super::MurmurRPC::DatabaseUser_List>);
    fn database_user_get(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::DatabaseUser, sink: ::grpcio::UnarySink<super::MurmurRPC::DatabaseUser>);
    fn database_user_update(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::DatabaseUser, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn database_user_register(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::DatabaseUser, sink: ::grpcio::UnarySink<super::MurmurRPC::DatabaseUser>);
    fn database_user_deregister(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::DatabaseUser, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn database_user_verify(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::DatabaseUser_Verify, sink: ::grpcio::UnarySink<super::MurmurRPC::DatabaseUser>);
    fn redirect_whisper_group_add(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::RedirectWhisperGroup, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
    fn redirect_whisper_group_remove(&mut self, ctx: ::grpcio::RpcContext, req: super::MurmurRPC::RedirectWhisperGroup, sink: ::grpcio::UnarySink<super::MurmurRPC::Void>);
}

pub fn create_v1<S: V1 + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_GET_UPTIME, move |ctx, req, resp| {
        instance.get_uptime(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_GET_VERSION, move |ctx, req, resp| {
        instance.get_version(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_V1_EVENTS, move |ctx, req, resp| {
        instance.events(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_SERVER_CREATE, move |ctx, req, resp| {
        instance.server_create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_SERVER_QUERY, move |ctx, req, resp| {
        instance.server_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_SERVER_GET, move |ctx, req, resp| {
        instance.server_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_SERVER_START, move |ctx, req, resp| {
        instance.server_start(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_SERVER_STOP, move |ctx, req, resp| {
        instance.server_stop(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_SERVER_REMOVE, move |ctx, req, resp| {
        instance.server_remove(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_V1_SERVER_EVENTS, move |ctx, req, resp| {
        instance.server_events(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CONTEXT_ACTION_ADD, move |ctx, req, resp| {
        instance.context_action_add(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CONTEXT_ACTION_REMOVE, move |ctx, req, resp| {
        instance.context_action_remove(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_V1_CONTEXT_ACTION_EVENTS, move |ctx, req, resp| {
        instance.context_action_events(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_TEXT_MESSAGE_SEND, move |ctx, req, resp| {
        instance.text_message_send(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_V1_TEXT_MESSAGE_FILTER, move |ctx, req, resp| {
        instance.text_message_filter(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_LOG_QUERY, move |ctx, req, resp| {
        instance.log_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CONFIG_GET, move |ctx, req, resp| {
        instance.config_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CONFIG_GET_FIELD, move |ctx, req, resp| {
        instance.config_get_field(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CONFIG_SET_FIELD, move |ctx, req, resp| {
        instance.config_set_field(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CONFIG_GET_DEFAULT, move |ctx, req, resp| {
        instance.config_get_default(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CHANNEL_QUERY, move |ctx, req, resp| {
        instance.channel_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CHANNEL_GET, move |ctx, req, resp| {
        instance.channel_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CHANNEL_ADD, move |ctx, req, resp| {
        instance.channel_add(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CHANNEL_REMOVE, move |ctx, req, resp| {
        instance.channel_remove(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_CHANNEL_UPDATE, move |ctx, req, resp| {
        instance.channel_update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_USER_QUERY, move |ctx, req, resp| {
        instance.user_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_USER_GET, move |ctx, req, resp| {
        instance.user_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_USER_UPDATE, move |ctx, req, resp| {
        instance.user_update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_USER_KICK, move |ctx, req, resp| {
        instance.user_kick(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_TREE_QUERY, move |ctx, req, resp| {
        instance.tree_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_BANS_GET, move |ctx, req, resp| {
        instance.bans_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_BANS_SET, move |ctx, req, resp| {
        instance.bans_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_ACL_GET, move |ctx, req, resp| {
        instance.acl_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_ACL_SET, move |ctx, req, resp| {
        instance.acl_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_ACL_GET_EFFECTIVE_PERMISSIONS, move |ctx, req, resp| {
        instance.acl_get_effective_permissions(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_ACL_ADD_TEMPORARY_GROUP, move |ctx, req, resp| {
        instance.acl_add_temporary_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_ACL_REMOVE_TEMPORARY_GROUP, move |ctx, req, resp| {
        instance.acl_remove_temporary_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_V1_AUTHENTICATOR_STREAM, move |ctx, req, resp| {
        instance.authenticator_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_DATABASE_USER_QUERY, move |ctx, req, resp| {
        instance.database_user_query(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_DATABASE_USER_GET, move |ctx, req, resp| {
        instance.database_user_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_DATABASE_USER_UPDATE, move |ctx, req, resp| {
        instance.database_user_update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_DATABASE_USER_REGISTER, move |ctx, req, resp| {
        instance.database_user_register(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_DATABASE_USER_DEREGISTER, move |ctx, req, resp| {
        instance.database_user_deregister(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_DATABASE_USER_VERIFY, move |ctx, req, resp| {
        instance.database_user_verify(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_V1_REDIRECT_WHISPER_GROUP_ADD, move |ctx, req, resp| {
        instance.redirect_whisper_group_add(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_V1_REDIRECT_WHISPER_GROUP_REMOVE, move |ctx, req, resp| {
        instance.redirect_whisper_group_remove(ctx, req, resp)
    });
    builder.build()
}
