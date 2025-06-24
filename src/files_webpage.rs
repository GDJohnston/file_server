use std::{
    fs::{self, OpenOptions},
    io::Write as _,
};

const WEBPAGE_ROOT: &str = "webpages/";
const SERVICE_ROOT: &str = "service_files/";

const WEBPAGE_FILES: &str = concat_const::concat!(WEBPAGE_ROOT, "files.html");

const FILES_WEBPAGE_CONTENT_1: &[u8; 343] = b"<!DOCTYPE html>
<html lang=\"en\">
    <head>
        <meta charset=\"utf-8\">
        <title>File Service</title>
    </head>
    <body>
        <p>
            These are the files available for download.
            Navigate to \"/file/x\" to download file x for example.
            The file extension must be included.
        </p>
        <ul>
";

const FILES_WEBPAGE_CONTENT_2: &[u8; 34] = b"
        </ul>
    </body>
</html>";

pub(crate) fn generate_files_webpage_new() {
    if fs::exists(WEBPAGE_FILES).unwrap() {
        fs::remove_file(WEBPAGE_FILES).unwrap();
    }

    let mut files_webpage = OpenOptions::new()
        .create(true)
        .write(true)
        .open(WEBPAGE_FILES)
        .unwrap();

    files_webpage.write_all(FILES_WEBPAGE_CONTENT_1).unwrap();
    let filenames = fs::read_dir(SERVICE_ROOT).unwrap();
    filenames.for_each(|file| {
        let filename = file.unwrap().file_name();
        writeln!(files_webpage, "           <li>{}</li>", filename.display()).unwrap();
    });
    files_webpage.write_all(FILES_WEBPAGE_CONTENT_2).unwrap();
}
