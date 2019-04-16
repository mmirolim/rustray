use crate::point::Point;
use crate::rendering::Intersectable;
use crate::vector3::Vector3;

use image::*;
use std::fmt;
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
    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: (rgba.data[0] as f32) / 255.0,
            green: (rgba.data[1] as f32) / 255.0,
            blue: (rgba.data[2] as f32) / 255.0,
        }
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
        let c = Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        };
        c
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        let c = Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        };
        c
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
        let c = Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        };
        c
    }
}

#[derive(Debug)]
pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct SurfaceType {
    pub diffuse_albedo: f32,
    pub reflect_ratio: f32,
    pub refractive_index: f32,
}

pub enum ColorType {
    Color(Color),
    Texture(DynamicImage),
}

impl ColorType {
    pub fn color(&self, coords: &TextureCoords) -> Color {
        match *self {
            ColorType::Color(c) => Color {
                red: c.red,
                green: c.green,
                blue: c.blue,
            },
            ColorType::Texture(ref tex) => {
                let tex_x = wrap(coords.x, tex.width());
                let tex_y = wrap(coords.y, tex.height());
                Color::from_rgba(tex.get_pixel(tex_x, tex_y))
            }
        }
    }
}

pub struct Material {
    pub color: ColorType,
    pub surface_type: SurfaceType,
}

impl Material {
    pub fn color(&self, coords: &TextureCoords) -> Color {
        self.color.color(coords)
    }
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    // store radius square
    pub radius_sq: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            radius_sq: radius * radius,
            material,
        }
    }
}

pub struct Plane {
    pub center: Point,
    pub normal: Vector3,
    pub material: Material,
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

pub enum Light {
    Direct(DirectLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn distance(&self, hit_point: &Point) -> f64 {
        match self {
            Light::Direct(_) => ::std::f64::INFINITY,
            Light::Spherical(l) => (l.position - *hit_point).length(),
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match self {
            Light::Direct(l) => l.intensity,
            Light::Spherical(l) => {
                let r2 = (l.position - *hit_point).norm() as f32;
                l.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn direction(&self, hit_point: &Point) -> Vector3 {
        match self {
            Light::Direct(l) => -l.direction.normalize(),
            Light::Spherical(l) => (l.position - *hit_point).normalize(),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Light::Direct(l) => l.color,
            Light::Spherical(l) => l.color,
        }
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Box<dyn Intersectable + Sync + Send>>,
    pub lights: Vec<Light>,
    pub bg_color: Color,
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene")
    }
}
