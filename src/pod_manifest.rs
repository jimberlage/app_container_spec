use image_manifest;
use rustc_serialize::json::Json;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator};

pub struct Annotation {
    name: ACName,
    value: String,
}

pub struct Image {
    id: ImageId,
    labels: Option<Vec<Json::Object>>,
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
    host_port: u32,
}

pub enum Kind {
    Empty,
    Host,
}

pub struct Volume {
    gid: u32,
    kind: Kind,
    mode: String,
    name: ACName,
    read_only: bool,
    source: Option<String>,
    uid: u32,
}

pub struct PodManifest {
    ac_kind: ACKind::PodManifest,
    ac_version: ACVersion,
    annotations: Option<Vec<Annotation>>,
    apps: Vec<App>,
    isolators: Option<Vec<Isolator>>,
    ports: Option<Vec<Port>>,
    volumes: Option<Vec<Volume>>,
}
