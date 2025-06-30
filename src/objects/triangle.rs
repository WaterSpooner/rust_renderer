use crate::materials::base_material::Material;
use crate::math::Vec3;
use crate::objects::base_object::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Triangle {
    position1: Vec3,
    material: Arc<Material>,
    normal: Vec3,
    p1_p2: Vec3,
    p1_p3: Vec3,
}

impl Triangle {
    pub fn new(
        position1: Vec3,
        position2: Vec3,
        position3: Vec3,
        material: Arc<Material>,
    ) -> Triangle {
        let p1_p2 = position2 - position1;
        let p1_p3 = position3 - position1;
        let normal = p1_p2.cross(&p1_p3).unit_vector();
        Triangle {
            position1,
            material,
            p1_p2,
            p1_p3,
            normal,
        }
    }
}

//Möller–Trumbore intersection algorithm
impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let epsilon = 1e-8;
        let edge1 = self.p1_p2;
        let edge2 = self.p1_p3;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a < epsilon {
            return None; // Ray is parallel to triangle or hit backface
        }

        let f = 1.0 / a;
        let s = ray.origin - self.position1;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        if t < ray_tmin || t > ray_tmax {
            return None;
        }

        let intersection_point = ray.at(t);
        let outward_normal = self.normal.unit_vector();
        let is_front_face = ray.direction.dot(&outward_normal) < 0.0;
        let intersection_normal = if is_front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Some(HitRecord {
            intersection_point,
            intersection_normal,
            t,
            is_front_face,
            material: self.material.clone(),
        })
    }
}
