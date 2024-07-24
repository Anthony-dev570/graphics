pub trait UniformStruct {
    type Fields;
    fn fields(&self) -> Self::Fields;
}