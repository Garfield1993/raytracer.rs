use super::{HitRecord, Ray, Texture, Vec3};

use super::Material;

mod diffuse_light;
pub use diffuse_light::DiffuseLight;
mod lambertian;
pub use lambertian::Lambertian;
