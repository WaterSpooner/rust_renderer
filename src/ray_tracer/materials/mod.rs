mod base_material;
mod dielectric;
mod lambertian;
mod metal;

pub use base_material::{Material, Scatterable};
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;