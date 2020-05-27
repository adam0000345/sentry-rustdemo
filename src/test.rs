extern crate actix_web;
extern crate sentry;
extern crate sentry_actix;

use std::num::ParseIntError;


use std::env;
use std::io;
use std::collections::HashMap;
use std::process;
use sentry::integrations::failure::capture_error;
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

/* async */ fn handled_new(_req: ()) -> Result<(), Error> {
    let result = match multiply_new("t", "2") {
        Ok(result) => result,
        Err(err) => {
            capture_error(&err);
            // Err(err); // This is not needed
            return Ok(());
        }
    };
    return Ok(());
}

fn guess(n: i32) -> bool {
    if n < 1 || n > 10 {
        panic!("Invalid number: {}", n);
    }
    n == 5
}

async fn handled(_req: &HttpRequest) -> Result<(), Error> {

    let result = match guess(11) {
        Ok(result) => result,
        Err(err) => {
           capture_error(&err);
           Err(err);
           return http;
        }
    };

    return HttpResponse::Ok().body("Multiplication Was");

}



    


// Guess a number between 1 and 10.
// If it matches the number we had in mind, return true. Else, return false.
//fn guessunhandled(n: i32) -> bool {
////    if n < 1 || n > 10 {
       //print "Done"
  //  }
 //   n == 5
//}

//fn unhandled(_req: &HttpRequest) {
    
 //   guessundandled(11);

//}

//fn checkout(_req: &HttpRequest) -> Result<String, Error> {
    
 //   return "Success"
//}



fn main() {

    register_panic_handler();


    let _guard = sentry::init("https://ef73d8aa7ac643d2b6f1d1e604d607eb@o87286.ingest.sentry.io/5250920");
    env::set_var("RUST_BACKTRACE", "1");
    
          //|r| r.f(handled)).resource("/unhandled", |r| r.f(unhandled))
        //.resource("/checkout", |r| r.f(checkout))
    server::new(|| {
        App::new().middleware(SentryMiddleware::new())
        .resource("/handled", |r| r.f(handled))}).bind("127.0.0.1:3001")
        .unwrap()
        .run();

        sentry::integrations::panic::register_panic_handler();

        // Sentry will capture this
        panic!("Everything is on fire!");
}