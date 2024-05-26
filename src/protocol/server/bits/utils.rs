use std::collections::HashMap;

use bitvec::{bitvec, prelude::Lsb0, vec::BitVec};
use lazy_static::lazy_static;

lazy_static! {
    /// A map of characters to bit vectors.
    /// Every common programming character is mapped to a bit vector of five-bit chains, known as fyve.
    pub static ref CHARS: HashMap<char, BitVec<u8>> = {
        let mut map = HashMap::new();
        map.insert(' ', bitvec![u8, Lsb0; 0, 0, 0, 0, 1]);
        map.insert('t', bitvec![u8, Lsb0; 0, 0, 0, 1, 0]);
        map.insert('a', bitvec![u8, Lsb0; 0, 0, 0, 1, 1]);
        map.insert('e', bitvec![u8, Lsb0; 0, 0, 1, 0, 0]);
        map.insert('i', bitvec![u8, Lsb0; 0, 0, 1, 0, 1]);
        map.insert('n', bitvec![u8, Lsb0; 0, 0, 1, 1, 0]);
        map.insert('o', bitvec![u8, Lsb0; 0, 0, 1, 1, 1]);
        map.insert('r', bitvec![u8, Lsb0; 0, 1, 0, 0, 0]);
        map.insert('s', bitvec![u8, Lsb0; 0, 1, 0, 0, 1]);
        map.insert('d', bitvec![u8, Lsb0; 0, 1, 0, 1, 0]);
        map.insert('l', bitvec![u8, Lsb0; 0, 1, 0, 1, 1]);
        map.insert('-', bitvec![u8, Lsb0; 0, 1, 1, 0, 0]);
        map.insert('"', bitvec![u8, Lsb0; 0, 1, 1, 0, 1]);
        map.insert('c', bitvec![u8, Lsb0; 0, 1, 1, 1, 0]);
        map.insert('p', bitvec![u8, Lsb0; 0, 1, 1, 1, 1]);
        map.insert('f', bitvec![u8, Lsb0; 1, 0, 0, 0, 0]);
        map.insert('>', bitvec![u8, Lsb0; 1, 0, 0, 0, 1]);
        map.insert('=', bitvec![u8, Lsb0; 1, 0, 0, 1, 0]);
        map.insert('.', bitvec![u8, Lsb0; 1, 0, 0, 1, 1]);
        map.insert('v', bitvec![u8, Lsb0; 1, 0, 1, 0, 0]);
        map.insert('<', bitvec![u8, Lsb0; 1, 0, 1, 0, 1]);
        map.insert('u', bitvec![u8, Lsb0; 1, 0, 1, 1, 0]);
        map.insert('m', bitvec![u8, Lsb0; 1, 0, 1, 1, 1]);
        map.insert(';', bitvec![u8, Lsb0; 1, 1, 0, 0, 0]);
        map.insert('g', bitvec![u8, Lsb0; 1, 1, 0, 0, 1]);
        map.insert(':', bitvec![u8, Lsb0; 1, 1, 0, 1, 0]);
        map.insert('/', bitvec![u8, Lsb0; 1, 1, 0, 1, 1]);
        map.insert('h', bitvec![u8, Lsb0; 1, 1, 1, 0, 0]);
        map.insert('y', bitvec![u8, Lsb0; 1, 1, 1, 0, 1]);
        map.insert('x', bitvec![u8, Lsb0; 1, 1, 1, 1, 0]);
        map.insert(
            'b',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        );
        map.insert(
            'k',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 1, 0],
        );
        map.insert(
            ')',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
        );
        map.insert(
            '(',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        );
        map.insert(
            'w',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        );
        map.insert(
            'E',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 1, 0],
        );
        map.insert(
            '#',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
        );
        map.insert(
            '}',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 0, 0],
        );
        map.insert(
            '{',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 0, 1],
        );
        map.insert(
            '0',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );
        map.insert(
            'N',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 1, 1],
        );
        map.insert(
            'A',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 0, 0],
        );
        map.insert(
            '2',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
        );
        map.insert(
            'R',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 1, 0],
        );
        map.insert(
            '1',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
        );
        map.insert(
            'T',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        );
        map.insert(
            'D',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 0, 1],
        );
        map.insert(
            'O',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
        );
        map.insert(
            'I',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        );
        map.insert(
            'S',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
        );
        map.insert(
            '_',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        );
        map.insert(
            'P',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 1, 0],
        );
        map.insert(
            'L',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
        );
        map.insert(
            '6',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        );
        map.insert(
            '4',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 0, 1],
        );
        map.insert(
            ',',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
        );
        map.insert(
            'z',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
        );
        map.insert(
            'M',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        );
        map.insert(
            'C',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        );
        map.insert(
            'B',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        );
        map.insert(
            'G',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        );
        map.insert(
            '%',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0],
        );
        map.insert(
            'j',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
        );
        map.insert(
            '3',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        );
        map.insert(
            'U',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        );
        map.insert(
            '8',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0],
        );
        map.insert(
            '*',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
        );
        map.insert(
            '5',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0],
        );
        map.insert(
            '9',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1],
        );
        map.insert(
            '+',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );
        map.insert(
            'F',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1],
        );
        map.insert(
            '|',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0],
        );
        map.insert(
            'W',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
        );
        map.insert(
            'V',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0],
        );
        map.insert(
            '@',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
        );
        map.insert(
            'q',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        );
        map.insert(
            '\'',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1],
        );
        map.insert(
            'Q',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
        );
        map.insert(
            'H',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        );
        map.insert(
            '!',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
        );
        map.insert(
            ']',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        );
        map.insert(
            '[',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0],
        );
        map.insert(
            '7',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
        );
        map.insert(
            'Z',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        );
        map.insert(
            'Y',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1],
        );
        map.insert(
            'X',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
        );
        map.insert(
            'J',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
        );
        map.insert(
            '^',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        );
        map.insert(
            'K',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        );
        map.insert(
            '?',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        );
        map.insert(
            '$',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        );
        map.insert(
            '\\',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0],
        );
        map.insert(
            '~',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
        );
        map.insert(
            '`',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        );
        map.insert(
            '&',
            bitvec![u8, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        );
        map
    };
}
