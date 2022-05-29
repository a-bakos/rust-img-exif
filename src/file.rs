pub fn get_file_meta(file_name: &str) -> std::io::Result<()> {
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

fn get_all_files() {}
