use super::space::FeatureSet;

/// A Recipe produces a specific kind of game object into a Space.
pub trait Recipe<F: super::space::FeatureSet>: 'static {
    fn spawn_vars(&self, key: super::space::MasterKey, feat: &mut F);
    fn spawn_consts(_key: super::space::MasterKey, _feat: &mut F) {}
}

/// Objects that can read recipes from RON and apply them to a Space.
/// Implementations are auto-generated with the `ecs::recipes!` macro.
pub trait DeserializeRecipes<F: FeatureSet> {
    fn deserialize_into_space<'a, 'de, D>(
        deserializer: D,
        space: &'a mut super::Space<F>,
    ) -> Result<(), D::Error>
    where
        D: serde::Deserializer<'de>;
}

pub use crate::recipes;
#[macro_export]
macro_rules! recipes {
    ($feat_type:ident, $( $recipe_type:ident, )+) => {
        #[derive(serde::Deserialize, serde::Serialize)]
        pub enum Recipes {
            $($recipe_type($recipe_type),)*
        }

        impl moleengine::core::recipe::DeserializeRecipes<$feat_type> for Recipes {
            fn deserialize_into_space<'a, 'de, D>(
                deserializer: D,
                space: &'a mut moleengine::core::Space<$feat_type>,
            ) -> Result<(), D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct RecipeVisitor<'a>(&'a mut moleengine::core::Space<$feat_type>);

                impl<'a, 'de> serde::de::Visitor<'de> for RecipeVisitor<'a> {
                    type Value = ();

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("A list of ObjectRecipes")
                    }

                    fn visit_seq<S>(self, mut seq: S) -> Result<(), S::Error>
                    where
                        S: serde::de::SeqAccess<'de>,
                    {
                        while let Some(value) = seq.next_element()? {
                            match value {
                                $(Recipes::$recipe_type(r) => {
                                    if let None = self.0.spawn(r) {
                                        use serde::de::Error;
                                        return Err(S::Error::custom("RON data did not fit in the space"));
                                    }
                                },)*
                            }
                        }

                        Ok(())
                    }
                }

                deserializer.deserialize_seq(RecipeVisitor(space))
            }
        }
    }
}
