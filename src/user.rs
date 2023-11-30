use sqlx::postgres::PgQueryResult;
use sqlx::{Pool, Postgres};

use crate::ingredient::IngredientId;
use crate::recipe::RecipeId;

/// Transparent UserId struct
#[derive(sqlx::Type, Debug, Default)]
#[sqlx(transparent)]
pub struct UserId(pub i32);

/// Row from the `users` table, as a struct
#[derive(sqlx::FromRow, sqlx::Type)]
pub struct User {
    user_id: UserId,
    user_name: String,
    saved_recipes: Vec<RecipeId>,
    avoidances: Vec<IngredientId>,
}

impl User {
    #[allow(missing_docs)]
    pub fn new(user_name: String) -> Self {
        Self {
            user_id: Default::default(),
            user_name,
            saved_recipes: Default::default(),
            avoidances: Default::default(),
        }
    }
}

/// Creates a new user in the database
pub async fn create_user(
    conn: &Pool<Postgres>,
    user: User,
) -> Result<PgQueryResult, sqlx::error::Error> {
    sqlx::query!(
        "INSERT INTO users (user_name, saved_recipes, avoidances) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        user.user_name,
        user.saved_recipes as _,
        user.avoidances as _
    )
    .execute(conn)
    .await
}
