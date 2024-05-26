use bitvec::order::{BitOrder, Lsb0, Msb0};

///
/// A trait for bit orders that can be reversed.
///
pub trait BitReversible: BitOrder {
    /// The opposite bit order.
    type Opposite: BitReversible<Opposite = Self>;
}

impl BitReversible for Msb0 {
    type Opposite = Lsb0;
}

impl BitReversible for Lsb0 {
    type Opposite = Msb0;
}
