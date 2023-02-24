use image::{DynamicImage, GenericImageView, ImageFormat};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "image-scaler", about = "A simple CLI tool to scale images")]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    #[structopt(short = "s", long = "scale")]
    scale: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let input_path = &opt.input;
    let scale_factor = opt.scale;

    // Load the image from the input path
    let img = image::open(&input_path)?;

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Calculate the new dimensions based on the scale factor
    let new_width = (width as f32 * scale_factor) as u32;
    let new_height = (height as f32 * scale_factor) as u32;

    // Scale the image
    let scaled_img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    // Construct the output path
    let output_path = input_path
        .with_file_name(format!(
            "{}{}{}",
            input_path.file_stem().unwrap().to_str().unwrap(),
            "-SCALED.",
            input_path.extension().unwrap().to_str().unwrap()
        ))
        .to_str()
        .unwrap()
        .to_owned();

    // Save the scaled image to the output path
    scaled_img.save_with_format(output_path, ImageFormat::Jpeg)?;

    Ok(())
}
