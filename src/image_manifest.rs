use image_manifest::app::environment_variable::EnvironmentVariable;
use image_manifest::app::event_handler::EventHandler;
use image_manifest::app::mount_point::MountPoint;
use image_manifest::app::port::Port;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator, Timestamps};
use url::Url;
use util::{Errors, Parseable, ParseResult, StringWrapper};

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

impl Parseable for App {
    fn from_json(json: &Json) -> ParseResult<App> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = BTreeMap::new();
                let mut environment = None;
                let mut event_handlers = None;
                let mut exec = None;
                let mut group = None;
                let mut isolators = None;
                let mut mount_points = None;
                let mut ports = None;
                let mut supplementary_gids = None;
                let mut user = None;
                let mut working_directory = None;

                match obj.get("environment") {
                    Some(&Json::Array(ref arr)) => {
                        let result = vec![];
                        let environment_errors = vec![];

                        for i in 0..arr.len() {
                            let ref var_json = arr[i];

                            match EnvironmentVariable::from_json(var_json) {
                                Ok(var) => {
                                    environment_errors.push(None);
                                    result.push(var);
                                },
                                Err(err) => { environment_errors.push(Some(err)); },
                            }
                        }

                        if environment_errors.iter().any(|&e| e.is_some()) {
                            errors.insert(String::from("environment"), Errors::Array(environment_errors));
                        } else {
                            environment = Some(result);
                        }
                    },
                    Some(_) => {
                        errors.insert(String::from("environment"), Errors::Node(vec![String::from("must be an array")]));
                    },
                    None => {},
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an object")])),
        }
    }
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
