use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use types::ACName;
use util::{Errors, Parseable, ParseResult};

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
