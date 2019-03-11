
use crate::point::Point;
use crate::vector3::Vector3;
use crate::rendering::Ray;

use image::*;

pub struct Color {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

impl Color {
	 pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (gamma_encode(self.red) * 255.0) as u8,
            (gamma_encode(self.green) * 255.0) as u8,
            (gamma_encode(self.blue) * 255.0) as u8,
            255,
        )
    }
}
pub struct Sphere {
	pub center: Point,
	pub radius: f64,
	pub color: Color,
}

pub struct Scene {
        pub width: u32,
        pub height: u32,
        pub fov: f64,
        pub sphere: Sphere,
    }

impl Sphere {
	pub fn intersect(&self, ray: &Ray) -> bool {
	let center = Point {
            x: self.center.x,
            y: self.center.y,
            z: self.center.z,
        };

        let ray_origin = Point {
        	x: ray.origin.x,
            y: ray.origin.y,
            z: ray.origin.z,
        };
		//Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = center - ray_origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
	}
}   
