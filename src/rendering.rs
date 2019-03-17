use crate::point::Point;
use crate::scene::{Scene, Color, Sphere, Plane};
use crate::vector3::Vector3;
use image::*;
use std::fmt;
use std::fmt::Debug;

pub fn render(scene: &Scene) -> DynamicImage {
	let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
	let black = Rgba::from_channels(0, 0, 0, 0);

	for x in 0..scene.width {
		for y in 0..scene.height {
			let ray = Ray::create_prime(x, y, scene);

			if let Some(v) = scene.trace(&ray) {
				image.put_pixel(x, y, v.obj.color().to_rgba());
			} else {
				image.put_pixel(x, y, black);
			}
		}
	}

	image
}

pub trait Intersectable {
	fn intersect(&self, ray: &Ray) -> Option<f64>;
	fn color(&self) -> Color;
}

pub struct Ray {
        pub origin: Point,
        pub direction: Vector3,
    }
    
impl Ray {
	pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
		// sensor dimension and position
		// the 2x2 sensor 1 unit from the camera
		// with coordinates (-1.0…1.0, -1.0…1.0)
		assert!(scene.width > scene.height);
		let aspect_ratio = (scene.width as f64) / (scene.height as f64);
		let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
		let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
		let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

		Ray {
			origin: Point::new(0.0, 0.0, 0.0), 
			direction: Vector3 {
				x: sensor_x,
				y: sensor_y,
				z: -1.0,
			}.
			normalize(),
		}
	}
}

impl Intersectable for Sphere {
	fn intersect(&self, ray: &Ray) -> Option<f64> {
		// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
		//Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (adj * adj);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
        	return None;
        }

        let d1 = (radius2 - d2).sqrt();
        let t0 = adj - d1;
        let t1 = adj + d2;

		if t0 < 0.0 && t1 < 0.0 {
			return None;
		}

		let distance = if t0 < t1 { t0 } else { t1 };

		Some(distance)
	}

	fn color(&self) -> Color {
		self.color
	}
}

impl Intersectable for Plane {
	fn intersect(&self, ray: &Ray) -> Option<f64> {
		let normal = &self.normal;
		let denom = normal.dot(&ray.direction);

		if denom > 1e-6 {
			let v = self.center - ray.origin;
			let distance = v.dot(&normal)/ denom;

			if distance >= 0.0 {
				return Some(distance);
			}
		}
		None
	}

	fn color(&self) -> Color {
		self.color
	}
}

pub struct Intersection<'a> {
	pub distance: f64,
	pub obj: &'a Box<dyn Intersectable>,
}


impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intersection")
    }
}


impl<'a> Intersection<'a> {
	pub fn new(distance: f64, obj: &'a Box<dyn Intersectable>) -> Intersection {
		Intersection{distance, obj}
	}
}

impl Scene {
	pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
		self.objects.iter()
			.filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
			.min_by(|a,b| a.distance.partial_cmp(&b.distance).unwrap())
	}
}
