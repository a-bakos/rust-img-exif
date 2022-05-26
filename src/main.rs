mod image;

fn main() {
    let file_name: &str = "img\\DSC00489.jpg";
    if open::that(&file_name).is_ok() {
        println!("Image opened");

        // Read exif data if opened
        match rexif::parse_file(&file_name) {
            Ok(exif) => {
                println!(
                    "File name: {}\nMime type: {}\nExif entries found: {}\n\n",
                    file_name,
                    exif.mime,
                    exif.entries.len()
                );

                // Read meta data
                let _file_meta = get_file_meta(&file_name);

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
                            println!("MeteringMode {:?}", entry.value_more_readable);
                        }
                        "White balance mode" => {
                            println!("white balance {:?}", entry.value_more_readable);
                        }
                        "Lens model" => {
                            image::add_exif(&mut img, image::ExifType::LensModel, exif_value);
                        }
                        _ => (),
                    }
                }

                println!("{:#?}", img);
            }
            Err(e) => {
                // Add error handling
                println!("ERROR {:?}", e);
            }
        }
    } else {
        println!("ERROR");
    }
}

fn get_file_meta(file_name: &str) -> std::io::Result<()> {
    use std::fs;

    let metadata = fs::metadata(&file_name)?;

    if let Ok(time) = metadata.modified() {
        println!("{:?}", time);
    } else {
        println!("Not supported on this platform");
    }

    println!("{:?}", metadata.file_type());
    Ok(())
}
