use mathematics::linear_algebra::matrix::Matrix;
use mathematics::linear_algebra::vector::Vector;
use mathematics::shared::traits::number::Number;
use crate::graphics_pointer::GraphicsPointer;

impl<const L: usize, N: Number> GraphicsPointer<N> for Vector<L, N> {
    fn as_ptr(&self) -> *const N {
        &self[0]
    }
}

impl<const C: usize, const R: usize, N: Number> GraphicsPointer<N> for Matrix<C, R, N> {
    fn as_ptr(&self) -> *const N {
        &self[0][0]
    }
}