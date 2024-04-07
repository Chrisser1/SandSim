use create::matter::{
    Direction, MatterCharacteristic, MatterDefinition, MatterDefinitions, MatterReaction,
    MatterState,
};

pub const MATTER_EMPTY: u32 = 0;
pub const MATTER_SAND: u32 = 1;
pub const MATTER_WATER: u32 = 2;
pub const MATTER_ROCK: u32 = 3;

pub fn default_matter_definitions() -> MatterDefinitions {
    MatterDefinitions {
        empty: MATTER_EMPTY,
        definitions: vec![
            MatterDefinition {
                id: MATTER_EMPTY,
                name: "Empty".to_string(),
                color: 0x0,
                weight: 0.0,
                state: MatterState::Empty,
                dispersion: 0,
                characteristics: MatterCharacteristic::empty(),
                reactions: [
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                ],
            },
            MatterDefinition {
                id: MATTER_SAND,
                name: "Sand".to_string(),
                color: 0xc2b280ff,
                weight: 1.5,
                state: MatterState::Powder,
                dispersion: 0,
                characteristics: (MatterCharacteristic::MELTS | MatterCharacteristic::CORRODES),
                reactions: [
                    MatterReaction {
                        reacts: MatterCharacteristic::MELTING,
                        direction: Direction::ALL,
                        probability: 0.6,
                        becomes: MATTER_GLASS,
                    },
                    MatterReaction {
                        reacts: MatterCharacteristic::CORROSIVE,
                        direction: Direction::ALL,
                        probability: 0.05,
                        becomes: MATTER_EMPTY,
                    },
                    MatterReaction::becomes_on_touch(
                        1.0,
                        MatterCharacteristic::ERASER,
                        MATTER_EMPTY,
                    ),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                ],
            },
            MatterDefinition {
                id: MATTER_WATER,
                name: "Water".to_string(),
                color: 0x1ca3ecff,
                weight: 1.0,
                state: MatterState::Liquid,
                dispersion: 10,
                characteristics: (MatterCharacteristic::RUSTING
                    | MatterCharacteristic::COOLING
                    | MatterCharacteristic::FREEZES
                    | MatterCharacteristic::VAPORIZES),
                reactions: [
                    MatterReaction {
                        reacts: (MatterCharacteristic::MELTING
                            | MatterCharacteristic::BURNING
                            | MatterCharacteristic::CORROSIVE),
                        direction: Direction::ALL,
                        probability: 0.6,
                        becomes: MATTER_STEAM,
                    },
                    MatterReaction {
                        reacts: (MatterCharacteristic::FREEZING),
                        direction: Direction::ALL,
                        probability: 0.005,
                        becomes: MATTER_ICE,
                    },
                    MatterReaction::becomes_on_touch(
                        1.0,
                        MatterCharacteristic::ERASER,
                        MATTER_EMPTY,
                    ),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                ],
            },
            MatterDefinition {
                id: MATTER_ROCK,
                name: "Rock".to_string(),
                color: 0x87898eff,
                weight: 2.5,
                state: MatterState::SolidGravity,
                dispersion: 0,
                characteristics: (MatterCharacteristic::CORRODES),
                reactions: [
                    MatterReaction {
                        reacts: (MatterCharacteristic::CORROSIVE),
                        direction: Direction::ALL,
                        probability: 0.05,
                        becomes: MATTER_EMPTY,
                    },
                    MatterReaction::becomes_on_touch(
                        1.0,
                        MatterCharacteristic::ERASER,
                        MATTER_EMPTY,
                    ),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                    MatterReaction::zero(),
                ],
            },
        ],
    }
}