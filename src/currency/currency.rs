use rocket::get;

#[get("/currencies")]
pub fn get_currencies() -> String {
    String::from("Currencies endpoint")
}
