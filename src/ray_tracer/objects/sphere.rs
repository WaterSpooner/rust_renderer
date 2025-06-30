use crate::ray_tracer::materials::Material;
use crate::ray_tracer::Vec3;
use crate::ray_tracer::objects::{HitRecord, Hittable};
use crate::ray_tracer::Ray;
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, material: Arc<Material>) -> Sphere {
        Sphere {
            position,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.position - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let intersection_point = ray.at(root);
        let outward_normal = (intersection_point - self.position) / self.radius;
        let is_front_face = ray.direction.dot(&outward_normal) < 0.0;
        let intersection_normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord {
            t: root,
            intersection_normal,
            intersection_point,
            is_front_face,
            material: self.material.clone(),
        })
    }
}
