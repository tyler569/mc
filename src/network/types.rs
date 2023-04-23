use super::{VarInt, VarLong};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub enum Slot {
    #[default]
    Nothing,
    Item {
        id: VarInt,
        count: i8,
        nbt: Option<nbt::Blob>,
    },
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Identifier(pub String);

impl PartialEq<&'_ str> for Identifier {
    fn eq(&self, other: &&'_ str) -> bool {
        &self.0 == other
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Chat(pub String);

impl PartialEq<&'_ str> for Chat {
    fn eq(&self, other: &&'_ str) -> bool {
        &self.0 == other
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Angle(pub f32);

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Nbt(pub nbt::Blob);

pub type BitSet = ();
pub type CommandNode = ();
pub type Statistic = ();
pub type EntityMetadata = ();
pub type EntityProperty = ();
pub type Recipe = ();
pub type Tag = ();
pub type BossBarAction = ();

pub trait Index {
    fn into_index(self) -> usize;
    fn to_value(index: usize) -> Self;
}

impl Index for VarInt {
    fn into_index(self) -> usize {
        self.0 as usize
    }

    fn to_value(index: usize) -> Self {
        Self(index as i32)
    }
}

#[derive(Clone, Debug, Default)]
pub struct LengthPrefixedArray<T: Index, U> {
    pub value: Vec<U>,
    _index: PhantomData<T>,
}

impl<T: Index, U> LengthPrefixedArray<T, U> {
    pub fn from_vec(value: Vec<U>) -> Self {
        Self {
            value,
            _index: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LengthPrefixedByteArray<T: Index> {
    pub value: Vec<u8>,
    _index: PhantomData<T>,
}

impl<T: Index> LengthPrefixedByteArray<T> {
    pub fn from_vec(value: Vec<u8>) -> Self {
        Self {
            value,
            _index: PhantomData,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ByteArray(pub Vec<u8>);

impl ByteArray {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Deref for ByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/*
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct BitSet {}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct CommandNode {}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Statistic {}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct EntityMetadata {
    // complicated.
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct EntityProperty {
    key: Identifier,
    value: f64,
    modifier_count: VarInt,
    modifiers: Vec<()>, // TODO
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Recipe {
    // complicated
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct Tag {
    // complicated
}

#[derive(Clone, Default, Debug, PartialEq)]
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
