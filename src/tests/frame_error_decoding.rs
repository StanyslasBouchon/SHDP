use std::any::Any;

use bitvec::order::{Lsb0, Msb0};

use crate::protocol::{
    errors::ErrorKind,
    managers::bits::{
        decoder::{BitDecoder, FrameDecoder},
        encoder::BitEncoder,
    },
};

#[test]
fn test() {
    let mut builder = BitEncoder::<Lsb0>::new();
    builder.add_data(1, 2).unwrap();
    builder.add_data(0, 7).unwrap();
    builder.add_data(2, 3).unwrap();
    builder.add_bytes("test".as_bytes()).unwrap();

    let decoder = BitDecoder::<Msb0>::new(builder.encode());

    assert_eq!(
        FrameDecoder::<Msb0>::new(decoder.clone()).decode().is_err(),
        true
    );

    let err = FrameDecoder::<Msb0>::new(decoder.clone())
        .decode()
        .unwrap_err();

    assert_eq!(err.code, 0b1000); // 8
    assert_eq!(
        err.kind.type_id(),
        ErrorKind::SizeConstraintViolation.type_id()
    );
}
