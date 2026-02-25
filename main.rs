/*rustのさーばーに書き換え中*/
use actix_web::{post,get, web, App, HttpResponse, HttpServer, Responder, http::StatusCode}; 

use serde::Deserialize;
use serde_json::json;
//use std::fs;
use actix_files::NamedFile;

use reqwest;

#[derive(Deserialize,Debug)]
struct ThreadRequest {
   
    id: String,
    token: String,
    text: String,
}
#[derive(Deserialize, Debug)]
struct JoinRequest { 
    id: String,
    token: String,
}

#[get("/")] 
async fn serve_html() -> actix_web::Result<NamedFile> {
  
    Ok(NamedFile::open("v2/home.html")?)
}

#[get("/threads")]
async fn thread_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("v2/thread.html")?)
}

#[get("/review")]
async fn review_page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("v2/review.html")?)
}


