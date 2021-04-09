use clap::{App, AppSettings, Arg};
use clap::{crate_name, crate_authors, crate_description, crate_version};
use std::path::Path;
use std::fs;

mod utils;


fn main() {
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
                let page = utils::parse_int(file_in);
                if page.is_none() {
                    println!("{} has no page index", file_in);
                    continue;
                }
                let file_out = Path::new(&output_path).join(file_in).with_extension("jpg");
                println!("processing {}", file_in);
                utils::decode_files(path.to_str().as_ref().unwrap(), file_out.to_str().as_ref().unwrap(), page.unwrap());
            }
        }
    }
}
