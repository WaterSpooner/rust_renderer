use crate::ray_tracer::Camera;
use crate::ray_tracer::objects::Object;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}
