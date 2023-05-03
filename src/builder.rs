use std::{fmt::Display, ops::Not};

use crate::{
    condition,
    hazard::{self, Hazard, JsonPointer},
    risk, Sifis,
};

#[derive(Debug, Default)]
pub struct Builder {
    risks: Vec<risk::Detail>,
    hazards: Vec<Hazard>,
}

macro_rules! impl_builder_hazard {
    ($($fn:ident => $id:ident),+ $(,)?) => {
        $(
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
    #[inline]
    #[must_use]
    pub fn when(self, pointer: impl Into<String>) -> PartialCondition<'a, NESTED> {
        self.add_condition(pointer.into())
    }
}

impl<'a, const NESTED: bool> Condition<'a, true, NESTED> {
    #[inline]
    #[must_use]
    pub fn and(self, pointer: impl Into<String>) -> PartialCondition<'a, NESTED> {
        self.add_condition(pointer.into())
    }
}

impl<'a> Condition<'a, true, false> {
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

#[derive(Debug)]
pub struct PartialCondition<'a, const NESTED: bool> {
    conditions: &'a mut Vec<Vec<hazard::Condition>>,
    outer_index: usize,
    pointer: String,
}

impl<'a, const NESTED: bool> PartialCondition<'a, NESTED> {
    #[inline]
    #[must_use]
    pub fn eq<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.op(value, condition::Condition::Value)
    }

    #[inline]
    #[must_use]
    pub fn ne<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Ne)
    }

    #[inline]
    #[must_use]
    pub fn lt<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Lt)
    }

    #[inline]
    #[must_use]
    pub fn le<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Le)
    }

    #[inline]
    #[must_use]
    pub fn ge<T, E>(self, value: T) -> Condition<'a, true, NESTED>
    where
        T: TryInto<condition::Value, Error = E>,
        E: Display + std::error::Error,
    {
        self.expr(value, condition::Operation::Ge)
    }

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
