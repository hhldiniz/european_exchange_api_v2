use rocket::get;

#[get("/history?<start_at>&<end_at>&<base>&<symbols>")]
pub fn get_currency_history(start_at: String, end_at: String, base: String, symbols: Option<String>) -> String{
    format!("Passed parameters were start_at = {}, end_at = {}, base = {}, symbols = {:?}", start_at, end_at, base, symbols)
}

#[get("/latest?<symbol>")]
pub fn get_currency_latest(symbol: String) -> String{
    format!("Passed parameters were symbol = {}", symbol)
}