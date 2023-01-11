mod image_convertor;
pub use image_convertor::*;

mod music_convertor;
pub use music_convertor::*;

pub trait Convertor {
    fn run(&self);
}
