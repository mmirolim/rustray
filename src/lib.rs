extern crate image;

pub mod point;
pub mod rendering;
pub mod scene;
pub mod vector3;

use image::*;
use point::*;
use rendering::*;
use scene::*;
use std::time::SystemTime;
use vector3::*;

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 95.0,
        bg_color: Color {
            red: 0.2,
            blue: 0.8,
            green: 0.6,
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
                    y: 0.0,
                    z: -8.0,
                },
                1.5,
                Material {
                    color: Color {
                        red: 0.0,
                        green: 1.0,
                        blue: 0.0,
                    },
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.0,
                        reflect_ratio: 0.0,
                        refractive_index: 1.3,
                    },
                },
            )),
            Box::new(Sphere::new(
                Point {
                    x: 3.5,
                    y: 0.0,
                    z: -7.0,
                },
                2.0,
                Material {
                    color: Color {
                        red: 0.0,
                        green: 0.0,
                        blue: 1.0,
                    },
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
                    y: 2.0,
                    z: -5.0,
                },
                2.0,
                Material {
                    color: Color {
                        red: 1.0,
                        green: 0.0,
                        blue: 0.0,
                    },
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.18,
                        reflect_ratio: 0.0,
                        refractive_index: 0.0,
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
                    color: Color {
                        red: 1.0,
                        green: 0.5,
                        blue: 0.5,
                    },
                    surface_type: SurfaceType {
                        diffuse_albedo: 0.18,
                        reflect_ratio: 0.0,
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
