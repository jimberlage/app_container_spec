use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use types::ACName;
use util::{Errors, Parseable, ParseResult};

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
