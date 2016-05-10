use regex::Regex;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator, Timestamps};
use url::Url;
use util::{Errors, Parseable, ParseResult, StringWrapper};

lazy_static! {
    static ref ENVIRONMENT_VARIABLE_NAME_REGEX: Regex = Regex::new("^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
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

impl StringWrapper for EnvironmentVariableName {
    fn from_string(s: &String) -> ParseResult<EnvironmentVariableName> {
        if ENVIRONMENT_VARIABLE_NAME_REGEX.is_match(s) {
            Ok(EnvironmentVariableName((*s).clone()))
        } else {
            Err(Errors::Node(vec![String::from("has invalid formatting.")]))
        }
    }
}

pub struct EnvironmentVariable {
    name: EnvironmentVariableName,
    value: String,
}

impl Parseable for EnvironmentVariable {
    fn from_json(json: &Json) -> ParseResult<EnvironmentVariable> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = BTreeMap::new();
                let mut name = None;
                let mut value = None;

                match obj.get("name") {
                    Some(name_json) => {
                        match EnvironmentVariableName::from_json(name_json) {
                            Ok(n) => { name = Some(n); },
                            Err(err) => { errors.insert(String::from("name"), err); },
                        };
                    },
                    None => {
                        errors.insert(String::from("name"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("value") {
                    Some(value_json) => {
                        match String::from_json(value_json) {
                            Ok(v) => { value = Some(v); },
                            Err(err) => { errors.insert(String::from("value"), err); },
                        };
                    },
                    None => {
                        errors.insert(String::from("value"), Errors::Node(vec![String::from("must be defined")]));
                    },
                }

                if errors.is_empty() {
                    Ok(EnvironmentVariable { name: name.unwrap(), value: value.unwrap() })
                } else {
                    Err(Errors::Object(errors))
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an object")])),
        }
    }
}

pub enum EventHandlerName {
    PreStart,
    PostStop,
}

impl StringWrapper for EventHandlerName {
    fn from_string(s: &String) -> ParseResult<EventHandlerName> {
        if s == "pre-start" {
            Ok(EventHandlerName::PreStart)
        } else if s == "post-stop" {
            Ok(EventHandlerName::PostStop)
        } else {
            Err(Errors::Node(vec![String::from("must be a valid event handler name.")]))
        }
    }
}

pub struct EventHandler {
    exec: Vec<String>,
    name: EventHandlerName,
}

impl EventHandler {
    fn exec_from_json(json: &Json) -> ParseResult<Vec<String>> {
        match json {
            &Json::Array(ref arr) => {
                let mut result = vec![];
                let mut errors = vec![];

                for i in 0..arr.len() {
                    let ref current_json = arr[i];

                    match String::from_json(current_json) {
                        Ok(cmd) => {
                            errors.push(None);
                            result.push(cmd);
                        },
                        Err(err) => { errors.push(Some(err)); },
                    }
                }

                if errors.is_empty() {
                    Ok(result)
                } else {
                    Err(Errors::Array(errors))
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an array")])),
        }
    }
}

impl Parseable for EventHandler {
    fn from_json(json: &Json) -> ParseResult<EventHandler> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = BTreeMap::new();
                let mut name = None;
                let mut exec = None;

                match obj.get("name") {
                    Some(name_json) => {
                        match EventHandlerName::from_json(name_json) {
                            Ok(n) => { name = Some(n); },
                            Err(err) => { errors.insert(String::from("name"), err); },
                        };
                    },
                    None => {
                        errors.insert(String::from("name"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("exec") {
                    Some(exec_json) => {
                        match EventHandler::exec_from_json(exec_json) {
                            Ok(e) => { exec = Some(e); },
                            Err(err) => { errors.insert(String::from("exec"), err); },
                        }
                    },
                    None => {
                        errors.insert(String::from("exec"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                if errors.is_empty() {
                    Ok(EventHandler { name: name.unwrap(), exec: exec.unwrap() })
                } else {
                    Err(Errors::Object(errors))
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an object")])),
        }
    }
}

pub struct MountPoint {
    name: ACName,
    path: String,
    read_only: bool,
}

impl Parseable for MountPoint {
    fn from_json(json: &Json) -> ParseResult<MountPoint> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = BTreeMap::new();
                let mut name = None;
                let mut path = None;
                let mut read_only = false;

                match obj.get("name") {
                    Some(name_json) => {
                        match ACName::from_json(name_json) {
                            Ok(n) => { name = Some(n); },
                            Err(err) => { errors.insert(String::from("name"), err); },
                        }
                    },
                    None => {
                        errors.insert(String::from("name"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("path") {
                    Some(path_json) => {
                        match String::from_json(path_json) {
                            Ok(p) => { path = Some(p); },
                            Err(err) => { errors.insert(String::from("path"), err); },
                        }
                    },
                    None => {
                        errors.insert(String::from("path"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("readOnly") {
                    Some(&Json::Boolean(ref ro)) => { read_only = *ro; },
                    Some(_) => { errors.insert(String::from("readOnly"), Errors::Node(vec![String::from("must be a boolean")])); },
                    None => {},
                };

                if errors.is_empty() {
                    Ok(MountPoint {
                        name: name.unwrap(),
                        path: path.unwrap(),
                        read_only: read_only
                    })
                } else {
                    Err(Errors::Object(errors))
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an object")])),
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

impl Parseable for Port {
    fn from_json(json: &Json) -> ParseResult<Port> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = BTreeMap::new();
                let mut count = 1;
                let mut name = None;
                let mut port = None;
                let mut protocol = None;
                let mut socket_activated = false;

                match obj.get("count") {
                    Some(&Json::U64(ref c)) => {
                        if (*c) < 1 {
                            errors.insert(String::from("count"), Errors::Node(vec![String::from("must be >= 1")]));
                        } else {
                            count = *c;
                        }
                    },
                    Some(_) => {
                        errors.insert(String::from("count"), Errors::Node(vec![String::from("must be a positive integer")]));
                    },
                    None => {},
                };

                match obj.get("name") {
                    Some(name_json) => {
                        match ACName::from_json(name_json) {
                            Ok(n) => { name = Some(n); },
                            Err(err) => { errors.insert(String::from("name"), err); },
                        }
                    },
                    None => {
                        errors.insert(String::from("name"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("port") {
                    Some(&Json::U64(ref p)) => {
                        if (*p) < 1 || (*p) > 65535 {
                            errors.insert(String::from("port"), Errors::Node(vec![String::from("must be >= 1 and <= 65535")]));
                        } else {
                            port = Some(*p as u16);
                        }
                    },
                    Some(_) => {
                        errors.insert(String::from("port"), Errors::Node(vec![String::from("must be a positive integer")]));
                    },
                    None => {
                        errors.insert(String::from("port"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("protocol") {
                    Some(protocol_json) => {
                        match String::from_json(protocol_json) {
                            Ok(p) => { protocol = Some(p); },
                            Err(err) => { errors.insert(String::from("protocol"), err); },
                        }
                    },
                    None => {
                        errors.insert(String::from("protocol"), Errors::Node(vec![String::from("must be defined")]));
                    },
                };

                match obj.get("socketActivated") {
                    Some(&Json::Boolean(ref sa)) => { socket_activated = *sa; },
                    Some(_) => {
                        errors.insert(String::from("socketActivated"), Errors::Node(vec![String::from("must be a boolean")]));
                    },
                    None => {},
                };

                if errors.is_empty() {
                    Ok(Port {
                        count: count,
                        name: name.unwrap(),
                        port: port.unwrap(),
                        protocol: protocol.unwrap(),
                        socket_activated: socket_activated,
                    })
                } else {
                    Err(Errors::Object(errors))
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an object")])),
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
