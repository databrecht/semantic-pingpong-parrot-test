const BASE_SPEED: f32 = 12.0;
const COCONUT_LOAD_FACTOR: f32 = 9.0;
const MAX_VOLTAGE_SPEED: f32 = 24.0;

/// A finite positive voltage that marks a Norwegian Blue as powered.
#[derive(Clone, Copy)]
struct OperatingVoltage(f32);

#[derive(Debug, PartialEq)]
enum InvalidOperatingVoltage {
    NotFinite,
    NotPositive,
}

impl OperatingVoltage {
    fn try_new(volts: f32) -> Result<Self, InvalidOperatingVoltage> {
        if !volts.is_finite() {
            return Err(InvalidOperatingVoltage::NotFinite);
        }
        if volts <= 0.0 {
            return Err(InvalidOperatingVoltage::NotPositive);
        }
        Ok(Self(volts))
    }

    fn volts(self) -> f32 {
        self.0
    }
}

enum NorwegianBluePower {
    Unpowered,
    Powered { voltage: OperatingVoltage },
}

enum NorwegianBlueFlightCondition {
    Nailed,
    Free,
}

/// Calculates one Norwegian Blue's speed and cry from its power and whether it is nailed.
/// `Parrot` selects the species; this value does not define other species' rules.
struct NorwegianBlueParrot {
    power: NorwegianBluePower,
    flight_condition: NorwegianBlueFlightCondition,
}

impl NorwegianBlueParrot {
    fn speed(&self) -> f32 {
        match (&self.power, &self.flight_condition) {
            (_, NorwegianBlueFlightCondition::Nailed) => 0.0,
            (NorwegianBluePower::Unpowered, NorwegianBlueFlightCondition::Free) => 0.0,
            (NorwegianBluePower::Powered { voltage }, NorwegianBlueFlightCondition::Free) => {
                Self::speed_for_voltage(*voltage)
            }
        }
    }

    fn cry(&self) -> &'static str {
        match self.power {
            NorwegianBluePower::Powered { .. } => "Bzzzzzz",
            NorwegianBluePower::Unpowered => "...",
        }
    }

    fn speed_for_voltage(voltage: OperatingVoltage) -> f32 {
        (voltage.volts() * BASE_SPEED).min(MAX_VOLTAGE_SPEED)
    }
}

enum Parrot {
    European,
    African { coconut_count: usize },
    NorwegianBlue(NorwegianBlueParrot),
}

impl Parrot {
    fn speed(&self) -> f32 {
        match self {
            Parrot::European => BASE_SPEED,
            Parrot::African { coconut_count } => {
                (BASE_SPEED - COCONUT_LOAD_FACTOR * *coconut_count as f32).max(0.0)
            }
            Parrot::NorwegianBlue(parrot) => parrot.speed(),
        }
    }

    fn cry(&self) -> &'static str {
        match self {
            Parrot::European => "Sqoork!",
            Parrot::African { .. } => "Sqaark!",
            Parrot::NorwegianBlue(parrot) => parrot.cry(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operating_voltage(volts: f32) -> OperatingVoltage {
        OperatingVoltage::try_new(volts).expect("test voltage should be finite and positive")
    }

    #[test]
    fn operating_voltage_rejects_nonpositive_and_nonfinite_values() {
        for volts in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(OperatingVoltage::try_new(volts).is_err());
        }
    }

    #[test]
    fn european_parrot_speed() {
        let parrot = Parrot::European;

        assert_eq!(parrot.speed(), 12.0);
    }

    #[test]
    fn african_parrot_speed_with_one_coconut() {
        let parrot = Parrot::African { coconut_count: 1 };

        assert_eq!(parrot.speed(), 3.0);
    }

    #[test]
    fn african_parrot_speed_with_two_coconuts() {
        let parrot = Parrot::African { coconut_count: 2 };

        assert_eq!(parrot.speed(), 0.0);
    }

    #[test]
    fn african_parrot_speed_with_no_coconuts() {
        let parrot = Parrot::African { coconut_count: 0 };

        assert_eq!(parrot.speed(), 12.0);
    }

    #[test]
    fn nailed_norwegian_blue_parrot_speed() {
        let parrot = Parrot::NorwegianBlue(NorwegianBlueParrot {
            power: NorwegianBluePower::Powered {
                voltage: operating_voltage(1.5),
            },
            flight_condition: NorwegianBlueFlightCondition::Nailed,
        });

        assert_eq!(parrot.speed(), 0.0);
    }

    #[test]
    fn free_norwegian_blue_parrot_speed() {
        let parrot = Parrot::NorwegianBlue(NorwegianBlueParrot {
            power: NorwegianBluePower::Powered {
                voltage: operating_voltage(1.5),
            },
            flight_condition: NorwegianBlueFlightCondition::Free,
        });

        assert_eq!(parrot.speed(), 18.0);
    }

    #[test]
    fn free_norwegian_blue_parrot_speed_is_capped() {
        let parrot = Parrot::NorwegianBlue(NorwegianBlueParrot {
            power: NorwegianBluePower::Powered {
                voltage: operating_voltage(4.0),
            },
            flight_condition: NorwegianBlueFlightCondition::Free,
        });

        assert_eq!(parrot.speed(), 24.0);
    }

    #[test]
    fn european_parrot_cry() {
        let parrot = Parrot::European;

        assert_eq!(parrot.cry(), "Sqoork!");
    }

    #[test]
    fn african_parrot_cry() {
        let parrot = Parrot::African { coconut_count: 0 };

        assert_eq!(parrot.cry(), "Sqaark!");
    }

    #[test]
    fn powered_norwegian_blue_parrot_cry() {
        let parrot = Parrot::NorwegianBlue(NorwegianBlueParrot {
            power: NorwegianBluePower::Powered {
                voltage: operating_voltage(4.0),
            },
            flight_condition: NorwegianBlueFlightCondition::Free,
        });

        assert_eq!(parrot.cry(), "Bzzzzzz");
    }

    #[test]
    fn unpowered_norwegian_blue_parrot_cry() {
        let parrot = Parrot::NorwegianBlue(NorwegianBlueParrot {
            power: NorwegianBluePower::Unpowered,
            flight_condition: NorwegianBlueFlightCondition::Free,
        });

        assert_eq!(parrot.cry(), "...");
    }
}
