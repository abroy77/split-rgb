use image::{self, DynamicImage, RgbImage};
use image::io::Reader as ImageReader;
use clap::{arg, command, value_parser};
use std::thread::{self, JoinHandle};
use std::sync::Arc;
//import path
use std::path::PathBuf;


fn main() -> Result<(), Box<dyn std::error::Error>>{

    // get the input and output path
    let matches = command!()
    .arg(arg!(["input file"]).required(true).value_parser(value_parser!(PathBuf)))
    .arg(arg!(["output dir"]).required(true).value_parser(value_parser!(PathBuf)))
    .get_matches();

    let input_path: &PathBuf = matches.get_one("input file").unwrap();
    let output_dir: &PathBuf = matches.get_one("output dir").unwrap();

    // check that the output dir exists
    if !output_dir.exists() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Output directory does not exist")));
    }

    let img =
        ImageReader::open(input_path)?.decode()?;

    let img = match img {
        DynamicImage::ImageRgb8(img) => img,
        _ => {
            println!("Image is not RGB");
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Image is not RGB")));
        }
    };

    // make Rc to share the image between threads
    let im_link = Arc::new(img);

    let channels = ["red", "green", "blue"];
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for (i,channel) in channels.iter().enumerate() {
        let im_link = Arc::clone(&im_link);
        let output_file = output_dir.join(format!("{}_{}.png", input_path.file_stem().unwrap().to_str().unwrap(), channel));
        let channel = i;
        let handle = thread::spawn(move || {
            let img = split_channel(im_link, channel);
            img.save(output_file).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    print!("complete!");
    Ok(())
}


fn split_channel(image: Arc<RgbImage>,channel: usize) -> RgbImage {
    let dimensions = image.dimensions();
    let mut new_img = RgbImage::new(dimensions.0, dimensions.1);
    for (pixel, new_pixel) in image.pixels().zip(new_img.pixels_mut()) {
        new_pixel[channel] = pixel[channel];
    }
    new_img
}
