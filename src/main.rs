use clap::{App, AppSettings, Arg};
use clap::{crate_name, crate_authors, crate_description, crate_version};
use std::path::Path;
use std::slice::Iter;
use std::io::{Read, Write};
use std::fs;


static HASHES: [[usize; 10]; 10] = [
    [0, 3, 5, 7, 8, 2, 9, 6, 1, 4],
    [9, 5, 8, 4, 6, 7, 2, 1, 0, 3],
    [6, 9, 5, 0, 2, 1, 4, 3, 7, 8],
    [8, 7, 3, 0, 5, 4, 9, 1, 2, 6],
    [8, 1, 7, 5, 0, 3, 2, 4, 6, 9],
    [3, 8, 2, 9, 1, 0, 4, 6, 7, 5],
    [7, 1, 9, 6, 0, 4, 8, 2, 3, 5],
    [7, 4, 1, 6, 2, 0, 9, 3, 5, 8],
    [5, 3, 1, 6, 9, 7, 0, 2, 8, 4],
    [1, 4, 6, 7, 2, 5, 9, 8, 0, 3]
];

fn decode(page: usize, input: Vec<u8>) -> Vec<u8> {
    let hash_slice: [usize; 10] = HASHES[page % HASHES.len()];
    let slice_size = input.len() / hash_slice.len();
    let mut ret = Vec::<u8>::new();
    let mut chunks = input.chunks(slice_size);
    for i in 0..10 {
        let idx = hash_slice.iter().position(|v| *v == i as usize);
        let mut chunks = input.chunks(slice_size);
        let buf = chunks.nth(idx.unwrap());
        ret.append(&mut buf.unwrap().clone().to_vec());
    }
    if chunks.len() > 10 {
        let buf = chunks.nth(10);
        ret.append(&mut buf.unwrap().clone().to_vec());
    }
    ret
}

fn parse_int(input: &str) -> Option<usize> {
    let num_part = input.chars().skip_while(|c| !c.is_digit(10)).collect::<String>();
    let res = usize::from_str_radix(num_part.as_str(), 10);
    match res {
        Ok(i) => Some(i),
        Err(e) => None
    }
}

fn f(i: &str, o: &str, page: usize) {
    let mut input = fs::OpenOptions::new().read(true).open(i).unwrap();
    let mut output = fs::OpenOptions::new().write(true).create(true).open(o).unwrap();
    let mut input_buf = Vec::new();
    if input.read_to_end(&mut input_buf).is_err() {
        println!("Reading Error!");
    };
    let output_buf = decode(page, input_buf);
    output.write_all(&output_buf);
}

fn main() {
    env_logger::init();
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .help("The input dir")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("The output dir")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    if input == output {
        println!("output should NOT be the same as input!");
        return;
    }
    let input_path = shellexpand::full(input).unwrap().into_owned();
    let output_path = shellexpand::full(output).unwrap().into_owned();
    let input_dir = Path::new(&input_path);
    let output_dir = Path::new(&output_path);
    if !input_dir.is_dir() {
        println!("input path is not a dir!");
        return;
    }
    if !output_dir.is_dir() {
        println!("output path is not a dir!");
        return;
    }
    for entry in fs::read_dir(input_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let ex = path.extension().unwrap();
            if ex.eq("png") || ex.eq("jpg") || ex.eq("jpeg") {
                let file_in = path.file_stem().unwrap().to_str().unwrap();
                let page = parse_int(file_in);
                if page.is_none() {
                    println!("{} has no page index", file_in);
                    continue;
                }
                let file_out = Path::new(&output_path).join(file_in).with_extension("jpg");
                println!("processing {}", file_in);
                f(path.to_str().as_ref().unwrap(), file_out.to_str().as_ref().unwrap(), page.unwrap());
            }
        }
    }
}
