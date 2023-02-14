use image::error::UnsupportedError;
use image::DynamicImage;
use image::ImageError;
use image::ImageResult;
use image::RgbImage;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
enum FileFormat {
    TextFile,
    RawBinaryFile,
    StandardBinaryFile,
    CustomBinaryFile,
    Undefined,
}

#[derive(Default)]
pub struct BinFileFormat {
    pub bits_per_sample: u8,
}

pub struct ImageConverter {
    from: String,
    to: String,
    has_custom_format: bool,

    bin_file_format: BinFileFormat,
    from_format: FileFormat,
    to_format: FileFormat,
    width: String,
    height: String,
}

impl ImageConverter {
    pub fn new(from: String, to: String, has_custom_format: bool) -> ImageConverter {
        ImageConverter {
            from,
            to,
            has_custom_format,
            bin_file_format: BinFileFormat::default(),
            from_format: FileFormat::Undefined,
            to_format: FileFormat::Undefined,
            width: String::new(),
            height: String::new(),
        }
    }

    pub fn set_bin_file_format(&mut self, format: BinFileFormat) {
        self.bin_file_format = format;
    }

    pub fn set_width_and_height(&mut self, width: String, height: String) {
        self.width = width;
        self.height = height;
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.from_format = self.get_file_format(&self.from, self.has_custom_format);
        self.to_format = self.get_file_format(&self.to, self.has_custom_format);

        // from_format and to_format should not be undefined
        if self.from_format == FileFormat::Undefined || self.to_format == FileFormat::Undefined {
            return Err("from_format and to_format should not be undefined".to_string());
        }

        let image = self.construct_data_from_file().unwrap();
        image.save(&self.to).unwrap();
        Ok(())
    }

    fn get_file_format(&self, file: &str, is_custom: bool) -> FileFormat {
        let file_split = file.split('.').collect::<Vec<&str>>();
        let mut format = FileFormat::Undefined;

        if file_split.len() == 1 {
            format = FileFormat::RawBinaryFile;
        } else {
            match file_split[file_split.len() - 1] {
                "txt" => {
                    format = FileFormat::TextFile;
                }
                "bin" => {
                    format = FileFormat::RawBinaryFile;
                }
                "png" | "bmp" | "jpg" => {
                    format = FileFormat::StandardBinaryFile;
                }
                _ => {}
            }
        }

        if format == FileFormat::Undefined && is_custom {
            format = FileFormat::CustomBinaryFile;
        }

        format
    }

    fn construct_data_from_file(&self) -> ImageResult<DynamicImage> {
        let ret;

        match self.from_format {
            FileFormat::TextFile => {
                let from_file = File::open(&self.from).expect("Failed to open from file");
                let from_buf = BufReader::new(from_file);
                let mut buf = Vec::new();

                for line in from_buf.lines() {
                    if let Ok(line) = line {
                        let line = line.trim_end();
                        if !line.is_empty() {
                            let tmp = line
                                .split(' ')
                                .map(|x| u8::from_str_radix(x, 16).unwrap())
                                .collect::<Vec<u8>>();
                            let iter =
                                tmp.chunks_exact(self.bin_file_format.bits_per_sample as usize);

                            for pixel in iter {
                                let mut len = pixel.len();
                                let mut value = 0 as u32;
                                while len > 0 {
                                    value = (value << 8) | pixel[len - 1] as u32;
                                    len -= 1;
                                }
                                buf.push(value);
                            }
                        }
                    }
                }

                let width = self
                    .width
                    .parse::<u32>()
                    .expect("Need to specify the width of the image!");
                let height = self
                    .height
                    .parse::<u32>()
                    .expect("Need to specify the height of the image!");

                let mut image = RgbImage::new(width, height);
                for (i, pixel) in buf.iter().enumerate() {
                    let r = pixel >> 11;
                    let g = (pixel >> 5) & 0x3F;
                    let b = pixel & 0x1F;

                    let r = (r as f32 * 255.0 / 31.0 + 0.5) as u8;
                    let g = (g as f32 * 255.0 / 63.0 + 0.5) as u8;
                    let b = (b as f32 * 255.0 / 31.0 + 0.5) as u8;
                    image.put_pixel(i as u32 % width, i as u32 / width, image::Rgb([r, g, b]));
                }

                // todo: other formats
                ret = Ok(DynamicImage::ImageRgb8(image));
            }
            FileFormat::RawBinaryFile => {
                panic!("undefined file format");
            }
            FileFormat::StandardBinaryFile => {
                ret = image::open(&self.from);
            }
            _ => {
                ret = Err(ImageError::Unsupported(
                    UnsupportedError::from_format_and_kind(
                        image::error::ImageFormatHint::Unknown,
                        image::error::UnsupportedErrorKind::GenericFeature("unknown".to_string()),
                    ),
                ));
            }
        }
        ret
    }
}
