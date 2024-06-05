use std::collections::HashMap;

use bitvec::order::Msb0;
use lazy_static::lazy_static;

use crate::protocol::prelude::common::{ bits::BitDecoder, error::Error };

lazy_static! {
    /// A map of characters to bit vectors.
    /// Every common programming character is mapped to a bit vector of five-bit chains, known as fyve.
    pub static ref CHARS: HashMap<u32, char> = {
        let mut map = HashMap::new();
        map.insert(1, ' ');
        map.insert(2, 't');
        map.insert(3, 'a');
        map.insert(4, 'e');
        map.insert(5, 'i');
        map.insert(6, 'n');
        map.insert(7, 'o');
        map.insert(8, 'r');
        map.insert(9, 's');
        map.insert(10, 'd');
        map.insert(11, 'l');
        map.insert(12, '-');
        map.insert(13, '"');
        map.insert(14, 'c');
        map.insert(15, 'p');
        map.insert(16, 'f');
        map.insert(17, '>');
        map.insert(18, '=');
        map.insert(19, '.');
        map.insert(20, 'v');
        map.insert(21, '<');
        map.insert(22, 'u');
        map.insert(23, 'm');
        map.insert(24, ';');
        map.insert(25, 'g');
        map.insert(26, ':');
        map.insert(27, '/');
        map.insert(28, 'h');
        map.insert(29, 'y');
        map.insert(30, 'x');
        map.insert(993,
            'b',
        );
        map.insert(994,
            'k',
        );
        map.insert(995,
            ')',
        );
        map.insert(996,
            '(',
        );
        map.insert(997,
            'w',
        );
        map.insert(998,
            'E',
        );
        map.insert(999,
            '#',
        );
        map.insert(1000,
            '}',
        );
        map.insert(1001,
            '{',
        );
        map.insert(1002,
            '0',
        );
        map.insert(1003,
            'N',
        );
        map.insert(1004,
            'A',
        );
        map.insert(1005,
            '2',
        );
        map.insert(1006,
            'R',
        );
        map.insert(1007,
            '1',
        );
        map.insert(1008,
            'T',
        );
        map.insert(1009,
            'D',
        );
        map.insert(1010,
            'O',
        );
        map.insert(1011,
            'I',
        );
        map.insert(1012,
            'S',
        );
        map.insert(1013,
            '_',
        );
        map.insert(1014,
            'P',
        );
        map.insert(1015,
            'L',
        );
        map.insert(1016,
            '6',
        );
        map.insert(1017,
            '4',
        );
        map.insert(1018,
            ',',
        );
        map.insert(1019,
            'z',
        );
        map.insert(1020,
            'M',
        );
        map.insert(1021,
            'C',
        );
        map.insert(1022,
            'B',
        );
        map.insert(32737,
            'G',
        );
        map.insert(32738,
            '%',
        );
        map.insert(32739,
            'j',
        );
        map.insert(32740,
            '3',
        );
        map.insert(32741,
            'U',
        );
        map.insert(32742,
            '8',
        );
        map.insert(32743,
            '*',
        );
        map.insert(32744,
            '5',
        );
        map.insert(32745,
            '9',
        );
        map.insert(32746,
            '+',
        );
        map.insert(32747,
            'F',
        );
        map.insert(32748,
            '|',
        );
        map.insert(32749,
            'W',
        );
        map.insert(32750,
            'V',
        );
        map.insert(32751,
            '@',
        );
        map.insert(32752,
            'q',
        );
        map.insert(32753,
            '\'',
        );
        map.insert(32754,
            'Q',
        );
        map.insert(32755,
            'H',
        );
        map.insert(32756,
            '!',
        );
        map.insert(32757,
            ']',
        );
        map.insert(32758,
            '[',
        );
        map.insert(32759,
            '7',
        );
        map.insert(32760,
            'Z',
        );
        map.insert(32761,
            'Y',
        );
        map.insert(32762,
            'X',
        );
        map.insert(32763,
            'J',
        );
        map.insert(32764,
            '^',
        );
        map.insert(32765,
            'K',
        );
        map.insert(32766,
            '?',
        );
        map.insert(1048545,
            '$',
        );
        map.insert(1048546,
            '\\',
        );
        map.insert(1048547,
            '~',
        );
        map.insert(1048548,
            '`',
        );
        map.insert(1048549,
            '&',
        );
        map
    };
}

#[derive(PartialEq, Eq, Debug)]
pub enum OperatingCode {
    System = 0x0,
    Character = 0x1f,
}

impl OperatingCode {
    fn from(fy: &u8) -> OperatingCode {
        match fy {
            0x0 => OperatingCode::System,
            0x1f => OperatingCode::Character,
            _ => OperatingCode::Character,
        }
    }
}

impl OperationCode {
    fn from(fy: &u8) -> OperationCode {
        match fy {
            0x00 => OperationCode::Utf8Chain,
            0x10 => OperationCode::StartOfTag,
            0x11 => OperationCode::StartOfAttributes,
            0x18 => OperationCode::StartOfData,
            0x19 => OperationCode::EndOfData,
            _ => OperationCode::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    pub kind: OperatingCode,
    pub code: Option<OperationCode>,
    pub value: Vec<u8>,
}

impl Operation {
    fn from(
        fy: &u8,
        decoder: &mut BitDecoder<Msb0>
    ) -> Result<Operation, Error> {
        let mut op = OperatingCode::from(fy);
        let mut code: Option<OperationCode> = None;

        let mut value = Vec::<u8>::new();
        value.push(*fy);

        match op {
            OperatingCode::System => {
                let operator = decoder.read_data(5)? as u8;
                code = Some(OperationCode::from(&operator));
                value.push(operator);
            }
            _ => {
                op = OperatingCode::Character;
                if *fy == 0x1f {
                    let mut next_fyve = decoder.read_data(5)? as u8;
                    value.push(next_fyve.clone());

                    while next_fyve == 0x1f {
                        next_fyve = decoder.read_data(5)? as u8;
                        value.push(next_fyve.clone());
                    }
                }
            }
        }

        Ok(Operation {
            kind: op,
            code,
            value,
        })
    }

    pub fn get_char(&self) -> Result<char, Error> {
        let mut values = self.value.clone();
        values.reverse();
        let value = values
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, &val)| acc | ((val as u32) << (i * 5)));

        match CHARS.get(&value) {
            Some(value) => Ok(value.to_owned()),
            None => {
                return Err(Error {
                    code: 400,
                    message: "Invalid character".to_string(),
                    kind: crate::protocol::prelude::common::error::ErrorKind::BadRequest,
                });
            }
        }
    }
}

#[derive(Debug)]
pub enum OperationCode {
    Utf8Chain = 0x00,
    StartOfTag = 0x10,
    StartOfAttributes = 0x11,
    StartOfData = 0x18,
    EndOfData = 0x19,
    Unknown = 0xff,
}

///
/// Permits fyve management.
///
pub struct FyveImpl {}

impl FyveImpl {
    /// Get the next operator from the bit vector.
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Msb0>`] to decode the request.
    ///
    pub fn get_op(decoder: &mut BitDecoder<Msb0>) -> Result<Operation, Error> {
        Operation::from(&Self::read_fyve(decoder)?, decoder)
    }

    ///
    /// Read the next fyve from the bit vector.
    ///
    /// # Arguments
    /// * `decoder` - The [`BitDecoder<Lsb0>`] to decode the request.
    ///
    /// # Returns
    /// * `u8` - The next fyve.
    ///
    pub fn read_fyve(decoder: &mut BitDecoder<Msb0>) -> Result<u8, Error> {
        Ok(decoder.read_data(5)? as u8)
    }
}
