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



#[post("/proxy/threads")]
async fn create_thread_proxy(
    req_body: web::Json<ThreadRequest>, 
    http_client: web::Data<reqwest::Client>,
) -> impl Responder {
    let target_url = "https://api.yay.space/v1/threads";
    let res = http_client
        .post(target_url)
      
        .header("authorization", &req_body.token)
       .header("User-Agent", "Yay/3.40.0 (com.yay; build:340000; iOS 17.5.1) Alamofire/5.9.1")
        .json(&json!({
            "group_id": req_body.id,
            "title": req_body.text,
            "thread_icon_filename": null
        }))
        .send()
        .await;

    println!("{:?}", res);
    match res{

        Ok(response) =>{
            let status = StatusCode::from_u16(response.status().as_u16())

                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                let body = response.json::<serde_json::Value>().await.unwrap_or_default();
            HttpResponse::build(status).json(body)

            
        }
        Err(e) =>{
      
        eprintln!("API Error Details: {:?}", e); 

      
            HttpResponse::InternalServerError().json(json!({
        "error": "Service Temporarily Unavailable",
            "message": "外部サービスとの通信に失敗しました。時間をおいて再度お試しください。"
            }))
        }
}}

#[post("/proxy/review")]
async fn send_review(
 req_body: web::Json<ThreadRequest>,
 http_client: web::Data<reqwest::Client>,
) -> impl Responder { 
    println!("{:#?}", req_body);


   let target_url = format!("https://api.yay.space/v1/users/reviews/{}",req_body.id);
    let res = http_client
        .post(target_url)
       
        .header("authorization", &req_body.token)
        .header("User-Agent", "Yay/3.40.0 (com.yay; build:340000; iOS 17.5.1) Alamofire/5.9.1")
       
        .json(&json!({
            "comment": &req_body.text
        }))
        .send()
        .await;
    println!("{:?}", res);
println!("{}",req_body.token);


match res{
   Ok(response) =>{
   let status = StatusCode::from_u16(response.status().as_u16())
.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
let body = response.json::<serde_json::Value>().await.unwrap_or_default();
    
  HttpResponse::build(status).json(body) 
   }
    Err(e) =>{

eprintln!("API Error Details: {:?}", e); 


HttpResponse::InternalServerError().json(json!({
    "error": "Service Temporarily Unavailable",
    "message": "外部サービスとの通信に失敗しました。時間をおいて再度お試しください。"
}))
}
}
}

#[post("/proxy/join")]
async fn join_group(
 req_body: web::Json<JoinRequest>,
 http_client: web::Data<reqwest::Client>,
) -> impl Responder { 
    println!("{:#?}", req_body);


   let target_url = format!("https://api.yay.space/v1/groups/{}/join",req_body.id);
    let res = http_client
        .post(target_url)
       
        .header("authorization", &req_body.token)
      .header("User-Agent", "Yay/3.40.0 (com.yay; build:340000; iOS 17.5.1) Alamofire/5.9.1")
        .send()
        .await;
    println!("{:?}", res);
println!("{}",req_body.token);


match res{
   Ok(response) =>{
   let status = StatusCode::from_u16(response.status().as_u16())
.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
let body = response.json::<serde_json::Value>().await.unwrap_or_default();

  HttpResponse::build(status).json(body) 
   }
    Err(e) =>{

eprintln!("API Error Details: {:?}", e); 


HttpResponse::InternalServerError().json(json!({
    "error": "Service Temporarily Unavailable",
    "message": "外部サービスとの通信に失敗しました。時間をおいて再度お試しください。"
}))
}
}
}




/*#[get("/home")]
async fn home() -> impl Responder {
HttpResponse::OK().
}
*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
  
    let client = reqwest::Client::new();

    println!("Starting server on 0.0.0.0:5000");

  
    HttpServer::new(move || {
        App::new()
          
            .app_data(web::Data::new(client.clone())) 
            
            .service(serve_html)
        .service(thread_page)
            .service(create_thread_proxy)
          .service(review_page)
         .service(send_review)
      .service(join_group)
            
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
