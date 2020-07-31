use super::Vec3;

mod onb;
pub use onb::Onb;

mod cosine_pdf;
pub use cosine_pdf::CosinePDF;
mod hitable_pdf;
pub use hitable_pdf::HitablePDF;
mod mixture_pdf;
pub use mixture_pdf::MixturePDF;
mod rect;
pub use rect::RectXZArea;
mod normal_hitable_pdf;
pub use normal_hitable_pdf::NormalHitablePDF;

use rand::rngs::SmallRng;

pub trait PDF: Send + Sync {
    fn value(&self, direction: Vec3) -> f32;
    fn generate(&self, rng: &mut SmallRng) -> Vec3;
}

pub trait PDFHitable: Send + Sync {
    fn pdf_value(&self, _o: Vec3, _v: Vec3) -> f32 {
        0.0
    }
    fn random(&self, _o: Vec3, _rng: &mut SmallRng) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
