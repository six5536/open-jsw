pub mod raw_to_tiled_converter;

pub trait Converter<I, O> {
    fn convert(&self, input: &I) -> O;
}
