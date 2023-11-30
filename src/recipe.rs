use crate::user::UserId;
use crate::{
    ingredient::IngredientId,
    types::{Diet, MealType},
};
use sqlx::{pool::Pool, postgres::PgQueryResult, Postgres};
use sqlx::{query_builder, QueryBuilder};

/// Public key in the `recipe` table
#[derive(sqlx::FromRow, sqlx::Type, Debug, Clone, Copy)]
#[sqlx(transparent)]
pub struct RecipeId(pub i32);

// #[derive(sqlx::Type)]
// #[sqlx(transparent, no_pg_array)]
// pub struct RecipeIdArray(Vec<Re>);

/// A database row from the `recipe` table.
///
#[derive(sqlx::FromRow, Debug)]
pub struct Recipe {
    /// Primary key
    pub recipe_id: RecipeId,
    /// Name of the recipe
    pub recipe_name: String,
    /// User that submitted the recipe
    pub recipe_author: UserId,
    /// Instructions associated with the recipe
    pub instructions: Vec<String>,
    /// Type of meal the recipe makes
    pub meal_type: Option<MealType>,
    /// Diets that the recipe is suitable for
    pub complies_with_diets: Vec<Diet>,
}

impl Recipe {
    #[allow(missing_docs, reason = "`new` is self-explanatory")]
    pub fn new(
        recipe_name: String,
        recipe_author: UserId,
        instructions: Vec<String>,
        meal_type: MealType,
        complies_with_diets: Vec<Diet>,
    ) -> Self {
        Self {
            recipe_id: RecipeId(-1),
            recipe_name,
            recipe_author,
            instructions,
            meal_type: Some(meal_type),
            complies_with_diets,
        }
    }
}

/// Inserts a `Recipe` struct into the `recipe` table
pub async fn upload_recipe(
    conn: &Pool<Postgres>,
    recipe: Recipe,
) -> Result<RecipeId, sqlx::error::Error> {
    sqlx::query_as(r#"INSERT INTO recipe (recipe_name, recipe_author, instructions, meal_type, complies_with_diets) VALUES ($1, $2, $3, $4, $5) RETURNING recipe_id as "RecipeId" "#)
        .bind(recipe.recipe_name)
        .bind(recipe.recipe_author as UserId)
        .bind(&recipe.instructions)
        .bind(recipe.meal_type as Option<MealType>)
        .bind(recipe.complies_with_diets as Vec<Diet>)
        .fetch_one(conn).await
}
