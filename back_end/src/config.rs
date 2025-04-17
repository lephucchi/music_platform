use dotenv::var;

pub fn get_dsn() -> String {
    var(key)
}

pub fn get_port() -> u16 {
    var("PORT").unwrap().parse().unwrap()
}

