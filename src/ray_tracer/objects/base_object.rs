use crate::ray_tracer::materials::Material;
use crate::ray_tracer::Vec3;
use crate::ray_tracer::objects::{Sphere, Triangle};
use crate::ray_tracer::Ray;
use std::sync::Arc;

pub struct HitRecord {
    pub intersection_point: Vec3,
    pub intersection_normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
    pub material: Arc<Material>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Object {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(ray, t_min, t_max),
            Object::Triangle(t) => t.hit(ray, t_min, t_max),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
