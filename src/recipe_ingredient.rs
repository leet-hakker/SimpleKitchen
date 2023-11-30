use crate::ingredient::IngredientId;
use crate::recipe::RecipeId;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::QueryBuilder;

/// An ingredient to a recipe
#[derive(sqlx::FromRow, Debug)]
pub struct RecipeIngredient {
    /// Foreign key to the `ingredient` table
    pub fk_ingredient: IngredientId,
    /// Foreign key to the `recipe` table,
    /// linking this recipe ingredient to the
    /// recipe it is used for.
    pub fk_recipe: RecipeId,
    /// How many `unit_of_measurement`
    pub quantity: f32,
    /// The unit that the `RecipeIngredient` is measured in
    pub unit_of_measurement: Option<String>,
}

impl RecipeIngredient {
    #[allow(missing_docs, reason = "`new` is self-explanatory")]
    pub fn new(
        fk_ingredient: IngredientId,
        fk_recipe: RecipeId,
        quantity: f32,
        unit_of_measurement: Option<String>,
    ) -> Self {
        Self {
            fk_ingredient,
            fk_recipe,
            quantity,
            unit_of_measurement,
        }
    }
}

/// Inserts `recipe_ingredients` into the `recipe_ingredient` table in a batch
pub async fn upload_recipe_ingredients(
    conn: &Pool<Postgres>,
    recipe_ingredients: Vec<RecipeIngredient>,
) -> Result<(), sqlx::error::Error> {
    let mut query_builder = QueryBuilder::new(
        "INSERT INTO recipe_ingredient (fk_ingredient, fk_recipe, quantity, unit_of_measurement)",
    );

    query_builder.push_values(
        recipe_ingredients,
        |mut b, new_recipe_ingredient: RecipeIngredient| {
            b.push_bind(new_recipe_ingredient.fk_ingredient)
                .push_bind(new_recipe_ingredient.fk_recipe)
                .push_bind(new_recipe_ingredient.quantity)
                .push_bind(new_recipe_ingredient.unit_of_measurement);
        },
    );

    let query = query_builder.build();

    query.execute(conn).await?;

    Ok(())
}
