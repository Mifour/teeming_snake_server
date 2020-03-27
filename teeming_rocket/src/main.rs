use std::collections::HashMap;

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/generate")]
fn generate() -> String {
    //ask for a code 
    return generate_code();
}

#[get("/free/<code>")]
fn free(code: &RawStr){
	// make a code available
	
}

#[get("/find/<code>")]
fn find(code: &RawStr) -> String {
	// given a code, return the adress bound to it
	// if the code doesn't link to any adress, return 204 status code
}

fn main() {
	let mut n:i64 = 0;
	let mut book = HashMap::new();


    rocket::ignite().mount("/", routes![index]).launch();
}
