use std::fs;
use std::path::PathBuf;

use std::error::Error;
use std::sync::{ Arc, Mutex };

use serde_json;
use std::collections::HashMap;

use uuid::Uuid;

use crate::http::*;
use crate::db;

type Sessions = Arc<Mutex<HashMap<String, Vec<db::Question>>>>;

fn get_content_type(path: PathBuf) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("ico")  => "image/x-icon",
        
        _ => "application/octet-stream",
    }
}

fn create_response(incoming: &str) -> Result<HttpResponse, Box<dyn Error>> {
    let static_dir: PathBuf = fs::canonicalize("./static")?;
    let filename = incoming.trim_start_matches("/static/");
    
    let full_path = static_dir.join(filename);
    let path: PathBuf = fs::canonicalize(&full_path)?;
    
    if path.starts_with(&static_dir) {
        let data = fs::read(&path)?;

        let content_type = get_content_type(path);

        Ok ( HttpResponse::new(
            200,
            "OK",
            content_type,
            None,
            data,
        ))
    } else {
        Err("Access denied".into())
    }
}

pub fn rout(request: &HttpRequest, db: &Arc<Mutex<db::Database>>, sessions: &Sessions) -> HttpResponse {
    let method = request.method.as_str();
    let path = request.path.as_str();
   
    match (method, path) {
        ("GET", path) if path.starts_with("/static/") => {
            match create_response(&request.path) {
                Ok(response) => response,
                Err(e) => {
                    eprintln!("Error: {e}");
                    HttpResponse::new(
                        404,
                        "Not Found",
                        "text/plain; charset=utf-8",
                        None,
                        format!("Path {} not found", request.path).into_bytes(),
                    )
                },
            }
        },

        ("GET", "/favicon.ico") => {
            match create_response("/static/favicon.ico") {
                Ok (response) => response,
                Err(e) => {
                    eprintln!("Error: {e}");
                    HttpResponse::new(
                        404,
                        "Not Found",
                        "text/plain; charset=utf-8",
                        None,
                        format!("Path {} not found", request.path).into_bytes(),
                    )
                },
            }
        },

        ("POST", "/api/questions/submit") => {
            match serde_json::from_slice::<Vec<db::Answer>>(&request.body) {
                Ok(answers) => {
                    let cookie  = if let Some(id) = request.headers.get("Cookie") { id.as_str() }
                    else {
                        return HttpResponse::new(
                            401,
                            "Unauthorized",
                            "text/plain; charset=utf-8",
                            None,
                            format!("Can not determine session").into_bytes(),
                        );
                    };

                    eprintln!("Got {} answers to evaluate; session: {}", answers.len(), cookie);

                    let session = sessions.lock().unwrap();
                    // dbg!("{:?}", &session);

                    let questions: &Vec<db::Question> = match session.get(cookie) {
                        Some(q) => q,
                        None => {
                            return HttpResponse::new(
                                401,
                                "Unauthorized",
                                "text/plain; charset=utf-8",
                                None,
                                format!("Can not determine session").into_bytes(),
                            );
                        },
                    };

                    let mut q = questions.iter();
                    let mut a = answers.iter();

                    let mut assussment: Vec<db::Assussment> = Vec::new();

                    loop {
                        let q = match q.next() {
                            Some(q) => q,
                            None => break,
                        };
                        let a = match a.next() {
                            Some(a) => a,
                            None => break,
                        };

                        assussment.push(db::Assussment {
                            id: a.id,
                            flag: db::Flag::get_flag(&a.answer, &q.answer),
                        });
                    };

                    dbg!("{:?}", &assussment);

                    let data = serde_json::to_string(&assussment).unwrap();

                    HttpResponse::json_with_cookie(data, cookie.to_string())

                    // HttpResponse::json(
                    //     serde_json::json!({
                    //         "status": "accepted",
                    //         "count": answers.len(),
                    //     })
                    //     .to_string(),
                    // )
                },
                Err(e) => HttpResponse::new(
                    400,
                    "Bad Request",
                    "application/json; charset=utf-8",
                    None,
                    format!(r#"{{"error":"{e}"}}"#).into_bytes(),
                ),
            }
        },

        ("GET", path) if path.starts_with("/api/") => {
            let segments: Vec<&str> = path
                .trim_start_matches('/')
                .split('/')
                .collect();

            match segments.as_slice() {
                ["api", "subjects"] => {
                    let db = db.lock().unwrap();
                    let data = serde_json::to_string(&db.get_subjects().unwrap()).unwrap();

                    HttpResponse::json(data)
                },
                ["api", "questions", subject] => {
                    let db = db.lock().unwrap();
                    let data = db.get_random_questions(subject).unwrap();

                    drop(db);

                    let mut sessions = sessions.lock().unwrap();
                    let cookie = Uuid::new_v4();

                    sessions.insert(cookie.to_string(), data);
                    dbg!("{:?}", &cookie);

                    let data = serde_json::to_string(
                        &db::FormatedQuestions::new(
                            sessions.get_key_value(
                                &cookie.to_string(),
                            ).unwrap().1
                        )
                    ).unwrap();

                    HttpResponse::json_with_cookie(
                        data,
                        cookie.to_string(),
                    )
                },

                _ => {
                    HttpResponse::new(
                        404,
                        "Not Found",
                        "text/plain; charset=utf-8",
                        None,
                        format!("Path {} not found", request.path).into_bytes(),
                    )
                },
            }
        },

        _ => {
            HttpResponse::new(
                404,
                "Not Found",
                "text/plain; charset=utf-8",
                None,
                format!("Path {} not found", request.path).into_bytes(),
            )
        },
    }
}
