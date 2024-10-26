use anyhow;

mod menu {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Unit {
        Cans(f64),
        Ounces(f64),
        Cups(f64),
        Teaspoons(f64),
        Tablespoons(f64),
        Pounds(f64),
        Other(String, f64),
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Ingredient {
        name: String,
        amount: Unit,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Meal {
        name: String,
        ingredients: Vec<Ingredient>,
    }

    #[cfg(test)]
    mod unit {
        use crate::menu::{Ingredient, Meal, Unit};

        #[test]
        fn json_to_rust() {
            let meals = r#"
                [
                    {
                        name: "Curried Chickpeas",
                        ingredients: [
                            {
                                name: "Chickpeas",
                                amount: {
                                    type: "cans",
                                    value: 2
                                }
                            },
                            {
                                name: "Tomato Sauce",
                                amount: {
                                    type: "ounces",
                                    value: 16
                                }
                            },
                            {
                                name: "Spinach",
                                amount: {
                                    type: "ounces",
                                    value: 8
                                }
                            }
                        ]
                    },
                    {}
                ]
            "#;

            let meal: Meal = serde_json::from_str(meals).unwrap();

            assert_eq!(
                meal,
                Meal {
                    name: "Curried Chickpeas".to_string(),
                    ingredients: vec![
                        Ingredient {
                            name: "Chickpeas".to_string(),
                            amount: Unit::Cans(2.0),
                        },
                        Ingredient {
                            name: "Tomato Sauce".to_string(),
                            amount: Unit::Ounces(16.0),
                        },
                        Ingredient {
                            name: "Spinach".to_string(),
                            amount: Unit::Ounces(8.0),
                        },
                    ],
                }
            );
        }
    }
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
