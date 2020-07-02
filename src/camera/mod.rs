use crate::vec3::*;
use crate::ray::*;


pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizonal: Vec3,
    vertical: Vec3
}


impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;
        let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
        let horizonal: Point3 = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical: Point3 = Vec3::new(0.0, viewport_height, 0.0);
        Self {
            origin,
            horizonal,
            vertical,
            lower_left_corner: origin - horizonal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length)
        }
    }


    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizonal + v*self.vertical - self.origin)
    }
}