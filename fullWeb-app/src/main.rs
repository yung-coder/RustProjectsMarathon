use actix_web::{get , patch , post, App , HttpResponse , HttpServer,  Responder };

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas avaliable")
}

#[post("/buypizza")]
async fn buy_pizza() -> impl Responder {
    HttpResponse::Ok().body("Buying a Pizza")
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a Pizza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_pizzas)
        .service(buy_pizza)
        .service(update_pizza)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await 
}
