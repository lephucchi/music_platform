use sqlx::{ Pool, Postgres};

#[derive(Debug, Clone)]
pub struct DBClients{
    pub pool: Pool<Postgres>
}

impl DBClients {
    pub fn new(pool: Pool<Postgres>)-> Self {
        DBClients { pool }
    }
}