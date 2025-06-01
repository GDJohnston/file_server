use file_server::ThreadPool;
use std::{
    fs::{self, OpenOptions},
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
};
extern crate web_server;

const WEBPAGE_ROOT: &'static str = "webpages/";
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
            Box::new(|request: web_server::Request, response| {
                // format!("{:#?}", request.get_path()).into()
                Path::new("service_files/video.mp4").into()
            }),
        )
        .not_found(Box::new(|_,_| Path::new(WEBPAGE_E404).into()))
        .launch(8080);
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // let pool = ThreadPool::new(4);

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // }
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

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", WEBPAGE_INDEX),
        "GET /files HTTP/1.1" => ("HTTP/1.1 200 OK", WEBPAGE_FILES),
        _ => ("HTTP/1.1 404 NOT FOUND", WEBPAGE_E404),
    };

    if filename == WEBPAGE_FILES {
        generate_files_webpage();
    }

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
