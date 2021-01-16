#![feature(decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::request::Form;
use rocket::response::content::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;

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

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        first_name: String,
        last_name: String,
    }

    let context = Context {
        first_name: String::from("Daniel"),
        last_name: String::from("Vilela")
    };

    Template::render("home", context)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index])
        .mount("/api", routes![hello, new_contact])
        .attach(Template::fairing())
        .launch();
}
