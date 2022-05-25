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

    if open::that("img\\DSC00489.jpg").is_ok() {
        println!("Image opened");
    }

    println!("{:#?}", img);
}
