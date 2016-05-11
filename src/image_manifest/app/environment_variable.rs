use regex::Regex;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use util::{Errors, Parseable, ParseResult, StringWrapper};

lazy_static! {
    static ref ENVIRONMENT_VARIABLE_NAME_REGEX: Regex = Regex::new("^[a-zA-Z][a-zA-Z0-9_]*$").unwrap();
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
