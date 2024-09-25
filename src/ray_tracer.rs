use core::f64;
use crate::math::Vec3;
use crate::objects::base_object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::scene::Scene;

pub fn render(scene: &Scene) -> (){
    let mut output: Vec<u8> = Vec::new();

    for j in 0..scene.camera.image_height{
        for i in 0..scene.camera.image_width{
            let mut pixel_color = Vec3::zero();
            for _k in 0..scene.camera.sample_per_pixel{
            let ray = scene.camera.get_ray(i, j);
            pixel_color = pixel_color + colour(scene,&ray,10);
            }
            pixel_color = pixel_color/scene.camera.sample_per_pixel as f64;
            output.push((255.999 * linear_to_gamma(pixel_color.x())) as u8);
            output.push((255.999 * linear_to_gamma(pixel_color.y())) as u8);
            output.push((255.999 * linear_to_gamma(pixel_color.z())) as u8);
        }
    }
    write_file(&output,scene.camera.image_height, scene.camera.image_width);
}


fn hit_world(ray: &Ray, scene: &Scene) -> Option<HitRecord>{
    let mut hit_record = None;
    let mut closest = f64::INFINITY;
    for object in &scene.objects{
        if let Some(hit_object) = object.hit(&ray, 0.000001, closest){
            closest = hit_object.t;
            hit_record = Some(hit_object);
        }
    }
    hit_record
}

fn colour(scene: &Scene, ray:&Ray, maxdepth : i32) -> Vec3 {
    if maxdepth <= 0 {return Vec3::zero();}
    let hit_record = hit_world(ray, scene);
    match hit_record{
        Some(hit_record) => {
            let n = hit_record.normal + Vec3::unit_random();
            return 0.5*colour(scene, &Ray::new(hit_record.p,n),maxdepth-1);
        }
        None => {
            let unit_direction = ray.direction.unit_vector();
            let a = 0.5*(unit_direction.y() + 1.0);
            return (1.0-a)*Vec3::new(1.0, 1.0, 1.0) + a*Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn write_file(output: &Vec<u8>,height:i32,width:i32) -> (){
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;

    let path = Path::new("output/test.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(output.as_slice()).unwrap(); // Save
}

fn linear_to_gamma(linear_component : f64) -> f64
{
    if linear_component > 0.0 {return linear_component.sqrt();}
    return 0.0;
}