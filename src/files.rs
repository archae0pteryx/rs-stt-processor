use std::path::Path;

pub fn get_shortname(src_path: &str) -> String {
    let src_file = Path::new(src_path);
    let stripped_ext = src_file.with_extension("");
    let raw_filename = stripped_ext.file_name().unwrap().to_str().unwrap();
    String::from(raw_filename)
}
