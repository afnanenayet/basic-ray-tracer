use super::{BSDFRecord, BSDF};
use hittable::HitRecord;
use na::Real;
use num::FromPrimitive;
use ray::Ray;
use sample::unit_sphere;

/// Holds the properties for a diffuse BSDF
pub struct Diffuse<N: Real> {
    /// The fraction of light that is absorbed by the material
    pub albedo: N,
}

impl<N: FromPrimitive + Real> BSDF<N> for Diffuse<N> {
    // note that the incoming angle doesn't matter for a lambertian surface, which is why we ignore
    // the incoming ray
    fn scatter(&self, _in_ray: &Ray<N>, hit_record: &HitRecord<N>) -> BSDFRecord<N> {
        let target = hit_record.p + hit_record.normal + unit_sphere();
        let scattered = Ray {
            origin: hit_record.p,
            direction: target - hit_record.p,
        };
        let atten = self.albedo;

        BSDFRecord {
            out_scattered: scattered,
            attenuated: atten,
        }
    }
}
