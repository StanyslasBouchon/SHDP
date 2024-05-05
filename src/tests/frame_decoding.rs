use bitvec::order::{Lsb0, Msb0};

use crate::protocol::managers::bits::{
    decoder::{BitDecoder, FrameDecoder},
    encoder::BitEncoder,
};

#[test]
fn test() {
    let mut builder = BitEncoder::<Lsb0>::new();
    builder.add_data(1, 8).unwrap();
    builder.add_data(0, 16).unwrap();
    builder.add_data(32, 32).unwrap();
    builder.add_bytes("test".as_bytes()).unwrap();

    let decoder = BitDecoder::<Msb0>::new(builder.encode());
    let data = FrameDecoder::<Msb0>::new(decoder.clone()).decode().unwrap();

    assert_eq!(data.version, 1);
    assert_eq!(data.event, 0);
    assert_eq!(data.data_size, 32);
}
