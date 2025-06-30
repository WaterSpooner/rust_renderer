mod camera;
mod ray;
mod ray_tracer;
mod math;
mod scene;

pub mod objects;
pub mod materials;

pub use camera::Camera;
pub use ray::Ray;
pub use math::Vec3;
pub use ray_tracer::render;
pub use scene::Scene;