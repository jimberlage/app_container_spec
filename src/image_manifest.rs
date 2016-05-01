use rustc_serialize::json::Json;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator, Timestamps};
use url::Url;

pub enum Annotation {
    Authors {
        name: ACIdentifier,
        value: String,
    },
    Created {
        name: ACIdentifier,
        value: Timestamps,
    },
    Documentation {
        name: ACIdentifier,
        value: Url,
    },
    Homepage {
        name: ACIdentifier,
        value: Url,
    },
    Normal {
        name: ACIdentifier,
        value: String,
    },
}

pub struct EnvironmentVariableName(String);

pub struct EnvironmentVariable {
    name: EnvironmentVariableName,
    value: String,
}

pub enum EventHandlerName {
    PreStart,
    PostStop,
}

pub struct EventHandler {
    exec: Vec<String>,
    name: EventHandlerName,
}

pub struct MountPoint {
    name: ACName,
    path: String,
    read_only: bool,
}

pub struct Port {
    count: u32,
    name: ACName,
    port: u16,
    protocol: String,
    socket_activated: bool,
}

pub struct App {
    environment: Option<Vec<EnvironmentVariable>>,
    event_handlers: Option<Vec<EventHandler>>,
    exec: Option<Vec<String>>,
    group: Option<String>,
    isolators: Option<Vec<Isolator>>,
    mount_points: Option<Vec<MountPoint>>,
    ports: Option<Vec<Port>>,
    supplementary_gids: Option<Vec<u32>>,
    user: Option<String>,
    working_directory: Option<String>,
}

pub struct Label {
    name: ACIdentifier,
    value: String,
}

pub struct Dependency {
    image_id: Option<ImageID>,
    image_name: ACIdentifier,
    labels: Option<Vec<Label>>,
    size: Option<u32>,
}

pub struct ImageManifest {
    ac_kind: ACKind,
    ac_version: ACVersion,
    annotations: Option<Annotation>,
    app: Option<App>,
    dependencies: Option<Vec<Dependency>>,
    labels: Option<Vec<Label>>,
    name: ACIdentifier,
    path_whitelist: Vec<String>,
}
