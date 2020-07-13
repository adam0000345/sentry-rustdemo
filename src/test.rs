extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;
extern crate failure;

#[macro_use]
extern crate lazy_static;


use std::num::ParseIntError;


use std::env;
use std::io;
use std::collections::HashMap;
use std::process;
use sentry::integrations::failure::capture_error;
use sentry::{configure_scope, User};
use actix_web::{http};
use actix_web::Json;
use actix_web::Result;
use actix_web::http::Method;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string;


use std::sync::Mutex;

use sentry::integrations::panic::register_panic_handler;



lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, u32>> = {
        let mut Inventory = HashMap::new();
        Inventory.insert("wrench", 1);
        Inventory.insert("nails", 1);
        Inventory.insert("hammer", 1);
        Mutex::new(Inventory)
    };    
}





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


#[derive(Deserialize, Clone, Debug)]
struct CardSubmittedPayload {
    card_id: i64,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Item {
    id: String,
    name: String,
    price: f64,
    img: String,
}




#[derive(Serialize, Clone, Debug, Deserialize)]
struct CheckoutPayload {

    email: String,
    cart: Vec<Item>,

}


fn process_order(cart: &Vec<Item>) -> HttpResponse {


    let mut map = HASHMAP.lock().unwrap();
    println!("The entry for `0` is \"{:?}\".", map.get("foo"));

    //println!("There're {} entries in map\".", map.list);

  

    for cartitem in cart.iter() {

        println!("CART ITEM HERE");
        dbg!(cartitem);
        
        if map.get(cartitem.id.as_str()).map(|id| id <= &0).unwrap_or(false) {

            
            println!("OUT OF ITEM HIT");

            let mut string = String::new();
            string.push_str("Not enough inventory for ");
            string.push_str(&cartitem.id);


            
            let result: HttpResponse = string.to_string().into();
    
            return result;
            
            

            //return Err(io::Error::new(io::ErrorKind::Other, string).into())

            //return Err(failure::format_err!("Not enough inventory for {:?}", cartitem.id));
            //panic!("Not enough invetory for {:?}");

        } else if map.get(cartitem.id.as_str()).map(|id| id > &0).unwrap_or(false) {
        
            
                if let Some (id) = map.get_mut(cartitem.id.as_str()) {
                    *id -= 1;
                    println!("Success: {:?} was purchased, remaining stock is {:?}", cartitem.id, cartitem.id.as_str());
                } else {
                    // handle the error case. maybe:
                    false;
                }
            
            

        }
        
    }

    let result: HttpResponse = (format!("Everything ok")).into();

    return result;
      

}


//fn checkout(body: Json<CheckoutPayload>) -> HttpResponse { 

fn checkout(req: HttpRequest, body: Json<CheckoutPayload>) -> HttpResponse { 

   
    //SETTING SENTRY EVENT CONTEXT//
    configure_scope(|scope| {
        //scope.set_tag("my-tag", "my value");
        scope.set_user(Some(User {
            //id: Some(42.to_string()),
            email: Some((*body.email).to_string()),
            ..Default::default()
        }));

        let mut string = String::new();

   
        string.push_str(req.headers().get("X-Transaction-ID").unwrap().to_str().unwrap());

        scope.set_tag("transaction_id", string);

        string = String::new();
        string.push_str(req.headers().get("X-Session-ID").unwrap().to_str().unwrap());

        scope.set_tag("session_id", string);

        string = String::new();

        string.push_str(req.headers().get("inventory").unwrap().to_str().unwrap());

        scope.set_tag("inventory", string);

    });
    
    return process_order(&body.cart);

}



fn main() {

    register_panic_handler();


    let _guard = sentry::init("https://ef73d8aa7ac643d2b6f1d1e604d607eb@o87286.ingest.sentry.io/5250920");
    env::set_var("RUST_BACKTRACE", "1");
    
    server::new(|| {
        App::new().middleware(SentryMiddleware::new())
        .resource("/handled_new",|r| r.method(http::Method::GET).f(handled_new))
        .resource("/checkout", |r| r.method(http::Method::POST).with(checkout))}).bind("127.0.0.1:3001")
        .unwrap()
        .run();

        //.resource("/checkout", |r| r.method(http::Method::POST).f(checkout))}).bind("127.0.0.1:3001").unwrap()
        sentry::integrations::panic::register_panic_handler();

        
}