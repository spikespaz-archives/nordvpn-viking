use reqwest::blocking as reqwest;
use std::{
    env, fs,
    fs::File,
    io,
    io::Cursor,
    path::{Path, PathBuf},
};
use zip::read::ZipArchive;

fn download_flag_icons(url: &str, dest_dir: &str) {
    let response = reqwest::get(url).unwrap();
    let content = response.bytes().unwrap();
    let mut zip = ZipArchive::new(Cursor::new(content)).unwrap();
    let dest_dir = Path::new(dest_dir);

    for index in 0..zip.len() {
        let mut zip_file = zip.by_index(index).unwrap();
        let zip_file_path = match zip_file.enclosed_name() {
            Some(path) => path,
            None => continue,
        };

        const FLAGS_PATH_PREFIX: &str = "flag-icons-main/flags";

        if !zip_file.is_file()
            || !zip_file_path.starts_with(FLAGS_PATH_PREFIX)
            || zip_file_path.extension().unwrap() != "svg"
        {
            continue;
        }

        let dest_file_path = dest_dir.join(zip_file_path.strip_prefix(FLAGS_PATH_PREFIX).unwrap());
        fs::create_dir_all(dest_file_path.parent().unwrap()).unwrap();
        let mut dest_file = File::create(&dest_file_path).unwrap();
        io::copy(&mut zip_file, &mut dest_file).unwrap();
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // let out_res_dir = out_dir.join("res");
    // let out_res_icons_dir = out_dir.join("res/icons");
    let out_res_icons_flags_dir = out_dir.join("res/icons/flags");

    let flag_icons_zip_url = env::var("FLAG_ICONS_ZIP_URL").unwrap();

    download_flag_icons(
        &flag_icons_zip_url,
        out_res_icons_flags_dir.to_str().unwrap(),
    );
}
