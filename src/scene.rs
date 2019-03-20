use crate::point::Point;
use crate::rendering::{Intersectable, Ray};
use crate::vector3::Vector3;

use image::*;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, Mul};

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
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Debug)]
pub struct Plane {
    pub center: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

pub struct DirectLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}
impl SphericalLight {
    pub fn distance(&self, hit_point: &Point) -> f64 {
        (self.position - *hit_point).length()
    }
}
pub enum Light {
    Direct(DirectLight),
    Spherical(SphericalLight),
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Light>,
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene")
    }
}
