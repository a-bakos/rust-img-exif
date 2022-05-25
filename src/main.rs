// use std::fs::Metadata;
mod image;

fn main() {
    // example image
    let mut img: image::Meta = image::Meta {
        name: String::from("DSC0001"),
        f_stop: 22.0_f32,
        iso: 100_i32,
        focal_len: 100,
        shutter: String::from("500"),
        date: String::from("2022-05-14 16:05:43"),
        keywords: vec![String::from("city"), String::from("urban")],
    };

    image::add_iso(&mut img, 200);
    println!("{:#?}", img);

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

                // Loop through the exif data
                for entry in &exif.entries {
                    //println!("{:?}", entry.tag); // read tags

                    println!("    {}: {}", entry.tag, entry.value_more_readable);
                    // .tag
                    // .value_more_readable
                }
            }
            Err(e) => {
                // Add error handling
                println!("ERROR {:?}", e);
            }
        }
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
