#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::content::Json;
use rocket::request::Form;

#[derive(FromForm, Debug)]
struct Contact {
    name: String,
    email: String,
}

#[post("/contact", data="<contact_form>")]
fn new_contact(contact_form: Form<Contact>) -> String {
    let contact: Contact = contact_form.into_inner();
    let mut dummy_db: Vec<Contact> = Vec::new();
    dummy_db.push(contact);
    format!("Contact added successfully: {:?}", dummy_db)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json("{
        \"status\": \"success\",
        \"message\": \"Hello API!\"
    }")
}

fn main() {
    rocket::ignite().mount("/api", routes![hello, new_contact]).launch();
}
