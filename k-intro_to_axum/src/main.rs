use std::{collections::HashMap, usize};

use axum::{
    body::Body, extract::{Path, Query, Request}, http::StatusCode, response::{IntoResponse, Response}, routing::{delete, get, options, patch, post, put}, Json, Router
};

// Dynamic Path: http://localhost:3000/silicon/student/<sic>
// Query Parameters: http://localhost:3000/silicon/student?sic=123&branch=cs

#[tokio::main]
async fn main() {
    let app = Router::new()
        // // Routes
        // .route("/", get(hello))
        // .route("/silicon", get(silicon))
        // .route("/silicon/student", get(silicon_student_query_handler)) // Student List
        // .route("/silicon/student/{sic}/{branch}", get(silicon_student_details_handler))
        .route("/submit", post(submit_form_handler))
        .route("/submit/manual", post(submit_form_manual_handler))
        .fallback(handle_not_found);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler functions
async fn handle_not_found() -> &'static str {
    "Not Found"
}

async fn silicon() -> Json<serde_json::Value> {
    let response = serde_json::json!({
        "message": 1,
        "status": "success"
    });
    Json(response)
}

// Axum Extractor: Path - Hard Limit = 16
// Path Parameters
async fn silicon_student_details_handler(Path((sic, branch)): Path<(String, String)>) -> String {
    format!("Hello, Silicon Student with SIC: {}, Branch: {}", sic, branch)
}


// Axum Extractor: Query 
// Query Parameters
async fn silicon_student_query_handler(Query(params): Query<HashMap<String, String>>) -> String {
    // let student_sic: &str = params.get("sic").unwrap_or("NA");
    let student_sic = params.get("sic").unwrap();
    let student_branch = params.get("branch").unwrap();

    format!("Hello, Silicon Student with SIC: {}, Branch: {}", student_sic, student_branch)
}

// Parsing Request Body Manually
// This function manually reads the request body and parses it into a User struct.
// It uses `axum::body::to_bytes` to read the body and `serde_json::from_slice` to parse the JSON.
async fn submit_form_manual_handler(req: Request) -> impl IntoResponse {
    println!("Received request: {:?}", req);

    let body = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
    println!("Request body: {:?}", body);
    let user: Result<User, ()> = serde_json::from_slice(&body)
        .map_err(|e| {
            eprintln!("Error parsing JSON: {}", e);
    });

    let Ok(user) = user else {
        return "Invalid JSON format";
    };

    println!("Parsed user manually: {:?}", user);

    "Form submitted successfully!"
}

// Handle JSON with Predefined Struct
// This function will handle the JSON payload sent in the request body
// It uses the `Json` extractor to parse the incoming JSON into a predefined struct.

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct InputUser {
    name: Option<String>,
    age: Option<u32>,
}

async fn submit_form_handler(Json(payload): Json<InputUser>) -> impl IntoResponse {

    // Validate request body / payload
    if payload.name.is_none() || payload.age.is_none() {
        return (StatusCode::BAD_REQUEST, "Invalid input: name and age are required");
    } 

    // Process the payload
    println!("Parsed User: {:?}", payload);

    (StatusCode::CREATED, "Form submitted successfully!")
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct User {
    name: String,
    age: u32,
}

// async fn hello() -> impl IntoResponse {
//     let user = User {
//         name: "John Doe".to_string(),
//         age: 30,
//     };
//     let json_data = serde_json::to_string(&user).unwrap_or("Error".to_string());
//     (axum::http::StatusCode::CREATED, json_data)
// }