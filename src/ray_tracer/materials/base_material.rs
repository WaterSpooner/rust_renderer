use crate::ray_tracer::materials::{Dielectric, Metal, Lambertian};
use crate::ray_tracer::Vec3;
use crate::ray_tracer::objects::HitRecord;
use crate::ray_tracer::Ray;

#[derive(serde::Deserialize, serde::Serialize)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)>;
}

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        match self {
            Material::Dielectric(d) => d.scatter(ray, hit_record),
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m) => m.scatter(ray, hit_record),
        }
    }
}
