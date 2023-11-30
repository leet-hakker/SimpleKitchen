DROP INDEX IF EXISTS recipe_ingredient_index;
CREATE INDEX IF NOT EXISTS recipe_ingredient_index ON recipe_ingredient(fk_recipe, fk_ingredient, quantity, unit_of_measurement);

DROP INDEX IF EXISTS user_inventory_index;
CREATE INDEX user_inventory_index ON user_inventory(fk_user, fk_ingredient, quantity, unit_of_measurement);
