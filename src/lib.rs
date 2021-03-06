use std::fs;
use std::io::{BufWriter, Write};
use image_base64;
use base64;
pub const PATH: &str = "./peter.png";
pub const IMG_STRING: &str = include_str!("./peter.txt");

pub fn get_base64_image(path: String){
    let img = image_base64::to_base64(&path);
    let file = fs::File::create("image.txt").unwrap();
    let mut file1 = BufWriter::new(file);
    writeln!(file1, "{}", img).unwrap();
}

pub fn prepare_image(){
    let img = image_base64::from_base64(IMG_STRING.to_string());
    let img_file = fs::File::create(PATH).unwrap();
    let mut sink = BufWriter::new(img_file);
    sink.write(&img).unwrap();
}

pub fn get_ico(){
    let mut img = include_str!("icon.txt").to_string(); 
    let offset = img.find(",").unwrap();
    let value = img.drain(..offset);
    let bytes = base64::decode(value).unwrap();
    let mut file = fs::File::create("icon.ico").unwrap();
    file.write(&bytes).unwrap();
}