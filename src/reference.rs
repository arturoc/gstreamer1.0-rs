pub trait Reference{
    fn reference(&self) -> Self where Self:Sized;
}
