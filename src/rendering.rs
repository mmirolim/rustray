use crate::point::Point;
use crate::scene::{Color, Material, Plane, Scene, Sphere};
use crate::vector3::Vector3;
use image::*;
use std::fmt;
use std::sync::Arc;
use std::thread;

pub fn render(scene: &Scene, start_width: u32, end_width: u32) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(end_width - start_width, scene.height);
    for x in start_width..end_width {
        let x_on_image = x - start_width;
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            image.put_pixel(x_on_image, y, get_color(scene, &ray, 0).to_rgba());
        }
    }

    image
}

fn get_color(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };
    // max depth
    if depth > 5 {
        return scene.bg_color;
    }

    let material: &Material;
    let hit_point: Point;
    let surface_normal: Vector3;

    if let Some(v) = scene.trace(&ray) {
        material = v.obj.material();
        hit_point = ray.origin + (ray.direction * v.distance);
        surface_normal = v.obj.surface_normal(&hit_point);
    } else {
        return scene.bg_color;
    }

    if material.surface_type.diffuse_albedo > 0.0 {
        let light_reflected = material.surface_type.diffuse_albedo / std::f32::consts::PI;
        for light in &scene.lights {
            let direction_to_light = light.direction(&hit_point);

            let shadow_ray = Ray {
                origin: hit_point + (surface_normal),
                direction: direction_to_light,
            };

            let shadow_intersection = scene.trace(&shadow_ray);
            let light_intensity = if shadow_intersection.is_none()
                || shadow_intersection.unwrap().distance > light.distance(&hit_point)
            {
                light.intensity(&hit_point)
            } else {
                0.0
            };
            let light_power =
                (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;

            let light_color = light.color() * light_power * light_reflected;
            color = color + material.color * light_color;
        }
    } else if material.surface_type.reflect_ratio > 0.0 {
        let reflected_ray = Ray {
            origin: hit_point + (surface_normal),
            direction: ray.reflect_direction(&surface_normal),
        };
        color = color
            + material.surface_type.reflect_ratio * get_color(scene, &reflected_ray, depth + 1);
    }

    color.clamp()
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn material(&self) -> &Material;
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
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }

    // returns reflection direction
    pub fn reflect_direction(&self, normal: &Vector3) -> Vector3 {
        let normal = normal.normalize();
        (self.direction - 2.0 * self.direction.dot(&normal) * normal).normalize()
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
        let d2 = l.dot(&l) - (adj * adj);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        if d2 > self.radius_sq {
            return None;
        }

        let d1 = (self.radius_sq - d2).sqrt();
        let t0 = adj - d1;
        let t1 = adj + d1;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);

        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(&normal) / denom;

            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _hit_point: &Point) -> Vector3 {
        -self.normal
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub obj: &'a Box<dyn Intersectable + Sync + Send>,
}

impl<'a> fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intersection")
    }
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, obj: &'a Box<dyn Intersectable + Sync + Send>) -> Intersection {
        Intersection { distance, obj }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap())
    }
}

pub fn render_in_threads(scene: Scene, threads_num: u32) -> DynamicImage {
    // TODO use randomized blocks to render scene
    let scene = Arc::new(scene);
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let mut workers = vec![];
    let mut images = vec![];
    let div = scene.width % threads_num;
    let stripe_size: u32 = if div == 0 {
        scene.width / threads_num
    } else {
        scene.width / threads_num + 1
    };

    for i in 0..threads_num {
        let scene = Arc::clone(&scene);
        workers.push(thread::spawn(move || -> (DynamicImage, u32, u32) {
            let start_width = i * stripe_size;
            let end_width = if (i + 1) * stripe_size > scene.width {
                scene.width
            } else {
                (i + 1) * stripe_size
            };
            let image = render(&scene, start_width, end_width);
            (image, start_width, end_width)
        }));
    }

    for worker in workers {
        let image = worker.join().unwrap();
        images.push(image);
    }

    for (i, v) in images.iter().enumerate() {
        if !image.copy_from(&v.0, v.1, 0) {
            panic!("image {} not copied", i);
        }
    }

    image
}
