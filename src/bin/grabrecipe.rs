//! Used to grab recipes from themealdb
#![feature(lint_reasons)]
#![allow(
    missing_docs,
    reason = "This is not a part of the project. This code is experimental"
)]
#![allow(clippy::missing_docs_in_private_items)]
use dotenvy::dotenv;
use mealdb::datamodel::Meal;
use mealdb::traits::MealDbBaseV1;
use simple_kitchen::ingredient::{upload_ingredients, Ingredient, IngredientId};
use simple_kitchen::recipe::{upload_recipe, Recipe, RecipeId};
use simple_kitchen::recipe_ingredient::{upload_recipe_ingredients, RecipeIngredient};
use simple_kitchen::types::{Diet, MealType};
use simple_kitchen::user::{create_user, User, UserId};
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;
use sqlx::{Pool, QueryBuilder};
use std::env;

fn recipe_from_meal(meal: &Meal) -> Recipe {
    let instructions: Vec<String> = meal
        .instructions
        .split("\r\n")
        .map(|s| s.to_string())
        .collect();

    Recipe::new(
        meal.name.clone(),
        UserId(1),
        instructions,
        MealType::Snack,
        vec![],
    )
}

/// Extracts ingredients from
fn ingredient_ids_from_meal(meal: &Meal) -> Vec<Ingredient> {
    meal.ingreedients
        .iter()
        .map(|ing_name| Ingredient::new(ing_name.to_string().to_lowercase(), None))
        .collect()
}

fn measures_from_meal(meal: &Meal) -> Vec<Option<String>> {
    meal.measures
        .iter()
        .map(|m| match m.len() {
            0 => None,
            _ => Some(m.to_owned()),
        })
        .collect()
}

fn gen_recipe_ingredients(
    recipe_id: RecipeId,
    ingredient_ids: &Vec<IngredientId>,
    measures: Vec<Option<String>>,
) -> Vec<RecipeIngredient> {
    (0..ingredient_ids.len())
        .map(|i| {
            RecipeIngredient::new(
                ingredient_ids[i],
                recipe_id,
                0.0,
                measures.get(i).unwrap_or(&None).clone(),
            )
        })
        .collect()
}

async fn meal_to_table_types(
    conn: &Pool<Postgres>,
    meal: Meal,
) -> Result<RecipeId, sqlx::error::Error> {
    let recipe: Recipe = recipe_from_meal(&meal);
    let recipe_id = upload_recipe(conn, recipe).await?;
    println!("Recipe uploaded successfully");
    let ingredients: Vec<Ingredient> = ingredient_ids_from_meal(&meal);
    let ingredient_ids = upload_ingredients(conn, ingredients).await?;
    println!("Ingredients uploaded successfully");
    let measures = measures_from_meal(&meal);
    let recipe_ingredients: Vec<RecipeIngredient> =
        gen_recipe_ingredients(recipe_id, &ingredient_ids, measures);
    upload_recipe_ingredients(conn, recipe_ingredients).await?;
    println!("Recipe ingredients uploaded successfully");
    Ok(recipe_id)
}

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[allow(
        clippy::expect_used,
        reason = "The server should not start without a .env file"
    )]
    dotenv().expect(".env file not found");

    #[allow(
        clippy::expect_used,
        reason = "The server cannot start without the DATABASE_URL environment variable"
    )]
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Something went wrong connecting to the database");

    let user = User::new("themealdb".to_string());
    create_user(&pool, user).await?;

    let api = mealdb::V1::new("https://themealdb.com", "1");

    for i in 0..100 {
        let meal = match api.get_random_meal().await {
            Ok(m) => m,
            Err(_) => return Ok(()),
        };

        let recipe_id = meal_to_table_types(&pool, meal).await?;

        println!("{:?}", recipe_id);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(())
}
