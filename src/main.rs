use rust_renderer::ray_tracer;
use rust_renderer::camera::Camera;
use rust_renderer::math::Vec3;
use rust_renderer::scene::Scene;
use rust_renderer::objects::sphere::Sphere;

use std::time::Instant;
fn main() {
    let camera = Camera::new(
        16.0/9.0, 
        1.0, 
        2.0, 
        Vec3::new(0.0,0.0,0.0), 
        Vec3::new(0.0,0.0,0.0), 
        400,
        100);
    let sphere1 = Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0);
    let sphere_vec = vec![sphere1,sphere2];
    let scene = Scene {objects:sphere_vec,camera};

    let now = Instant::now();
    println!("starting render now...");
    ray_tracer::render(&scene);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}