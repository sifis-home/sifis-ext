use std::{borrow::Cow, ops::Deref};

use serde::{Deserialize, Serialize};

use crate::condition;

/// An hazard element.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Hazard {
    #[serde(rename = "sho:risk")]
    pub risk: Risk,

    #[serde(rename = "sho:conditions")]
    pub conditions: Vec<Vec<Condition>>,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Risk {
    /// The id of the associated risk.
    #[serde(rename = "@id")]
    pub id: Id,

    /// The risk level for the hazard.
    #[serde(rename = "sho:level")]
    pub level: u8,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Condition {
    /// The JSON pointer for the expression to be evaluated.
    ///
    /// The relative document is the Thing Description.
    #[serde(rename = "sho:pointer")]
    pub pointer: JsonPointer,

    /// The condition that need to be satisfied.
    #[serde(rename = "sho:condition")]
    pub condition: condition::Condition,
}

/// An opaque abstraction for a JSON Pointer (RFC 6901).
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct JsonPointer(pub(crate) jsonptr::Pointer);

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
