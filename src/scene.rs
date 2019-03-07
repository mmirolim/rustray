
use crate::point::Point;
use crate::vector3::Vector3;

pub struct Color {
	pub red: f32,
	pub green: f32,
	pub blue: f32,
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
   
