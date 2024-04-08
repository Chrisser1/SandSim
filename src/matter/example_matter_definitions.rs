use crate::matter::{
    Direction, 
    MatterCharacteristic, 
    MatterDefinition, 
    MatterReaction, 
    MatterState,
    MatterId,
};

pub const MATTER_EMPTY: MatterDefinition = MatterDefinition {
    id: MatterId::Empty,
    color: 0x0,
    weight: 0.0,
    state: MatterState::Empty,
    characteristics: MatterCharacteristic::empty(),
    reactions: [
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
    ],
};

const SAND_CHARACTERISTICS: MatterCharacteristic = MatterCharacteristic::from_bits_truncate(
    MatterCharacteristic::MELTS.bits() 
    | MatterCharacteristic::CORRODES.bits()
);

pub const MATTER_SAND: MatterDefinition = MatterDefinition {
    id: MatterId::Sand,
    color: 0xc2b280ff,
    weight: 1.5,
    state: MatterState::Powder,
    characteristics: SAND_CHARACTERISTICS,
    reactions: [
        MatterReaction {
            reacts: MatterCharacteristic::CORROSIVE,
            direction: Direction::ALL,
            probability: 0.05,
            becomes: MatterId::Empty,
        },
        MatterReaction::becomes_on_touch(
            1.0,
            MatterCharacteristic::ERASER,
            MatterId::Empty,
        ),
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
    ],
};

const WATER_CHARACTERISTICS: MatterCharacteristic = MatterCharacteristic::from_bits_truncate(
    MatterCharacteristic::RUSTING.bits()
    | MatterCharacteristic::COOLING.bits()
    | MatterCharacteristic::FREEZES.bits()
    | MatterCharacteristic::VAPORIZES.bits()
);


pub const MATTER_WATER: MatterDefinition = MatterDefinition {
    id: MatterId::Water,
    color: 0x0f5e9cff,
    weight: 1.0,
    state: MatterState::Liquid,
    characteristics: WATER_CHARACTERISTICS,
    reactions: [
        MatterReaction::becomes_on_touch(
            1.0,
            MatterCharacteristic::ERASER,
            MatterId::Empty,
        ),
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
    ],
};

pub const MATTER_ROCK: MatterDefinition = MatterDefinition {
    id: MatterId::Rock,
    color: 0x787a79ff,
    weight: 2.5,
    state: MatterState::SolidGravity,
    characteristics: (MatterCharacteristic::CORRODES),
    reactions: [
        MatterReaction {
            reacts: (MatterCharacteristic::CORROSIVE),
            direction: Direction::ALL,
            probability: 0.05,
            becomes: MatterId::Empty,
        },
        MatterReaction::becomes_on_touch(
            1.0,
            MatterCharacteristic::ERASER,
            MatterId::Empty,
        ),
        MatterReaction::zero(),
        MatterReaction::zero(),
        MatterReaction::zero(),
    ],
};