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
enum ColorFormat {
    #[default]
    Rgb332,
    Rgb565,
    Rgb888,

    // ARgb332,
    // ARgb565,
    // ARgb888,
}

#[derive(Default)]
pub struct BinFileFormat {
    bytes_per_sample: u8,
    rgb_type: ColorFormat,
    is_small_endian: bool,
}

impl BinFileFormat {
    pub fn new(rgb_type: String) -> Result<Self, String> {
        let bytes_per_sample;
        let rgb;
        let mut is_small_endian = true;

        match rgb_type.as_str() {
            "rgb332" => {
                bytes_per_sample = 1;
                rgb = ColorFormat::Rgb332;
            }
            "rgb565" => {
                bytes_per_sample = 2;
                rgb = ColorFormat::Rgb565;
            }
            "rgb565-swap" => {
                bytes_per_sample = 2;
                rgb = ColorFormat::Rgb565;
                is_small_endian = false;
            }
            "rgb888" => {
                bytes_per_sample = 4;
                rgb = ColorFormat::Rgb888;
            }

            // TODO: alpha support
            // "argb332" => {
            //     bytes_per_sample = 2;
            //     rgb = ColorFormat::ARgb332;
            // }
            // "argb565" => {
            //     bytes_per_sample = 4;
            //     rgb = ColorFormat::ARgb565;
            // }
            // "argb565-swap" => {
            //     bytes_per_sample = 4;
            //     rgb = ColorFormat::ARgb565;
            //     is_small_endian = false;
            // }
            // "argb888" => {
            //     bytes_per_sample = 4;
            //     rgb = ColorFormat::ARgb888;
            // }
            _ => return Err("Unsupported rgb type".to_string()),
        }

        Ok(BinFileFormat {
            bytes_per_sample,
            rgb_type: rgb,
            is_small_endian,
        })
    }
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

type PixelToRgb = fn(u32) -> [u8; 3];
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
                                tmp.chunks_exact(self.bin_file_format.bytes_per_sample as usize);

                            for pixel in iter {
                                let mut len = pixel.len();
                                let mut value = 0 as u32;

                                if self.bin_file_format.is_small_endian {
                                    while len > 0 {
                                        value = (value << 8) | pixel[len - 1] as u32;
                                        len -= 1;
                                    }
                                } else {
                                    while len < pixel.len() {
                                        value = (value << 8) | pixel[len] as u32;
                                        len += 1;
                                    }
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

                let image = self.image_from_vec(width, height, buf);
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

    fn image_from_vec(&self, width: u32, height: u32, buf: Vec<u32>) -> RgbImage {
        let mut image = RgbImage::new(width, height);

        let pixel_fn: PixelToRgb;
        match self.bin_file_format.rgb_type {
            ColorFormat::Rgb332 => pixel_fn = rgb332_to_rgb,
            ColorFormat::Rgb565 => pixel_fn = rgb565_to_rgb,
            ColorFormat::Rgb888 => pixel_fn = rgb888_to_rgb,
        }

        for (i, pixel) in buf.iter().enumerate() {
            image.put_pixel(
                i as u32 % width,
                i as u32 / width,
                image::Rgb(pixel_fn(*pixel)),
            );
        }
        image
    }
}

fn rgb332_to_rgb(pixel: u32) -> [u8; 3] {
    let pixel = pixel as u8;
    let r = pixel & 0xE0;
    let g = ((pixel & 0x1C) >> 2) << 5;
    let b = (pixel & 0x3) << 6;

    [r, g, b]
}

fn rgb565_to_rgb(pixel: u32) -> [u8; 3] {
    let pixel = pixel as u16;

    let r = pixel >> 11;
    let g = (pixel >> 5) & 0x3F;
    let b = pixel & 0x1F;

    let r = (r as f32 * 255.0 / 31.0 + 0.5) as u8;
    let g = (g as f32 * 255.0 / 63.0 + 0.5) as u8;
    let b = (b as f32 * 255.0 / 31.0 + 0.5) as u8;

    [r, g, b]
}

fn rgb888_to_rgb(pixel: u32) -> [u8; 3] {
    let pixel = pixel & 0xffffff;
    let r = (pixel >> 16) as u8;
    let g = (pixel >> 8) as u8;
    let b = pixel as u8;

    [r, g, b]
}
