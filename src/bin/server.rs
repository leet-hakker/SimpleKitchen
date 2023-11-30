//! SimpleKitchen server

#![feature(lint_reasons)]
use dotenvy::dotenv;
use simple_kitchen::ingredient::Ingredient;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

async fn insert_ingredients_to_table(
    conn: &Pool<Postgres>,
    ingredients: Vec<String>,
) -> Result<(), sqlx::Error> {
    for ingr in ingredients {
        sqlx::query::<Postgres>("INSERT INTO ingredient(ingredient_name) VALUES $1")
            .bind(ingr)
            .execute(conn)
            .await?;
    }

    Ok(())
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    #[allow(
        clippy::expect_used,
        reason = "The server should not start if there is no .env file"
    )]
    dotenv().expect(".env file not found");

    #[allow(
        clippy::expect_used,
        reason = "The server should not start if the database is not known"
    )]
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await?;

    // let row: (i64,) = sqlx::query_as("SELECT $1")
    //     .bind(150_i64)
    //     .fetch_one(&pool)
    //     .await?;
    //
    // assert_eq!(row.0, 150);

    let row: Vec<Ingredient> = sqlx::query_as("SELECT * FROM ingredient")
        .fetch_all(&pool)
        .await?;
    println!("{:?}", row);

    Ok(())
}
