// use std::fs::Metadata;
use rexif::ExifTag;

mod image;

fn main() {
    // Test
    // Open an img
    //
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

                // example image
                let mut img: image::Meta = image::Meta {
                    name: String::from(file_name),
                    f_stop: String::new(),
                    iso: String::new(),
                    focal_len: String::new(),
                    shutter: String::new(),
                    date: String::from("2022-05-14 16:05:43"),
                    keywords: vec![String::from("city"), String::from("urban")],
                };

                // Loop through the exif data
                for entry in &exif.entries {
                    //println!("    {:#?}", entry.tag.to_string());

                    let exif_tag: String = entry.tag.to_string();
                    let exif_value: String = entry.value_more_readable.to_string();

                    match exif_tag.as_str().trim() {
                        "ISO speed ratings" => {
                            image::add_iso(&mut img, exif_value);
                        }
                        "Aperture" => {
                            image::add_f_stop(&mut img, exif_value);
                        }
                        "Exposure time" => {
                            image::add_shutter(&mut img, exif_value);
                        }
                        "Focal length" => {
                            println!("FocalLength {:?}", entry.value_more_readable);
                        }
                        "Meteting mode" => {
                            println!("MeteringMode {:?}", entry.value_more_readable);
                        }
                        "White balance mode" => {
                            println!("white balance {:?}", entry.value_more_readable);
                        }
                        "Lens model" => {
                            println!("Lensmode {:?}", entry.value_more_readable);
                        }
                        "Date of original image" => {
                            println!("DATETIME {:?}", entry.value_more_readable);
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
