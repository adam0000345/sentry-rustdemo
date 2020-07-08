extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;



use std::num::ParseIntError;


use std::env;
use std::io;
use std::collections::HashMap;
use std::process;
use sentry::integrations::failure::capture_error;
use actix_web::{http};
use actix_web::Json;
use actix_web::http::Method;


use sentry::integrations::panic::register_panic_handler;





use actix_web::{server, App, Error, HttpRequest, http::StatusCode, HttpResponse};
use sentry_actix::SentryMiddleware;

use actix_web::Responder;




// My version
fn multiply_new(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number: i32 = first_number_str.parse()?;
    let second_number: i32 = second_number_str.parse()?;
    Ok(first_number * second_number)
}

fn handled_new(_req: &HttpRequest) -> HttpResponse {
    let first = "t";
    let second = "2";
    let result = match multiply_new(first, second) {
        Ok(result) => result,
        Err(err) => {
            // Foo is the ParseIntError turned into a failure::Error.
            let foo = err.into();
            capture_error(&foo);
            let result: HttpResponse = "try again".to_string().into();

            return result;
        }
    };
    // Not sure what the plan is for the 'result' variable.
    let result: HttpResponse = (format!("{} * {} => {}", first, second, result)).into();

    return result;
    //return Ok(format!("{} * {} => {}", first, second, result))


}

// fn process_order(inventory):
//     global Inventory
//     tempInventory = Inventory
//     for item in cart:
//         if Inventory[item['id']] <= 0:
//             raise Exception("Not enough inventory for " + item['id'])
//         else:
//             tempInventory[item['id']] -= 1
//             //print 'Success: ' + item['id'] + ' was purchased, remaining stock is ' + str(tempInventory[item['id']])
//     Inventory = tempInventory 

struct CardSubmittedPayload {
    card_id: i64,
}


fn checkout(body: Json<CardSubmittedPayload>, req: HttpRequest) -> HttpResponse {

    let card_id = body.card_id;

    println!("card it:   {}", card_id);

    //println!("HERE IS THE REQUET");
    //println!("{:?}", _req);
    
    let foo: HttpResponse = "success".to_string().into();
    return foo;
    //HttpResponse::new(http::StatusCode::from_u16(200u16).unwrap());

}

    //return foo;

   
    //return OK(_req.to_string());
    
    //let mut inventory = HashMap::new();

    //inventory.insert("wrench", "1");
    //inventory.insert("nails", "1");
    //inventory.insert("hammer", "1");

    //order = json.loads(_req.body());
    
    //Err(io::Error::new(io::ErrorKind::Other, "An error happens here").into());
    //order = json.loads(&HttpRequest.data);
    //print "Processing order for: " + order["email"]
    //cart = order["cart"]
    
    //process_order(cart);


fn main() {

    register_panic_handler();


    let _guard = sentry::init("https://ef73d8aa7ac643d2b6f1d1e604d607eb@o87286.ingest.sentry.io/5250920");
    env::set_var("RUST_BACKTRACE", "1");
    
          //|r| r.f(handled)).resource("/unhandled", |r| r.f(unhandled))
        //.resource("/checkout", |r| r.f(checkout))
    server::new(|| {
        App::new().middleware(SentryMiddleware::new())
        .resource("/handled_new",|r| r.method(http::Method::GET).f(handled_new))
        .resource("/checkout", |r| r.method(http::Method::POST).with(checkout))}).bind("127.0.0.1:3001")
        .unwrap()
        .run();

        //.resource("/checkout", |r| r.method(http::Method::POST).f(checkout))}).bind("127.0.0.1:3001").unwrap()
        sentry::integrations::panic::register_panic_handler();

        
}