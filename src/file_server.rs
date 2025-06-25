use std::{fs, path::Path};

mod files_webpage;

extern crate web_server;

const WEBPAGE_ROOT: &str = "webpages/";
const EXAMPLE_SERVICE_ROOT: &str = "src/file_server/example_service_files/";

const WEBPAGE_INDEX: &str = concat_const::concat!(WEBPAGE_ROOT, "index.html");
const WEBPAGE_FILES: &str = concat_const::concat!(WEBPAGE_ROOT, "files.html");

const WEBPAGE_E404: &str = concat_const::concat!(WEBPAGE_ROOT, "404.html");

/// Launches a webserver on `port` which offers the files in `service_folder` for download.
///
/// # Panics
///
/// Panics if a download request is received with no ids which should not be possible.
pub fn launch_server(service_folder: &'static str, port: i32) {
    setup(service_folder);

    let id_handler = Box::new(|request: web_server::Request, _| {
        let filepath = service_folder.to_owned() + request.params.values().into_iter().next().unwrap();
        let mut response: web_server::Response = Path::new(filepath.as_str()).into();
        // Imply generic downloadable content, avoids playing mp4s
        response.set_header("content-type", "application/octet-stream");
        response
    });

    let files_handler = Box::new(|_, _| {
        files_webpage::generate_files_webpage(WEBPAGE_FILES, service_folder);
        Path::new(WEBPAGE_FILES).into()
    });

    web_server::new()
        .get("/", Box::new(|_, _| Path::new(WEBPAGE_INDEX).into()))
        .get("/files", files_handler)
        .get("/file/:id", id_handler)
        .not_found(Box::new(|_, _| Path::new(WEBPAGE_E404).into()))
        .launch(port);
}

/// Populates the `service_folder` with example service files if
/// it is empty and generates an indexing webpage.  
fn setup(service_folder: &str) {
    if service_folder_empty(service_folder) {
        populate_service_files_folder(service_folder, EXAMPLE_SERVICE_ROOT);
    }

    files_webpage::generate_files_webpage(WEBPAGE_FILES, service_folder);
}

/// Tests to see if `service_folder` is empty. Creates `service_folder`
/// if it does not already exist.
///
/// # Panics
///
/// Panics if an error is returned while trying to create or read from
/// the `service_folder`.
fn service_folder_empty(service_folder: &str) -> bool {
    if !Path::new(service_folder).exists() {
        fs::create_dir(service_folder).unwrap();
        return true; // Definitely empty if we had to create it
    }

    fs::read_dir(service_folder).unwrap().next().is_none()
}

/// Populates the `service_folder` with files from the `files_source` folder
///
/// # Panics
///
/// Panics if reading the `files_source` directory returns an error or
/// copying a file from the`files_source` directory to the `service_folder`
/// reurns an error.
fn populate_service_files_folder(service_folder: &str, files_source: &str) {
    for file in fs::read_dir(files_source).unwrap() {
        if let Ok(file) = file {
            let from_path = file.path();
            let to_path = Path::new(service_folder).join(file.file_name());
            fs::copy(from_path, to_path).unwrap();
        }
    }
}
