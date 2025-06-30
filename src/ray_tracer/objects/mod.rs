mod base_object;
mod sphere;
mod triangle;

pub use base_object::{HitRecord, Object, Hittable};
pub use sphere::Sphere;
pub use triangle::Triangle;