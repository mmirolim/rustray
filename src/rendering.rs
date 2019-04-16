use crate::point::Point;
use crate::scene::{Color, Material, Plane, Scene, Sphere, TextureCoords};
use crate::vector3::Vector3;
use image::*;
use std::f32;
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
    let texture_coords: TextureCoords;

    if let Some(v) = scene.trace(&ray) {
        material = v.obj.material();
        hit_point = ray.origin + (ray.direction * v.distance);
        surface_normal = v.obj.surface_normal(&hit_point);
        texture_coords = v.obj.texture_coords(&hit_point);
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
            color = color + material.color(&texture_coords) * light_color;
        }
    }

    if material.surface_type.refractive_index > 0.0 {
        let reflection_color: Color;
        let mut refraction_color = Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        };

        let coeff_r = ray.fresnel(&surface_normal, 1.0, material.surface_type.refractive_index);
        if coeff_r < 1.0 {
            if let Some(dir) =
                ray.refract(&surface_normal, 1.0, material.surface_type.refractive_index)
            {
                let refracted_ray = Ray {
                    origin: if ray.direction.dot(&surface_normal) < 0.0 {
                        hit_point + surface_normal
                    } else {
                        hit_point - surface_normal
                    },

                    direction: dir,
                };
                refraction_color = get_color(scene, &refracted_ray, depth + 1);
            }
        }
        let reflected_ray = Ray {
            origin: hit_point + (surface_normal),
            direction: ray.reflect_direction(&surface_normal),
        };
        reflection_color = get_color(scene, &reflected_ray, depth + 1);
        color = color + reflection_color * coeff_r + refraction_color * (1.0 - coeff_r);
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
    fn surface_normal(&self, point: &Point) -> Vector3;
    fn material(&self) -> &Material;
    fn texture_coords(&self, point: &Point) -> TextureCoords;
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

    pub fn refract(&self, normal: &Vector3, mut ior_from: f32, mut ior_to: f32) -> Option<Vector3> {
        let mut normal = normal.normalize();
        // cos theta
        let mut cosi = normal.dot(&self.direction) as f32;

        if cosi < 0.0 {
            // hit from outside of surface
            cosi = -cosi;
        } else {
            // we are inside the surface
            normal = -normal;
            // swap the refraction indices
            let tmp = ior_from;
            ior_from = ior_to;
            ior_to = tmp;
        }

        // n = n1/n2
        let eta = ior_from / ior_to;

        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            // total internal reflection
            return None;
        }

        Some(eta * self.direction + (eta * cosi - k.sqrt()) * normal)
    }

    // returns coeff of reflected light
    fn fresnel(&self, normal: &Vector3, mut ior_from: f32, mut ior_to: f32) -> f32 {
        let mut cosi = self.direction.dot(normal).min(1.0).max(-1.0) as f32;
        if cosi > 0.0 {
            // swap the refraction indices
            let tmp = ior_from;
            ior_from = ior_to;
            ior_to = tmp;
        }
        let eta = ior_from / ior_to;
        let sint = eta * (1.0 - cosi * cosi).max(0.0).sqrt();

        if sint >= 1.0 {
            // total internal reflection
            return 1.0;
        }

        let cost = (1.0 - sint * sint).max(0.0).sqrt();
        cosi = cosi.abs();
        let r1 = (ior_to * cosi - ior_from * cost) / (ior_to * cosi + ior_from * cost);
        let r2 = (ior_from * cost - ior_to * cosi) / (ior_from * cost + ior_from * cosi);

        (r1 * r1 + r2 * r2) / 2.0
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

    fn texture_coords(&self, point: &Point) -> TextureCoords {
        let vec_to_point = *point - self.center;
        TextureCoords {
            x: (1.0 + (vec_to_point.z.atan2(vec_to_point.x) as f32) / f32::consts::PI) * 0.5,
            y: (vec_to_point.y / self.radius).acos() as f32 / f32::consts::PI,
        }
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
    fn texture_coords(&self, point: &Point) -> TextureCoords {
        let formard_vec = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let up_vec = Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let mut x_axis = self.normal.cross(&formard_vec);
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&up_vec);
        }

        let y_axis = self.normal.cross(&x_axis);

        let vec_to_point = *point - self.center;

        TextureCoords {
            x: vec_to_point.dot(&x_axis) as f32,
            y: vec_to_point.dot(&y_axis) as f32,
        }
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
