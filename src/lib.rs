extern crate image;

pub mod point;
pub mod rendering;
pub mod scene;
pub mod vector3;

use image::*;
use point::*;
use rendering::*;
use scene::{
    Color, ColorType, DirectLight, Light, Material, Plane, Scene, Sphere, SphericalLight,
    SurfaceType,
};
use std::time::SystemTime;
use vector3::*;

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        bg_color: Color {
            red: 0.01,
            green: 0.02,
            blue: 0.05,
        },
        lights: vec![
            Light::Direct(DirectLight {
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 8.0,
                direction: Vector3 {
                    x: 1.0,
                    y: -3.5,
                    z: -4.0,
                },
            }),
            Light::Spherical(SphericalLight {
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 2000.0,
                position: Point {
                    x: 2.0,
                    y: -1.0,
                    z: -4.5,
                },
            }),
        ],
        objects: vec![
            Box::new(Sphere::new(
                Point {
                    x: -0.5,
                    y: 0.5,
                    z: -8.5,
                },
                1.5,
                Material {
                    color: ColorType::Color(Color {
                        red: 0.0,
                        green: 1.0,
                        blue: 0.0,
                    }),
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.0,
                        reflect_ratio: 0.7,
                        refractive_index: 0.0,
                    },
                },
            )),
            Box::new(Sphere::new(
                Point {
                    x: -3.6,
                    y: 1.5,
                    z: -7.0,
                },
                2.0,
                Material {
                    color: ColorType::Texture(image::open("chessboard.png").unwrap()),
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.3,
                        reflect_ratio: 0.0,
                        refractive_index: 0.0,
                    },
                },
            )),
            Box::new(Sphere::new(
                Point {
                    x: 2.0,
                    y: 1.7,
                    z: -5.0,
                },
                2.0,
                Material {
                    color: ColorType::Color(Color {
                        red: 1.0,
                        green: 0.0,
                        blue: 0.0,
                    }),
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.0,
                        reflect_ratio: 0.0,
                        refractive_index: 1.5,
                    },
                },
            )),
            Box::new(Plane {
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                center: Point {
                    x: 0.0,
                    y: -3.0,
                    z: 0.0,
                },
                material: Material {
                    color: ColorType::Texture(image::open("chessboard.png").unwrap()),
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.18,
                        reflect_ratio: 0.5,
                        refractive_index: 0.0,
                    },
                },
            }),
        ],
    };

    let sys_time = SystemTime::now();
    let img: DynamicImage = render_in_threads(scene, 8);
    println!(
        "render_in_threads execution time {} ms",
        sys_time.elapsed().unwrap().as_millis()
    );
    img.save("test-multithreaded.png").unwrap();
}
