//! Hazard-related structures.

use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::{condition, risk};

/// An hazard element.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Hazard {
    /// The associated risk.
    #[serde(rename = "sho:risk")]
    pub risk: Risk,

    /// The set of conditions that are required in order to trigger the hazard.
    ///
    /// There are two nested levels of vectors. The inner one express a logic `AND` between
    /// [`Condition`]s, the outer one express a logic `OR` between sets of conditions.
    ///
    /// For instance, this case:
    /// ```ignore
    /// let conditions = vec![
    ///     vec![A, B],
    ///     vec![C],
    ///     vec![D, E, F],
    /// ];
    /// ```
    /// is equivalent to `(A && B) || C || (D && E && F)`.
    #[serde(rename = "sho:conditions")]
    pub conditions: Vec<Vec<Condition>>,
}

/// The risk associated with the hazard, with the level of risk.
///
/// This structure only contains the `id` of the risk and not all its details. These are available
/// in [`risk::Detail`] instead.
///
/// [`risk::Detail`]: crate::risk::Detail
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Risk {
    /// The id of the associated risk.
    #[serde(rename = "@id")]
    pub id: Id,

    /// The risk level for the hazard.
    #[serde(rename = "sho:level")]
    pub level: u8,
}

/// A condition that needs to be satisfied based on the data available from the Thing.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// The JSON pointer for the expression to be evaluated.
    ///
    /// The relative document is the Thing Description.
    #[serde(rename = "sho:pointer")]
    pub pointer: JsonPointer,

    /// The condition that needs to be satisfied.
    #[serde(rename = "sho:condition")]
    pub condition: condition::Condition,
}

/// An opaque abstraction for a JSON Pointer (RFC 6901).
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct JsonPointer(pub(crate) jsonptr::Pointer);

/// An error caused by an invalid JSON pointer.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InvalidJsonPointer<'a>(Cow<'a, str>);

impl<'a> TryFrom<&'a str> for JsonPointer {
    type Error = InvalidJsonPointer<'a>;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        jsonptr::Pointer::try_from(value)
            .map_err(|_| InvalidJsonPointer(Cow::Borrowed(value)))
            .map(Self)
    }
}

impl TryFrom<String> for JsonPointer {
    type Error = InvalidJsonPointer<'static>;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        jsonptr::Pointer::try_from(&*value)
            .map_err(|_| InvalidJsonPointer(Cow::Owned(value)))
            .map(Self)
    }
}

impl Deref for JsonPointer {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for JsonPointer {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// The ID of an hazard.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Id {
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

    /// Burn
    #[serde(rename = "sho:Burn")]
    Burn,

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

    /// Scald
    #[serde(rename = "sho:Scald")]
    Scald,

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

impl Id {
    pub(crate) const fn risk(self) -> risk::Detail {
        match self {
            Id::AirPoisoning => risk::AIR_POISONING,
            Id::Asphyxia => risk::ASPHYXIA,
            Id::AudioVideoRecordAndStore => risk::AUDIO_VIDEO_RECORD_AND_STORE,
            Id::AudioVideoStream => risk::AUDIO_VIDEO_STREAM,
            Id::Burn => risk::BURN,
            Id::ElectricEnergyConsumption => risk::ELECTRIC_ENERGY_CONSUMPTION,
            Id::Explosion => risk::EXPLOSION,
            Id::FireHazard => risk::FIRE_HAZARD,
            Id::GasConsumption => risk::GAS_CONSUMPTION,
            Id::LogEnergyConsumption => risk::LOG_ENERGY_CONSUMPTION,
            Id::LogUsageTime => risk::LOG_USAGE_TIME,
            Id::PaySubscriptionFee => risk::PAY_SUBSCRIPTION_FEE,
            Id::PowerOutage => risk::POWER_OUTAGE,
            Id::PowerSurge => risk::POWER_SURGE,
            Id::RecordIssuedCommands => risk::RECORD_ISSUED_COMMANDS,
            Id::RecordUserPreferences => risk::RECORD_USER_PREFERENCES,
            Id::Scald => risk::SCALD,
            Id::SpendMoney => risk::SPEND_MONEY,
            Id::SpoiledFood => risk::SPOILED_FOOD,
            Id::TakeDeviceScreenshots => risk::TAKE_DEVICE_SCREENSHOTS,
            Id::TakePictures => risk::TAKE_PICTURES,
            Id::UnauthorisedPhysicalAccess => risk::UNAUTHORISED_PHYSICAL_ACCESS,
            Id::WaterConsumption => risk::WATER_CONSUMPTION,
            Id::WaterFlooding => risk::WATER_FLOODING,
        }
    }
}

/// The category of an hazard.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Category {
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
