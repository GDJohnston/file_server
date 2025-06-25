use std::{fs, path::Path};

mod files_webpage;

extern crate web_server;

const WEBPAGE_ROOT: &str = "webpages/";
const EXAMPLE_SERVICE_ROOT: &str = "src/file_server/example_service_files/";

const WEBPAGE_INDEX: &str = concat_const::concat!(WEBPAGE_ROOT, "index.html");
const WEBPAGE_FILES: &str = concat_const::concat!(WEBPAGE_ROOT, "files.html");

const WEBPAGE_E404: &str = concat_const::concat!(WEBPAGE_ROOT, "404.html"); 

pub fn launch_server(service_root: &'static str, port: i32) {
    setup(service_root);

    let id_handler = Box::new(|request: web_server::Request, _| {
        let foo = service_root.to_owned() + request.params.values().into_iter().next().unwrap();
        let mut response: web_server::Response = Path::new(foo.as_str()).into();
         // Imply generic downloadable content, avoids playing mp4s
        response.set_header("content-type", "application/octet-stream");
        response
    });

    web_server::new()
        .get("/", Box::new(|_, _| Path::new(WEBPAGE_INDEX).into()))
        .get("/files", Box::new(|_, _| Path::new(WEBPAGE_FILES).into()))
        .get("/file/:id", id_handler)
        .not_found(Box::new(|_, _| Path::new(WEBPAGE_E404).into()))
        .launch(port);
}

fn setup(service_root: &str) {
    if service_folder_empty(service_root) {
        populate_service_files_folder(service_root, EXAMPLE_SERVICE_ROOT);
    }

    files_webpage::generate_files_webpage(WEBPAGE_FILES, service_root);
}

fn service_folder_empty(service_folder: &str) -> bool {
    if !Path::new(service_folder).exists() {
        fs::create_dir(service_folder).unwrap();
        return true; // Definitely empty if we had to create it
    }

    fs::read_dir(service_folder).unwrap().next().is_none()
}

fn populate_service_files_folder(service_folder: &str, files_source: &str) {
    for file in fs::read_dir(files_source).unwrap() {
        if let Ok(file) = file {
            let from_path = file.path();
            let to_path = Path::new(service_folder).join( file.file_name());
            fs::copy(from_path, to_path).unwrap();
        }
    }
}