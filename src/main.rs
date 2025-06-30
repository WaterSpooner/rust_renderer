use rust_renderer::ray_tracer::*;
use std::sync::Arc;
use std::time::Instant;
use std::fs;
use std::path::Path;

fn main() {
    let camera = Camera::new(
            16.0/9.0,
            1000,
            60,
            10,
            20.0,
            Vec3::new(13.0,2.0,3.0),
            Vec3::new(0.0,0.0,0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.2,
            10.0,
        
    );

    let lambertian_material = Arc::new(materials::Material::Lambertian(
        materials::Lambertian::new(Vec3::new(
            0.6588, 0.6863, 0.7137,
    ))));

    let metal_material = Arc::new(materials::Material::Metal(
        materials::Metal::new(Vec3::new(
            0.5, 0.3, 0.8), 0.0
    )));

    let dielectric_material = Arc::new(materials::Material::Dielectric(
        materials::Dielectric::new(1.0/1.3))
    );

    let sphere1 = objects::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, dielectric_material.clone());
    let sphere3 = objects::Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, metal_material.clone());
    let triangle1 = objects::Triangle::new(
        Vec3::new(100.0, -0.5, 100.0),
        Vec3::new(-100.0, -0.5, -100.0),
        Vec3::new(-100.0, -0.5, 100.0),
        lambertian_material.clone(),
    );
    let triangle2 = objects::Triangle::new(
        Vec3::new(100.0, -0.5, -100.0),
        Vec3::new(-100.0, -0.5, -100.0),
        Vec3::new(100.0, -0.5, 100.0),
        lambertian_material.clone(),
    );

    let object_vec: Vec<objects::Object> = vec![
        objects::Object::Sphere(sphere1),
        objects::Object::Sphere(sphere3),
        objects::Object::Triangle(triangle1),
        objects::Object::Triangle(triangle2),
    ];
    let scene = Scene {
        objects: object_vec,
        camera,
    };

    let now = Instant::now();
    println!("starting render now...");
    render(&scene);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);


    let json = serde_json::to_string_pretty(&scene).unwrap();
    let out_dir = Path::new("output");
    fs::create_dir_all(out_dir).unwrap();
    let out_path = out_dir.join("scene.json");
    fs::write(out_path, json).unwrap();

}