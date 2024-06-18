pub mod imp;

#[derive(Debug, Clone)]
pub enum AOrB<A, B> {
    A(A),
    B(B)
}