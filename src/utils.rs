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
    for i in 0..10usize {
        let idx = hash_slice.iter().position(|v| *v == i);
        let mut chunks = input.chunks(slice_size);
        let buf = chunks.nth(idx.unwrap());
        ret.append(&mut buf.unwrap().to_vec());
    }
    if chunks.len() > 10 {
        let buf = chunks.nth(10);
        ret.append(&mut buf.unwrap().to_vec());
    }
    ret
}

pub(crate) fn parse_int(input: &str) -> Option<usize> {
    let num_part = input.chars().skip_while(|c| !c.is_digit(10)).collect::<String>();
    let res = usize::from_str_radix(num_part.as_str(), 10);
    match res {
        Ok(i) => Some(i),
        Err(_e) => None
    }
}
pub(crate) fn decode_files(i: &str, o: &str, page: usize) {
    let mut input = fs::OpenOptions::new().read(true).open(i).unwrap();
    let mut output = fs::OpenOptions::new().write(true).create(true).open(o).unwrap();
    let mut input_buf = Vec::new();
    if input.read_to_end(&mut input_buf).is_err() {
        println!("Reading Error!");
    };
    let output_buf = decode(page, input_buf);
    let r = output.write_all(&output_buf);
    if r.is_err(){
        println!("Writing error");
    }
}