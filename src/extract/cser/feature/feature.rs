pub trait Feature {
    fn value(&self, out: &mut Vec<f32>);
}
