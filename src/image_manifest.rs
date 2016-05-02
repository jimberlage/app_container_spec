use rustc_serialize::json::{self, Json};
use types::{ACIdentifier, ACKind, ACName, ACVersion, Errors, ImageID, Isolator, ParseResult, Timestamps, TypeResult};
use url::Url;

fn parse_error(message: &String) -> String {
    format!("{}
https://github.com/appc/spec/blob/v0.7.4/spec/aci.md#image-manifest-schema", message)
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

pub struct EventHandler {
    exec: Vec<String>,
    name: EventHandlerName,
}

fn parse_name_field(obj: &json::Object, path: &String) -> ParseResult<ACName> {
    let new_path = format!("{}[\"name\"]", path);

    match obj.get("name") {
        Some(&Json::String(ref n)) => ACName::from_string((*n).clone(), Some(new_path)),
        Some(_) => Err(Errors(vec![parse_error(&format!("{} must be a string.", new_path))])),
        None => Err(Errors(vec![parse_error(&format!("{} must be defined.", new_path))])),
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
            Some(_) => Err(Errors(vec![parse_error(&format!("{} must be a string.", new_path))])),
            None => Err(Errors(vec![parse_error(&format!("{} must be defined.", new_path))])),
        }
    }

    fn parse_read_only_field(obj: &json::Object, path: &String) -> ParseResult<bool> {
        let new_path = format!("{}[\"readOnly\"]", path);

        match obj.get("readOnly") {
            Some(&Json::Boolean(ref ro)) => Ok((*ro).clone()),
            Some(_) => Err(Errors(vec![parse_error(&format!("{} must be a boolean.", new_path))])),
            None => Ok(false),
        }
    }

    pub fn from_json(json: Json, path: Option<String>) -> ParseResult<MountPoint> {
        let mut errors = Errors(vec![]);
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
                match parse_name_field(&obj, &error_path) {
                    Ok(ac_name) => { name = Some(ac_name); },
                    Err(name_errors) => errors.combine(name_errors),
                };
                match MountPoint::parse_path_field(&obj, &error_path) {
                    Ok(p) => { path = Some(p); },
                    Err(path_errors) => errors.combine(path_errors),
                };
                match MountPoint::parse_read_only_field(&obj, &error_path) {
                    Ok(ro) => { read_only = Some(ro); },
                    Err(read_only_errors) => errors.combine(read_only_errors),
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
                errors.push(parse_error(&format!("{} must be an object.", error_path)));
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
                    Err(Errors(vec![parse_error(&format!("{} must be >= 1.", new_path))]))
                } else {
                    Ok((*c).clone())
                }
            },
            Some(_) => {
                Err(Errors(vec![parse_error(&format!("{} must be a positive integer.", new_path))]))
            },
            None => Ok(1),
        }
    }

    pub fn from_json(json: Json, path: Option<String>) -> ParseResult<Port> {
        let mut errors = Errors(vec![]);
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
                match Port::parse_count_field(&obj, &error_path) {
                    Ok(c) => { count = Some(c); },
                    Err(count_errors) => errors.combine(count_errors),
                };
                match parse_name_field(&obj, &error_path) {
                    Ok(ac_name) => { name = Some(ac_name) },
                    Err(name_errors) => errors.combine(name_errors),
                };
            },
            _ => {
                errors.push(parse_error(&format!("{} must be an object.", error_path)));
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
