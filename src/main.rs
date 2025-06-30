use rust_renderer::camera::Camera;
use rust_renderer::materials::base_material::Material;
use rust_renderer::materials::dielectric::Dielectric;
use rust_renderer::materials::lambertian::Lambertian;
use rust_renderer::materials::metal::Metal;
use rust_renderer::math::Vec3;
use rust_renderer::objects::base_object::Object;
use rust_renderer::objects::sphere::Sphere;
use rust_renderer::objects::triangle::Triangle;
use rust_renderer::ray_tracer;
use rust_renderer::scene::Scene;
use std::sync::Arc;
use std::time::Instant;
use std::fs;
use std::path::Path;

fn main() {
    let camera = Camera::new(
            16.0/9.0,
            1920,
            600,
            10,
            20.0,
            Vec3::new(13.0,2.0,3.0),
            Vec3::new(0.0,0.0,0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.2,
            10.0,
        
    );

    let lambertian_material = Arc::new(Material::Lambertian(Lambertian::new(Vec3::new(
        0.1, 0.4, 0.01,
    ))));
    let metal_material = Arc::new(Material::Metal(Metal::new(Vec3::new(0.5, 0.3, 0.8), 0.0)));
    let dielectric_material = Arc::new(Material::Dielectric(Dielectric::new(1.0/1.3)));

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, dielectric_material.clone());
    let sphere3 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, metal_material.clone());
    let triangle1 = Triangle::new(
        Vec3::new(100.0, -0.5, 100.0),
        Vec3::new(-100.0, -0.5, -100.0),
        Vec3::new(-100.0, -0.5, 100.0),
        lambertian_material.clone(),
    );
    let triangle2 = Triangle::new(
        Vec3::new(100.0, -0.5, -100.0),
        Vec3::new(-100.0, -0.5, -100.0),
        Vec3::new(100.0, -0.5, 100.0),
        lambertian_material.clone(),
    );

    let object_vec: Vec<Object> = vec![
        Object::Sphere(sphere1),
        Object::Sphere(sphere3),
        Object::Triangle(triangle1),
        Object::Triangle(triangle2),
    ];
    let scene = Scene {
        objects: object_vec,
        camera,
    };

    let now = Instant::now();
    println!("starting render now...");
    ray_tracer::render(&scene);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    let json = serde_json::to_string_pretty(&scene).unwrap();
    let out_dir = Path::new("output");
    fs::create_dir_all(out_dir).unwrap();
    let out_path = out_dir.join("scene.json");
    fs::write(out_path, json).unwrap();

}