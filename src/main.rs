use std::path::Path;

mod files_webpage;

extern crate web_server;

const PORT: i32 = 8080;

const WEBPAGE_ROOT: &str = "webpages/";
const SERVICE_ROOT: &str = "service_files/";

const WEBPAGE_INDEX: &str = concat_const::concat!(WEBPAGE_ROOT, "index.html");
const WEBPAGE_FILES: &str = concat_const::concat!(WEBPAGE_ROOT, "files.html");

const WEBPAGE_E404: &str = concat_const::concat!(WEBPAGE_ROOT, "404.html");

fn main() {
    let id_handler = Box::new(|request: web_server::Request, _| {
        let foo = SERVICE_ROOT.to_owned() + request.params.values().into_iter().next().unwrap();
        let mut response: web_server::Response = Path::new(foo.as_str()).into();
         // Imply generic downloadable content, avoids playing mp4s
        response.set_header("content-type", "application/octet-stream");
        response
    });

    files_webpage::generate_files_webpage_new();
    web_server::new()
        .get("/", Box::new(|_, _| Path::new(WEBPAGE_INDEX).into()))
        .get("/files", Box::new(|_, _| Path::new(WEBPAGE_FILES).into()))
        .get("/file/:id", id_handler)
        .not_found(Box::new(|_, _| Path::new(WEBPAGE_E404).into()))
        .launch(PORT);
}
