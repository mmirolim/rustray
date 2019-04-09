use crate::point::Point;
use crate::scene::{Color, Light, Plane, Scene, Sphere};
use crate::vector3::Vector3;
use image::*;
use std::fmt;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::thread;

pub fn render(scene: &Scene, start_width: u32, end_width: u32) -> DynamicImage {
    let bg = Rgba::from_channels(125, 125, 125, 0);
    let mut image = DynamicImage::new_rgb8(end_width - start_width, scene.height);
    for x in start_width..end_width {
        let x_on_image = x - start_width;
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if let Some(v) = scene.trace(&ray) {
                let hit_point = ray.origin + (ray.direction * v.distance);
                let surface_normal = v.obj.surface_normal(&hit_point);
                let light_reflected = v.obj.albedo() / std::f32::consts::PI;
                let mut color = Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                };
                for light in &scene.lights {
                    let light_color: Color;
                    let direction_to_light: Vector3;
                    let mut light_intensity: f32;

                    match light {
                        Light::Direct(light) => {
                            light_color = light.color;
                            light_intensity = light.intensity;
                            direction_to_light = -light.direction.normalize();
                        }

                        Light::Spherical(light) => {
                            light_color = light.color;
                            let r2 = (light.position - hit_point).norm() as f32;
                            light_intensity = light.intensity / (4.0 * ::std::f32::consts::PI * r2);
                            direction_to_light = (light.position - hit_point).normalize();
                        }
                    };

                    let shadow_ray = Ray {
                        origin: hit_point + (surface_normal * 1e-13),
                        direction: direction_to_light,
                    };
                    let shadow_intersection = scene.trace(&shadow_ray);
                    match light {
                        Light::Direct(light) => {
                            if shadow_intersection.is_some() {
                                light_intensity = 0.0;
                            }
                        }

                        Light::Spherical(light) => {
                            if shadow_intersection.is_some()
                                && shadow_intersection.unwrap().distance
                                    < light.distance(&hit_point)
                            {
                                light_intensity = 0.0;
                            }
                        }
                    };

                    let light_power =
                        (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;

                    color = color + v.obj.color() * light_color * light_power * light_reflected;
                }

                // TODO clamp color?
                image.put_pixel(x_on_image, y, color.clamp().to_rgba());
            } else {
                image.put_pixel(x_on_image, y, bg);
            }
        }
    }

    image
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn color(&self) -> Color;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn albedo(&self) -> f32;
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

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn albedo(&self) -> f32 {
        self.albedo
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

    fn color(&self) -> Color {
        self.color
    }

    fn surface_normal(&self, _hit_point: &Point) -> Vector3 {
        -self.normal.normalize()
    }

    fn albedo(&self) -> f32 {
        self.albedo
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
