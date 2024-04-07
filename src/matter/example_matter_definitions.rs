use create::matter::{
    Direction, MatterCharacteristic, MatterDefinition, MatterDefinitions, MatterReaction,
    MatterState,
};

pub const MATTER_EMPTY: MatterDefinition = MatterDefinition {
    id: 0,
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
};

pub const MATTER_SAND: MatterDefinition = MatterDefinition {
    id: 1,
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
};

pub const MATTER_WATER: MatterDefinition = MatterDefinition {
    id: 2,
    name: "Water".to_string(),
    color: 0x0f5e9cff,
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
};

pub const MATTER_ROCK: MatterDefinition = MatterDefinition {
    id: 3,
    name: "Rock".to_string(),
    color: 0x787a79ff,
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
};