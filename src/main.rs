use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
};

use rust_networking::{Bucket, ThreadPool};

// Max 4 threads
// 3 requests are allowed in a 4 second sliding window
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7452").unwrap();
    let mut bucket = Bucket::new(0.25, 3);
    bucket.listen();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match bucket.decrement() {
            true => (),
            false => continue,
        };
        let _stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
