use super::{VarInt, VarLong};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Default, PartialEq)]
pub enum Slot {
    #[default]
    Nothing,
    Item {
        id: VarInt,
        count: i8,
        nbt: Option<nbt::Blob>,
    },
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Identifier(pub String);

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Chat(pub String);

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Angle(pub f32);

#[derive(Clone, Default, PartialEq)]
pub struct Nbt(pub nbt::Blob);

pub type BitSet = ();
pub type CommandNode = ();
pub type Statistic = ();
pub type EntityMetadata = ();
pub type EntityProperty = ();
pub type Recipe = ();
pub type Tag = ();
pub type BossBarAction = ();

/*
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct BitSet {}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct CommandNode {}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Statistic {}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct EntityMetadata {
    // complicated.
}

#[derive(Clone, Default, PartialEq)]
pub struct EntityProperty {
    key: Identifier,
    value: f64,
    modifier_count: VarInt,
    modifiers: Vec<()>, // TODO
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Recipe {
    // complicated
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Tag {
    // complicated
}

#[derive(Clone, Default, PartialEq)]
pub enum BossBarAction {
    Add {
        title: String,
        health: f32,
        color: VarInt,
        division: VarInt,
        flags: u8,
    },
    #[default]
    Remove,
    UpdateHealth {
        health: f32,
    },
    UpdateTitle {
        title: String,
    },
    UpdateStyle {
        color: VarInt,
        dividers: VarInt,
    },
    UpdateFlags {
        flags: u8,
    },
}
 */
