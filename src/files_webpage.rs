use std::{fs::{self, OpenOptions}, io::Write as _};

const WEBPAGE_ROOT: &'static str = "webpages/";
const SERVICE_ROOT: &'static str = "service_files/";

const WEBPAGE_FILES: &str = concat_const::concat!(WEBPAGE_ROOT, "files.html");

const FILES_WEBPAGE_CONTENT: &[u8; 469] = b"<!DOCTYPE html>
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
            <li>hello.html</li>
            <li>video.mp4</li>
            <li>404.html</li>
        </ul>
    </body>
</html>";

pub(crate) fn generate_files_webpage_new() {
    let mut files_webpage = OpenOptions::new()
        .create(true)
        .write(true)
        .open(WEBPAGE_FILES)
        .unwrap();
    
    files_webpage.write_all(FILES_WEBPAGE_CONTENT).unwrap();
    let filenames = fs::read_dir(SERVICE_ROOT).unwrap();
    filenames.for_each(|file| {
        let filename = file.unwrap().file_name();
        writeln!(files_webpage, "{:?}", filename).unwrap();
    });
}
