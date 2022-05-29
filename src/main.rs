mod file;
mod image;
use std::fs::File;
use std::io;
use std::io::prelude::*;

// WORK IN PROGRESS

fn main() {
    let file_name: &str = "DSC00406";
    let file_name_with_path: &str = "img\\DSC00406.jpg";

    if open::that(&file_name_with_path).is_ok() {
        println!("Image opened");

        // Read exif data if opened
        match rexif::parse_file(&file_name_with_path) {
            Ok(exif) => {
                println!(
                    "File name: {}\nMime type: {}\nExif entries found: {}\n\n",
                    file_name,
                    exif.mime,
                    exif.entries.len()
                );

                // Read meta data
                let _file_meta = file::get_file_meta(&file_name_with_path);

                let mut img: image::Meta = image::Meta {
                    name: String::from(file_name),
                    f_stop: String::new(),
                    iso: String::new(),
                    focal_length: String::new(),
                    shutter: String::new(),
                    date: String::new(),
                    keywords: vec![],
                    lens: String::new(),
                    white_balance: String::new(),
                    metering_mode: String::new(),
                };

                // Loop through the exif data
                for entry in &exif.entries {
                    //println!("    {:#?}", entry.tag.to_string());

                    let exif_tag: String = entry.tag.to_string();
                    let exif_value: String = entry.value_more_readable.to_string();

                    match exif_tag.as_str().trim() {
                        "Date of original image" => {
                            image::add_date(&mut img, exif_value);
                        }
                        "ISO speed ratings" => {
                            image::add_exif(&mut img, image::ExifType::Iso, exif_value);
                        }
                        "Aperture" => {
                            image::add_exif(&mut img, image::ExifType::FStop, exif_value);
                        }
                        "Exposure time" => {
                            image::add_exif(&mut img, image::ExifType::Shutter, exif_value);
                        }
                        "Focal length" => {
                            image::add_exif(&mut img, image::ExifType::FocalLen, exif_value);
                        }
                        "Meteting mode" => {
                            // Yeah, typo in the exif tag's name...
                            image::add_exif(&mut img, image::ExifType::MeteringMode, exif_value);
                        }
                        "White balance mode" => {
                            image::add_exif(&mut img, image::ExifType::WhiteBalance, exif_value);
                        }
                        "Lens model" => {
                            image::add_exif(&mut img, image::ExifType::LensModel, exif_value);
                        }
                        _ => (),
                    }
                }

                // ask for keywords
                println!("Enter keywords");
                let mut user_input = String::new();
                let mut processed_keywords: Vec<String> = Vec::new();
                let keywords: Vec<&str> = match io::stdin().read_line(&mut user_input) {
                    Ok(_) => user_input.split(' ').collect(),
                    Err(_) => vec![""],
                };
                for word in keywords.iter() {
                    processed_keywords.push(word.trim().to_string());
                }
                image::add_keywords(&mut img, processed_keywords);
                let _file_written = write_to_file(file_name, &img);
                println!("{:#?}", img);
            }
            Err(e) => {
                // Add error handling
                println!("ERROR - exif not found - {:?}", e);
            }
        }
    } else {
        println!("ERROR - image could not be opened");
    }
}

fn write_to_file(file_name: &str, img_exif_content: &image::Meta) -> std::io::Result<()> {
    let file_name = file_name.to_owned() + ".txt";
    let mut file = File::create(file_name)?;

    file.write_all(b"ISO: ")?;
    file.write_all(img_exif_content.iso.as_bytes())?;

    Ok(())
}
