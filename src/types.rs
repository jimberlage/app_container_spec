use chrono::{DateTime, FixedOffset};
use regex::Regex;
use rustc_serialize::json::{self, Json};

lazy_static! {
    // Regex taken from https://github.com/appc/spec/blob/master/spec/types.md#ac-identifier-type
    static ref AC_IDENTIFIER_REGEX: Regex = Regex::new("^[a-z0-9]+([-._~/][a-z0-9]+)*$").unwrap();
    // Regex taken from https://github.com/appc/spec/blob/master/spec/types.md#ac-name-type
    static ref AC_NAME_REGEX: Regex = Regex::new("^[a-z0-9]+([-][a-z0-9]+)*$").unwrap();
    static ref SEMVER_REGEX: Regex = Regex::new("^(?P<major>\\d|([1-9]\\d*))\\.(?P<minor>\\d|([1-9]\\d*))\\.(?P<patch>\\d|([1-9]\\d*))$").unwrap();
    static ref IMAGE_ID_REGEX: Regex = Regex::new("^(?P<hash>[^-]+)-(?P<value>[0-9A-Fa-f]+)$").unwrap();
}

pub struct Errors(pub Vec<String>);

impl Errors {
    pub fn push(&mut self, error: String) {
        self.0.push(error);
    }

    pub fn combine(&mut self, other: Errors) {
        for error in other.0 {
            self.0.push(error.clone());
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub type ParseResult<T> = Result<T, Errors>;

pub type TypeResult<T> = Result<T, String>;

pub struct ACIdentifier(String);

impl ACIdentifier {
    fn parse_error(path: &String, message: &str) -> String {
        format!("{} {}
https://github.com/appc/spec/blob/v0.7.4/spec/types.md#ac-identifier-type", path, message)
    }

    pub fn from_string(identifier: &String, path: &String) -> ParseResult<ACIdentifier> {
        if AC_IDENTIFIER_REGEX.is_match(identifier) {
            Ok(ACIdentifier((*identifier).clone()))
        } else {
            Err(Errors(vec![ACIdentifier::parse_error(path, "has invalid formatting.")]))
        }
    }

    pub fn from_json(json: &Json, path: &String) -> ParseResult<ACIdentifier> {
        match json {
            &Json::String(ref identifier) => ACIdentifier::from_string(identifier, path),
            _ => Err(Errors(vec![ACIdentifier::parse_error(path, "must be a string.")])),
        }
    }
}

pub enum ACKind {
    ImageManifest,
    PodManifest
}

impl ACKind {
    pub fn from_json(json: Json) -> TypeResult<ACKind> {
        match json {
            Json::String(kind) => {
                if kind == "ImageManifest" {
                    Ok(ACKind::ImageManifest)
                } else if kind == "PodManifest" {
                    Ok(ACKind::PodManifest)
                } else {
                    Err(String::from("Invalid AC Kind"))
                }
            },
            _ => Err(String::from("Invalid AC Kind")),
        }
    }
}

pub struct ACName(String);

impl ACName {
    fn error_path(path: Option<String>) -> String {
        match path {
            Some(path) => path,
            None => String::from("AC Name"),
        }
    }

    pub fn from_string(name: String, path: Option<String>) -> ParseResult<ACName> {
        if AC_NAME_REGEX.is_match(&name) {
            Ok(ACName(name))
        } else {
            let error = format!("{} must be a valid AC Name.
https://github.com/appc/spec/blob/v0.7.4/spec/types.md#ac-name-type", ACName::error_path(path));
            Err(Errors(vec![error]))
        }
    }

    pub fn from_json(json: Json, path: Option<String>) -> ParseResult<ACName> {
        match json {
            Json::String(name) => ACName::from_string(name, path),
            _ => {
                let error = format!("{} must be a string.
https://github.com/appc/spec/blob/v0.7.4/spec/types.md#ac-name-type", ACName::error_path(path));
                Err(Errors(vec![error]))
            },
        }
    }
}

pub struct ACVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl ACVersion {
    pub fn from_json(json: Json) -> TypeResult<ACVersion> {
        match json {
            Json::String(version) => {
                match SEMVER_REGEX.captures(&version) {
                    Some(captures) => {
                        // Since we got here by matching the above regex, these can be unwrapped and
                        // coerced to u64 safely.
                        let major = captures.name("major").unwrap().parse::<u64>().unwrap();
                        let minor = captures.name("minor").unwrap().parse::<u64>().unwrap();
                        let patch = captures.name("patch").unwrap().parse::<u64>().unwrap();

                        Ok(ACVersion { major: major, minor: minor, patch: patch })
                    },
                    None => Err(String::from("Invalid AC Version")),
                }
            },
            _ => Err(String::from("Invalid AC Version")),
        }
    }
}

pub enum HashAlgorithm {
    SHA512,
}

pub struct ImageID {
    hash: HashAlgorithm,
    value: String
}

impl ImageID {
    pub fn from_json(json: Json) -> TypeResult<ImageID> {
        match json {
            Json::String(image_id) => {
                match IMAGE_ID_REGEX.captures(&image_id) {
                    Some(captures) => {
                        let hash = captures.name("hash").unwrap();
                        if hash == "sha512" {
                            Ok(ImageID {
                                hash: HashAlgorithm::SHA512,
                                value: String::from(captures.name("value").unwrap())
                            })
                        } else {
                            Err(String::from("Invalid Image ID"))
                        }
                    },
                    None => Err(String::from("Invalid Image ID")),
                }
            },
            _ => Err(String::from("Invalid Image ID")),
        }
    }
}

pub struct Isolator {
    name: ACIdentifier,
    value: Json,
}

impl Isolator {
    fn parse_error(path: &String, message: &str) -> String {
        format!("{} {}
https://github.com/appc/spec/blob/v0.7.4/spec/types.md#isolator-type", path, message)
    }

    fn parse_name_field(obj: &json::Object, path: &String) -> ParseResult<ACIdentifier> {
        let new_path = format!("{}[\"name\"]", path);

        // "name" must be present, a JSON string, and be in the AC Identifier format.
        match obj.get("name") {
            Some(&Json::String(ref name)) => ACIdentifier::from_string(name, &new_path),
            Some(_) => Err(Errors(vec![Isolator::parse_error(&new_path, "must be a string.")])),
            None => Err(Errors(vec![Isolator::parse_error(&new_path, "must be defined.")])),
        }
    }

    pub fn from_json(json: &Json, path: &String) -> ParseResult<Isolator> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = Errors(vec![]);
                let mut name = None;
                let mut value = None;

                match Isolator::parse_name_field(obj, path) {
                    Ok(n) => { name = Some(n); },
                    Err(name_errors) => errors.combine(name_errors),
                }
                // "value" must be present, but may be any valid JSON.
                match obj.get("value") {
                    Some(v) => { value = Some(v.clone()); },
                    None => {
                        let new_path = format!("{}[\"value\"]", path);
                        errors.push(Isolator::parse_error(&new_path, "must be defined."));
                    },
                };

                if !errors.is_empty() {
                    return Err(errors);
                }

                Ok(Isolator { name: name.unwrap(), value: value.unwrap() })
            },
            _ => Err(Errors(vec![Isolator::parse_error(path, "must be an object.")])),
        }
    }
}

pub struct Timestamps(DateTime<FixedOffset>);

impl Timestamps {
    pub fn from_json(json: Json) -> TypeResult<Timestamps> {
        match json {
            Json::String(timestamps) => {
                match DateTime::parse_from_rfc3339(&timestamps) {
                    Ok(time) => Ok(Timestamps(time)),
                    Err(_) => Err(String::from("Invalid Timestamps")),
                }
            },
            _ => Err(String::from("Invalid Timestamps")),
        }
    }
}
