mod file_server;

const PORT: i32 = 8080;

const SERVICE_ROOT: &str = "service_files/";

fn main() {
    file_server::launch_server(SERVICE_ROOT, PORT);
}