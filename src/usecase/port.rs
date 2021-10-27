pub trait Port<In, Out> {
    fn exec(&self, input: In) -> Out;
}
