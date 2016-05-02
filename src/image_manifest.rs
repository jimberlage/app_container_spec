use rustc_serialize::json::Json;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator, ParseResult, Timestamps, TypeResult};
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

impl MountPoint {
    pub fn from_json(json: Json, path: Option<String>) -> ParseResult<MountPoint> {
        let mut errors = vec![];
        let error_path = match path {
            Some(path) => path,
            None => String::from("mountPoint"),
        };

        match json {
            Json::Object(obj) => {
                let mut name = None;
                let mut path = None;
                let mut read_only = None;

                // Validate fields.
                match obj.get("name") {
                    Some(&Json::String(ref n)) => {
                        let name_error_path = Some(format!("{}[\"name\"]", error_path));

                        match ACName::from_string((*n).clone(), name_error_path) {
                            Ok(ac_name) => {
                                name = Some(ac_name);
                            },
                            Err(name_errors) => {
                                for error in name_errors {
                                    errors.push(error);
                                }
                            },
                        };
                    },
                    Some(_) => errors.push(format!("{}[\"name\"] must be a string.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path)),
                    None => errors.push(format!("{}[\"name\"] must be defined.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path)),
                };
                match obj.get("path") {
                    Some(&Json::String(ref p)) => {
                        path = Some((*p).clone());
                    },
                    Some(_) => errors.push(format!("{}[\"path\"] must be a string.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path)),
                    None => errors.push(format!("{}[\"path\"] must be defined.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path)),
                };
                match obj.get("readOnly") {
                    Some(&Json::Boolean(ref ro)) => {
                        read_only = Some((*ro).clone());
                    },
                    Some(_) => errors.push(format!("{}[\"readOnly\"] must be a boolean.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path)),
                    None => {
                        read_only = Some(false);
                    },
                };

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(MountPoint {
                    name: name.unwrap(),
                    path: path.unwrap(),
                    read_only: read_only.unwrap(),
                })
            },
            _ => {
                errors.push(format!("{} must be an object.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path));
                Err(errors)
            },
        }
    }
}

pub struct Port {
    count: u32,
    name: ACName,
    port: u16,
    protocol: String,
    socket_activated: bool,
}

impl Port {
    pub fn from_json(json: Json, path: Option<String>) -> ParseResult<Port> {
        let mut errors = vec![];
        let error_path = match path {
            Some(path) => path,
            None => String::from("Port"),
        };

        match json {
            Json::Object(obj) => {
                let mut count = None;
                let mut name = None;
                let mut port = None;
                let mut protocol = None;
                let mut socket_activated = None;

                // Validate fields.
                match obj.get("count") {
                    Some(&Json::U64(ref c)) => {
                        if c < 1 {
                            errors.push(format!("{}[\"count\"] must be >= 1.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path));
                        } else {
                            count = Some((*c).clone() as u32);
                        }
                    },
                    Some(_) => errors.push(format!("{}[\"count\"] must be a positive integer.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path)),
                    None => {
                        count = Some(1);
                    },
                };
                match obj.get("name") {
                    Some(&Json::String(ref n)) => {
                    },
                    Some(_) => {
                    },
                    None => {
                    },
                };
            },
            _ => {
                errors.push(format!("{} must be an object.
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", error_path));
                Err(errors)
            },
        }
    }
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
