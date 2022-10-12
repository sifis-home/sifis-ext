#![warn(clippy::pedantic)]

use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};
use wot_td::extend::ExtendableThing;

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

#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InteractionAffordance {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub hazards: Vec<Hazard>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hazard {
    pub json_path: Cow<'static, str>,

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
    #[must_use]
    pub fn id(&self) -> HazardId {
        self.id
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub fn category(&self) -> HazardCategory {
        self.category
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum HazardId {
    #[serde(rename = "sho:AirPoisoning")]
    AirPoisoning,

    #[serde(rename = "sho:Asphyxia")]
    Asphyxia,

    #[serde(rename = "sho:AudioVideoRecordAndStore")]
    AudioVideoRecordAndStore,

    #[serde(rename = "sho:AudioVideoStream")]
    AudioVideoStream,

    #[serde(rename = "sho:ElectricEnergyConsumption")]
    ElectricEnergyConsumption,

    #[serde(rename = "sho:Explosion")]
    Explosion,

    #[serde(rename = "sho:FireHazard")]
    FireHazard,

    #[serde(rename = "sho:GasConsumption")]
    GasConsumption,

    #[serde(rename = "sho:LogEnergyConsumption")]
    LogEnergyConsumption,

    #[serde(rename = "sho:LogUsageTime")]
    LogUsageTime,

    #[serde(rename = "sho:PaySubscriptionFee")]
    PaySubscriptionFee,

    #[serde(rename = "sho:PowerOutage")]
    PowerOutage,

    #[serde(rename = "sho:PowerSurge")]
    PowerSurge,

    #[serde(rename = "sho:RecordIssuedCommands")]
    RecordIssuedCommands,

    #[serde(rename = "sho:RecordUserPreferences")]
    RecordUserPreferences,

    #[serde(rename = "sho:SpendMoney")]
    SpendMoney,

    #[serde(rename = "sho:SpoiledFood")]
    SpoiledFood,

    #[serde(rename = "sho:TakeDeviceScreenshots")]
    TakeDeviceScreenshots,

    #[serde(rename = "sho:TakePictures")]
    TakePictures,

    #[serde(rename = "sho:UnauthorisedPhysicalAccess")]
    UnauthorisedPhysicalAccess,

    #[serde(rename = "sho:WaterConsumption")]
    WaterConsumption,

    #[serde(rename = "sho:WaterFlooding")]
    WaterFlooding,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub enum HazardCategory {
    #[serde(rename = "sho:Financial")]
    Financial,

    #[serde(rename = "sho:Privacy")]
    Privacy,

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
    AIR_POISONING = {
        id: HazardId::AirPoisoning,
        description: "The execution may release toxic gases",
        name: "Air poisoning",
        category: HazardCategory::Safety,
        helper: air_poisoning,
    },

    ASPHYXIA = {
        id: HazardId::Asphyxia,
        description: "The execution may cause oxygen deficiency by gaseous substances",
        name: "Asphyxia",
        category: HazardCategory::Safety,
        helper: asphyxia,
    },

    AUDIO_VIDEO_RECORD_AND_STORE = {
        id: HazardId::AudioVideoRecordAndStore,
        description: "The execution authorises the app to record and save a video with audio on \
            persistent storage",
        name: "Audio video record and store",
        category: HazardCategory::Privacy,
        helper: audio_video_record_and_store,
    },

    AUDIO_VIDEO_STREAM = {
        id: HazardId::AudioVideoStream,
        description: "The execution authorises the app to obtain a video stream with audio",
        name: "Audio video stream",
        category: HazardCategory::Privacy,
        helper: audio_video_stream,
    },

    ELECTRIC_ENERGY_CONSUMPTION = {
        id: HazardId::ElectricEnergyConsumption,
        description: "The execution enables a device that consumes electricity",
        name: "Electric energy consumption",
        category: HazardCategory::Financial,
        helper: electric_energy_consumption,
    },

    EXPLOSION = {
        id: HazardId::Explosion,
        description: "The execution may cause an explosion",
        name: "Explosion",
        category: HazardCategory::Safety,
        helper: explosion,
    },

    FIRE_HAZARD = {
        id: HazardId::FireHazard,
        description: "The execution may cause fire",
        name: "Fire hazard",
        category: HazardCategory::Safety,
        helper: fire_hazard,
    },

    GAS_CONSUMPTION = {
        id: HazardId::GasConsumption,
        description: "The execution enables a device that consumes gas",
        name: "Gas consumption",
        category: HazardCategory::Financial,
        helper: gas_consumption,
    },

    LOG_ENERGY_CONSUMPTION = {
        id: HazardId::LogEnergyConsumption,
        description: "The execution authorises the app to get and save information about the app's \
            energy impact on the device the app runs on",
        name: "Log energy consumption",
        category: HazardCategory::Privacy,
        helper: log_energy_consumption,
    },

    LOG_USAGE_TIME = {
        id: HazardId::LogUsageTime,
        description: "The execution authorises the app to get and save information about the app's \
            duration of use",
        name: "Log usage time",
        category: HazardCategory::Privacy,
        helper: log_usage_time,
    },

    PAY_SUBSCRIPTION_FEE = {
        id: HazardId::PaySubscriptionFee,
        description: "The execution authorises the app to use payment information and make a \
            periodic payment",
        name: "Pay subscription fee",
        category: HazardCategory::Financial,
        helper: pay_subscription_fee,
    },

    POWER_OUTAGE = {
        id: HazardId::PowerOutage,
        description: "The execution may cause an interruption in the supply of electricity",
        name: "Power outage",
        category: HazardCategory::Safety,
        helper: power_outage,
    },

    POWER_SURGE = {
        id: HazardId::PowerSurge,
        description: "The execution may lead to exposure to high voltages",
        name: "Power surge",
        category: HazardCategory::Safety,
        helper: power_surge,
    },

    RECORD_ISSUED_COMMANDS = {
        id: HazardId::RecordIssuedCommands,
        description: "The execution authorises the app to get and save user inputs",
        name: "Record issued commands",
        category: HazardCategory::Privacy,
        helper: record_issued_commands,
    },

    RECORD_USER_PREFERENCES = {
        id: HazardId::RecordUserPreferences,
        description: "The execution authorises the app to get and save information about the \
            user's preferences",
        name: "Record user preferences",
        category: HazardCategory::Privacy,
        helper: record_user_preferences,
    },

    SPEND_MONEY = {
        id: HazardId::SpendMoney,
        description: "The execution authorises the app to use payment information and make a \
            payment transaction",
        name: "Spend money",
        category: HazardCategory::Financial,
        helper: spend_money,
    },

    SPOILED_FOOD = {
        id: HazardId::SpoiledFood,
        description: "The execution may lead to rotten food",
        name: "Spoiled food",
        category: HazardCategory::Safety,
        helper: spoiled_food,
    },

    TAKE_DEVICE_SCREENSHOTS = {
        id: HazardId::TakeDeviceScreenshots,
        description: "The execution authorises the app to read the display output and take \
            screenshots of it",
        name: "Take device screenshots",
        category: HazardCategory::Privacy,
        helper: take_device_screenshots,
    },

    TAKE_PICTURES = {
        id: HazardId::TakePictures,
        description: "The execution authorises the app to use a camera and take photos",
        name: "Take pictures",
        category: HazardCategory::Privacy,
        helper: take_pictures,
    },

    UNAUTHORISED_PHYSICAL_ACCESS = {
        id: HazardId::UnauthorisedPhysicalAccess,
        description: "The execution disables a protection mechanism and unauthorised individuals \
            may physically enter home",
        name: "Unauthorised physical access",
        category: HazardCategory::Safety,
        helper: unauthorised_physical_access,
    },

    WATER_CONSUMPTION = {
        id: HazardId::WaterConsumption,
        description: "The execution enables a device that consumes water",
        name: "Water consumption",
        category: HazardCategory::Financial,
        helper: water_consumption,
    },

    WATER_FLOODING = {
        id: HazardId::WaterFlooding,
        description: "The execution allows water usage which may lead to flood",
        name: "Water flooding",
        category: HazardCategory::Safety,
        helper: water_flooding,
    },
);
