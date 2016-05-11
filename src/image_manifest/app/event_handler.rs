use rustc_serialize::json::Json;
use util::{Errors, Parseable, ParseResult, StringWrapper};

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

fn exec_from_json(json: &Json) -> ParseResult<Vec<String>> {
    match json {
        &Json::Array(ref arr) => {
            let mut result = vec![];
            let mut errors = vec![];

            for i in 0..arr.len() {
                let ref cmd_json = arr[i];

                match String::from_json(cmd_json) {
                    Ok(cmd) => {
                        errors.push(None);
                        result.push(cmd);
                    },
                    Err(err) => { errors.push(Some(err)); },
                }
            }

            if errors.iter().any(|&e| e.is_some()) {
                Err(Errors::Array(errors))
            } else {
                Ok(result)
            }
        },
        _ => Err(Errors::Node(vec![String::from("must be an array")])),
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
                        match exec_from_json(exec_json) {
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
