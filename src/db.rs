use std::{env, io::Error};

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn db_init() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in `.env`");
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Connection link is not valid or no running database found");
    Ok(db)
}

pub async fn db_fill() -> Result<(), Error> {
    Ok(())
}
