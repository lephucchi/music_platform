use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialed_db(dsm: &str , max_conns: u32) -> PgPool {
    let db = PgPoolOptions::new().max_connections(max_conns).connect(dsm).await.expect("Cannot connect to Database");

    sqlx::migrate!().run(&db).await.expect("cannot migrate database");
    
    db

}
