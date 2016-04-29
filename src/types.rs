use chrono::{DateTime, FixedOffset};
use regex::Regex;
use rustc_serialize::json::Json;

lazy_static! {
    // Regex taken from https://github.com/appc/spec/blob/master/spec/types.md#ac-identifier-type
    static ref AC_IDENTIFIER_REGEX: Regex = Regex::new("^[a-z0-9]+([-._~/][a-z0-9]+)*$").unwrap();
    // Regex taken from https://github.com/appc/spec/blob/master/spec/types.md#ac-name-type
    static ref AC_NAME_REGEX: Regex = Regex::new("^[a-z0-9]+([-][a-z0-9]+)*$").unwrap();
    static ref SEMVER_REGEX: Regex = Regex::new("^(?P<major>\\d|([1-9]\\d*))\\.(?P<minor>\\d|([1-9]\\d*))\\.(?P<patch>\\d|([1-9]\\d*))$").unwrap();
    static ref IMAGE_ID_REGEX: Regex = Regex::new("^(?P<hash>[^-]+)-(?P<value>[0-9A-Fa-f]+)$").unwrap();
}

pub type TypeResult<T> = Result<T, String>;

pub struct ACIdentifier(String);

impl ACIdentifier {
    pub fn from_string(identifier: String) -> TypeResult<ACIdentifier> {
        if AC_IDENTIFIER_REGEX.is_match(&identifier) {
            Ok(ACIdentifier(identifier))
        } else {
            Err(String::from("Invalid AC Identifier"))
        }
    }

    pub fn from_json(json: Json) -> TypeResult<ACIdentifier> {
        match json {
            Json::String(identifier) => ACIdentifier::from_string(identifier),
            _ => Err(String::from("Invalid AC Identifier")),
        }
    }
}

pub struct ACName(String);

impl ACName {
    pub fn from_json(json: Json) -> TypeResult<ACName> {
        match json {
            Json::String(name) => {
                if AC_NAME_REGEX.is_match(&name) {
                    Ok(ACName(name))
                } else {
                    Err(String::from("Invalid AC Name"))
                }
            },
            _ => Err(String::from("Invalid AC Name")),
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

pub struct ACVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

impl ACVersion {
    pub fn from_json(json: Json) -> TypeResult<ACVersion> {
        match json {
            Json::String(version) => {
                match SEMVER_REGEX.captures(&version) {
                    Some(captures) => {
                        // Since we got here by matching the above regex, these can be unwrapped and
                        // coerced to u32 safely.
                        let major = captures.name("major").unwrap().parse::<u32>().unwrap();
                        let minor = captures.name("minor").unwrap().parse::<u32>().unwrap();
                        let patch = captures.name("patch").unwrap().parse::<u32>().unwrap();

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
    pub fn from_json(json: Json) -> TypeResult<Isolator> {
        match json {
            Json::Object(obj) => {
                // "value" must be present, but may be any valid JSON.
                match obj.get("value") {
                    Some(value) => {
                        // "name" must be present, a JSON string, and be in the AC Identifier
                        // format.
                        match obj.get("name") {
                            Some(&Json::String(ref name)) => {
                                match ACIdentifier::from_string(name.clone()) {
                                    Ok(identifier) => Ok(Isolator {
                                        name: identifier,
                                        value: value.clone()
                                    }),
                                    _ => Err(String::from("Invalid Isolator")),
                                }
                            },
                            _ => Err(String::from("Invalid Isolator")),
                        }
                    },
                    None => Err(String::from("Invalid Isolator")),
                }
            },
            _ => Err(String::from("Invalid Isolator")),
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
