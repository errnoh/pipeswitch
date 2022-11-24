use pipewire::{
    keys::*,
    registry::GlobalObject,
    spa::{ForeignDict, ReadableDict},
    types::ObjectType,
};

use crate::PipewireError;

pub const VERSION: u32 = 3;

type PwIdType = u32;

#[derive(Debug)]
pub struct Port {
    pub id: PwIdType,
    pub port_id: PwIdType,
    pub path: Option<String>,
    pub node_id: PwIdType,
    pub dsp: Option<String>,
    pub channel: Option<String>,
    pub name: String,
    pub direction: String,
    pub alias: String,
    pub physical: Option<bool>,
    pub terminal: Option<bool>,
}

impl Port {
    pub fn from_global(global: &GlobalObject<ForeignDict>) -> Result<Self, PipewireError> {
        let props = global
            .props
            .as_ref()
            .ok_or(PipewireError::MissingProps(global.id))?;
        let get_prop = |property| props.get(property).map(|v| v.to_string());
        let get_prop_or =
            |property| get_prop(property).ok_or(PipewireError::PropNotFound("Port", property));
        Ok(Port {
            id: global.id,
            port_id: get_prop_or(*PORT_ID)?.parse()?,
            path: get_prop(*OBJECT_PATH),
            node_id: get_prop_or(*NODE_ID)?.parse()?,
            dsp: get_prop(*FORMAT_DSP),
            channel: get_prop(*AUDIO_CHANNEL),
            name: get_prop_or(*PORT_NAME)?,
            direction: get_prop_or(*PORT_DIRECTION)?,
            alias: get_prop_or(*PORT_ALIAS)?,
            physical: get_prop(*PORT_PHYSICAL).map(|v| v.parse()).transpose()?,
            terminal: get_prop(*PORT_TERMINAL).map(|v| v.parse()).transpose()?,
        })
    }
}

#[derive(Debug)]
pub struct Node {
    pub id: PwIdType,
    pub path: Option<String>,
    pub factory_id: PwIdType,
    pub client_id: PwIdType,
    pub device_id: Option<PwIdType>,
    pub application_name: Option<String>,
    pub node_description: Option<String>,
    pub node_name: String,
    pub node_nick: Option<String>,
    pub media_class: String,
    pub media_role: Option<String>,
}

impl Node {
    pub fn from_global(global: &GlobalObject<ForeignDict>) -> Result<Self, PipewireError> {
        let props = global
            .props
            .as_ref()
            .ok_or(PipewireError::MissingProps(global.id))?;
        let get_prop = |property| props.get(property).map(|v| v.to_string());
        let get_prop_or =
            |property| get_prop(property).ok_or(PipewireError::PropNotFound("Node", property));

        Ok(Node {
            id: global.id,
            path: get_prop(*OBJECT_PATH),
            factory_id: get_prop_or(*FACTORY_ID)?.parse()?,
            client_id: get_prop_or(*CLIENT_ID)?.parse()?,
            device_id: get_prop(*DEVICE_ID).map(|v| v.parse()).transpose()?,
            application_name: get_prop(*APP_NAME),
            node_description: get_prop(*NODE_DESCRIPTION),
            node_name: get_prop_or(*NODE_NAME)?,
            node_nick: get_prop(*NODE_NICK),
            media_class: get_prop_or(*MEDIA_CLASS)?,
            media_role: get_prop(*MEDIA_ROLE),
        })
    }
}

#[derive(Debug)]
pub struct Link {
    pub id: PwIdType,
    pub factory_id: PwIdType,
    pub client_id: PwIdType,
    pub output_node: PwIdType,
    pub output_port: PwIdType,
    pub input_node: PwIdType,
    pub input_port: PwIdType,
}

impl Link {
    pub fn from_global(global: &GlobalObject<ForeignDict>) -> Result<Self, PipewireError> {
        let props = global
            .props
            .as_ref()
            .ok_or(PipewireError::MissingProps(global.id))?;
        let get_prop = |property| props.get(property).map(|v| v.to_string());
        let get_prop_or =
            |property| get_prop(property).ok_or(PipewireError::PropNotFound("Node", property));

        Ok(Link {
            id: global.id,
            factory_id: get_prop_or(*FACTORY_ID)?.parse()?,
            client_id: get_prop_or(*CLIENT_ID)?.parse()?,
            output_node: get_prop_or(*LINK_OUTPUT_NODE)?.parse()?,
            output_port: get_prop_or(*LINK_OUTPUT_PORT)?.parse()?,
            input_node: get_prop_or(*LINK_INPUT_NODE)?.parse()?,
            input_port: get_prop_or(*LINK_INPUT_PORT)?.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Client {
    pub id: PwIdType,
    pub module_id: PwIdType,
    pub protocol: String,
    pub pid: PwIdType,
    pub uid: PwIdType,
    pub gid: PwIdType,
    pub label: String,
    pub application_name: String,
}

impl Client {
    pub fn from_global(global: &GlobalObject<ForeignDict>) -> Result<Self, PipewireError> {
        let props = global
            .props
            .as_ref()
            .ok_or(PipewireError::MissingProps(global.id))?;
        let get_prop = |property| props.get(property).map(|v| v.to_string());
        let get_prop_or =
            |property| get_prop(property).ok_or(PipewireError::PropNotFound("Node", property));

        Ok(Client {
            id: global.id,
            module_id: get_prop_or(*MODULE_ID)?.parse()?,
            protocol: get_prop_or(*PROTOCOL)?,
            pid: get_prop_or(*SEC_PID)?.parse()?,
            uid: get_prop_or(*SEC_UID)?.parse()?,
            gid: get_prop_or(*SEC_GID)?.parse()?,
            label: get_prop_or(*SEC_LABEL)?,
            application_name: get_prop_or(*APP_NAME)?,
        })
    }
}

pub(crate) enum PipewireObject {
    Port(Port),
    Node(Node),
    Link(Link),
    Client(Client),
}

impl PipewireObject {
    pub fn from_global(global: &GlobalObject<ForeignDict>) -> Result<Option<Self>, PipewireError> {
        if global.version != VERSION {
            Err(PipewireError::InvalidVersion(global.version))?
        }
        match global.type_ {
            ObjectType::Port => Ok(Some(Self::Port(Port::from_global(global)?))),
            ObjectType::Node => Ok(Some(Self::Node(Node::from_global(global)?))),
            _ => Ok(None),
        }
    }
}
