use rocket::{launch, routes};
pub mod history;
pub mod currency;
mod data;
mod util;

use crate::history::history::{get_currency_history, get_currency_latest};
use crate::currency::currency::get_currencies;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_currency_history, get_currency_latest, get_currencies])
}
