#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub iso: String,
    pub f_stop: String,
    pub focal_len: String,
    pub shutter: String,
    pub date: String,
    pub keywords: Vec<String>,
}

pub fn add_iso(meta: &mut Meta, iso_value: String) {
    meta.iso = iso_value;
}

pub fn add_f_stop(meta: &mut Meta, f_value: String) {
    meta.f_stop = f_value;
}

pub fn add_shutter(meta: &mut Meta, shutter: String) {
    meta.shutter = shutter;
}

pub fn add_keywords(meta: &mut Meta, keywords: Vec<String>) {
    meta.keywords = keywords;
}
