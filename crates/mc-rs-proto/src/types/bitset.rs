use crate::buffer::{Decode, Encode};

/// A set of bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitSet<const N: usize>
where
    [(); N.div_ceil(8)]: Sized,
{
    data: [u8; N.div_ceil(8)],
}

impl<const N: usize> Default for BitSet<N>
where
    [(); N.div_ceil(8)]: Sized,
{
    fn default() -> Self {
        Self {
            data: [0; N.div_ceil(8)],
        }
    }
}

impl<const N: usize> BitSet<N>
where
    [(); N.div_ceil(8)]: Sized,
{
    pub fn new() -> Self { Self::default() }

    pub fn index(&self, index: usize) -> bool { (self.data[index / 8] & (1u8 << (index % 8))) != 0 }

    pub fn set_bit(&mut self, index: usize, value: bool) {
        if value {
            self.data[index / 8] |= 1u8 << (index % 8);
        } else {
            self.data[index / 8] &= !(1u8 << (index % 8));
        }
    }

    pub fn set(&mut self, index: usize) { self.set_bit(index, true); }

    pub fn clear(&mut self, index: usize) { self.set_bit(index, false); }
}

impl<const N: usize> Encode for BitSet<N>
where
    [(); N.div_ceil(8)]: Sized,
{
    fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
        buf.write_all(&self.data)?;
        Ok(())
    }
}

impl<const N: usize> Decode for BitSet<N>
where
    [(); N.div_ceil(8)]: Sized,
{
    fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
        let mut data = [0; N.div_ceil(8)];
        buf.read_exact(&mut data)?;
        Ok(Self { data })
    }
}

#[test]
fn test_bitset() {
    let mut bitset = BitSet::<2>::new();
    assert!(!bitset.index(0));
    assert!(!bitset.index(1));

    bitset.set(0);
    assert!(bitset.index(0));
    assert!(!bitset.index(1));

    bitset.set(1);
    assert!(bitset.index(0));
    assert!(bitset.index(1));

    bitset.clear(0);
    bitset.clear(1);
    assert!(!bitset.index(0));
    assert!(!bitset.index(1));
}

#[test]
fn test_bitset_encode() {
    let mut buf = Vec::new();

    let mut bitset = BitSet::<8>::new();
    bitset.set(0);
    bitset.set(1);

    assert!(bitset.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0b0000_0011]);

    buf.clear();
    bitset.clear(0);
    bitset.clear(1);

    assert!(bitset.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0b0000_0000]);

    buf.clear();
    bitset.set(6);
    bitset.set(7);

    assert!(bitset.encode(&mut buf).is_ok());
    assert_eq!(buf, vec![0b1100_0000]);
}

#[test]
fn test_bitset_decode() {
    {
        let buf = [0b0000_0011];
        let mut cursor = std::io::Cursor::new(buf);

        let bitset = BitSet::<8>::decode(&mut cursor).unwrap();
        assert!(bitset.index(0));
        assert!(bitset.index(1));
        assert!(!bitset.index(2));
        assert!(!bitset.index(3));
        assert!(!bitset.index(4));
        assert!(!bitset.index(5));
        assert!(!bitset.index(6));
        assert!(!bitset.index(7));
    }

    {
        let buf = [0b1100_0000];
        let mut cursor = std::io::Cursor::new(buf);

        let bitset = BitSet::<8>::decode(&mut cursor).unwrap();
        assert!(!bitset.index(0));
        assert!(!bitset.index(1));
        assert!(!bitset.index(2));
        assert!(!bitset.index(3));
        assert!(!bitset.index(4));
        assert!(!bitset.index(5));
        assert!(bitset.index(6));
        assert!(bitset.index(7));
    }
}
