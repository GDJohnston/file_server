use std::env;

mod file_server;

const PORT: i32 = 8080;

const SERVICE_FOLDER: &str = "service_files/";

fn main() {
    let mut port = PORT;

    let mut args = env::args();
    if let Some(port_arg) = args.next() {
        port = i32::from_str_radix(&port_arg, 10).unwrap();
    }

    file_server::launch_server(SERVICE_FOLDER, port);
}