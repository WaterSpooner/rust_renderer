use crate::{camera::Camera, objects::sphere::Sphere};
pub struct Scene{
    pub objects: Vec<Sphere>,
    pub camera: Camera,
}

