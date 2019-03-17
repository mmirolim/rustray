
use crate::point::Point;
use crate::vector3::Vector3;
use crate::rendering::{Ray, Intersectable};

use image::*;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
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
#[derive(Debug)]
pub struct Sphere {
	pub center: Point,
	pub radius: f64,
	pub color: Color,
}

#[derive(Debug)]
pub struct Plane {
	pub center: Point,
	pub normal: Vector3,
	pub color: Color,
}

pub struct Scene {
        pub width: u32,
        pub height: u32,
        pub fov: f64,
        pub objects: Vec<Box<dyn Intersectable>>,
    }

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene")
    }
}


