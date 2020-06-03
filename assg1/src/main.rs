#![feature(proc_macro_hygiene,decl_macro)]
#[macro_use] extern crate rocket;
use rocket::response::content::Html;
extern crate reqwest;
// use std::collections::HashMap;
// use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
// use std::io::Read;





#[get("/test")]
fn hello2()->String{
    "hello2".to_string()
}



#[get("/")]
fn index() -> Html<&'static str> {
   Html(r"
   <h1>Hello form page 1</h1>"
)

}


fn main(){

    rocket::ignite().mount("/",routes![index]).launch();
    // rocket::ignite().mount("/test",routes![hello2]).launch();

}