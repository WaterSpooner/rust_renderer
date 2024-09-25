use crate::math::Vec3;
use crate::ray::Ray;

pub struct HitRecord{
    pub p: Vec3,
    pub normal :Vec3,
    pub t:f64,
    pub front_face:bool,
}

pub trait Hittable {
    fn hit(&self, ray:&Ray, ray_tmin:f64, ray_tmax:f64) -> Option<HitRecord>;


}