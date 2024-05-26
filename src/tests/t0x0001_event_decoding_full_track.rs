use bitvec::order::{Lsb0, Msb0};

use crate::{
    client::prelude::versions::v1::r0x0001::HtmlTag,
    protocol::{
        prelude::common::{
            bits::{BitDecoder, Frame, FrameDecoder, FrameEncoder},
            event::EventDecoder,
        },
        versions::Version,
    },
};

#[test]
fn test() {
    // First, we create the frame closure to wrap the data.
    let mut encoder: FrameEncoder<Lsb0> = FrameEncoder::<Lsb0>::new(Version::V1.to_u8()).unwrap();

    // Then, we encode the data, and append it to the frame.
    let frame: Vec<u8> = encoder
        .encode(Box::new(
            crate::server::prelude::versions::v1::c0x0001::HtmlFileResponse::new(
                "src/tests/res/test.html".to_string(),
            ),
        ))
        .unwrap();

    // Now, we create the main decoder.
    let mut decoder = BitDecoder::<Msb0>::new(frame);

    // then, the frame decoder to extract the frame data.
    let mut frame_decoder = FrameDecoder::<Msb0>::new(decoder.clone());

    // We then decode the frame closure.
    let data: Frame<Msb0> = frame_decoder.decode().unwrap();

    // We then get the modified decoder from the frame decoder.
    decoder = frame_decoder.get_decoder().to_owned();

    // Then, we create the data wrapper based on the event id and the frame version.
    // Here, the event id and the frame version are not checked because we already know these values due to test purposes.
    let mut decoded_data =
        crate::client::prelude::versions::v1::r0x0001::HtmlFileResponse::new(decoder);

    // We set the parent tag to the html tag.
    decoded_data.parent = HtmlTag::new("html".to_string());

    // Finally, we decode the data.
    decoded_data.decode(data).unwrap();

    println!("{:?}", decoded_data.parent); // FIXME: HtmlTag { name: "htmlp", attributes: {}, data: [Text("Test")] }

    assert_eq!(decoded_data.name, "test.html".to_string());
}
