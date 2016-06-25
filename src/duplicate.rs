pub trait Duplicate{
    fn duplicate(&self) -> Self where Self:Sized;
}
