use clap::{App, AppSettings, Arg};
use clap::{crate_name, crate_authors, crate_description, crate_version};
use std::path::Path;
use tokio::fs;

mod utils;

#[tokio::main]
async fn main() {
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
    let input_dir = fs::read_dir(input_path).await;
    let mut input_dir = input_dir.unwrap();
    while let Ok(Some(v)) = input_dir.next_entry().await {
        let path_in = v.path();
        if !path_in.is_file() {
            continue;
        }
        let ex = path_in.extension().unwrap();
        if ex.eq("png") || ex.eq("jpg") || ex.eq("jpeg") {
            let filename_in = path_in.file_stem().unwrap().to_str().unwrap();
            let page = utils::parse_int(filename_in);
            if page.is_none() {
                println!("{} has no page index", filename_in);
                continue;
            }
            let path_out = Path::new(&output_path).join(filename_in).with_extension("jpg");
            println!("processing {}", filename_in);
            utils::decode_files(&path_in, &path_out, page.unwrap()).await;
        }
    };
}
