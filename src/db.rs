use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn db_new() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();


    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS listings (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            title VARCHAR(250) NOT NULL,
            url TEXT NOT NULL,
            price REAL NOT NULL,
            details TEXT NOT NULL,
            seller TEXT NOT NULL,
            shipping TEXT NOT NULL,
            location TEXT NOT NULL,
            sold BOOLEAN NOT NULL
        );"
    ).execute(&db).await?;

    println!("Create user table result: {:?}", result);

    Ok(db)
}
