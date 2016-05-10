use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use std::marker::Sized;

/*
 * `Errors` is a JSON-like data structure intended to store errors corresponding to a particular
 * node in given JSON data.
 *
 * An `Errors::Node` can contain errors for any JSON type (Array, Object, String, etc.).
 * An `Errors::Array` contains errors for each entry in a given JSON array.
 * An `Errors::Object` contains errors for each type in a given JSON object.
 *
 */
pub enum Errors {
    Node(Vec<String>),
    Array(Vec<Option<Errors>>),
    Object(BTreeMap<String, Errors>)
}

pub type ParseResult<T> = Result<T, Errors>;

pub trait Parseable where Self: Sized {
    fn from_json(json: &Json) -> ParseResult<Self>;
}

pub trait StringWrapper where Self: Sized {
    fn from_string(s: &String) -> ParseResult<Self>;
}

impl<T> Parseable for T where T: StringWrapper {
    fn from_json(json: &Json) -> ParseResult<T> {
        match json {
            &Json::String(ref s) => T::from_string(s),
            _ => Err(Errors::Node(vec![String::from("must be a string")]))
        }
    }
}

impl StringWrapper for String {
    fn from_string(s: &String) -> ParseResult<String> {
        Ok((*s).clone())
    }
}
