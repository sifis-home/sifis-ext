//! The Sifis extension for Web of Things Description.
//!
//! This crate is mainly focused on specifying hazards for Thing Descriptions for the crate
//! [wot-td]. Using the helper methods, it is possible to specify hazards with custom _risk level_
//! and _JSON path_.
//!
//! The _context_ of the JSON path depends on the type of affordance the hazard is applied to. See
//! [`Hazard::json_path`] for more information.
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
//! use sifis_td::{explosion, fire_hazard, Sifis};
//! use wot_td::{
//!     builder::{
//!         IntegerDataSchemaBuilderLike, NumberDataSchemaBuilderLike, ObjectDataSchemaBuilderLike,
//!         SpecializableDataSchema,
//!     },
//!     Thing,
//! };
//!
//! let thing = Thing::builder("My Thing")
//!     .ext(Sifis::default())
//!     .finish_extend()
//!     .context_map(|b| b.context("sho", "https://purl.org/sifis/hazards"))
//!     .property("prop", |b| {
//!         b.ext(())
//!             .ext_data_schema(())
//!             .finish_extend_data_schema()
//!             .object()
//!             .property("inner1", false, |b| {
//!                 b.ext(()).finish_extend().number().minimum(0.).maximum(1.)
//!             })
//!             .property("inner2", false, |b| {
//!                 b.ext(()).finish_extend().integer().minimum(1).maximum(10)
//!             })
//!             .ext_interaction(sifis_td::InteractionAffordance {
//!                 hazards: vec![
//!                     fire_hazard(3, "[?(@.inner1 < 0.5 && @.inner2 < 6)]"),
//!                     fire_hazard(7, "[?(@.inner1 >= 0.5 || @.inner2 >= 6)]"),
//!                     explosion(1, "[?(@.inner1 >= 0.9)]"),
//!                 ],
//!             })
//!     })
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(
//!     serde_json::to_value(thing).unwrap(),
//!     json!({
//!         "@context": [
//!             "https://www.w3.org/2019/wot/td/v1.1",
//!             {
//!                 "sho": "https://purl.org/sifis/hazards",
//!             },
//!         ],
//!         "title":"My Thing",
//!         "properties":{
//!             "prop": {
//!                 "forms": [],
//!                 "hazards": [
//!                     {
//!                         "jsonPath": "[?(@.inner1 < 0.5 && @.inner2 < 6)]",
//!                         "riskLevel": 3,
//!                         "@id": "sho:FireHazard",
//!                         "description": "The execution may cause fire",
//!                         "name": "Fire hazard",
//!                         "category":"sho:Safety",
//!                     },
//!                     {
//!                         "jsonPath": "[?(@.inner1 >= 0.5 || @.inner2 >= 6)]",
//!                         "riskLevel": 7,
//!                         "@id": "sho:FireHazard",
//!                         "description": "The execution may cause fire",
//!                         "name": "Fire hazard",
//!                         "category": "sho:Safety",
//!                     },
//!                     {
//!                         "jsonPath": "[?(@.inner1 >= 0.9)]",
//!                         "riskLevel": 1,
//!                         "@id": "sho:Explosion",
//!                         "description": "The execution may cause an explosion",
//!                         "name": "Explosion",
//!                         "category": "sho:Safety",
//!                     },
//!                 ],
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
//!     }),
//! )
//! ```

#![warn(clippy::pedantic)]

use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};
use wot_td::extend::ExtendableThing;

/// The Sifis extension for a Thing Description.
///
/// When this is used to extend a [`Thing`], then it is necessary to add a context with the prefix
/// `sho` to points to `https://purl.org/sifis/hazards`.
///
/// [`Thing`]: wot_td::Thing
#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Sifis {}

impl ExtendableThing for Sifis {
    type InteractionAffordance = InteractionAffordance;
    type PropertyAffordance = ();
    type ActionAffordance = ();
    type EventAffordance = ();
    type Form = ();
    type ExpectedResponse = ();
    type DataSchema = ();
    type ObjectSchema = ();
    type ArraySchema = ();
}

/// The Sifis extension for the Interaction Affordance of a Thing Description.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InteractionAffordance {
    /// The hazards associated with the Interaction Affordance.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub hazards: Vec<Hazard>,
}

/// An hazard element.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hazard {
    /// The JSON Path for the hazard with the current risk level.
    ///
    /// This is required in order to apply the hazard depending on some values related to the
    /// affordance. The expression only needs to match a valid path, the _extracted_ value is
    /// irrelevant for our purposes.
    ///
    /// The context for the JSON Path depends on the type of affordance the hazard is applied on.
    ///
    /// - When the hazard is applied to a [Property Affordance], the context of the JSON Path is
    /// the property itself, described by its [`DataSchema`].
    ///
    /// - When the hazard is applied to an [Action Affordance], the context of the JSON Path is the
    /// [`input`] data schema field. This means that the hazard needs to match a valid expression
    /// inside the data passed to the action.
    ///
    /// - When the hazard is applied to a [Event Affordance], the context of the JSON Path is the
    /// [`data`] data schema field. This means that the hazard needs to match a valid expression
    /// inside the data returned by the event.
    ///
    /// [Property Affordance]: wot_td::thing::PropertyAffordance
    /// [`DataSchema`]: wot_td::thing::DataSchema
    /// [Action Affordance]: wot_td::thing::ActionAffordance
    /// [`input`]: wot_td::thing::ActionAffordance::input
    /// [Event Affordance]: wot_td::thing::EventAffordance
    /// [`data`]: wot_td::thing::EventAffordance::data
    pub json_path: Cow<'static, str>,

    /// The risk level for the hazard.
    pub risk_level: u8,

    #[serde(flatten)]
    inner: HazardInfo,
}

impl Deref for Hazard {
    type Target = HazardInfo;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// The constant information of an hazard.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HazardInfo {
    #[serde(rename = "@id")]
    id: HazardId,
    description: Cow<'static, str>,
    name: Cow<'static, str>,
    category: HazardCategory,
}

impl HazardInfo {
    /// The ID of the hazard.
    #[must_use]
    pub fn id(&self) -> HazardId {
        self.id
    }

    /// The description of the hazard, in human-readable form.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// The name of the hazard, in human-readable form.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The category of the hazard.
    #[must_use]
    pub fn category(&self) -> HazardCategory {
        self.category
    }
}

/// The ID of an hazard.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HazardId {
    /// Air poisoning
    #[serde(rename = "sho:AirPoisoning")]
    AirPoisoning,

    /// Asphyxia
    #[serde(rename = "sho:Asphyxia")]
    Asphyxia,

    /// Audio video record and store
    #[serde(rename = "sho:AudioVideoRecordAndStore")]
    AudioVideoRecordAndStore,

    /// Audio video stream
    #[serde(rename = "sho:AudioVideoStream")]
    AudioVideoStream,

    /// Electric energy consumption
    #[serde(rename = "sho:ElectricEnergyConsumption")]
    ElectricEnergyConsumption,

    /// Explosion
    #[serde(rename = "sho:Explosion")]
    Explosion,

    /// Fire hazard
    #[serde(rename = "sho:FireHazard")]
    FireHazard,

    /// Gas consumption
    #[serde(rename = "sho:GasConsumption")]
    GasConsumption,

    /// Log energy consumption
    #[serde(rename = "sho:LogEnergyConsumption")]
    LogEnergyConsumption,

    /// Log usage time
    #[serde(rename = "sho:LogUsageTime")]
    LogUsageTime,

    /// Pay subscription fee
    #[serde(rename = "sho:PaySubscriptionFee")]
    PaySubscriptionFee,

    /// Power outage
    #[serde(rename = "sho:PowerOutage")]
    PowerOutage,

    /// Power surge
    #[serde(rename = "sho:PowerSurge")]
    PowerSurge,

    /// Record issued commands
    #[serde(rename = "sho:RecordIssuedCommands")]
    RecordIssuedCommands,

    /// Record user preferences
    #[serde(rename = "sho:RecordUserPreferences")]
    RecordUserPreferences,

    /// Spend money
    #[serde(rename = "sho:SpendMoney")]
    SpendMoney,

    /// Spoiled food
    #[serde(rename = "sho:SpoiledFood")]
    SpoiledFood,

    /// Take device screenshots
    #[serde(rename = "sho:TakeDeviceScreenshots")]
    TakeDeviceScreenshots,

    /// Take pictures
    #[serde(rename = "sho:TakePictures")]
    TakePictures,

    /// Unauthorised physical access
    #[serde(rename = "sho:UnauthorisedPhysicalAccess")]
    UnauthorisedPhysicalAccess,

    /// Water consumption
    #[serde(rename = "sho:WaterConsumption")]
    WaterConsumption,

    /// Water flooding
    #[serde(rename = "sho:WaterFlooding")]
    WaterFlooding,
}

/// The category of an hazard.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum HazardCategory {
    /// Financial
    #[serde(rename = "sho:Financial")]
    Financial,

    /// Privacy
    #[serde(rename = "sho:Privacy")]
    Privacy,

    /// Safety
    #[serde(rename = "sho:Safety")]
    Safety,
}

macro_rules! hazard {
    ($(
        $(#[$meta:meta])*
        $hazard_name:ident = {
            id: $id:expr,
            description: $description:literal,
            name: $name:literal,
            category: $category:expr,
            helper: $helper:ident $(,)?
        }
    ),+ $(,)?) => {
        $(
            const $hazard_name: HazardInfo = HazardInfo {
                id: $id,
                description: Cow::Borrowed($description),
                name: Cow::Borrowed($name),
                category: $category,
            };

            $(#[$meta])*
            pub fn $helper(risk_level: u8, json_path: impl Into<Cow<'static, str>>) -> Hazard {
                let json_path = json_path.into();

                Hazard {
                    risk_level,
                    json_path,
                    inner: $hazard_name,
                }
            }
        )+
    };
}

hazard!(
    /// Creates an air poisoning hazard.
    AIR_POISONING = {
        id: HazardId::AirPoisoning,
        description: "The execution may release toxic gases",
        name: "Air poisoning",
        category: HazardCategory::Safety,
        helper: air_poisoning,
    },

    /// Creates an asphyxia hazard.
    ASPHYXIA = {
        id: HazardId::Asphyxia,
        description: "The execution may cause oxygen deficiency by gaseous substances",
        name: "Asphyxia",
        category: HazardCategory::Safety,
        helper: asphyxia,
    },

    /// Creates an audio video record and store hazard.
    AUDIO_VIDEO_RECORD_AND_STORE = {
        id: HazardId::AudioVideoRecordAndStore,
        description: "The execution authorises the app to record and save a video with audio on \
            persistent storage",
        name: "Audio video record and store",
        category: HazardCategory::Privacy,
        helper: audio_video_record_and_store,
    },

    /// Creates an audio video stream hazard.
    AUDIO_VIDEO_STREAM = {
        id: HazardId::AudioVideoStream,
        description: "The execution authorises the app to obtain a video stream with audio",
        name: "Audio video stream",
        category: HazardCategory::Privacy,
        helper: audio_video_stream,
    },

    /// Creates an electric energy consumption hazard.
    ELECTRIC_ENERGY_CONSUMPTION = {
        id: HazardId::ElectricEnergyConsumption,
        description: "The execution enables a device that consumes electricity",
        name: "Electric energy consumption",
        category: HazardCategory::Financial,
        helper: electric_energy_consumption,
    },

    /// Creates an explosion hazard.
    EXPLOSION = {
        id: HazardId::Explosion,
        description: "The execution may cause an explosion",
        name: "Explosion",
        category: HazardCategory::Safety,
        helper: explosion,
    },

    /// Creates a fire hazard.
    FIRE_HAZARD = {
        id: HazardId::FireHazard,
        description: "The execution may cause fire",
        name: "Fire hazard",
        category: HazardCategory::Safety,
        helper: fire_hazard,
    },

    /// Creates a gas consumption hazard.
    GAS_CONSUMPTION = {
        id: HazardId::GasConsumption,
        description: "The execution enables a device that consumes gas",
        name: "Gas consumption",
        category: HazardCategory::Financial,
        helper: gas_consumption,
    },

    /// Creates a log energy consumption hazard.
    LOG_ENERGY_CONSUMPTION = {
        id: HazardId::LogEnergyConsumption,
        description: "The execution authorises the app to get and save information about the app's \
            energy impact on the device the app runs on",
        name: "Log energy consumption",
        category: HazardCategory::Privacy,
        helper: log_energy_consumption,
    },

    /// Creates a log usage time hazard.
    LOG_USAGE_TIME = {
        id: HazardId::LogUsageTime,
        description: "The execution authorises the app to get and save information about the app's \
            duration of use",
        name: "Log usage time",
        category: HazardCategory::Privacy,
        helper: log_usage_time,
    },

    /// Creates a pay subscription fee hazard.
    PAY_SUBSCRIPTION_FEE = {
        id: HazardId::PaySubscriptionFee,
        description: "The execution authorises the app to use payment information and make a \
            periodic payment",
        name: "Pay subscription fee",
        category: HazardCategory::Financial,
        helper: pay_subscription_fee,
    },

    /// Creates a power outage hazard.
    POWER_OUTAGE = {
        id: HazardId::PowerOutage,
        description: "The execution may cause an interruption in the supply of electricity",
        name: "Power outage",
        category: HazardCategory::Safety,
        helper: power_outage,
    },

    /// Creates a power surge hazard.
    POWER_SURGE = {
        id: HazardId::PowerSurge,
        description: "The execution may lead to exposure to high voltages",
        name: "Power surge",
        category: HazardCategory::Safety,
        helper: power_surge,
    },

    /// Creates a record issued commands hazard.
    RECORD_ISSUED_COMMANDS = {
        id: HazardId::RecordIssuedCommands,
        description: "The execution authorises the app to get and save user inputs",
        name: "Record issued commands",
        category: HazardCategory::Privacy,
        helper: record_issued_commands,
    },

    /// Creates a record user preferences hazard.
    RECORD_USER_PREFERENCES = {
        id: HazardId::RecordUserPreferences,
        description: "The execution authorises the app to get and save information about the \
            user's preferences",
        name: "Record user preferences",
        category: HazardCategory::Privacy,
        helper: record_user_preferences,
    },

    /// Creates a spend money hazard.
    SPEND_MONEY = {
        id: HazardId::SpendMoney,
        description: "The execution authorises the app to use payment information and make a \
            payment transaction",
        name: "Spend money",
        category: HazardCategory::Financial,
        helper: spend_money,
    },

    /// Creates a spoiled food hazard.
    SPOILED_FOOD = {
        id: HazardId::SpoiledFood,
        description: "The execution may lead to rotten food",
        name: "Spoiled food",
        category: HazardCategory::Safety,
        helper: spoiled_food,
    },

    /// Creates a take device screenshots hazard.
    TAKE_DEVICE_SCREENSHOTS = {
        id: HazardId::TakeDeviceScreenshots,
        description: "The execution authorises the app to read the display output and take \
            screenshots of it",
        name: "Take device screenshots",
        category: HazardCategory::Privacy,
        helper: take_device_screenshots,
    },

    /// Creates a take pictures hazard.
    TAKE_PICTURES = {
        id: HazardId::TakePictures,
        description: "The execution authorises the app to use a camera and take photos",
        name: "Take pictures",
        category: HazardCategory::Privacy,
        helper: take_pictures,
    },

    /// Creates an unauthorised physical access hazard.
    UNAUTHORISED_PHYSICAL_ACCESS = {
        id: HazardId::UnauthorisedPhysicalAccess,
        description: "The execution disables a protection mechanism and unauthorised individuals \
            may physically enter home",
        name: "Unauthorised physical access",
        category: HazardCategory::Safety,
        helper: unauthorised_physical_access,
    },

    /// Creates a water consumption hazard.
    WATER_CONSUMPTION = {
        id: HazardId::WaterConsumption,
        description: "The execution enables a device that consumes water",
        name: "Water consumption",
        category: HazardCategory::Financial,
        helper: water_consumption,
    },

    /// Creates a water flooding hazard.
    WATER_FLOODING = {
        id: HazardId::WaterFlooding,
        description: "The execution allows water usage which may lead to flood",
        name: "Water flooding",
        category: HazardCategory::Safety,
        helper: water_flooding,
    },
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_hazards {
        (
            $(
                $fn_name:ident = {
                    id: $id:expr,
                    category: $category:expr,
                    name: $name:expr,
                    description: $description:expr $(,)?
                }
             ),+
            $(,)?
        ) => {
            $(
                #[test]
                fn $fn_name() {
                    let hazard = super::$fn_name(1, ".");

                    assert_eq!(
                        hazard,
                        Hazard {
                            json_path: Cow::Borrowed("."),
                            risk_level: 1,
                            inner: HazardInfo {
                                id: $id,
                                description: Cow::Borrowed($description),
                                name: Cow::Borrowed($name),
                                category: $category,
                            },
                        }
                    );
                }
            )+
        };
    }

    test_hazards!(
        air_poisoning = {
            id: HazardId::AirPoisoning,
            category: HazardCategory::Safety,
            name: "Air poisoning",
            description: "The execution may release toxic gases",
        },
        asphyxia = {
            id: HazardId::Asphyxia,
            category: HazardCategory::Safety,
            name: "Asphyxia",
            description: "The execution may cause oxygen deficiency by gaseous substances",
        },
        audio_video_record_and_store = {
            id: HazardId::AudioVideoRecordAndStore,
            category: HazardCategory::Privacy,
            name: "Audio video record and store",
            description: "The execution authorises the app to record and save a video with audio on persistent storage",
        },
        audio_video_stream = {
            id: HazardId::AudioVideoStream,
            category: HazardCategory::Privacy,
            name: "Audio video stream",
            description: "The execution authorises the app to obtain a video stream with audio",
        },
        electric_energy_consumption = {
            id: HazardId::ElectricEnergyConsumption,
            category: HazardCategory::Financial,
            name: "Electric energy consumption",
            description: "The execution enables a device that consumes electricity",
        },
        explosion = {
            id: HazardId::Explosion,
            category: HazardCategory::Safety,
            name: "Explosion",
            description: "The execution may cause an explosion",
        },
        fire_hazard = {
            id: HazardId::FireHazard,
            category: HazardCategory::Safety,
            name: "Fire hazard",
            description: "The execution may cause fire",
        },
        gas_consumption = {
            id: HazardId::GasConsumption,
            category: HazardCategory::Financial,
            name: "Gas consumption",
            description: "The execution enables a device that consumes gas",
        },
        log_energy_consumption = {
            id: HazardId::LogEnergyConsumption,
            category: HazardCategory::Privacy,
            name: "Log energy consumption",
            description: "The execution authorises the app to get and save information about the app's energy impact on the device the app runs on",
        },
        log_usage_time = {
            id: HazardId::LogUsageTime,
            category: HazardCategory::Privacy,
            name: "Log usage time",
            description: "The execution authorises the app to get and save information about the app's duration of use",
        },
        pay_subscription_fee = {
            id: HazardId::PaySubscriptionFee,
            category: HazardCategory::Financial,
            name: "Pay subscription fee",
            description: "The execution authorises the app to use payment information and make a periodic payment",
        },
        power_outage = {
            id: HazardId::PowerOutage,
            category: HazardCategory::Safety,
            name: "Power outage",
            description: "The execution may cause an interruption in the supply of electricity",
        },
        power_surge = {
            id: HazardId::PowerSurge,
            category: HazardCategory::Safety,
            name: "Power surge",
            description: "The execution may lead to exposure to high voltages",
        },
        record_issued_commands = {
            id: HazardId::RecordIssuedCommands,
            category: HazardCategory::Privacy,
            name: "Record issued commands",
            description: "The execution authorises the app to get and save user inputs",
        },
        record_user_preferences = {
            id: HazardId::RecordUserPreferences,
            category: HazardCategory::Privacy,
            name: "Record user preferences",
            description: "The execution authorises the app to get and save information about the user's preferences",
        },
        spend_money = {
            id: HazardId::SpendMoney,
            category: HazardCategory::Financial,
            name: "Spend money",
            description: "The execution authorises the app to use payment information and make a payment transaction",
        },
        spoiled_food = {
            id: HazardId::SpoiledFood,
            category: HazardCategory::Safety,
            name: "Spoiled food",
            description: "The execution may lead to rotten food",
        },
        take_device_screenshots = {
            id: HazardId::TakeDeviceScreenshots,
            category: HazardCategory::Privacy,
            name: "Take device screenshots",
            description: "The execution authorises the app to read the display output and take screenshots of it",
        },
        take_pictures = {
            id: HazardId::TakePictures,
            category: HazardCategory::Privacy,
            name: "Take pictures",
            description: "The execution authorises the app to use a camera and take photos",
        },
        unauthorised_physical_access = {
            id: HazardId::UnauthorisedPhysicalAccess,
            category: HazardCategory::Safety,
            name: "Unauthorised physical access",
            description: "The execution disables a protection mechanism and unauthorised individuals may physically enter home",
        },
        water_consumption = {
            id: HazardId::WaterConsumption,
            category: HazardCategory::Financial,
            name: "Water consumption",
            description: "The execution enables a device that consumes water",
        },
        water_flooding = {
            id: HazardId::WaterFlooding,
            category: HazardCategory::Safety,
            name: "Water flooding",
            description: "The execution allows water usage which may lead to flood",
        },
    );
}
