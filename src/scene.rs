use crate::camera::Camera;
use crate::objects::base_object::Object;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}
