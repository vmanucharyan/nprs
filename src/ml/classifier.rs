pub trait Classifier<A> {
    fn classify(&self) -> A;
}
