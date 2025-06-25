use std::{
    fs::{self, OpenOptions},
    io::Write as _,
};

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


/// Creates a simple html webpage listing all the files in the `service_folder`.
/// The file is writen to `files_webpage`, the file is overwritten if it already exists. 
///
/// # Panics
///
/// Panics if `files_webpage` could not be opened or created or if the file could not be
/// written to at any stage.
pub(crate) fn generate_files_webpage(files_webpage: &str, service_folder: &str) {
    let mut files_webpage = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(files_webpage)
        .unwrap();

    files_webpage.write_all(FILES_WEBPAGE_CONTENT_1).unwrap();
    let filenames = fs::read_dir(service_folder).unwrap();
    filenames.for_each(|file| {
        let filename = file.unwrap().file_name();
        writeln!(files_webpage, "           <li>{}</li>", filename.display()).unwrap();
    });
    files_webpage.write_all(FILES_WEBPAGE_CONTENT_2).unwrap();
}
