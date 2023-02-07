use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::Write;
use std::io::{Seek, SeekFrom};
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
    label: String,
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
    pub fn new(from: Vec<&str>, to: Vec<&str>, label: &str) -> Syncer {
        Syncer {
            from: from.iter().map(|s| s.to_string()).collect(),
            to: to.iter().map(|s| s.to_string()).collect(),
            type_of_from: FromFileType::Header,
            label: label.to_string(),
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
        let tmp_file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&tmp_name)
            .unwrap();

        for f in &self.from {
            let file = File::open(f);
            match file {
                Ok(file) => {
                    if let Some((start, end)) =
                        self.get_label_position_in_file(&file, "start", "end")
                    {
                        let mut line = String::new();
                        let mut reader = BufReader::new(&file);
                        reader.seek(SeekFrom::Start(start as u64)).unwrap();

                        // let name = f.split("\\").last().unwrap();
                        // writeln!(&tmp_file, "// {}", name).unwrap();
                        loop {
                            reader.read_line(&mut line).unwrap();
                            if reader.stream_position().unwrap() >= end as u64 {
                                break;
                            }
                            writeln!(&tmp_file, "{}", line.trim_end()).unwrap();
                            line.clear();
                        }
                    }
                }
                Err(e) => {
                    panic!("{} open failed, {}", &f, e);
                }
            }
        }
        drop(tmp_file);

        for f in &self.to {
            let file = File::open(f);

            match file {
                Ok(file) => {
                    let start_string = "autogen start";
                    let end_string = "autogen end";

                    // println!(
                    //     "in to file {:?}",
                    //     self.get_label_position_in_file(&file, start_string, end_string)
                    // );

                    if let Some((start, end)) =
                        self.get_label_position_in_file(&file, start_string, end_string)
                    {
                        let new_file = File::create(f.to_string() + ".new").unwrap();
                        let mut line = String::new();
                        let mut reader = BufReader::new(&file);

                        reader.rewind().unwrap();
                        loop {
                            reader.read_line(&mut line).unwrap();
                            if reader.stream_position().unwrap() >= start as u64 {
                                line.clear();
                                break;
                            }
                            // println!("1 line {}", line.trim_end());
                            writeln!(&new_file, "{}", line.trim_end()).unwrap();
                            line.clear();
                        }

                        let mut label: Vec<&str> = self.label.split(' ').collect();
                        let label_last = label.pop().unwrap();

                        // write start label
                        let mut slabel = label.clone();
                        slabel.push(start_string);
                        slabel.push(label_last);
                        writeln!(&new_file, "{}", slabel.join(" ")).unwrap();

                        // need to copy
                        let tmp_file = File::open(&tmp_name).unwrap();
                        let tmp_reader = BufReader::new(&tmp_file);

                        for line in tmp_reader.lines() {
                            if let Ok(line) = line {
                                // println!("line {}", line);
                                writeln!(&new_file, "{}", line).unwrap();
                            }
                        }

                        // write end label
                        label.push(end_string);
                        label.push(label_last);
                        writeln!(&new_file, "{}", label.join(" ")).unwrap();

                        reader.seek(SeekFrom::Start(end as u64)).unwrap();

                        while let Ok(size) = reader.read_line(&mut line) {
                            if size == 0 {
                                break;
                            }

                            // println!("2 line {}", line.trim_end());
                            writeln!(&new_file, "{}", line.trim_end()).unwrap();
                            line.clear();
                        }

                        drop(new_file);

                        fs::rename(f.to_string(), f.to_string() + ".old").unwrap();
                        fs::rename(f.to_string() + ".new", f.to_string()).unwrap();
                        fs::remove_file(f.to_string() + ".old").unwrap();
                    }
                }
                Err(e) => {
                    println!("{} open failed, {}", &f, e);
                }
            }
        }
        fs::remove_file(&tmp_name).unwrap();
    }

    /// Returns the position of the label in the given file
    fn get_label_position_in_file(
        &self,
        file: &File,
        start: &str,
        end: &str,
    ) -> Option<(usize, usize)> {
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut res = None;
        let mut start_pos = 0;
        let end_pos;

        // need to rewind the file
        reader.rewind().unwrap();

        while let Ok(size) = reader.read_line(&mut line) {
            if size == 0 {
                break;
            }
            line = line.trim_end().to_string();
            match self.check_label(&line, start, end) {
                CheckLabelRsp::Start => {
                    start_pos = reader.stream_position().unwrap();
                }
                CheckLabelRsp::End => {
                    end_pos = reader.stream_position().unwrap();
                    res = Some((start_pos as usize, end_pos as usize));
                    break;
                }
                _ => {}
            }
            line.clear();
        }

        res
    }

    fn check_label(&self, line: &str, start: &str, end: &str) -> CheckLabelRsp {
        let label: Vec<&str> = self.label.split(' ').collect();
        let mut need_copy = CheckLabelRsp::None;

        let iter_line: Vec<&str> = line.split(' ').collect();
        if label.iter().all(|x| iter_line.contains(&x)) {
            // println!(
            //     "check if start or end, line:{:?} start:{} end:{}",
            //     iter_line, start, end
            // );
            if start.split(' ').count() + label.len() == iter_line.len()
                && start.split(' ').all(|x| iter_line.contains(&x))
            {
                need_copy = CheckLabelRsp::Start;
            } else if end.split(' ').count() + label.len() == iter_line.len()
                && end.split(' ').all(|x| iter_line.contains(&x))
            {
                need_copy = CheckLabelRsp::End;
            }
        }

        need_copy
    }
}

#[cfg(test)]
mod tests {
    use crate::Syncer;

    #[test]
    fn test_check_update_status() {
        let from = vec![concat!(env!("CARGO_MANIFEST_DIR"), "\\examples\\test1.h")];
        let to = vec![concat!(env!("CARGO_MANIFEST_DIR"), "\\examples\\api.h")];
        let mut syncer = Syncer::new(from, to, "/* header-sync */");

        syncer.run();
    }
}
