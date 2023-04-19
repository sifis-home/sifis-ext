use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::hazard;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Detail {
    #[serde(rename = "@id")]
    id: hazard::Id,

    #[serde(rename = "sho:category")]
    category: hazard::Category,

    /// The description of the hazard, in human-readable form.
    #[serde(rename = "sho:description")]
    pub description: Cow<'static, str>,

    /// The name of the hazard, in human-readable form.
    #[serde(rename = "sho:name")]
    pub name: Cow<'static, str>,
}

impl Detail {
    /// The ID of the hazard.
    #[must_use]
    pub fn id(&self) -> hazard::Id {
        self.id
    }

    /// The category of the hazard.
    #[must_use]
    pub fn category(&self) -> hazard::Category {
        self.category
    }
}

macro_rules! risk {
    ($(
        $(#[$meta:meta])*
        $risk_name:ident = {
            id: $id:expr,
            description: $description:literal,
            name: $name:literal,
            category: $category:expr,
        }
    ),+ $(,)?) => {
        $(
            $(#[$meta])*
            pub const $risk_name: Detail = Detail {
                id: $id,
                description: Cow::Borrowed($description),
                name: Cow::Borrowed($name),
                category: $category,
            };
        )+
    };
}

risk!(
    /// An air poisoning risk.
    AIR_POISONING = {
        id: hazard::Id::AirPoisoning,
        description: "The execution may release toxic gases",
        name: "Air poisoning",
        category: hazard::Category::Safety,
    },

    /// An asphyxia risk.
    ASPHYXIA = {
        id: hazard::Id::Asphyxia,
        description: "The execution may cause oxygen deficiency by gaseous substances",
        name: "Asphyxia",
        category: hazard::Category::Safety,
    },

    /// An audio video record and store risk.
    AUDIO_VIDEO_RECORD_AND_STORE = {
        id: hazard::Id::AudioVideoRecordAndStore,
        description: "The execution authorises the app to record and save a video with audio on \
            persistent storage",
        name: "Audio video record and store",
        category: hazard::Category::Privacy,
    },

    /// An audio video stream risk.
    AUDIO_VIDEO_STREAM = {
        id: hazard::Id::AudioVideoStream,
        description: "The execution authorises the app to obtain a video stream with audio",
        name: "Audio video stream",
        category: hazard::Category::Privacy,
    },

    /// An electric energy consumption risk.
    ELECTRIC_ENERGY_CONSUMPTION = {
        id: hazard::Id::ElectricEnergyConsumption,
        description: "The execution enables a device that consumes electricity",
        name: "Electric energy consumption",
        category: hazard::Category::Financial,
    },

    /// An explosion risk.
    EXPLOSION = {
        id: hazard::Id::Explosion,
        description: "The execution may cause an explosion",
        name: "Explosion",
        category: hazard::Category::Safety,
    },

    /// A fire risk.
    FIRE_HAZARD = {
        id: hazard::Id::FireHazard,
        description: "The execution may cause fire",
        name: "Fire hazard",
        category: hazard::Category::Safety,
    },

    /// A gas consumption risk.
    GAS_CONSUMPTION = {
        id: hazard::Id::GasConsumption,
        description: "The execution enables a device that consumes gas",
        name: "Gas consumption",
        category: hazard::Category::Financial,
    },

    /// A log energy consumption risk.
    LOG_ENERGY_CONSUMPTION = {
        id: hazard::Id::LogEnergyConsumption,
        description: "The execution authorises the app to get and save information about the app's \
            energy impact on the device the app runs on",
        name: "Log energy consumption",
        category: hazard::Category::Privacy,
    },

    /// A log usage time risk.
    LOG_USAGE_TIME = {
        id: hazard::Id::LogUsageTime,
        description: "The execution authorises the app to get and save information about the app's \
            duration of use",
        name: "Log usage time",
        category: hazard::Category::Privacy,
    },

    /// A pay subscription fee risk.
    PAY_SUBSCRIPTION_FEE = {
        id: hazard::Id::PaySubscriptionFee,
        description: "The execution authorises the app to use payment information and make a \
            periodic payment",
        name: "Pay subscription fee",
        category: hazard::Category::Financial,
    },

    /// A power outage risk.
    POWER_OUTAGE = {
        id: hazard::Id::PowerOutage,
        description: "The execution may cause an interruption in the supply of electricity",
        name: "Power outage",
        category: hazard::Category::Safety,
    },

    /// A power surge risk.
    POWER_SURGE = {
        id: hazard::Id::PowerSurge,
        description: "The execution may lead to exposure to high voltages",
        name: "Power surge",
        category: hazard::Category::Safety,
    },

    /// A record issued commands risk.
    RECORD_ISSUED_COMMANDS = {
        id: hazard::Id::RecordIssuedCommands,
        description: "The execution authorises the app to get and save user inputs",
        name: "Record issued commands",
        category: hazard::Category::Privacy,
    },

    /// A record user preferences risk.
    RECORD_USER_PREFERENCES = {
        id: hazard::Id::RecordUserPreferences,
        description: "The execution authorises the app to get and save information about the \
            user's preferences",
        name: "Record user preferences",
        category: hazard::Category::Privacy,
    },

    /// A spend money risk.
    SPEND_MONEY = {
        id: hazard::Id::SpendMoney,
        description: "The execution authorises the app to use payment information and make a \
            payment transaction",
        name: "Spend money",
        category: hazard::Category::Financial,
    },

    /// A spoiled food risk.
    SPOILED_FOOD = {
        id: hazard::Id::SpoiledFood,
        description: "The execution may lead to rotten food",
        name: "Spoiled food",
        category: hazard::Category::Safety,
    },

    /// A take device screenshots risk.
    TAKE_DEVICE_SCREENSHOTS = {
        id: hazard::Id::TakeDeviceScreenshots,
        description: "The execution authorises the app to read the display output and take \
            screenshots of it",
        name: "Take device screenshots",
        category: hazard::Category::Privacy,
    },

    /// A take pictures risk.
    TAKE_PICTURES = {
        id: hazard::Id::TakePictures,
        description: "The execution authorises the app to use a camera and take photos",
        name: "Take pictures",
        category: hazard::Category::Privacy,
    },

    /// An unauthorised physical access risk.
    UNAUTHORISED_PHYSICAL_ACCESS = {
        id: hazard::Id::UnauthorisedPhysicalAccess,
        description: "The execution disables a protection mechanism and unauthorised individuals \
            may physically enter home",
        name: "Unauthorised physical access",
        category: hazard::Category::Safety,
    },

    /// A water consumption risk.
    WATER_CONSUMPTION = {
        id: hazard::Id::WaterConsumption,
        description: "The execution enables a device that consumes water",
        name: "Water consumption",
        category: hazard::Category::Financial,
    },

    /// A water flooding risk.
    WATER_FLOODING = {
        id: hazard::Id::WaterFlooding,
        description: "The execution allows water usage which may lead to flood",
        name: "Water flooding",
        category: hazard::Category::Safety,
    },
);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_risks {
        (
            $(
                #[test = $test_name:ident]
                $risk_name:ident = {
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
                fn $test_name() {
                    assert_eq!(
                        $risk_name,
                        Detail {
                            id: $id,
                            description: Cow::Borrowed($description),
                            name: Cow::Borrowed($name),
                            category: $category,
                        }
                    );
                }
            )+
        };
    }

    test_risks!(
        #[test = air_poisoning]
        AIR_POISONING = {
            id: hazard::Id::AirPoisoning,
            category: hazard::Category::Safety,
            name: "Air poisoning",
            description: "The execution may release toxic gases",
        },
        #[test = asphyxia]
        ASPHYXIA = {
            id: hazard::Id::Asphyxia,
            category: hazard::Category::Safety,
            name: "Asphyxia",
            description: "The execution may cause oxygen deficiency by gaseous substances",
        },
        #[test = audio_video_record_and_store]
        AUDIO_VIDEO_RECORD_AND_STORE = {
            id: hazard::Id::AudioVideoRecordAndStore,
            category: hazard::Category::Privacy,
            name: "Audio video record and store",
            description: "The execution authorises the app to record and save a video with audio on persistent storage",
        },
        #[test = audio_video_stream]
        AUDIO_VIDEO_STREAM = {
            id: hazard::Id::AudioVideoStream,
            category: hazard::Category::Privacy,
            name: "Audio video stream",
            description: "The execution authorises the app to obtain a video stream with audio",
        },
        #[test = electric_energy_consumption]
        ELECTRIC_ENERGY_CONSUMPTION = {
            id: hazard::Id::ElectricEnergyConsumption,
            category: hazard::Category::Financial,
            name: "Electric energy consumption",
            description: "The execution enables a device that consumes electricity",
        },
        #[test = explosion]
        EXPLOSION = {
            id: hazard::Id::Explosion,
            category: hazard::Category::Safety,
            name: "Explosion",
            description: "The execution may cause an explosion",
        },
        #[test = fire_hazard]
        FIRE_HAZARD = {
            id: hazard::Id::FireHazard,
            category: hazard::Category::Safety,
            name: "Fire hazard",
            description: "The execution may cause fire",
        },
        #[test = gas_consumption]
        GAS_CONSUMPTION = {
            id: hazard::Id::GasConsumption,
            category: hazard::Category::Financial,
            name: "Gas consumption",
            description: "The execution enables a device that consumes gas",
        },
        #[test = log_energy_consumption]
        LOG_ENERGY_CONSUMPTION = {
            id: hazard::Id::LogEnergyConsumption,
            category: hazard::Category::Privacy,
            name: "Log energy consumption",
            description: "The execution authorises the app to get and save information about the app's energy impact on the device the app runs on",
        },
        #[test = log_usage_time]
        LOG_USAGE_TIME = {
            id: hazard::Id::LogUsageTime,
            category: hazard::Category::Privacy,
            name: "Log usage time",
            description: "The execution authorises the app to get and save information about the app's duration of use",
        },
        #[test = pay_subscription_fee]
        PAY_SUBSCRIPTION_FEE = {
            id: hazard::Id::PaySubscriptionFee,
            category: hazard::Category::Financial,
            name: "Pay subscription fee",
            description: "The execution authorises the app to use payment information and make a periodic payment",
        },
        #[test = power_outage]
        POWER_OUTAGE = {
            id: hazard::Id::PowerOutage,
            category: hazard::Category::Safety,
            name: "Power outage",
            description: "The execution may cause an interruption in the supply of electricity",
        },
        #[test = power_surge]
        POWER_SURGE = {
            id: hazard::Id::PowerSurge,
            category: hazard::Category::Safety,
            name: "Power surge",
            description: "The execution may lead to exposure to high voltages",
        },
        #[test = record_issued_commands]
        RECORD_ISSUED_COMMANDS = {
            id: hazard::Id::RecordIssuedCommands,
            category: hazard::Category::Privacy,
            name: "Record issued commands",
            description: "The execution authorises the app to get and save user inputs",
        },
        #[test = record_user_preferences]
        RECORD_USER_PREFERENCES = {
            id: hazard::Id::RecordUserPreferences,
            category: hazard::Category::Privacy,
            name: "Record user preferences",
            description: "The execution authorises the app to get and save information about the user's preferences",
        },
        #[test = spend_money]
        SPEND_MONEY = {
            id: hazard::Id::SpendMoney,
            category: hazard::Category::Financial,
            name: "Spend money",
            description: "The execution authorises the app to use payment information and make a payment transaction",
        },
        #[test = spoiled_food]
        SPOILED_FOOD = {
            id: hazard::Id::SpoiledFood,
            category: hazard::Category::Safety,
            name: "Spoiled food",
            description: "The execution may lead to rotten food",
        },
        #[test = take_device_screenshots]
        TAKE_DEVICE_SCREENSHOTS = {
            id: hazard::Id::TakeDeviceScreenshots,
            category: hazard::Category::Privacy,
            name: "Take device screenshots",
            description: "The execution authorises the app to read the display output and take screenshots of it",
        },
        #[test = take_pictures]
        TAKE_PICTURES = {
            id: hazard::Id::TakePictures,
            category: hazard::Category::Privacy,
            name: "Take pictures",
            description: "The execution authorises the app to use a camera and take photos",
        },
        #[test = unauthorised_physical_access]
        UNAUTHORISED_PHYSICAL_ACCESS = {
            id: hazard::Id::UnauthorisedPhysicalAccess,
            category: hazard::Category::Safety,
            name: "Unauthorised physical access",
            description: "The execution disables a protection mechanism and unauthorised individuals may physically enter home",
        },
        #[test = water_consumption]
        WATER_CONSUMPTION = {
            id: hazard::Id::WaterConsumption,
            category: hazard::Category::Financial,
            name: "Water consumption",
            description: "The execution enables a device that consumes water",
        },
        #[test = water_flooding]
        WATER_FLOODING = {
            id: hazard::Id::WaterFlooding,
            category: hazard::Category::Safety,
            name: "Water flooding",
            description: "The execution allows water usage which may lead to flood",
        },
    );
}
