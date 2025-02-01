use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, 
    thread, 
    time::Duration,
};

use hellohello::ThreadPool;

#[allow(dead_code)]
enum Mode {
    SingleThreaded,
    ThreadPerRequest,
    ThreadPool(usize),
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mode = Mode::ThreadPool(4);

    match mode {
        Mode::SingleThreaded | Mode::ThreadPerRequest => {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                match mode {
                    Mode::SingleThreaded => run_single_threaded(stream),
                    Mode::ThreadPerRequest => run_req_per_thread(stream),
                    _ => (),
                }
            }
        },
        Mode::ThreadPool(num) => {
            let pool = ThreadPool::new(num);
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            println!("Shutting down.");
        }
    }

    
}


fn run_single_threaded(stream: TcpStream) {
    handle_connection(stream);
}

fn run_req_per_thread(stream: TcpStream) {
    thread::spawn(|| {
        handle_connection(stream);
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
