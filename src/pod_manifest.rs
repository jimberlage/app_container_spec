use image_manifest;
use rustc_serialize::json::Json;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator};

pub struct Annotation {
    name: ACName,
    value: String,
}

pub struct Image {
    id: ImageID,
    labels: Option<Vec<Json>>,
    name: Option<ACIdentifier>,
}

pub struct Mount {
    path: String,
    volume: ACName,
}

pub struct App {
    annotations: Option<Vec<Annotation>>,
    app: Option<image_manifest::App>,
    image: Image,
    mounts: Option<Vec<Mount>>,
    name: ACName,
}

pub struct Port {
    name: ACName,
    host_port: u64,
}

pub enum Kind {
    Empty,
    Host,
}

pub struct Volume {
    gid: u64,
    kind: Kind,
    mode: String,
    name: ACName,
    read_only: bool,
    source: Option<String>,
    uid: u64,
}

pub struct PodManifest {
    ac_kind: ACKind,
    ac_version: ACVersion,
    annotations: Option<Vec<Annotation>>,
    apps: Vec<App>,
    isolators: Option<Vec<Isolator>>,
    ports: Option<Vec<Port>>,
    volumes: Option<Vec<Volume>>,
}
