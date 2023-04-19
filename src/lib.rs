//! The Sifis extension for Web of Things Description.
//!
//! This crate is mainly focused on specifying hazards for Thing Descriptions for the crate
//! [wot-td].
//!
//! The _context_ of the JSON pointer depends on the type of affordance the hazard is applied to.
//! See [`hazard::Condition::pointer`] for more information.
//!
//! When the [`Sifis`] extension is used, it is necessary to add a context to the [`Thing`] with
//! the prefix `sho` that points to `https://purl.org/sifis/hazards`.
//!
//! [wot-td]: wot_td
//! [`Thing`]: wot_td::Thing
//!
//! # Example
//!
//! ```
//! use serde_json::json;
//! use sifis_td::{
//!     condition::{Condition, self},
//!     hazard::{self, Hazard},
//!     risk::{EXPLOSION, FIRE_HAZARD},
//!     Sifis,
//! };
//! use wot_td::{
//!     builder::{
//!         IntegerDataSchemaBuilderLike, NumberDataSchemaBuilderLike, ObjectDataSchemaBuilderLike,
//!         SpecializableDataSchema,
//!     },
//!     Thing,
//! };
//!
//! let thing = Thing::builder("My Thing")
//!     .ext(Sifis {
//!         risks: vec![FIRE_HAZARD, EXPLOSION],
//!         hazards: vec![
//!             Hazard {
//!                 risk: hazard::Risk {
//!                     id: hazard::Id::FireHazard,
//!                     level: 3,
//!                 },
//!                 conditions: vec![vec![
//!                     hazard::Condition {
//!                         pointer: "/properties/prop/inner1".try_into().unwrap(),
//!                         condition: Condition::Expr(condition::Expr {
//!                             value: 0.5.try_into().unwrap(),
//!                             op: condition::Operation::Lt,
//!                         }),
//!                     },
//!                     hazard::Condition {
//!                         pointer: "/properties/prop/inner2".try_into().unwrap(),
//!                         condition: Condition::Expr(condition::Expr {
//!                             value: 6.into(),
//!                             op: condition::Operation::Lt,
//!                         }),
//!                     },
//!                 ]],
//!             },
//!             Hazard {
//!                 risk: hazard::Risk {
//!                     id: hazard::Id::FireHazard,
//!                     level: 7,
//!                 },
//!                 conditions: vec![
//!                     vec![hazard::Condition {
//!                         pointer: "/properties/prop/inner1".try_into().unwrap(),
//!                         condition: Condition::Expr(condition::Expr {
//!                             value: 0.5.try_into().unwrap(),
//!                             op: condition::Operation::Ge,
//!                         }),
//!                     }],
//!                     vec![hazard::Condition {
//!                         pointer: "/properties/prop/inner2".try_into().unwrap(),
//!                         condition: Condition::Expr(condition::Expr {
//!                             value: 6.into(),
//!                             op: condition::Operation::Ge,
//!                         }),
//!                     }],
//!                 ],
//!             },
//!             Hazard {
//!                 risk: hazard::Risk {
//!                     id: hazard::Id::Explosion,
//!                     level: 1,
//!                 },
//!                 conditions: vec![vec![hazard::Condition {
//!                     pointer: "/properties/prop/inner1".try_into().unwrap(),
//!                     condition: Condition::Expr(condition::Expr {
//!                         value: 0.9.try_into().unwrap(),
//!                         op: condition::Operation::Ge,
//!                     }),
//!                 }]],
//!             },
//!         ],
//!     })
//!     .finish_extend()
//!     .context_map(|b| b.context("sho", "https://purl.org/sifis/hazards"))
//!     .property("prop", |b| {
//!         b.ext(())
//!             .ext_interaction(())
//!             .ext_data_schema(())
//!             .finish_extend_data_schema()
//!             .object()
//!             .property("inner1", false, |b| {
//!                 b.ext(()).finish_extend().number().minimum(0.).maximum(1.)
//!             })
//!             .property("inner2", false, |b| {
//!                 b.ext(()).finish_extend().integer().minimum(1).maximum(10)
//!             })
//!     })
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(
//!     serde_json::to_value(thing).unwrap(),
//!     json!({
//!         "@context": [
//!             "https://www.w3.org/2022/wot/td/v1.1",
//!             {
//!                 "sho": "https://purl.org/sifis/hazards",
//!             },
//!         ],
//!         "title":"My Thing",
//!         "properties":{
//!             "prop": {
//!                 "forms": [],
//!                 "readOnly": false,
//!                 "writeOnly": false,
//!                 "type": "object",
//!                 "properties": {
//!                     "inner2": {
//!                         "readOnly": false,
//!                         "writeOnly": false,
//!                         "type": "integer",
//!                         "maximum": 10,
//!                         "minimum": 1,
//!                     },
//!                     "inner1": {
//!                         "readOnly": false,
//!                         "writeOnly": false,
//!                         "type": "number",
//!                         "maximum": 1.0,
//!                         "minimum": 0.0,
//!                     },
//!                 },
//!             },
//!         },
//!         "security": [],
//!         "securityDefinitions": {},
//!         "sho:risks": [
//!             {
//!                 "@id": "sho:FireHazard",
//!                 "sho:description": "The execution may cause fire",
//!                 "sho:name": "Fire hazard",
//!                 "sho:category":"sho:Safety",
//!             },
//!             {
//!                 "@id": "sho:Explosion",
//!                 "sho:description": "The execution may cause an explosion",
//!                 "sho:name": "Explosion",
//!                 "sho:category": "sho:Safety",
//!             },
//!         ],
//!         "sho:hazards": [
//!             {
//!                 "sho:risk": {
//!                     "@id": "sho:FireHazard",
//!                     "sho:level": 3,
//!                 },
//!                 "sho:conditions": [[
//!                     {
//!                         "sho:pointer": "/properties/prop/inner1",
//!                         "sho:condition": {
//!                             "sho:value": 0.5,
//!                             "sho:op": "lt",
//!                         },
//!                     },
//!                     {
//!                         "sho:pointer": "/properties/prop/inner2",
//!                         "sho:condition": {
//!                             "sho:value": 6,
//!                             "sho:op": "lt",
//!                         },
//!                     },
//!                 ]],
//!             },
//!             {
//!                 "sho:risk": {
//!                     "@id": "sho:FireHazard",
//!                     "sho:level": 7,
//!                 },
//!                 "sho:conditions": [
//!                     [{
//!                         "sho:pointer": "/properties/prop/inner1",
//!                         "sho:condition": {
//!                             "sho:value": 0.5,
//!                             "sho:op": "ge",
//!                         },
//!                     }],
//!                     [{
//!                         "sho:pointer": "/properties/prop/inner2",
//!                         "sho:condition": {
//!                             "sho:value": 6,
//!                             "sho:op": "ge",
//!                         },
//!                     }],
//!                 ],
//!             },
//!             {
//!                 "sho:risk": {
//!                     "@id": "sho:Explosion",
//!                     "sho:level": 1,
//!                 },
//!                 "sho:conditions": [[{
//!                   "sho:pointer": "/properties/prop/inner1",
//!                   "sho:condition": {
//!                       "sho:value": 0.9,
//!                       "sho:op": "ge",
//!                   },
//!                 }]],
//!             },
//!         ],
//!     }),
//! )
//! ```

#![warn(clippy::pedantic)]

pub mod condition;
pub mod hazard;
pub mod risk;

use hazard::Hazard;
use serde::{Deserialize, Serialize};
use wot_td::extend::ExtendableThing;

/// The Sifis extension for a Thing Description.
///
/// When this is used to extend a [`Thing`], then it is necessary to add a context with the prefix
/// `sho` to points to `https://purl.org/sifis/hazards`.
///
/// [`Thing`]: wot_td::Thing
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Sifis {
    #[serde(skip_serializing_if = "Vec::is_empty", rename = "sho:risks", default)]
    pub risks: Vec<risk::Detail>,

    #[serde(skip_serializing_if = "Vec::is_empty", rename = "sho:hazards", default)]
    pub hazards: Vec<Hazard>,
}

impl ExtendableThing for Sifis {
    type InteractionAffordance = ();
    type PropertyAffordance = ();
    type ActionAffordance = ();
    type EventAffordance = ();
    type Form = ();
    type ExpectedResponse = ();
    type DataSchema = ();
    type ObjectSchema = ();
    type ArraySchema = ();
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use jsonptr::Resolve;
    use serde_json::{json, Number};

    use crate::condition::Condition;

    use super::*;

    fn evaluate_hazards<T>(sifis: &Sifis, thing_data: &T) -> Vec<hazard::Risk>
    where
        T: Resolve,
    {
        use serde_json::Value;

        sifis.hazards.iter().fold(Vec::new(), |mut risks, hazard| {
            let condition_satisfied = hazard.conditions.iter().any(|conditions| {
                conditions.iter().all(|condition| {
                    use jsonptr::Error;

                    let ptr = &condition.pointer.0;
                    assert!(ptr.is_valid());

                    match thing_data.resolve(ptr).map(|v| (v, &condition.condition)) {
                        Ok((Value::String(s1), Condition::Value(condition::Value::String(s2)))) => {
                            s1 == s2
                        }

                        Ok((
                            Value::String(s1),
                            Condition::Expr(condition::Expr {
                                value: condition::Value::String(s2),
                                op,
                            }),
                        )) => apply_op(s1.as_str(), s2.as_ref(), *op),

                        Ok((Value::String(_), _)) => {
                            panic!("invalid comparison, {ptr} is unexpectedly a string")
                        }

                        Ok((Value::Number(n1), Condition::Value(condition::Value::Number(n2)))) => {
                            ComparableNumber(n1) == ComparableNumber(n2)
                        }

                        Ok((
                            Value::Number(n1),
                            Condition::Expr(condition::Expr {
                                value: condition::Value::Number(n2),
                                op,
                            }),
                        )) => apply_op(&ComparableNumber(n1), &ComparableNumber(n2), *op),

                        Ok((Value::Number(_), _)) => {
                            panic!("invalid comparison, {ptr} is unexpectedly a number")
                        }

                        Ok((Value::Bool(b1), Condition::Value(condition::Value::Bool(b2)))) => {
                            b1 == b2
                        }

                        Ok((
                            Value::Bool(b1),
                            Condition::Expr(condition::Expr {
                                value: condition::Value::Bool(b2),
                                op,
                            }),
                        )) => apply_op(b1, b2, *op),

                        Ok((Value::Bool(_), _)) => {
                            panic!("invalid comparison, {ptr} is unexpectedly a boolean")
                        }

                        Ok((Value::Null | Value::Array(_) | Value::Object(_), _)) => {
                            panic!("invalid element, expected string, number or boolean")
                        }

                        Err(Error::NotFound(_) | Error::Unresolvable(_) | Error::Index(_)) => false,

                        Err(Error::MalformedPointer(_)) => unreachable!(),
                    }
                })
            });

            if condition_satisfied {
                match risks.iter_mut().find(|risk| risk.id == hazard.risk.id) {
                    Some(risk) => risk.level = risk.level.max(hazard.risk.level),
                    None => risks.push(hazard.risk),
                }
            }

            risks
        })
    }

    fn apply_op<T>(a: &T, b: &T, op: condition::Operation) -> bool
    where
        T: PartialEq + PartialOrd + ?Sized,
    {
        use condition::Operation;

        match op {
            Operation::Lt => a < b,
            Operation::Le => a <= b,
            Operation::Ne => a != b,
            Operation::Gt => a > b,
            Operation::Ge => a >= b,
        }
    }

    #[derive(Debug, Clone)]
    struct ComparableNumber<'a>(&'a Number);

    impl PartialEq for ComparableNumber<'_> {
        fn eq(&self, other: &Self) -> bool {
            if let Some((a, b)) = self.0.as_u64().zip(other.0.as_u64()) {
                return a == b;
            }

            if let Some((a, b)) = self.0.as_i64().zip(other.0.as_i64()) {
                return a == b;
            }

            let a = self.0.as_f64().unwrap();
            let b = other.0.as_f64().unwrap();
            a == b
        }
    }

    impl Eq for ComparableNumber<'_> {}

    impl Ord for ComparableNumber<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            if let Some((a, b)) = self.0.as_u64().zip(other.0.as_u64()) {
                return a.cmp(&b);
            }

            if let Some((a, b)) = self.0.as_i64().zip(other.0.as_i64()) {
                return a.cmp(&b);
            }

            let a = self.0.as_f64().unwrap();
            let b = other.0.as_f64().unwrap();
            a.partial_cmp(&b).unwrap()
        }
    }

    impl PartialOrd for ComparableNumber<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    #[test]
    fn serialize_empty() {
        let sifis = Sifis::default();
        assert_eq!(serde_json::to_value(&sifis).unwrap(), json!({}));
    }

    #[test]
    fn deserialize_empty() {
        let sifis: Sifis = serde_json::from_value(json!({})).unwrap();
        assert_eq!(sifis, Sifis::default());
    }

    #[test]
    fn serialize_simple() {
        let sifis = Sifis {
            risks: vec![risk::FIRE_HAZARD],
            hazards: vec![Hazard {
                risk: hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 2,
                },
                conditions: vec![vec![hazard::Condition {
                    pointer: "/properties/test".try_into().unwrap(),
                    condition: Condition::Value(3.into()),
                }]],
            }],
        };

        assert_eq!(
            serde_json::to_value(sifis).unwrap(),
            json!({
                "sho:risks": [risk::FIRE_HAZARD],
                "sho:hazards": [{
                    "sho:risk": {
                        "@id": "sho:FireHazard",
                        "sho:level": 2,
                    },
                    "sho:conditions": [[{
                        "sho:pointer": "/properties/test",
                        "sho:condition": 3,
                    }]],
                }]
            })
        );
    }

    #[test]
    fn deserialize_simple() {
        let sifis = json!({
            "sho:risks": [risk::FIRE_HAZARD],
            "sho:hazards": [{
                "sho:risk": {
                    "@id": "sho:FireHazard",
                    "sho:level": 2,
                },
                "sho:conditions": [[{
                    "sho:pointer": "/properties/test",
                    "sho:condition": 3,
                }]],
            }]
        });

        assert_eq!(
            serde_json::from_value::<Sifis>(sifis).unwrap(),
            Sifis {
                risks: vec![risk::FIRE_HAZARD],
                hazards: vec![Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 2,
                    },
                    conditions: vec![vec![hazard::Condition {
                        pointer: "/properties/test".try_into().unwrap(),
                        condition: Condition::Value(3.into()),
                    }]],
                }],
            }
        );
    }

    // The reason behind this test is we don't actually have any implementation to evaluate
    // the resulting hazards & risk levels given some data. What we are trying to test is, given a
    // pretty simple testing evaluator, the expressiveness of the `Sifis` structure.
    #[test]
    #[allow(clippy::too_many_lines)]
    fn resolve_hazards() {
        #[derive(Debug, Default, Serialize)]
        struct Data {
            properties: Props,
        }

        #[derive(Debug, Default, Serialize)]
        struct Props {
            prop1: Prop1,
            prop2: u32,
            prop3: bool,
        }

        #[derive(Debug, Default, Serialize)]
        struct Prop1 {
            value1: &'static str,
            value2: u32,
        }

        let sifis = Sifis {
            risks: Vec::new(),
            hazards: vec![
                Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 1,
                    },
                    conditions: vec![
                        vec![
                            hazard::Condition {
                                pointer: "/properties/prop2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 1.into(),
                                    op: condition::Operation::Ge,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop3".try_into().unwrap(),
                                condition: Condition::Value(true.into()),
                            },
                        ],
                        vec![hazard::Condition {
                            pointer: "/properties/prop1/value1".try_into().unwrap(),
                            condition: Condition::Value("fire".into()),
                        }],
                    ],
                },
                Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 4,
                    },
                    conditions: vec![
                        vec![hazard::Condition {
                            pointer: "/properties/prop1/value1".try_into().unwrap(),
                            condition: Condition::Value("big fire".into()),
                        }],
                        vec![
                            hazard::Condition {
                                pointer: "/properties/prop2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 1.into(),
                                    op: condition::Operation::Gt,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop3".try_into().unwrap(),
                                condition: Condition::Value(true.into()),
                            },
                        ],
                    ],
                },
                Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 2,
                    },
                    conditions: vec![vec![hazard::Condition {
                        pointer: "/properties/prop1/value1".try_into().unwrap(),
                        condition: Condition::Value("medium fire".into()),
                    }]],
                },
                Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::Explosion,
                        level: 1,
                    },
                    conditions: vec![
                        vec![
                            hazard::Condition {
                                pointer: "/properties/prop2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 8.into(),
                                    op: condition::Operation::Ge,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop3".try_into().unwrap(),
                                condition: Condition::Value(true.into()),
                            },
                        ],
                        vec![hazard::Condition {
                            pointer: "/properties/prop1/value1".try_into().unwrap(),
                            condition: Condition::Value("boom".into()),
                        }],
                        vec![
                            hazard::Condition {
                                pointer: "/properties/prop1/value2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 10.into(),
                                    op: condition::Operation::Ge,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop1/value2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 15.into(),
                                    op: condition::Operation::Le,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop3".try_into().unwrap(),
                                condition: Condition::Value(true.into()),
                            },
                        ],
                    ],
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data::default()).unwrap(),
            ),
            [],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1::default(),
                        prop2: 0,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1::default(),
                        prop2: 1,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 1,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value1: "fire",
                            value2: 0,
                        },
                        prop2: 0,
                        prop3: false,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 1,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1::default(),
                        prop2: 2,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 4,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value1: "medium fire",
                            value2: 0,
                        },
                        prop2: 0,
                        prop3: false,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 2,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value1: "medium fire",
                            value2: 0,
                        },
                        prop2: 2,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 4,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value1: "boom",
                            value2: 0,
                        },
                        prop2: 2,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 4,
                },
                hazard::Risk {
                    id: hazard::Id::Explosion,
                    level: 1,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value2: 12,
                            ..Default::default()
                        },
                        prop2: 2,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::FireHazard,
                    level: 4,
                },
                hazard::Risk {
                    id: hazard::Id::Explosion,
                    level: 1,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value2: 15,
                            ..Default::default()
                        },
                        prop2: 0,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [
                hazard::Risk {
                    id: hazard::Id::Explosion,
                    level: 1,
                },
            ],
        };

        assert_eq! {
            evaluate_hazards(
                &sifis,
                &serde_json::to_value(Data {
                    properties: Props {
                        prop1: Prop1 {
                            value2: 16,
                            ..Default::default()
                        },
                        prop2: 0,
                        prop3: true,
                    },
                })
                .unwrap(),
            ),
            [],
        };
    }
}
