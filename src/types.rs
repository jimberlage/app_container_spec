use chrono::{DateTime, FixedOffset};
use regex::Regex;
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use util::{Errors, Parseable, ParseResult, StringWrapper};

lazy_static! {
    // Regex taken from https://github.com/appc/spec/blob/master/spec/types.md#ac-identifier-type
    static ref AC_IDENTIFIER_REGEX: Regex = Regex::new("^[a-z0-9]+([-._~/][a-z0-9]+)*$").unwrap();
    // Regex taken from https://github.com/appc/spec/blob/master/spec/types.md#ac-name-type
    static ref AC_NAME_REGEX: Regex = Regex::new("^[a-z0-9]+([-][a-z0-9]+)*$").unwrap();
    static ref SEMVER_REGEX: Regex = Regex::new("^(?P<major>\\d|([1-9]\\d*))\\.(?P<minor>\\d|([1-9]\\d*))\\.(?P<patch>\\d|([1-9]\\d*))$").unwrap();
    static ref IMAGE_ID_REGEX: Regex = Regex::new("^(?P<hash>[^-]+)-(?P<value>[0-9A-Fa-f]+)$").unwrap();
}

pub struct ACIdentifier(String);

impl StringWrapper for ACIdentifier {
    fn from_string(s: &String) -> ParseResult<ACIdentifier> {
        if AC_IDENTIFIER_REGEX.is_match(s) {
            Ok(ACIdentifier((*s).clone()))
        } else {
            Err(Errors::Node(vec![String::from("has invalid formatting")]))
        }
    }
}

pub enum ACKind {
    ImageManifest,
    PodManifest
}

impl StringWrapper for ACKind {
    fn from_string(s: &String) -> ParseResult<ACKind> {
        if s == "ImageManifest" {
            Ok(ACKind::ImageManifest)
        } else if s == "PodManifest" {
            Ok(ACKind::PodManifest)
        } else {
            Err(Errors::Node(vec![String::from("must be one of \"ImageManifest\" or \"PodManifest\"")]))
        }
    }
}

pub struct ACName(String);

impl StringWrapper for ACName {
    fn from_string(s: &String) -> ParseResult<ACName> {
        if AC_NAME_REGEX.is_match(s) {
            Ok(ACName((*s).clone()))
        } else {
            Err(Errors::Node(vec![String::from("must be a valid AC Name")]))
        }
    }
}

pub struct ACVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl StringWrapper for ACVersion {
    fn from_string(s: &String) -> ParseResult<ACVersion> {
        match SEMVER_REGEX.captures(s) {
            Some(captures) => {
                // Since we got here by matching the above regex, these can be unwrapped and
                // coerced to u64 safely.
                let major = captures.name("major").unwrap().parse::<u64>().unwrap();
                let minor = captures.name("minor").unwrap().parse::<u64>().unwrap();
                let patch = captures.name("patch").unwrap().parse::<u64>().unwrap();

                Ok(ACVersion { major: major, minor: minor, patch: patch })
            },
            None => Err(Errors::Node(vec![String::from("must be a valid AC Version")])),
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

impl StringWrapper for ImageID {
    fn from_string(s: &String) -> ParseResult<ImageID> {
        match IMAGE_ID_REGEX.captures(s) {
            Some(captures) => {
                if captures.name("hash").unwrap() == "sha512" {
                    Ok(ImageID {
                        hash: HashAlgorithm::SHA512,
                        value: String::from(captures.name("value").unwrap())
                    })
                } else {
                    Err(Errors::Node(vec![String::from("must be a valid Image ID (invalid hash algorithm)")]))
                }
            },
            None => Err(Errors::Node(vec![String::from("must be a valid Image ID")])),
        }
    }
}

pub struct Isolator {
    name: ACIdentifier,
    value: Json,
}

impl Parseable for Isolator {
    fn from_json(json: &Json) -> ParseResult<Isolator> {
        match json {
            &Json::Object(ref obj) => {
                let mut errors = BTreeMap::new();
                let mut name = None;
                let mut value = None;

                match obj.get("name") {
                    Some(json) => {
                        match ACIdentifier::from_json(json) {
                            Ok(n) => { name = Some(n); },
                            Err(err) => { errors.insert(String::from("name"), err); },
                        };
                    },
                    None => {
                        errors.insert(String::from("name"), Errors::Node(vec![String::from("must be defined.")]));
                    },
                };

                match obj.get("value") {
                    Some(json) => { value = Some(json.clone()); },
                    None => {
                        errors.insert(String::from("value"), Errors::Node(vec![String::from("must be defined.")]));
                    },
                };

                if errors.is_empty() {
                    Ok(Isolator { name: name.unwrap(), value: value.unwrap() })
                } else {
                    Err(Errors::Object(errors))
                }
            },
            _ => Err(Errors::Node(vec![String::from("must be an object")])),
        }
    }
}

pub struct Timestamps(DateTime<FixedOffset>);

impl StringWrapper for Timestamps {
    fn from_string(s: &String) -> ParseResult<Timestamps> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(time) => Ok(Timestamps(time)),
            Err(_) => Err(Errors::Node(vec![String::from("must be a valid RFC3339 timestamp")])),
        }
    }
}
