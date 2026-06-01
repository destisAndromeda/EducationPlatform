mod http;
mod router;
mod thread_pool;
mod db;

use http::*;
// use db::*;

use std::io::{ Write };
use std::error::Error;

use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

use std::net::{ TcpListener, TcpStream };
// use uuid::Uuid;

type Sessions = Arc<Mutex<HashMap<String, Vec<db::Question>>>>;

fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<db::Database>>, sessions: Sessions) {
    let peer = stream.peer_addr().unwrap();

    match HttpRequest::parse_request(&stream) {
        Some (request) => {
            let client_ip = request.headers
                .get("X-Forwarded-For")
                .map(|s| s.as_str())
                .unwrap_or("unkown");

            println!("{} {} [{}]", request.method, request.path, client_ip);
            let response = router::rout(&request, &db, &sessions);
            stream.write_all(&response.to_bytes()).unwrap();
        },
        None => {
            eprintln!("[{}] Unable to parse the request", peer);
            let response = HttpResponse::new(400, "Bad request", "text/plain; charset=utf-8", None, "Invalid HTTP Request".to_string().into_bytes());
            stream.write_all(&response.to_bytes()).unwrap();
        },
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let listener = TcpListener::bind("0.0.0.0:7070").unwrap();
    println!("HTTP Server listening http://localhost:7070");

    let thread_pool = thread_pool::ThreadPool::new(4);
    
    let database: Arc<Mutex<db::Database>> =
        Arc::new( Mutex::new( db::Database::new("questions.db")? ) );

    let sessions: Sessions =  Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) =>  {
                let db = Arc::clone(&database);
                let sessions = Arc::clone(&sessions);
                thread_pool.execute(move || handle_connection(stream, db, sessions));
            },
            Err(e) => eprintln!("Error: {}", e),
        };
    }

    Ok(())
}
