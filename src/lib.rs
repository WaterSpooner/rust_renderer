pub mod camera;
pub mod math;
pub mod ray;
pub mod ray_tracer;
pub mod objects {
    pub mod base_object;
    pub mod sphere;
    pub mod triangle;
}
pub mod scene;
pub mod materials {
    pub mod base_material;
    pub mod dielectric;
    pub mod lambertian;
    pub mod metal;
}
