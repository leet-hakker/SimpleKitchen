DROP TYPE IF EXISTS diet;
CREATE TYPE diet AS ENUM (
  'vegetarian',
  'vegan',
  'ovo-lacto vegetarian',
  'ovo vegetarian',
  'lacto vegetarian',
  'jain vegetarian',
  'sattvic vegetarian',
  'halal',
  'ital',
  'kosher',
  'seventh-day adventist',
  'pescatarian',
  'pollotarianism',
  'gluten-free'
);

DROP TYPE IF EXISTS meal_type;
CREATE TYPE meal_type AS ENUM ('breakfast', 'lunch', 'dinner', 'snack', 'dessert');

DROP TABLE IF EXISTS ingredient;
CREATE TABLE ingredient (
  ingredient_id INT GENERATED ALWAYS AS IDENTITY,
  ingredient_name VARCHAR(50) NOT NULL,
  excludes_diets diet[],
  PRIMARY KEY(ingredient_id)
);

DROP TABLE IF EXISTS users;
CREATE TABLE users (
  user_id INT GENERATED ALWAYS AS IDENTITY,
  user_name VARCHAR(25) NOT NULL,
  saved_recipes INT[],
  avoidances INT[],
  PRIMARY KEY(user_id)
  -- FOREIGN KEY(EACH ELEMENT OF avoidances)
  --   REFERENCES ingredient(ingredient_id)
  --   ON DELETE CASCADE
);



DROP TABLE IF EXISTS recipe;
CREATE TABLE recipe (
  recipe_id INT GENERATED ALWAYS AS IDENTITY,
  recipe_name VARCHAR(50) NOT NULL,
  recipe_author INT NOT NULL,
  instructions TEXT[],
  meal_type meal_type,
  complies_with_diets diet[],
  PRIMARY KEY (recipe_id),
  FOREIGN KEY (recipe_author)
    REFERENCES users(user_id)
    ON DELETE CASCADE
);


DROP TABLE IF EXISTS recipe_ingredient;
CREATE TABLE recipe_ingredient (
  fk_recipe INT NOT NULL,
  fk_ingredient INT NOT NULL,
  quantity REAL NOT NULL,
  unit_of_measurement VARCHAR(50),
  FOREIGN KEY(fk_recipe)
    REFERENCES recipe(recipe_id)
    ON DELETE CASCADE,
  FOREIGN KEY(fk_ingredient)
    REFERENCES ingredient(ingredient_id)
    ON DELETE CASCADE
);

DROP TABLE IF EXISTS user_inventory;
CREATE TABLE user_inventory (
  fk_user INT NOT NULL,
  fk_ingredient INT NOT NULL,
  quantity REAL NOT NULL,
  unit_of_measurement VARCHAR(50),
  FOREIGN KEY(fk_user)
    REFERENCES users(user_id)
    ON DELETE CASCADE,
  FOREIGN KEY(fk_ingredient)
    REFERENCES ingredient(ingredient_id)
    ON DELETE CASCADE
);
