extern crate image;

pub mod point;
pub mod rendering;
pub mod scene;
pub mod vector3;

use image::*;
use point::*;
use rendering::*;
use scene::*;
use std::fs::{File, OpenOptions};
use std::time::SystemTime;
use vector3::*;

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        lights: vec![
            Light::Direct(DirectLight {
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 1.0,
                direction: Vector3 {
                    x: 3.0,
                    y: -1.5,
                    z: -2.0,
                },
            }),
            Light::Direct(DirectLight {
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 1.0,
                direction: Vector3 {
                    x: -3.0,
                    y: -4.0,
                    z: -2.0,
                },
            }),
            Light::Spherical(SphericalLight {
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 40.0,
                position: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -2.0,
                },
            }),
        ],
        objects: vec![
            Box::new(Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
                albedo: 0.3,
            }),
            Box::new(Sphere {
                center: Point {
                    x: 1.0,
                    y: 2.0,
                    z: -7.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                },
                albedo: 0.58,
            }),
            Box::new(Sphere {
                center: Point {
                    x: -1.0,
                    y: 1.0,
                    z: -3.0,
                },
                radius: 1.0,
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
                albedo: 0.3,
            }),
            Box::new(Plane {
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                center: Point {
                    x: 0.0,
                    y: -2.0,
                    z: 0.0,
                },
                color: Color {
                    red: 1.0,
                    green: 0.5,
                    blue: 0.5,
                },
                albedo: 0.18,
            }),
        ],
    };
    let sys_time = SystemTime::now();
    let img: DynamicImage = render(&scene, 0, scene.width);
    println!(
        "render execution time {} ms",
        sys_time.elapsed().unwrap().as_millis()
    );
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    img.save("test.png").unwrap();

    let sys_time = SystemTime::now();
    let img: DynamicImage = render_in_threads(scene, 8);
    println!(
        "render_in_threads execution time {} ms",
        sys_time.elapsed().unwrap().as_millis()
    );
    img.save("test-multithreaded.png").unwrap();
}
