use sea_orm::{Database, DbErr};
use futures::executor::block_on;
const DATABASE_URL: &str = "123";
const DB_NAME: &str = "fzsdbdev";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
