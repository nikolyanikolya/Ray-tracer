use crate::util::material::{Material, GLASS, IVORY, MIRROR, RED_RUBBER};
use crate::util::vec3::{PixelOperations, Vec3};

#[derive(Clone, Copy)]
pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f32,
    pub(crate) material: Material,
}

const SPHERES: [Sphere; 4] = [
    Sphere {
        center: Vec3 {
            x: -3.,
            y: 0.,
            z: -16.,
        },
        radius: 2.,
        material: IVORY,
    },
    Sphere {
        center: Vec3 {
            x: -1.0,
            y: -1.5,
            z: -12.,
        },
        radius: 2.,
        material: GLASS,
    },
    Sphere {
        center: Vec3 {
            x: 1.5,
            y: -0.5,
            z: -18.,
        },
        radius: 3.,
        material: RED_RUBBER,
    },
    Sphere {
        center: Vec3 {
            x: 7.,
            y: 5.,
            z: -18.,
        },
        radius: 4.,
        material: MIRROR,
    },
];

const LIGHTS: [Vec3; 3] = [
    Vec3 {
        x: -20.,
        y: 20.,
        z: 20.,
    },
    Vec3 {
        x: 30.,
        y: 50.,
        z: -25.,
    },
    Vec3 {
        x: 30.,
        y: 20.,
        z: 30.,
    },
];

fn reflect(direction: &Vec3, vec_from_center_to_nearest_intersect: &Vec3) -> Vec3 {
    *direction
        - *vec_from_center_to_nearest_intersect
            * 2.
            * (*direction * *vec_from_center_to_nearest_intersect)
}

fn refract(
    direction: &Vec3,
    vec_from_center_to_nearest_intersect: &Vec3,
    eta_t: f32,
    eta_i: f32,
) -> Vec3 {
    let cos_direction: f32 =
        -((*direction * *vec_from_center_to_nearest_intersect).min(1.)).max(-1.);
    if cos_direction < 0. {
        return refract(
            direction,
            &(-(*vec_from_center_to_nearest_intersect)),
            eta_i,
            eta_t,
        );
    }
    let eta: f32 = eta_i / eta_t;
    let k: f32 = 1. - eta * eta * (1. - cos_direction * cos_direction);
    if k < 0. {
        Vec3 {
            x: 1.,
            y: 0.,
            z: 0.,
        }
    } else {
        *direction * eta + *vec_from_center_to_nearest_intersect * (eta * cos_direction - k.sqrt())
    }
}

fn scene_intersect(origin: &Vec3, direction: &Vec3) -> (bool, Vec3, Vec3, Material) {
    let mut nearest_distance: f32 = 1e10;
    let mut vec_from_center_to_nearest_intersect: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut vec_to_nearest_intersect: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    let mut material = Material {
        refractive_index: 1.,
        albedo: [2., 0., 0., 0.],
        diffuse_color: Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        specular_exponent: 0.0,
    };
    if direction.y.abs() > 0.001 {
        let distance: f32 = -(origin.y + 4.) / direction.y;
        let pixel: Vec3 = *origin + *direction * distance;
        if distance > 0.001
            && distance < nearest_distance
            && pixel.x.abs() < 10.
            && pixel.z < -10.
            && pixel.z > -30.
        {
            nearest_distance = distance;
            vec_from_center_to_nearest_intersect = Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            };
            material.diffuse_color =
                if (((0.5 * pixel.x + 1000.) as i32 + (0.5 * pixel.z) as i32) & 1) == 1 {
                    Vec3 {
                        x: 0.3,
                        y: 0.3,
                        z: 0.3,
                    }
                } else {
                    Vec3 {
                        x: 0.3,
                        y: 0.2,
                        z: 0.1,
                    }
                }
        }
    }

    for sphere in SPHERES {
        let (intersected, distance) = ray_sphere_intersect(origin, direction, &sphere);
        if !intersected || distance > nearest_distance {
            continue;
        }
        nearest_distance = distance;
        vec_to_nearest_intersect = *origin + *direction * nearest_distance;
        vec_from_center_to_nearest_intersect =
            (vec_to_nearest_intersect - sphere.center).normalized();
        material = sphere.material;
    }

    (
        nearest_distance < 1000.,
        vec_to_nearest_intersect,
        vec_from_center_to_nearest_intersect,
        material,
    )
}

fn ray_sphere_intersect(origin: &Vec3, ray_direction: &Vec3, sphere: &Sphere) -> (bool, f32) {
    let vec_to_center: Vec3 = sphere.center - *origin;
    let vec_to_center_projection: f32 = *ray_direction * vec_to_center;
    let distance2: f32 =
        vec_to_center * vec_to_center - vec_to_center_projection * vec_to_center_projection;
    if distance2 > sphere.radius * sphere.radius {
        return (false, 0.);
    }
    let chord_half: f32 = (sphere.radius * sphere.radius - distance2).sqrt();
    let nearest_intersect: f32 = vec_to_center_projection - chord_half;
    let farthest_intersection: f32 = vec_to_center_projection + chord_half;
    if nearest_intersect > 0.001 {
        return (true, nearest_intersect);
    }
    if farthest_intersection > 0.001 {
        (true, farthest_intersection)
    } else {
        (false, 0.)
    }
}

pub(crate) fn cast_ray(origin: &Vec3, direction: &Vec3, depth: i32) -> Vec3 {
    let (hit, vec_to_nearest_intersect, vec_from_center_to_nearest_intersect, material) =
        scene_intersect(origin, direction);

    if depth > 4 || !hit {
        return Vec3 {
            x: 0.2,
            y: 0.7,
            z: 0.8,
        };
    }

    let reflect_direction: Vec3 =
        reflect(direction, &vec_from_center_to_nearest_intersect).normalized();

    let refract_direction: Vec3 = refract(
        direction,
        &vec_from_center_to_nearest_intersect,
        material.refractive_index,
        1.,
    )
    .normalized();

    let reflect_color: Vec3 = cast_ray(&vec_to_nearest_intersect, &reflect_direction, depth + 1);
    let refract_color: Vec3 = cast_ray(&vec_to_nearest_intersect, &refract_direction, depth + 1);
    let mut diffuse_light_intensity: f32 = 0.;
    let mut specular_light_intensity: f32 = 0.;

    for light in LIGHTS {
        let light_direction: Vec3 = (light - vec_to_nearest_intersect).normalized();
        let (hit, shadow_pt, ..) = scene_intersect(&vec_to_nearest_intersect, &light_direction);
        if hit
            && (shadow_pt - vec_to_nearest_intersect).norm()
                < (light - vec_to_nearest_intersect).norm()
        {
            continue;
        }
        diffuse_light_intensity += (light_direction * vec_from_center_to_nearest_intersect).max(0.);
        specular_light_intensity +=
            (-reflect(&(-light_direction), &vec_from_center_to_nearest_intersect) * *direction)
                .max(0.)
                .powf(material.specular_exponent);
    }

    material.diffuse_color * diffuse_light_intensity * material.albedo[0]
        + Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        } * specular_light_intensity
            * material.albedo[1]
        + reflect_color * material.albedo[2]
        + refract_color * material.albedo[3]
}
