use crate::ray_tracer::materials::Scatterable;
use crate::ray_tracer::Vec3;
use crate::ray_tracer::objects::HitRecord;
use crate::ray_tracer::objects::Hittable;
use crate::ray_tracer::Ray;
use crate::ray_tracer::Scene;
use core::f64;
use rayon::prelude::*;

pub fn render(scene: &Scene) {
    let image_pixel_count = scene.camera.image_height * scene.camera.image_width;
    let mut output = vec![0u8; image_pixel_count as usize * 3];
    output.par_chunks_mut(3).enumerate().for_each(|(m, pixel)| {
        let j = m / scene.camera.image_width as usize;
        let i = m % scene.camera.image_width as usize;
        let mut pixel_color = Vec3::zero();
        for _k in 0..scene.camera.samples_per_pixel {
            let ray = scene.camera.get_ray(i as i32, j as i32);
            pixel_color = pixel_color + colour(scene, &ray, scene.camera.max_depth);
        }
        pixel_color = pixel_color / scene.camera.samples_per_pixel as f64;
        pixel[0] = (255.999 * linear_to_gamma(pixel_color.x())) as u8;
        pixel[1] = (255.999 * linear_to_gamma(pixel_color.y())) as u8;
        pixel[2] = (255.999 * linear_to_gamma(pixel_color.z())) as u8;
    });
    write_file(&output, scene.camera.image_height, scene.camera.image_width);
}

fn ray_intersection(ray: &Ray, scene: &Scene) -> Option<HitRecord> {
    let mut hit_record = None;
    let mut closest = f64::INFINITY;
    for object in &scene.objects {
        if let Some(hit_object) = object.hit(ray, 0.000001, closest) {
            closest = hit_object.t;
            hit_record = Some(hit_object);
        }
    }
    hit_record
}

fn colour(scene: &Scene, ray: &Ray, max_depth: i32) -> Vec3 {
    if max_depth <= 0 {
        return Vec3::zero();
    }
    //Object color
    if let Some(hit) = ray_intersection(ray, scene) {
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            return attenuation * colour(scene, &scattered, max_depth - 1);
        } else {
            return Vec3::zero();
        }
    }
    // Background color
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

fn write_file(output: &Vec<u8>, height: i32, width: i32) {
    use std::fs::File;
    use std::io::BufWriter;
    use std::path::Path;

    let path = Path::new("output/test.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(output.as_slice()).unwrap(); // Save
}
