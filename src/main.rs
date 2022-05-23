pub mod image {
    #[derive(Debug)]
    pub struct Meta {
        pub f_stop: f32,
        pub iso: i32,
        pub shutter: String,
    }
}

fn main() {
    let img: image::Meta = image::Meta {
        f_stop: 22.0,
        iso: 100,
        shutter: String::from("1/500"),
    };

    println!("{:#?}", img);
}
