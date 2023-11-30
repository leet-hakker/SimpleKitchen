use sqlx::{postgres::PgHasArrayType, postgres::PgTypeInfo};

// For automatic classification, diets should be in their most restrictive format. That way, any
// recipe that is classified that way is guaranteed to be in compliance with the diet (except for
// factors out of the recipe's scope, such as slaughtering rituals or farming practices).
//
/// Custom type used within the database. Represents many different types of diets
/// that people conform to.
///
/// It is in no way expected that all foods that are listed in this documentation will
/// all become a part of the database (for example, mice, lizard and snake), but are
/// included here for completeness.
///
/// Implements PgHasArrayType, Encode, Decode, and Type
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "diet", rename_all = "snake_case")]
pub enum Diet {
    /// Abstaining from eating any part of an animal.
    Vegetarian,
    /// Abstaining from eating any part of an animal, or anything derived from an animal.
    Vegan,
    /// Abstaining from eating any part of an animal, or anything derived from an animal,
    /// except for milk and eggs.
    OvoLactoVegetarian,
    /// Abstaining form eating any part of an animal, or anything derived from an animal,
    /// except for eggs.
    OvoVegetarian,
    /// Abstaining form eating any part of an animal, or anything derived from an animal,
    /// except for milk.
    LactoVegetarian,
    /// Jains follow a lacto vegetarian diet, as well as excluding all root and underground
    /// vegetables, and all fungi.
    JainVegetarian,
    // It will likely be incredibly difficult or impossible to automatically categorise Sattvic
    // recipes, based on the complexities of it. Any implementation may be best left to somebody
    // who currently does, or has followed a Sattvic diet in the past.
    /// Sattvic diets are highly involved. They are typically vegetarian and restrict certain
    /// food pairings and certain foods altogether.
    SattvicVegetarian,
    // Like a Sattvic diet, an implementation of automatic classification would be best left to
    // somebody who either strictly observes Muslim dietary laws, or is knowledgable of them.
    /// Following Muslim dietary laws. Excludes alcohol, blood (would exclude something like black
    /// pudding), pork, horse, mule, donkey, fanged animals, birds of prey, lizard, snake,
    /// scorpion, mice
    Halal,
    /// Following Rastafarian dietary laws, which involve following a strict vegetarian diet,
    /// as well as abstaining from alcohol.
    Ital,
    /// Following Kashrut. Excludes any mammals that do not both have cloven hoves and ruminate,
    /// any birds of prey, any fish without fins or scales, all invertebrates (for all intents and
    /// purposes) and all reptiles and amphibians, blood.
    /// The mixture of meat and milk is also forbidden.
    Kosher,
    /// Following the dietary reccomendations of the Seventh Day Adventist Church. Involves
    /// following a vegetarian diet, as well as adhering to Kosher laws and abstaining from
    /// alcohol.
    SeventhDay,
    /// Vegetarian diet that allows the consumption of seafood.
    Pescatarian,
    /// Vegetarian diet that allows the consumption of poultry.
    Pollotarian,
    /// Diet that excludes gluten. Excludes wheat, barley, rye and oats
    GlutenFree,
}

impl PgHasArrayType for Diet {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        PgTypeInfo::with_name("diet[]")
    }
}

// impl<'r> Encode<'r, Postgres> for Diet {
//     fn encode_by_ref(
//         &self,
//         buf: &mut <Postgres as sqlx::database::HasArguments<'r>>::ArgumentBuffer,
//     ) -> IsNull {
//         buf.extend(match self {
//             Diet::Vegetarian => "vegetarian".as_bytes(),
//             Diet::Vegan => "vegan".as_bytes(),
//             Diet::OvoLactoVegetarian => "ovo-lacto vegetarian".as_bytes(),
//             Diet::OvoVegetarian => "ovo vegetarian".as_bytes(),
//             Diet::LactoVegetarian => "lacto vegetarian".as_bytes(),
//             Diet::JainVegetarian => "jain vegetarian".as_bytes(),
//             Diet::SattvicVegetarian => "sattvic vegetarian".as_bytes(),
//             Diet::Halal => "halal".as_bytes(),
//             Diet::Ital => "ital".as_bytes(),
//             Diet::Kosher => "kosher".as_bytes(),
//             Diet::SeventhDay => "seventh-day adventists".as_bytes(),
//             Diet::Pescatarian => "pescatarian".as_bytes(),
//             Diet::Pollotarian => "pollotarian".as_bytes(),
//             Diet::GlutenFree => "gluten-free".as_bytes(),
//         });
//
//         IsNull::No
//     }
// }
//
// impl<'r> Decode<'r, Postgres> for Diet {
//     fn decode(
//         value: <Postgres as sqlx::database::HasValueRef<'r>>::ValueRef,
//     ) -> Result<Self, sqlx::error::BoxDynError> {
//         let diet_name: &str = <&str as Decode<Postgres>>::decode(value)?;
//         let diet = match diet_name {
//             "vegetarian" => Diet::Vegetarian,
//             "vegan" => Diet::Vegan,
//             "ovo-lacto vegetarian" => Diet::OvoLactoVegetarian,
//             "ovo vegetarian" => Diet::OvoVegetarian,
//             "lacto vegetarian" => Diet::LactoVegetarian,
//             "jain vegetarian" => Diet::JainVegetarian,
//             "sattvic vegetarian" => Diet::SattvicVegetarian,
//             "halal" => Diet::Halal,
//             "i-tal" => Diet::Ital,
//             "kosher" => Diet::Kosher,
//             "seventh-day adventists" => Diet::SeventhDay,
//             "pescatarian" => Diet::Pescatarian,
//             "pollotarian" => Diet::Pollotarian,
//             "gluten-free" => Diet::GlutenFree,
//             _ => unreachable!(),
//         };
//
//         Ok(diet)
//     }
// }
//
// impl sqlx::Type<Postgres> for Diet {
//     fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
//         PgTypeInfo::with_name("diet")
//     }
// }

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "meal_type", rename_all = "snake_case")]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Desert,
}

// impl<'r> Encode<'r, Postgres> for MealType {
//     fn encode_by_ref(
//         &self,
//         buf: &mut <Postgres as sqlx::database::HasArguments<'r>>::ArgumentBuffer,
//     ) -> sqlx::encode::IsNull {
//         buf.extend(match self {
//             MealType::Breakfast => "breakfast".as_bytes(),
//             MealType::Lunch => "lunch".as_bytes(),
//             MealType::Dinner => "dinner".as_bytes(),
//             MealType::Snack => "snack".as_bytes(),
//             MealType::Desert => "desert".as_bytes(),
//         });
//
//         IsNull::No
//     }
// }
//
// impl<'r, DB: Database> Decode<'r, DB> for MealType
// where
//     &'r str: Decode<'r, DB>,
// {
//     fn decode(
//         value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
//     ) -> Result<Self, sqlx::error::BoxDynError> {
//         let meal_name: &str = <&str as Decode<DB>>::decode(value)?;
//         let meal_type: MealType = match meal_name {
//             "breakfast" => MealType::Breakfast,
//             "lunch" => MealType::Lunch,
//             "dinner" => MealType::Dinner,
//             "snack" => MealType::Snack,
//             "desert" => MealType::Desert,
//             _ => unreachable!(),
//         };
//
//         Ok(meal_type)
//     }
// }

// impl sqlx::Type<Postgres> for MealType {
//     fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
//         PgTypeInfo::with_name("meal_type")
//     }
// }
