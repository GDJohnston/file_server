use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    path::Path,
};
extern crate web_server;

const PORT: i32 = 8080;

const WEBPAGE_ROOT: &'static str = "webpages/";
const SERVICE_ROOT: &'static str = "service_files/";

const WEBPAGE_INDEX: &str = concat_const::concat!(WEBPAGE_ROOT, "hello.html");
const WEBPAGE_E404: &str = concat_const::concat!(WEBPAGE_ROOT, "404.html");
const WEBPAGE_FILES: &str = concat_const::concat!(WEBPAGE_ROOT, "files.html");

fn main() {
    generate_files_webpage();
    web_server::new()
        .get("/", Box::new(|_, _| Path::new(WEBPAGE_INDEX).into()))
        .get("/files", Box::new(|_, _| Path::new(WEBPAGE_FILES).into()))
        .get(
            "/file/:id",
            Box::new(|request: web_server::Request, _response| {
                let foo = SERVICE_ROOT.to_owned() + request.params.values().into_iter().next().unwrap();
                Path::new(foo.as_str()).into()
            }),
        )
        .not_found(Box::new(|_,_| Path::new(WEBPAGE_E404).into()))
        .launch(PORT);
}

fn generate_files_webpage() {
    let mut files_webpage = OpenOptions::new()
        .create(true)
        .write(true)
        .open(WEBPAGE_FILES)
        .unwrap();

    let filenames = fs::read_dir("service_files").unwrap();
    filenames.for_each(|file| {
        let filename = file.unwrap().file_name();
        writeln!(files_webpage, "{:?}", filename).unwrap();
    });
}
