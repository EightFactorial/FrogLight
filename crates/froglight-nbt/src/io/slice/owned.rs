use super::{
    NamedNbtRef, NbtCompoundRef, NbtListTagRef, NbtListTagRefData, NbtTagRef, NbtTagRefData,
    UnnamedNbtRef,
};
use crate::{
    mutf8::Mutf8Str,
    nbt::{
        ByteArray, DoubleArray, FloatArray, IntArray, LongArray, NamedNbt, NbtCompound, NbtListTag,
        NbtTag, ShortArray, UnnamedNbt,
    },
};

impl NamedNbtRef<'_> {
    /// Convert a [`NamedNbtRef`] into an [`NamedNbt`].
    #[must_use]
    pub fn as_owned(&self) -> NamedNbt {
        match (self.name(), self.compound()) {
            (Some(n), Some(c)) => NamedNbt::new(n.to_mutf8_string(), c.as_owned()),
            (None, None) => NamedNbt::new_empty(),
            _ => unreachable!("Both the name and compound are stored in the same `Option`"),
        }
    }
}
impl From<NamedNbtRef<'_>> for NamedNbt {
    fn from(nbt: NamedNbtRef) -> Self { nbt.as_owned() }
}

impl UnnamedNbtRef<'_> {
    /// Convert a [`UnnamedNbtRef`] into an [`UnnamedNbt`].
    #[must_use]
    pub fn as_owned(&self) -> UnnamedNbt {
        match self.compound() {
            Some(c) => UnnamedNbt::new(c.as_owned()),
            None => UnnamedNbt::new_empty(),
        }
    }
}
impl From<UnnamedNbtRef<'_>> for UnnamedNbt {
    fn from(nbt: UnnamedNbtRef) -> Self { nbt.as_owned() }
}

impl NbtCompoundRef<'_> {
    /// Convert a [`NbtCompoundRef`] into an [`NbtCompound`].
    #[must_use]
    pub fn as_owned(&self) -> NbtCompound { self.iter().collect() }
}
impl From<NbtCompoundRef<'_>> for NbtCompound {
    fn from(compound: NbtCompoundRef) -> Self { compound.as_owned() }
}

impl NbtTagRef<'_> {
    /// Convert a [`NbtTagRef`] into an [`NbtTag`].
    #[must_use]
    pub fn as_owned(&self) -> NbtTag {
        match self.tag_data() {
            NbtTagRefData::Byte(data) => NbtTag::Byte(data),
            NbtTagRefData::Short(data) => NbtTag::Short(data),
            NbtTagRefData::Int(data) => NbtTag::Int(data),
            NbtTagRefData::Long(data) => NbtTag::Long(data),
            NbtTagRefData::Float(data) => NbtTag::Float(data),
            NbtTagRefData::Double(data) => NbtTag::Double(data),
            NbtTagRefData::String(string) => NbtTag::String(string.to_mutf8_string()),
            NbtTagRefData::List(list) => NbtTag::List(list.as_owned()),
            NbtTagRefData::Compound(compound) => NbtTag::Compound(compound.as_owned()),
            NbtTagRefData::ByteArray(array) => NbtTag::ByteArray(ByteArray::from(array.to_vec())),
            NbtTagRefData::IntArray(array) => NbtTag::IntArray(IntArray::from(array.to_vec())),
            NbtTagRefData::LongArray(array) => NbtTag::LongArray(LongArray::from(array.to_vec())),
        }
    }
}
impl From<NbtTagRef<'_>> for NbtTag {
    fn from(tag: NbtTagRef) -> Self { tag.as_owned() }
}

impl NbtListTagRef<'_> {
    /// Convert a [`NbtListTagRef`] into an [`NbtListTag`].
    #[must_use]
    pub fn as_owned(&self) -> NbtListTag {
        match self.list_data() {
            NbtListTagRefData::Empty => NbtListTag::Empty,
            NbtListTagRefData::Byte(list) => NbtListTag::Byte(ByteArray::from(list.to_vec())),
            NbtListTagRefData::Short(list) => NbtListTag::Short(ShortArray::from(list.to_vec())),
            NbtListTagRefData::Int(list) => NbtListTag::Int(IntArray::from(list.to_vec())),
            NbtListTagRefData::Long(list) => NbtListTag::Long(LongArray::from(list.to_vec())),
            NbtListTagRefData::Float(list) => NbtListTag::Float(FloatArray::from(list.to_vec())),
            NbtListTagRefData::Double(list) => NbtListTag::Double(DoubleArray::from(list.to_vec())),
            NbtListTagRefData::String(list) => {
                NbtListTag::String(list.into_iter().map(Mutf8Str::to_mutf8_string).collect())
            }
            NbtListTagRefData::List(list) => {
                NbtListTag::List(list.into_iter().map(|list| list.as_owned()).collect())
            }
            NbtListTagRefData::Compound(list) => {
                NbtListTag::Compound(list.into_iter().map(|comp| comp.as_owned()).collect())
            }
            NbtListTagRefData::ByteArray(list) => NbtListTag::ByteArray(
                list.into_iter().map(|byte| ByteArray::from(byte.to_vec())).collect(),
            ),
            NbtListTagRefData::IntArray(list) => NbtListTag::IntArray(
                list.into_iter().map(|int| IntArray::from(int.to_vec())).collect(),
            ),
            NbtListTagRefData::LongArray(list) => NbtListTag::LongArray(
                list.into_iter().map(|long| LongArray::from(long.to_vec())).collect(),
            ),
        }
    }
}
impl From<NbtListTagRef<'_>> for NbtListTag {
    fn from(compound: NbtListTagRef) -> Self { compound.as_owned() }
}
