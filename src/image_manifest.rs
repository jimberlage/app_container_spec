use rustc_serialize::json::Json;
use types::{ACIdentifier, ACKind, ACName, ACVersion, ImageID, Isolator};

pub struct ImageManifest {
    ac_kind: ACKind::ImageManifest,
    ac_version: ACVersion,
    annotations: Option<Annotation>,
    app: Option<App>,
    dependencies: Option<Vec<Dependency>>,
    labels: Option<Vec<Label>>,
    name: ACIdentifier,
    path_whitelist: Vec<String>,
}
