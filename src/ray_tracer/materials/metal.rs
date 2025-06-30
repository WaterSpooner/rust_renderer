use crate::ray_tracer::materials::Scatterable;
use crate::ray_tracer::Vec3;
use crate::ray_tracer::objects::HitRecord;
use crate::ray_tracer::Ray;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Metal {
    pub colour: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(colour: Vec3, fuzz: f64) -> Self {
        Metal {
            colour,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected_ray = Vec3::reflect(&ray.direction, &hit_record.intersection_normal)
            + (self.fuzz * Vec3::unit_random());
        let scattered_ray = Ray::new(hit_record.intersection_point, reflected_ray);
        Some((scattered_ray, self.colour))
    }
}
