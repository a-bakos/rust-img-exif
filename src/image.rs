#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub iso: String,
    pub f_stop: String,
    pub focal_length: String,
    pub shutter: String,
    pub date: String,
    pub lens: String,
    pub white_balance: String,
    pub metering_mode: String,
    pub keywords: Vec<String>,
}

pub fn add_iso(meta: &mut Meta, iso_value: String) {
    meta.iso = iso_value;
}

pub fn add_f_stop(meta: &mut Meta, f_value: String) {
    meta.f_stop = f_value;
}

pub fn add_focal_length(meta: &mut Meta, focal_len: String) {
    meta.focal_length = focal_len;
}

pub fn add_shutter(meta: &mut Meta, shutter: String) {
    meta.shutter = shutter;
}

pub fn add_date(meta: &mut Meta, date: String) {
    meta.date = date;
}

pub fn add_keywords(meta: &mut Meta, keywords: Vec<String>) {
    meta.keywords = keywords;
}

pub enum ExifType {
    LensModel,
    FStop,
    Iso,
    FocalLen,
    Shutter,
}

pub fn add_exif(meta: &mut Meta, exif_type: ExifType, exif_value: String) {
    match exif_type {
        ExifType::LensModel => meta.lens = exif_value,
        ExifType::FStop => meta.f_stop = exif_value,
        ExifType::Iso => meta.iso = exif_value,
        ExifType::FocalLen => meta.focal_length = exif_value,
        ExifType::Shutter => meta.shutter = exif_value,
    }
}
