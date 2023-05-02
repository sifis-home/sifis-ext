//! Uitilities for building a [`Sifis`] extension.
//!
//! Building a [`Sifis`] extension is pretty straightforward, but it can be tedious nonetheless.
//! To make things more _developer-friendly_, the crate exposes a [`Builder`] and a set of related
//! utilities.
//!
//! Here a small example:
//! ```
//! use sifis_td::Sifis;
//!
//! let sifis = Sifis::builder()
//!     .fire_hazard(2, |cond| {
//!         cond.when("/properties/on")
//!             .eq(true)
//!             .and("/properties/level")
//!             .ge(3)
//!             .or(|cond| cond.when("/properties/burn_everything").eq(true))
//!     })
//!     .fire_hazard(5, |cond| {
//!         cond.when("/properties/on")
//!             .eq(true)
//!             .and("/properties/level")
//!             .ge(7)
//!     })
//!     .build();
//! ```
//!
//! This snippets:
//!
//! 1. automatically adds a [`FIRE_HAZARD`] to the internal buffer (just once);
//! 2. creates a new [`Hazard`] with `level` 2, the [`FireHazard`] id and a set of two conditions,
//!    the former composed of two and the latter from one;
//! 2. creates an analogous [`Hazard`] with `level` 5 with just one set of two conditions.
//!
//! Keep in mind that the builder will not allow to create an invalid set of conditions (but it
//! cannot infer wrong logic), which will fail at compile-time. See [`Condition`] for more
//! information.
//!
//! [`FIRE_HAZARD`]: risk::FIRE_HAZARD
//! [`FireHazard`]: hazard::Id::FireHazard

use std::{fmt::Display, ops::Not};

use crate::{
    condition,
    hazard::{self, Hazard, JsonPointer},
    risk, Sifis,
};

/// A _builder_ for [`Sifis`].
///
/// Simplifies the creation of `Sifis` extensions exposing a _build pattern_. For instance, the two
/// instances in the following example are equivalent:
///
/// ```
/// use sifis_td::{
///     condition::{self, Condition},
///     hazard::{self, Hazard},
///     risk, Sifis,
/// };
///
/// let sifis_by_builder = Sifis::builder()
///     .fire_hazard(2, |cond| {
///         cond.when("/properties/on")
///             .eq(true)
///             .and("/properties/level")
///             .ge(3)
///             .or(|cond| cond.when("/properties/burn_everything").eq(true))
///     })
///     .build();
///
/// let sifis_manual = Sifis {
///     risks: vec![risk::FIRE_HAZARD],
///     hazards: vec![Hazard {
///         risk: hazard::Risk {
///             id: hazard::Id::FireHazard,
///             level: 2,
///         },
///         conditions: vec![
///             vec![
///                 hazard::Condition {
///                     pointer: "/properties/on".try_into().unwrap(),
///                     condition: Condition::Value(condition::Value::Bool(true)),
///                 },
///                 hazard::Condition {
///                     pointer: "/properties/level".try_into().unwrap(),
///                     condition: Condition::Expr(condition::Expr {
///                         value: condition::Value::Number(3.into()),
///                         op: condition::Operation::Ge,
///                     }),
///                 },
///             ],
///             vec![hazard::Condition {
///                 pointer: "/properties/burn_everything".try_into().unwrap(),
///                 condition: Condition::Value(condition::Value::Bool(true)),
///             }],
///         ],
///     }],
/// };
///
/// assert_eq!(sifis_by_builder, sifis_manual);
/// ```
///
/// See module level documentation for more information.
#[derive(Debug, Default)]
pub struct Builder {
    risks: Vec<risk::Detail>,
    hazards: Vec<Hazard>,
}

macro_rules! impl_builder_hazard {
    ($($fn:ident => $id:ident),+ $(,)?) => {
        $(
            #[doc = concat!("Creates a new hazard with id [`", stringify!($id),  "`].\n\n")]
            #[doc = "See [`hazard`] for more information.\n\n"]
            #[doc = concat!("[`", stringify!($id), "`]: crate::hazard::Id::", stringify!($id), "\n")]
            #[doc = "[`hazard`]: Builder::hazard"]
            #[inline]
            #[must_use]
            pub fn $fn<F, const INIT: bool>(self, level: u8, condition: F) -> Self
            where
                F: FnOnce(Condition<false, false>) -> Condition<INIT, false>,
            {
                self.hazard(hazard::Id::$id, level, condition)
            }
        )+
    };
}

impl Builder {
    /// Creates a hazard given its [`Id`], its level and the condition.
    ///
    /// This adds a new hazard to the builder, automatically including the default associated [risk
    /// details] if necessary.
    ///
    /// The `condition` argument is a _builder function_ related to the condition. See the module
    /// documentation for more information and examples.
    ///
    /// [`Id`]: hazard::Id
    /// [risk details]: risk::Detail
    #[inline]
    #[must_use]
    pub fn hazard<F, const INIT: bool>(mut self, id: hazard::Id, level: u8, condition: F) -> Self
    where
        F: FnOnce(Condition<false, false>) -> Condition<INIT, false>,
    {
        let mut hazard = Hazard {
            risk: hazard::Risk { id, level },
            conditions: Vec::new(),
        };
        condition(Condition(&mut hazard.conditions));
        self.hazards.push(hazard);

        let risk = self.risks.iter().find(|risk| risk.id() == id);
        if risk.is_none() {
            self.risks.push(id.risk());
        }

        self
    }

    /// Builds the [`Sifis`] extension.
    #[inline]
    #[must_use]
    pub fn build(self) -> Sifis {
        let Self { risks, hazards } = self;
        Sifis { risks, hazards }
    }

    impl_builder_hazard!(
        air_poisoning => AirPoisoning,
        asphyxia => Asphyxia,
        audio_video_record_and_store => AudioVideoRecordAndStore,
        audio_video_stream => AudioVideoStream,
        burn => Burn,
        electric_energy_consumption => ElectricEnergyConsumption,
        explosion => Explosion,
        fire_hazard => FireHazard,
        gas_consumption => GasConsumption,
        log_energy_consumption => LogEnergyConsumption,
        log_usage_time => LogUsageTime,
        pay_subscription_fee => PaySubscriptionFee,
        power_outage => PowerOutage,
        power_surge => PowerSurge,
        record_issued_commands => RecordIssuedCommands,
        record_user_preferences => RecordUserPreferences,
        scald => Scald,
        spend_money => SpendMoney,
        spoiled_food => SpoiledFood,
        take_device_screenshots => TakeDeviceScreenshots,
        take_pictures => TakePictures,
        unauthorised_physical_access => UnauthorisedPhysicalAccess,
        water_consumption => WaterConsumption,
        water_flooding => WaterFlooding,
    );
}

/// A builder for [hazard conditions].
///
/// This builder avoids the boilerplate for creating conditions for hazards, improving
/// expressiveness and code readability.
///
/// A `Condition` is internally created from [`Builder::hazard`] or any _simpler_ helper function
/// like [`Builder::fire_hazard`]. The struct is given as function argument and one of _its
/// variants_ (see below) must be given back as return value.
///
/// Take this example:
///
/// ```
/// use sifis_td::Sifis;
///
/// let sifis = Sifis::builder()
///     .fire_hazard(2, |cond| {
///         cond.when("/properties/on")
///             .eq(true)
///             .and("/properties/level")
///             .ge(3)
///             .or(|cond| cond.when("/properties/burn_everything").eq(true))
///     })
///     .build();
/// ```
///
/// In this snippet, `cond` is an _uninitialized_[^uninitialized] `Condition`, which only exposes
/// the [`when`] function that requires a JSON pointer to start expressing a condition. This
/// operation gives back a [_partial condition_] which _forces_[^forces] the user to express the
/// logic operation and the value to compare.
///
/// After `.eq(true)` is called, the `Condition` is in an _initialized_ state, which means that it
/// is possible to express a logic [`and`] or a logic [`or`]. Using `and` is equivalent to using
/// `when`[^and-when], on the other hand `or` is handled in a different way.
///
/// In fact, the `or` function requires a closure that instantiates a different _variant_ of
/// `Condition`, which only allows the use of `when` and `and` functions. The following code does
/// not compile:
///
/// ```compile_fail
/// use sifis_td::Sifis;
///
/// let sifis = Sifis::builder()
///     .fire_hazard(2, |cond| {
///         cond.when("/properties/on")
///             .eq(true)
///             .or(|cond| cond.when("/properties/burn_everything").eq(true).or(|cond| cond))
///     })
///     .build();
/// ```
///
/// This behavior reflects the structure of the [Sifis conditions], which only allows a level of
/// `AND` logic nested into a level of `OR` logic. See its documentation for some context.
///
/// `Condition` behaves in different ways because it has multiple _variants_ expressed as _const_:
///
/// - `INIT` tracks whether the condition is _initialized_, in order to distinguish the
///   availability of `when` and `and` functions;
/// - `NESTED` tracks the depth of the condition, in order to avoid nested `or` expressions.
///
/// ### Panics
///
/// The values given to this builder needs to be expressed in JSON. If an invalid value is given
/// (i.e.: [`std::f64::NAN`]), the builder will panic.
///
/// [^uninitialized]: The term "_uninitialized_" is used to identify a _logic state_, not the
///     underlying memory of the struct. The implementation only uses safe Rust code, therefore it
///     is not possible to have access to uninitialized memory.
///
/// [^forces]: The closure needs a `Condition` as a return type, and calling `when` consumes the
///     original value. This forces the user to use one of the [`PartialCondition`] methods in
///     order to make the code compile.
///
/// [^and-when]: It is the same underlying implementation. They are two separated functions to
///     improve the expressiveness of the code.
///
/// [hazard conditions]: hazard::Condition
/// [`when`]: Condition::when
/// [_partial condition_]: PartialCondition
/// [`and`]: Condition::and
/// [`or`]: Condition::or
/// [Sifis conditions]: hazard::Hazard::conditions
#[derive(Debug)]
pub struct Condition<'a, const INIT: bool, const NESTED: bool>(&'a mut Vec<Vec<hazard::Condition>>);

impl<'a, const INIT: bool, const NESTED: bool> Condition<'a, INIT, NESTED> {
    #[inline]
    #[must_use]
    fn add_condition(self, pointer: String) -> PartialCondition<'a, NESTED> {
        let conditions = self.0;
        let outer_index = conditions.len().checked_sub(1).unwrap_or_else(|| {
            conditions.push(Vec::new());
            0
        });

        PartialCondition {
            conditions,
            outer_index,
            pointer,
        }
    }
}

impl<'a, const NESTED: bool> Condition<'a, false, NESTED> {
    /// Initializes a condition, given a JSON pointer.
    ///
    /// This can only be called on an _uninitialized_ condition. Otherwise, use the
    /// [`Condition::and`] function.
    #[inline]
    #[must_use]
    pub fn when(self, pointer: impl Into<String>) -> PartialCondition<'a, NESTED> {
        self.add_condition(pointer.into())
    }
}

impl<'a, const NESTED: bool> Condition<'a, true, NESTED> {
    /// Combines two or more conditions with a logic `AND`, given a JSON pointer.
    #[inline]
    #[must_use]
    pub fn and(self, pointer: impl Into<String>) -> PartialCondition<'a, NESTED> {
        self.add_condition(pointer.into())
    }
}

impl<'a> Condition<'a, true, false> {
    /// Combines sets of two or more conditions with a logic `OR`, given a JSON pointer.
    #[inline]
    #[must_use]
    pub fn or<F>(self, f: F) -> Condition<'a, true, false>
    where
        F: FnOnce(Condition<'a, false, true>) -> Condition<'a, true, true>,
    {
        let conditions = self.0;
        debug_assert!(conditions.is_empty().not());
        conditions.push(Vec::new());

        let conditions = f(Condition(conditions)).0;
        Condition(conditions)
    }
}

/// A partially specified condition.
///
/// This represents a condition for which the JSON pointer has been specified but the _condition
/// rule_ itself is still missing.
#[derive(Debug)]
pub struct PartialCondition<'a, const NESTED: bool> {
    conditions: &'a mut Vec<Vec<hazard::Condition>>,
    outer_index: usize,
    pointer: String,
}

impl<'a, const NESTED: bool> PartialCondition<'a, NESTED> {
    /// Creates an _equality_ condition between an already specified JSON pointer and the given
    /// `value`.
    ///
    /// # Panics
    ///
    /// The function will panic if the `value` cannot be converted to a JSON value (i.e.:
    /// [`std::f64::NAN`]).
    #[inline]
    #[must_use]
    pub fn eq<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.op(value, condition::Condition::Value)
    }

    /// Creates an _inequality_ condition between an already specified JSON pointer and the given
    /// `value`.
    ///
    /// # Panics
    ///
    /// The function will panic if the `value` cannot be converted to a JSON value (i.e.:
    /// [`std::f64::NAN`]).
    #[inline]
    #[must_use]
    pub fn ne<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Ne)
    }

    /// Creates a _less than_ condition between an already specified JSON pointer and the given
    /// `value`.
    ///
    /// # Panics
    ///
    /// The function will panic if the `value` cannot be converted to a JSON value (i.e.:
    /// [`std::f64::NAN`]).
    #[inline]
    #[must_use]
    pub fn lt<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Lt)
    }

    /// Creates a _less or equal than_ condition between an already specified JSON pointer and the
    /// given `value`.
    ///
    /// # Panics
    ///
    /// The function will panic if the `value` cannot be converted to a JSON value (i.e.:
    /// [`std::f64::NAN`]).
    #[inline]
    #[must_use]
    pub fn le<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Le)
    }

    /// Creates a _greater or equal than_ condition between an already specified JSON pointer and
    /// the given `value`.
    ///
    /// # Panics
    ///
    /// The function will panic if the `value` cannot be converted to a JSON value (i.e.:
    /// [`std::f64::NAN`]).
    #[inline]
    #[must_use]
    pub fn ge<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Ge)
    }

    /// Creates a _greater than_ condition between an already specified JSON pointer and the given
    /// `value`.
    ///
    /// # Panics
    ///
    /// The function will panic if the `value` cannot be converted to a JSON value (i.e.:
    /// [`std::f64::NAN`]).
    #[inline]
    #[must_use]
    pub fn gt<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Gt)
    }

    fn op<T, E, F>(self, value: T, make_condition: F) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
        F: FnOnce(condition::Value) -> condition::Condition,
    {
        let Self {
            conditions,
            outer_index,
            pointer,
        } = self;

        let pointer: JsonPointer = pointer.try_into().expect("invalid JSON pointer");
        let value = value.try_into().unwrap_or_else(|err| {
            panic!(
                "invalid conditional value used for pointer \"{}\": {err}",
                pointer.as_ref()
            );
        });
        let condition = make_condition(value);

        conditions[outer_index].push(hazard::Condition { pointer, condition });
        Condition(conditions)
    }

    #[inline]
    fn expr<T, E>(self, value: T, op: condition::Operation) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.op(value, |value| {
            condition::Condition::Expr(condition::Expr { value, op })
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        condition::{self, Condition},
        hazard::{self, Hazard},
        risk::{EXPLOSION, FIRE_HAZARD},
        Sifis,
    };

    #[test]
    fn hazard_helper() {
        let sifis = Sifis::builder()
            .hazard(hazard::Id::FireHazard, 1, |cond| cond)
            .hazard(hazard::Id::FireHazard, 3, |cond| cond)
            .hazard(hazard::Id::Explosion, 4, |cond| cond)
            .build();

        assert_eq!(
            sifis,
            Sifis {
                risks: vec![FIRE_HAZARD, EXPLOSION],
                hazards: vec![
                    Hazard {
                        risk: hazard::Risk {
                            id: hazard::Id::FireHazard,
                            level: 1,
                        },
                        conditions: Vec::new(),
                    },
                    Hazard {
                        risk: hazard::Risk {
                            id: hazard::Id::FireHazard,
                            level: 3,
                        },
                        conditions: Vec::new(),
                    },
                    Hazard {
                        risk: hazard::Risk {
                            id: hazard::Id::Explosion,
                            level: 4,
                        },
                        conditions: Vec::new(),
                    }
                ],
            },
        );
    }

    #[test]
    fn condition_and() {
        let sifis = Sifis::builder()
            .hazard(hazard::Id::FireHazard, 1, |cond| {
                cond.when("/properties/prop1")
                    .eq(5)
                    .and("/properties/prop2")
                    .gt(3)
            })
            .build();

        assert_eq!(
            sifis,
            Sifis {
                risks: vec![FIRE_HAZARD],
                hazards: vec![Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 1
                    },
                    conditions: vec![vec![
                        hazard::Condition {
                            pointer: "/properties/prop1".try_into().unwrap(),
                            condition: Condition::Value(5.into()),
                        },
                        hazard::Condition {
                            pointer: "/properties/prop2".try_into().unwrap(),
                            condition: Condition::Expr(condition::Expr {
                                value: 3.into(),
                                op: condition::Operation::Gt,
                            }),
                        }
                    ]],
                }],
            },
        );
    }

    #[test]
    fn condition_or() {
        let sifis = Sifis::builder()
            .hazard(hazard::Id::FireHazard, 1, |cond| {
                cond.when("/properties/prop1")
                    .eq(5)
                    .or(|cond| cond.when("/properties/prop2").gt(3))
            })
            .build();

        assert_eq!(
            sifis,
            Sifis {
                risks: vec![FIRE_HAZARD],
                hazards: vec![Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 1
                    },
                    conditions: vec![
                        vec![hazard::Condition {
                            pointer: "/properties/prop1".try_into().unwrap(),
                            condition: Condition::Value(5.into()),
                        }],
                        vec![hazard::Condition {
                            pointer: "/properties/prop2".try_into().unwrap(),
                            condition: Condition::Expr(condition::Expr {
                                value: 3.into(),
                                op: condition::Operation::Gt,
                            }),
                        }]
                    ],
                }],
            },
        );
    }

    #[test]
    fn condition_mixed() {
        let sifis = Sifis::builder()
            .hazard(hazard::Id::FireHazard, 1, |cond| {
                cond.when("/properties/prop1")
                    .ge(3)
                    .and("/properties/prop1")
                    .lt(10)
                    .or(|cond| {
                        cond.when("/properties/prop2")
                            .gt(5)
                            .and("/properties/prop2")
                            .le(15)
                    })
            })
            .build();

        assert_eq!(
            sifis,
            Sifis {
                risks: vec![FIRE_HAZARD],
                hazards: vec![Hazard {
                    risk: hazard::Risk {
                        id: hazard::Id::FireHazard,
                        level: 1
                    },
                    conditions: vec![
                        vec![
                            hazard::Condition {
                                pointer: "/properties/prop1".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 3.into(),
                                    op: condition::Operation::Ge,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop1".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 10.into(),
                                    op: condition::Operation::Lt,
                                }),
                            }
                        ],
                        vec![
                            hazard::Condition {
                                pointer: "/properties/prop2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 5.into(),
                                    op: condition::Operation::Gt,
                                }),
                            },
                            hazard::Condition {
                                pointer: "/properties/prop2".try_into().unwrap(),
                                condition: Condition::Expr(condition::Expr {
                                    value: 15.into(),
                                    op: condition::Operation::Le,
                                }),
                            }
                        ]
                    ],
                }],
            },
        );
    }

    #[test]
    fn short_form() {
        let sifis = Sifis::builder()
            .fire_hazard(1, |cond| cond)
            .fire_hazard(3, |cond| cond)
            .explosion(4, |cond| cond)
            .build();

        assert_eq!(
            sifis,
            Sifis {
                risks: vec![FIRE_HAZARD, EXPLOSION],
                hazards: vec![
                    Hazard {
                        risk: hazard::Risk {
                            id: hazard::Id::FireHazard,
                            level: 1,
                        },
                        conditions: Vec::new(),
                    },
                    Hazard {
                        risk: hazard::Risk {
                            id: hazard::Id::FireHazard,
                            level: 3,
                        },
                        conditions: Vec::new(),
                    },
                    Hazard {
                        risk: hazard::Risk {
                            id: hazard::Id::Explosion,
                            level: 4,
                        },
                        conditions: Vec::new(),
                    }
                ],
            },
        );
    }
}
