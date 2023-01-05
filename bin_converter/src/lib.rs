use image::RgbImage;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
};

pub struct Convertor {
    from: String,
    to: String,
}

pub struct ImageConverter {
    from: String,
    to: String,
    width: usize,
    height: usize,
}

impl ImageConverter {
    pub fn gen_img_from_file(&self) {
        let width = self.width as u32;
        let height = self.height as u32;

        if Path::new(&self.to).exists() {
            fs::remove_file(&self.to).unwrap();
        }

        let in_file = File::open(&self.from);

        let in_file = match in_file {
            Ok(file) => file,
            Err(err) => panic!("Failed to open {}, the error is {}", &self.from, err),
        };

        let in_buf = BufReader::new(in_file);
        let mut buf: Vec<u16> = Vec::new();

        for line in in_buf.lines() {
            if let Ok(line) = line {
                let line = line.trim_end();
                if !line.is_empty() {
                    let tmp = line
                        .split(' ')
                        .map(|x| u8::from_str_radix(x, 16).unwrap())
                        .collect::<Vec<u8>>();
                    let mut tmp: Vec<u16> = tmp
                        .chunks_exact(2)
                        .into_iter()
                        .map(|a| u16::from_ne_bytes([a[0], a[1]]))
                        .collect();
                    buf.append(&mut tmp);
                }
            }
        }

        // out_file.write_all(&buf).unwrap();

        // rgb565 to rgb888
        let mut img = RgbImage::new(width, height);
        for (i, pixel) in buf.iter().enumerate() {
            let r = pixel >> 11;
            let g = (pixel >> 5) & 0x3F;
            let b = pixel & 0x1F;

            let r = (r as f32 * 255.0 / 31.0 + 0.5) as u8;
            let g = (g as f32 * 255.0 / 63.0 + 0.5) as u8;
            let b = (b as f32 * 255.0 / 31.0 + 0.5) as u8;
            img.put_pixel(i as u32 % width, i as u32 / width, image::Rgb([r, g, b]));
        }
        img.save(&self.to).unwrap();
    }
}

pub fn gen_img_from_file(filename: &str, image: &str, width: u32, height: u32) {
    if Path::new(image).exists() {
        fs::remove_file(image).unwrap();
    }

    let in_file = File::open(filename);

    let in_file = match in_file {
        Ok(file) => file,
        Err(err) => panic!("Failed to open {}, the error is {}", filename, err),
    };

    let in_buf = BufReader::new(in_file);
    let mut buf: Vec<u16> = Vec::new();

    for line in in_buf.lines() {
        if let Ok(line) = line {
            let line = line.trim_end();
            if !line.is_empty() {
                let tmp = line
                    .split(' ')
                    .map(|x| u8::from_str_radix(x, 16).unwrap())
                    .collect::<Vec<u8>>();
                let mut tmp: Vec<u16> = tmp
                    .chunks_exact(2)
                    .into_iter()
                    .map(|a| u16::from_ne_bytes([a[0], a[1]]))
                    .collect();
                buf.append(&mut tmp);
            }
        }
    }

    // out_file.write_all(&buf).unwrap();

    // rgb565 to rgb888
    let mut img = RgbImage::new(width, height);
    for (i, pixel) in buf.iter().enumerate() {
        let r = pixel >> 11;
        let g = (pixel >> 5) & 0x3F;
        let b = pixel & 0x1F;

        let r = (r as f32 * 255.0 / 31.0 + 0.5) as u8;
        let g = (g as f32 * 255.0 / 63.0 + 0.5) as u8;
        let b = (b as f32 * 255.0 / 31.0 + 0.5) as u8;
        img.put_pixel(i as u32 % width, i as u32 / width, image::Rgb([r, g, b]));
    }
    img.save(image).unwrap();
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
