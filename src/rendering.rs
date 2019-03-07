use crate::point::Point;
use crate::scene::{Scene};
use crate::vector3::Vector3;
use image::*;

pub fn render(scene: &Scene) -> DynamicImage {
	DynamicImage::new_rgb8(scene.width, scene.height)
}

pub struct Ray {
        pub origin: Point,
        pub direction: Vector3,
    }
    
impl Ray {
	pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
		Ray { 
			origin: Point::new(0.0, 0.0, 0.0), 
			direction: Vector3::new(0.0, 0.0, 0.0),
		}
	}
}

