use sqlx::Pool;
use sqlx::Postgres;
use sqlx::QueryBuilder;

use crate::types::Diet;

#[derive(sqlx::FromRow, sqlx::Type, Debug, Copy, Clone)]
#[sqlx(transparent)]
pub struct IngredientId(pub i32);

#[derive(sqlx::FromRow, Debug)]
pub struct Ingredient {
    pub ingredient_id: IngredientId,
    pub ingredient_name: String,
    pub excludes_diets: Option<Vec<Diet>>,
}

impl Ingredient {
    #[allow(missing_docs, reason = "`new` is self-explanatory")]
    pub fn new(ingredient_name: String, excludes_diets: Option<Vec<Diet>>) -> Self {
        Ingredient {
            ingredient_id: IngredientId(-1),
            ingredient_name,
            excludes_diets,
        }
    }
}

pub async fn upload_ingredients(
    conn: &Pool<Postgres>,
    ingredients: Vec<Ingredient>,
) -> Result<Vec<IngredientId>, sqlx::error::Error> {
    let mut query_builder =
        QueryBuilder::new("INSERT INTO ingredient (ingredient_name, excludes_diets)");

    query_builder.push_values(ingredients, |mut b, new_ingredient: Ingredient| {
        b.push_bind(new_ingredient.ingredient_name)
            .push_bind(new_ingredient.excludes_diets);
    });

    query_builder.push("ON CONFLICT DO NOTHING RETURNING ingredient_id");

    let query = query_builder.build_query_as();

    let ingredient_ids: Vec<IngredientId> = query.fetch_all(conn).await?;

    Ok(ingredient_ids)
    // Ok(ingredients.iter().map(|ing| ing.ingredient_id).collect())
}
