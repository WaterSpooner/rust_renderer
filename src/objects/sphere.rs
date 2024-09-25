use crate::math::Vec3;
use crate::objects::base_object::{Hittable,HitRecord};
use crate::ray::Ray;

pub struct Sphere{
    pub position:Vec3,
    pub radius:f64,
}

impl Sphere{
    pub fn new(position:Vec3,radius:f64) -> Sphere {
        Sphere {
            position,
            radius,
        }
        
    }
}

impl Hittable for Sphere{
    fn hit(&self,ray:&Ray, ray_tmin:f64, ray_tmax:f64) -> Option<HitRecord> {
        let oc = self.position - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {return None}
        }


        let p = ray.at(root);
        let outward_normal = (p-self.position) /self.radius;
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        return Some(HitRecord{
            t:root,
            normal,
            p,
            front_face,

        });
    }
}