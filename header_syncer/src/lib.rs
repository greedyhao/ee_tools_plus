use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::{
    fs::{self, File},
    io::BufReader,
};

// use lang_c::driver::{Config, parse};
use rand::prelude::*;

pub enum FromFileType {
    Header,
    GnuLinkScript,
}

pub struct Syncer {
    from: Vec<String>,
    to: Vec<String>,
    type_of_from: FromFileType,
    lable: String,
    class_name: String,
    ignore_symbols: Vec<String>,
    mark_symbols: Vec<String>,
    compress: bool,
}

#[derive(Debug)]
enum CheckLabelRsp {
    Start,
    End,
    None,
}

impl Syncer {
    pub fn new(from: Vec<&str>, to: Vec<&str>, lable: &str) -> Syncer {
        Syncer {
            from: from.iter().map(|s| s.to_string()).collect(),
            to: to.iter().map(|s| s.to_string()).collect(),
            type_of_from: FromFileType::Header,
            lable: lable.to_string(),
            class_name: String::new(),
            ignore_symbols: Vec::new(),
            mark_symbols: Vec::new(),
            compress: true,
        }
    }

    pub fn set_type_of_form(&mut self, type_of_from: FromFileType) {
        self.type_of_from = type_of_from;
    }

    pub fn set_class_name(&mut self, name: &str) {
        self.class_name = name.to_string();
    }

    pub fn set_ignore_symbols(&mut self, ignore: Vec<&str>) {
        self.ignore_symbols = ignore.iter().map(|s| s.to_string()).collect();
    }
    
    pub fn set_mark_symbols(&mut self, mark: Vec<&str>) {
        self.mark_symbols = mark.iter().map(|s| s.to_string()).collect();
    }

    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }

    pub fn run(&mut self) {
        let mut rng = rand::thread_rng();
        let tmp: u32 = rng.gen();
        let tmp_name = format!("header_syncer_{}", tmp);
        // println!("{}", tmp_name);
        let mut tmp_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&tmp_name)
            .unwrap();

        for f in &self.from {
            let file = File::open(f);
            match file {
                Ok(file) => {
                    self.copy_specific_content_to_another_file(&file, &mut tmp_file);
                }
                Err(e) => {
                    println!("{} open failed, {}", &f, e);
                }
            }
        }

        for f in &self.to {
            let file = File::open(f);
            
            match file {
                Ok(_file) => {
                    // self.copy_specific_content_to_another_file(&file, &mut tmp_file);
                }
                Err(e) => {
                    println!("{} open failed, {}", &f, e);
                }
            }
        }
        fs::remove_file(&tmp_name).unwrap();
    }

    fn copy_specific_content_to_another_file(&self, from: &File, to: &mut File) {
        let reader = BufReader::new(from);
        let mut need_copy = false;

        for line in reader.lines() {
            if let Ok(line) = line {
                match self.check_label(&line, "start", "end") {
                    CheckLabelRsp::Start => {
                        need_copy = true;
                        continue;
                    }
                    CheckLabelRsp::End => {
                        need_copy = false;
                    }
                    _ => {}
                }

                if need_copy {
                    // println!("copyed {}", line);
                    writeln!(to, "{}", line).unwrap();
                }
            }
        }
    }

    fn check_label(&self, line: &str, start: &str, end: &str) -> CheckLabelRsp {
        let mut field = line.split(' ');
        let lable: Vec<&str> = self.lable.split(' ').collect();
        let mut need_copy = CheckLabelRsp::None;

        if field.next() == Some(lable[0]) && field.next() == Some(lable[1]) {
            let state = field.next();
            if state == Some(start) {
                need_copy = CheckLabelRsp::Start;
            } else if state == Some(end) {
                need_copy = CheckLabelRsp::End;
            }
            // mismatch
            if field.next() != Some(lable[2]) {
                need_copy = CheckLabelRsp::None;
            }
        }
        need_copy
    }
}
