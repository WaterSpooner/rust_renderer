use crate::{math::Vec3, ray::Ray};

use rand::Rng;
pub struct Camera{
    pub pixel_delta_u:Vec3,
    pub pixel_delta_v:Vec3,
    pub pixel00_loc:Vec3,
    pub image_height:i32,
    pub image_width:i32,
    pub camera_position:Vec3,
    pub camera_direction:Vec3,
    pub sample_per_pixel:i32,
}

impl Camera{
    pub fn new(aspect_ratio:f64,focal_length:f64,viewport_height:f64,camera_position:Vec3,camera_direction:Vec3,image_width:i32,sample_per_pixel:i32) -> Camera {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = if image_height < 1 {1} else {image_height};
        let viewport_width = viewport_height * (image_width as f64/image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left = camera_position - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        Camera {
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            image_height,
            image_width,
            camera_position,
            camera_direction,
            sample_per_pixel,
        }
        
    }

    pub fn get_ray(&self,i:i32,j:i32) -> Ray{
        let mut rng = rand::thread_rng();
        let offset = Vec3::new(rng.gen_range(-0.5..=0.5), rng.gen_range(-0.5..=0.5), 0.0);
        let pixel_sample = self.pixel00_loc + ((i as f64 + offset.x()) * self.pixel_delta_u) + ((j as f64 + offset.y()) * self.pixel_delta_v);
        Ray::new(self.camera_position, pixel_sample - self.camera_position)
    }
}