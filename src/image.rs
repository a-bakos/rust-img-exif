#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub f_stop: f32,
    pub iso: i32,
    pub focal_len: u16,
    pub shutter: String,
    pub date: String,
    pub keywords: Vec<String>,
}

pub fn add_iso(meta: &mut Meta, iso: i32) {
    meta.iso = iso;
}

pub fn add_f_stop(meta: &mut Meta, f_value: f32) {
    meta.f_stop = f_value;
}

pub fn add_shutter(meta: &mut Meta, shutter: String) {
    meta.shutter = shutter;
}

pub fn add_keywords(meta: &mut Meta, keywords: Vec<String>) {
    meta.keywords = keywords;
}
