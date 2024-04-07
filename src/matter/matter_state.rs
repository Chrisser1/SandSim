use bitflags::bitflags;

/// Matter state defines how matter moves
#[repr(u32)]
pub enum MatterState {
    Empty = 0,
    Powder = 1,
    Liquid = 2,
    Solid = 4,
    SolidGravity = 8,
    Gas = 16,
    Energy = 32,
    Object = 64,
}

bitflags! {
    /// Reaction cause defines whether a matter causes a reaction
    pub struct Direction: u32 {
        const UP_LEFT = 1 << 0;
        const UP = 1 << 1;
        const UP_RIGHT = 1 << 2;
        const RIGHT = 1 << 3;
        const DOWN_RIGHT = 1 << 4;
        const DOWN = 1 << 5;
        const DOWN_LEFT = 1 << 6;
        const LEFT = 1 << 7;
        const ALL = 0b11111111;
        const NONE = 0;
    }
}

pub const ALL_DIRECTIONS: [(Direction, &str); 8] = [
    (Direction::UP_LEFT, "Up Left"),
    (Direction::UP, "Up"),
    (Direction::UP_RIGHT, "Up Right"),
    (Direction::RIGHT, "Right"),
    (Direction::DOWN_RIGHT, "Down Right"),
    (Direction::DOWN, "Down"),
    (Direction::DOWN_LEFT, "Down Left"),
    (Direction::LEFT, "Left"),
];

bitflags! {
    /// Reaction cause defines whether a matter causes a reaction
    pub struct MatterCharacteristic: u32 {
        /// A material that is corrosive
        const CORROSIVE = 1 << 0;
        /// A material that reacts to corrosive
        const CORRODES = 1 << 1;

        /// A material that can melt others
        const MELTING = 1 << 2;
        /// A material that is melted by melting
        const MELTS = 1 << 3;

        /// A material that burns others
        const BURNING = 1 << 4;
        /// A material that is burnt by burning
        const BURNS = 1 << 5;

        /// A material that freezes others
        const FREEZING = 1 << 6;
        /// A material that is frozen by freezing
        const FREEZES = 1 << 7;

        /// A material that explodes others
        const EXPLODING = 1 << 8;
        /// A material that is exploded by exploding
        const EXPLODES = 1 << 9;

        /// A material that electrifiecs others
        const ELECTRIFIES = 1 << 10;
        /// A material that can become electric by electrifies
        const CONDUCTS = 1 << 11;

        /// A material that cools others
        const COOLING = 1 << 12;
        /// A material that is cooled by cooling
        const COOLS = 1 << 13;

        /// A material that rusts others
        const RUSTING = 1 << 14;
        /// A material that can turn to rust by rusting
        const RUSTS = 1 << 15;

        /// A material that can vaporize for whatever reason
        const VAPORIZES = 1 << 16;
        /// Eraser
        const ERASER = 1 << 17;
    }
}

pub const ALL_CHARACTERISTICS: [(MatterCharacteristic, &str, &str); 18] = [
    (
        MatterCharacteristic::CORROSIVE,
        "Corrosive",
        "Matter is like acid (destroys other matter)",
    ),
    (
        MatterCharacteristic::CORRODES,
        "Corrodes",
        "Matter is corroded by other matter",
    ),
    (
        MatterCharacteristic::MELTING,
        "Melting",
        "Matter can melt others",
    ),
    (
        MatterCharacteristic::MELTS,
        "Melts",
        "Matter melts by melting matters",
    ),
    (
        MatterCharacteristic::BURNING,
        "Burning",
        "Matter burns others",
    ),
    (
        MatterCharacteristic::BURNS,
        "Burns",
        "Matter burns by burning matters",
    ),
    (
        MatterCharacteristic::FREEZING,
        "Freezing",
        "Matter freezes others",
    ),
    (
        MatterCharacteristic::FREEZES,
        "Freezes",
        "Matter is freezed by freezing matter",
    ),
    (
        MatterCharacteristic::EXPLODING,
        "Exploding",
        "Matter explodes others",
    ),
    (
        MatterCharacteristic::EXPLODES,
        "Explodes",
        "Matter explodes by exploding matters",
    ),
    (
        MatterCharacteristic::ELECTRIFIES,
        "Electrifies",
        "Matter electrifies others",
    ),
    (
        MatterCharacteristic::CONDUCTS,
        "Conducts",
        "Matter conducts electricity (on touch with electrifies)",
    ),
    (
        MatterCharacteristic::COOLING,
        "Cooling",
        "Matter cools others",
    ),
    (
        MatterCharacteristic::COOLS,
        "Cools",
        "Matter becomes cooled by cooling matter",
    ),
    (
        MatterCharacteristic::RUSTING,
        "Rusting",
        "Matter rusts others",
    ),
    (
        MatterCharacteristic::RUSTS,
        "Rusts",
        "Matter might become rusty by rusting",
    ),
    (
        MatterCharacteristic::VAPORIZES,
        "Vaporizes",
        "Matter is vaporizes others",
    ),
    (
        MatterCharacteristic::ERASER,
        "Eraser",
        "Matter erases others",
    ),
];