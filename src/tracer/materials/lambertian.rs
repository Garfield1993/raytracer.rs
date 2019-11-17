use super::{HitRecord, Material, Ray, Texture, Vec3};
use crate::tracer::utils::random_in_unit_sphere;
use std::sync::Arc;
use std::f32::consts::PI;

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub fn new_arc(albedo: Box<dyn Texture>) -> Arc<Self> {
        Arc::new(Self { albedo })
    }
}

impl Material for Lambertian {
    /*
            bool scatter(const ray& r_in,
            const hit_record& rec, vec3& alb, ray& scattered, float& pdf) const {
            vec3 target = rec.p + rec.normal + random_in_unit_sphere();
            scattered = ray(rec.p, unit_vector(target-rec.p), r_in.time());
            alb = albedo->value(rec.u, rec.v, rec.p);
            pdf = dot(rec.normal, scattered.direction()) / M_PI;
            return true;
        }
        */
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray, f32)> {
        let direction = hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, direction);
        Some((
            self.albedo.value(hit_record.u, hit_record.v, hit_record.p),
            Ray::new(hit_record.p, direction),
            Vec3::dot(hit_record.normal, scattered.direction) / PI
        ))
    }

    fn scattering_pdf(&self, _: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f32 {
        let cosine = Vec3::dot(hit_record.normal, scattered.direction.unit());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
