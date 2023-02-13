mod image_convertor;
pub use image_convertor::*;

mod music_convertor;
pub use music_convertor::*;

pub trait Convertor {
    fn run(&self);
}

#[cfg(test)]
mod tests {
    use crate::ImageConverter;
    use crate::BinFileFormat;

    #[test]
    fn test_convertor() {
        let from = concat!(env!("CARGO_MANIFEST_DIR"), "\\examples\\img1.txt").to_string();
        let to = concat!(env!("CARGO_MANIFEST_DIR"), "\\examples\\img1.png").to_string();
        let width = String::from("320");
        let height = String::from("385");
        
        let mut converter = ImageConverter::new(from, to, true);
        let format = BinFileFormat { bits_per_sample: 2 };

        converter.set_width_and_height(width, height);
        converter.set_bin_file_format(format);
        converter.run().unwrap();
    }
}
