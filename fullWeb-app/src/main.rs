use actix_web::{web::Path, get , patch , post, web::Json, App , HttpResponse , HttpServer,  Responder };
mod models;
use validator::Validate;
use crate::models::pizza::{ BuyPizzaRequest , UpdatePizzaURL};

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas avaliable")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body(format!("pizza entered is {pizza_name}"))
        }
        Err(_) => HttpResponse::Ok().body("Pizza name is required"),
    }
}

#[patch("/updatepizza/{uuid}")]
async fn update_pizza(update_pizza_url: Path<UpdatePizzaURL>) -> impl Responder {
    let uuid = update_pizza_url.into_inner().uuid;

    HttpResponse::Ok().body(format!("Updating the pizza with {uuid}"))
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

    // basic web program setup now can connect with any database 
}
