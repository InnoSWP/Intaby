use rocket::get;

pub fn rocket(config: rocket::Config) -> rocket::Rocket<rocket::Build> {
    rocket::custom(config).mount("/", rocket::routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "This server handles games\n"
}
