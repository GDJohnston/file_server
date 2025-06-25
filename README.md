# File server

Intented to be a follow on from the last chapter of [The Rust Programming Language Book](https://doc.rust-lang.org/book/), this project uses the [web_server](https://crates.io/crates/web_server) crate to implement a files server which lists the files available for download and serves them when requested.

## Operation

Run the executable with no arguments and the server will be launched at <http://127.0.0.1:8080/>, the first argument allows you to set the port number to launch to.

```bash
$ ./target/debug/file_server
Server launching at 127.0.0.1:8080

$ ./target/debug/file_server 8081
Server launching at 127.0.0.1:8081
```

After the first launch, the `service_files` folder will be generated with some example files. The files in this folder can be replaced with files of your choice. If this folder is deleted or is left empty, it will be repopulated the next time the server is started. Any file put in this folder will be available for download through this server.

Navigate to the `/files` webpage to see a list of files for service. This webpage is generated everytime the webpage is loaded and lists the files in the `service_files` folder. Once you have identified a file you would like to download, nagivate to `file/x` where `x` is the file you would like to download. For example, if you would like to download `hello.html` navigate to `file/hello.html` the file extention must be included. This file should now download.

## Further work

There is some areas I have found that I could work further on in the future:

- Handle errors better so the server doesn't need to panic so often.
- Add a external download feature so the server can download from the internet
over slow connections and stream to other devices locally when convenient.
- Make it possible to select a different service folder.
