use crate::point::Point;
use crate::scene::{Scene, Color};
use crate::vector3::Vector3;
use image::*;

pub fn render(scene: &Scene) -> DynamicImage {
	let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
	let black = Rgba::from_channels(0, 0, 0, 0);

	for x in 0..scene.width {
		for y in 0..scene.height {
			let ray = Ray::create_prime(x, y, scene);

			if scene.sphere.intersect(&ray) {
				image.put_pixel(x, y, scene.sphere.color.to_rgba());
			} else {
				image.put_pixel(x, y, black);
			}
		}
	}

	image
}

pub struct Ray {
        pub origin: Point,
        pub direction: Vector3,
    }
    
impl Ray {
	pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
		assert!(scene.width > scene.height);
		let aspect_ratio = (scene.width as f64) / (scene.height as f64);
		let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
		let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
		let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

		Ray {
			origin: Point::new(0.0, 0.0, 0.0), 
			direction: Vector3 {
				x: sensor_x,
				y: sensor_y,
				z: -1.0,
			}.
			normalize(),
		}
	}
}

pub trait Intersectable {
	fn intersect(&self, ray: &Ray) -> bool;
}


