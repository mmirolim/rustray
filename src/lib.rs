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
        fov: 90.0,
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
            Box::new(Sphere {
                center: Point {
                    x: -0.5,
                    y: 0.0,
                    z: -8.0,
                },
                radius: 1.5,
                color: Color {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                },
                albedo: 0.18,
            }),
            Box::new(Sphere {
                center: Point {
                    x: 3.5,
                    y: 0.0,
                    z: -7.0,
                },
                radius: 2.0,
                color: Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 1.0,
                },
                albedo: 0.3,
            }),
            Box::new(Sphere {
                center: Point {
                    x: 2.0,
                    y: 2.0,
                    z: -5.0,
                },
                radius: 2.0,
                color: Color {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                },
                albedo: 0.18,
            }),
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
                color: Color {
                    red: 1.0,
                    green: 0.5,
                    blue: 0.5,
                },
                albedo: 0.18,
            }),
        ],
    };
    // let sys_time = SystemTime::now();
    // let img: DynamicImage = render(&scene, 0, scene.width);
    // println!(
    //     "render execution time {} ms",
    //     sys_time.elapsed().unwrap().as_millis()
    // );
    // assert_eq!(scene.width, img.width());
    // assert_eq!(scene.height, img.height());

    // img.save("test.png").unwrap();

    let sys_time = SystemTime::now();
    let img: DynamicImage = render_in_threads(scene, 8);
    println!(
        "render_in_threads execution time {} ms",
        sys_time.elapsed().unwrap().as_millis()
    );
    img.save("test-multithreaded.png").unwrap();
}

#[test]
fn test_image_blend() {
    // use liner blend
    let img_l = image::open("img-left-light.png").unwrap();
    let img_r = image::open("img-right-light.png").unwrap();
    let mut img_comb = DynamicImage::new_rgb8(img_l.width(), img_l.height());

    for (x, y, pixel) in img_l.pixels() {
        let p_r = img_r.get_pixel(x, y);
        let r: u32 = (p_r[0] as u32 + pixel[0] as u32).min(255);
        let g: u32 = (p_r[1] as u32 + pixel[1] as u32).min(255);
        let b: u32 = (p_r[2] as u32 + pixel[2] as u32).min(255);

        let p = image::Rgba([r as u8, g as u8, b as u8, 0]);
        img_comb.put_pixel(x, y, p);
    }

    img_comb.save("img-combined.png").unwrap();
}
