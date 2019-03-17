extern crate image;

pub mod scene;
pub mod vector3;
pub mod point;
pub mod rendering;

use std::fs::{File, OpenOptions};

use image::*;
use point::*;
use scene::*;
use rendering::*;
use vector3::*;

#[test]
fn test_can_render_scene() {
	let scene = Scene {
		width: 800,
		height: 600,
		fov: 65.0,
		objects: vec![Box::new(Sphere {
			center: Point { x: 0.0, y: 0.0, z: -5.0,},
			radius: 1.0,
			color: Color { red: 0.0, green: 1.0, blue: 0.0},
		}), Box::new(Sphere{
			center: Point { x: 1.0, y: 2.0, z: -7.0,},
			radius: 1.0,
			color: Color { red: 0.0, green: 0.0, blue: 1.0},
		}), Box::new(Sphere{
			center: Point { x: -1.0, y: 1.0, z: -3.0,},
			radius: 1.0,
			color: Color { red: 1.0, green: 0.0, blue: 0.0},
		}), Box::new(Plane{
			normal: Vector3{x: 0.0, y: -1.0, z: 0.0},
			center: Point{ x: 0.0, y: -2.0, z: 0.0},
			color: Color{ red: 1.0, green: 0.5, blue: 0.5},
		})],
	};
	
	let img: DynamicImage = render(&scene);
	assert_eq!(scene.width, img.width());
	assert_eq!(scene.height, img.height());

    img.save("test.png").unwrap();
}
