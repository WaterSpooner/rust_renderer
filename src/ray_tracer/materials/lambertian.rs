use crate::ray_tracer::materials::Scatterable;
use crate::ray_tracer::Vec3;
use crate::ray_tracer::objects::HitRecord;
use crate::ray_tracer::Ray;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Lambertian {
    pub colour: Vec3,
}

impl Lambertian {
    pub fn new(colour: Vec3) -> Self {
        Lambertian { colour }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scattered_dir = hit_record.intersection_normal + Vec3::unit_random();
        if scattered_dir.near_zero() {
            scattered_dir = hit_record.intersection_normal;
        }
        let scattered_ray = Ray::new(hit_record.intersection_point, scattered_dir);
        Some((scattered_ray, self.colour))
    }
}
