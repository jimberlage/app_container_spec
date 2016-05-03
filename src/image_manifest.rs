use rustc_serialize::json::{self, Json};
use types::{ACIdentifier, ACKind, ACName, ACVersion, Errors, ImageID, Isolator, ParseResult, Timestamps, TypeResult};
use url::Url;

fn parse_error(path: &String, message: &str) -> String {
    format!("{} {}
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", path, message)
}

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

impl EventHandlerName {
    pub fn from_json(json: &Json, path: &String) -> ParseResult<EventHandlerName> {
        match json {
            &Json::String(ref name) => {
                if name == "pre-start" {
                    Ok(EventHandlerName::PreStart)
                } else if name == "post-stop" {
                    Ok(EventHandlerName::PostStop)
                } else {
                    Err(Errors(vec![parse_error(path, "must be a valid event handler name.")]))
                }
            },
            _ => Err(Errors(vec![parse_error(path, "must be a string.")]))
        }
    }
}

pub struct EventHandler {
    exec: Vec<String>,
    name: EventHandlerName,
}

impl EventHandler {
    fn parse_name_field(obj: &json::Object, path: &String) -> ParseResult<EventHandlerName> {
        let new_path = format!("{}[\"name\"]", path);

        match obj.get("name") {
            Some(value) => EventHandlerName::from_json(value, &new_path),
            None => Err(Errors(vec![parse_error(&new_path, "must be defined.")])),
        }
    }

    // TODO: Write this.
    fn parse_exec_field(json: &json::Object, path: &String) -> ParseResult<Vec<String>> {
    }

    pub fn from_json(json: &Json, path: &String) -> ParseResult<EventHandler> {
        let mut errors = Errors(vec![]);

        match json {
            &Json::Object(ref obj) => {
                let mut name = None;
                let mut exec = None;

                match EventHandler::parse_name_field(obj, path) {
                    Ok(n) => { name = Some(n); },
                    Err(name_errors) => errors.combine(name_errors),
                };
                match EventHandler::parse_exec_field(obj, path) {
                    Ok(e) => { exec = Some(e); },
                    Err(exec_errors) => errors.combine(exec_errors),
                };

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(EventHandler { exec: exec.unwrap(), name: name.unwrap() })
            },
            _ => {
                errors.push(parse_error(path, "must be an object."));
                Err(errors)
            },
        }
    }
}

fn parse_name_field(obj: &json::Object, path: &String) -> ParseResult<ACName> {
    let new_path = format!("{}[\"name\"]", path);

    match obj.get("name") {
        Some(&Json::String(ref n)) => ACName::from_string((*n).clone(), Some(new_path)),
        Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a string.")])),
        None => Err(Errors(vec![parse_error(&new_path, "must be defined.")])),
    }
}

pub struct MountPoint {
    name: ACName,
    path: String,
    read_only: bool,
}

impl MountPoint {
    fn parse_path_field(obj: &json::Object, path: &String) -> ParseResult<String> {
        let new_path = format!("{}[\"path\"]", path);

        match obj.get("path") {
            Some(&Json::String(ref p)) => Ok((*p).clone()),
            Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a string.")])),
            None => Err(Errors(vec![parse_error(&new_path, "must be defined.")])),
        }
    }

    fn parse_read_only_field(obj: &json::Object, path: &String) -> ParseResult<bool> {
        let new_path = format!("{}[\"readOnly\"]", path);

        match obj.get("readOnly") {
            Some(&Json::Boolean(ref ro)) => Ok((*ro).clone()),
            Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a boolean.")])),
            None => Ok(false),
        }
    }

    pub fn from_json(json: &Json, path: &String) -> ParseResult<MountPoint> {
        let mut errors = Errors(vec![]);

        match json {
            &Json::Object(ref obj) => {
                let mut name = None;
                let mut mp_path = None;
                let mut read_only = None;

                // Validate fields.
                match parse_name_field(obj, path) {
                    Ok(ac_name) => { name = Some(ac_name); },
                    Err(name_errors) => errors.combine(name_errors),
                };
                match MountPoint::parse_path_field(obj, path) {
                    Ok(p) => { mp_path = Some(p); },
                    Err(path_errors) => errors.combine(path_errors),
                };
                match MountPoint::parse_read_only_field(obj, path) {
                    Ok(ro) => { read_only = Some(ro); },
                    Err(read_only_errors) => errors.combine(read_only_errors),
                };

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(MountPoint {
                    name: name.unwrap(),
                    path: mp_path.unwrap(),
                    read_only: read_only.unwrap(),
                })
            },
            _ => {
                errors.push(parse_error(path, "must be an object."));
                Err(errors)
            },
        }
    }
}

pub struct Port {
    count: u64,
    name: ACName,
    port: u16,
    protocol: String,
    socket_activated: bool,
}

impl Port {
    fn parse_count_field(obj: &json::Object, path: &String) -> ParseResult<u64> {
        let new_path = format!("{}[\"count\"]", path);

        match obj.get("count") {
            Some(&Json::U64(ref c)) => {
                if (*c) < 1 {
                    Err(Errors(vec![parse_error(&new_path, "must be >= 1.")]))
                } else {
                    Ok((*c).clone())
                }
            },
            Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a positive integer.")])),
            None => Ok(1),
        }
    }

    fn parse_port_field(obj: &json::Object, path: &String) -> ParseResult<u16> {
        let new_path = format!("{}[\"port\"]", path);

        match obj.get("port") {
            Some(&Json::U64(ref p)) => {
                if (*p) < 1 || (*p) > 65535 {
                    Err(Errors(vec![parse_error(&new_path, "must be >= 1 and <= 65535.")]))
                } else {
                    Ok((*p).clone() as u16)
                }
            },
            Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a positive integer.")])),
            None => Err(Errors(vec![parse_error(&new_path, "must be a defined.")])),
        }
    }

    fn parse_protocol_field(obj: &json::Object, path: &String) -> ParseResult<String> {
        let new_path = format!("{}[\"protocol\"]", path);

        match obj.get("protocol") {
            Some(&Json::String(ref p)) => Ok((*p).clone()),
            Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a string.")])),
            None => Err(Errors(vec![parse_error(&new_path, "must be a defined.")])),
        }
    }

    fn parse_socket_activated_field(obj: &json::Object, path: &String) -> ParseResult<bool> {
        let new_path = format!("{}[\"socketActivated\"]", path);

        match obj.get("socketActivated") {
            Some(&Json::Boolean(ref sa)) => Ok(*sa),
            Some(_) => Err(Errors(vec![parse_error(&new_path, "must be a boolean.")])),
            None => Ok(false),
        }
    }

    pub fn from_json(json: Json, path: String) -> ParseResult<Port> {
        let mut errors = Errors(vec![]);

        match json {
            Json::Object(obj) => {
                let mut count = None;
                let mut name = None;
                let mut port = None;
                let mut protocol = None;
                let mut socket_activated = None;

                // Validate fields.
                match Port::parse_count_field(&obj, &path) {
                    Ok(c) => { count = Some(c); },
                    Err(count_errors) => errors.combine(count_errors),
                };
                match parse_name_field(&obj, &path) {
                    Ok(ac_name) => { name = Some(ac_name) },
                    Err(name_errors) => errors.combine(name_errors),
                };
                match Port::parse_port_field(&obj, &path) {
                    Ok(p) => { port = Some(p); },
                    Err(port_errors) => errors.combine(port_errors),
                };
                match Port::parse_protocol_field(&obj, &path) {
                    Ok(p) => { protocol = Some(p); },
                    Err(protocol_errors) => errors.combine(protocol_errors),
                };
                match Port::parse_socket_activated_field(&obj, &path) {
                    Ok(sa) => { socket_activated = Some(sa); },
                    Err(socket_activated_errors) => errors.combine(socket_activated_errors),
                };

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(Port {
                    count: count.unwrap(),
                    name: name.unwrap(),
                    port: port.unwrap(),
                    protocol: protocol.unwrap(),
                    socket_activated: socket_activated.unwrap(),
                })
            },
            _ => {
                errors.push(parse_error(&path, "must be an object."));
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
    supplementary_gids: Option<Vec<u64>>,
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
    size: Option<u64>,
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
