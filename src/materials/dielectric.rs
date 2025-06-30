use crate::materials::base_material::Scatterable;
use crate::math::Vec3;
use crate::objects::base_object::HitRecord;
use crate::ray::Ray;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Dielectric {
    pub refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vec3)> {
        let relative_refractive_index = if hit_record.is_front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_ray = ray.direction.unit_vector();
        let cos_theta = f64::min((-unit_ray).dot(&hit_record.intersection_normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);
        let cannot_refract = relative_refractive_index * sin_theta > 1.0;

        let ray_direction = if cannot_refract || reflectance(cos_theta, relative_refractive_index) > rand::random::<f64>() {
            Vec3::reflect(&unit_ray, &hit_record.intersection_normal)
        } else {
            refract(unit_ray, hit_record.intersection_normal, relative_refractive_index)
        };

        let scattered_ray = Ray::new(hit_record.intersection_point, ray_direction);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        Some((scattered_ray, attenuation))
    }
}

fn refract(unit_ray: Vec3, surface_normal: Vec3, relative_refractive_index: f64) -> Vec3 {
    let cos_theta = f64::min((-unit_ray).dot(&surface_normal), 1.0);
    let r_out_perp = (unit_ray + cos_theta * surface_normal) * relative_refractive_index;
    let r_out_parallel =
        -1.0 * f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * surface_normal;
    return r_out_perp + r_out_parallel;
}

// Use Schlick's approximation for reflectance.
fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}
