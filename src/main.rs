#[macro_use] extern crate rocket;
use rocket::State;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
struct AppState {
    yes_responses: Vec<&'static str>
}

#[get("/")]
fn index() -> &'static str {
    "GET /yes\nExample:\ncurl http://localhost:8000/yes\n> \"Why of course!\""
}

#[get("/yes")]
fn yes(state: &State<AppState>) -> String {
    let response = state.yes_responses.choose(&mut rand::thread_rng()).unwrap();
    response.to_string()
}

#[launch]
fn rocket() -> _ {
    let yes_responses: Vec<&'static str> = serde_json::from_str(include_str!("../responses.json")).unwrap();

    rocket::build()
        .mount("/", routes![index, yes])
        .manage(AppState {
            yes_responses,
        })
}