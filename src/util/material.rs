use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3,
    pub specular_exponent: f32,
}

pub const IVORY: Material = Material {
    refractive_index: 1.0,
    albedo: [0.9, 0.5, 0.1, 0.0],
    diffuse_color: Vec3 {
        x: 0.4,
        y: 0.4,
        z: 0.3,
    },
    specular_exponent: 50.,
};

pub const GLASS: Material = Material {
    refractive_index: 1.5,
    albedo: [0.0, 0.9, 0.1, 0.8],
    diffuse_color: Vec3 {
        x: 0.6,
        y: 0.7,
        z: 0.8,
    },
    specular_exponent: 125.,
};

pub const RED_RUBBER: Material = Material {
    refractive_index: 1.0,
    albedo: [1.4, 0.3, 0.0, 0.0],
    diffuse_color: Vec3 {
        x: 0.3,
        y: 0.1,
        z: 0.1,
    },
    specular_exponent: 10.,
};

pub const MIRROR: Material = Material {
    refractive_index: 1.0,
    albedo: [0.0, 16.0, 0.8, 0.0],
    diffuse_color: Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    specular_exponent: 1425.,
};
