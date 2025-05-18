pub use micromath::{
    vector::{Component, F32x2, Vector},
    F32Ext,
};
pub trait VectorExt<C>: Vector<C>
where
    C: Component,
{
    fn div(&self, c: C) -> Self;
    fn normalized(&self) -> Self;
}

impl VectorExt<f32> for F32x2 {
    #[inline]
    fn div(&self, c: f32) -> Self {
        Self {
            x: self.x / c,
            y: self.y / c,
        }
    }
    fn normalized(&self) -> Self {
        let length = self.magnitude();
        self.div(length)
    }
}
