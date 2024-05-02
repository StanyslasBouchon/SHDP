use std::collections::HashMap;

use bitvec::{bitvec, prelude::Lsb0, vec::BitVec};
use lazy_static::lazy_static;

lazy_static! {
    /// A map of characters to bit vectors.
    /// Every common programming character is mapped to a bit vector of five-bit chains, known as fyve.
    static ref CHARS: HashMap<String, BitVec<u16>> = {
        let mut map = HashMap::new();
        map.insert(" ".to_string(), bitvec![u16, Lsb0; 0, 0, 0, 0, 1]);
        map.insert("t".to_string(), bitvec![u16, Lsb0; 0, 0, 0, 1, 0]);
        map.insert("a".to_string(), bitvec![u16, Lsb0; 0, 0, 0, 1, 1]);
        map.insert("e".to_string(), bitvec![u16, Lsb0; 0, 0, 1, 0, 0]);
        map.insert("i".to_string(), bitvec![u16, Lsb0; 0, 0, 1, 0, 1]);
        map.insert("n".to_string(), bitvec![u16, Lsb0; 0, 0, 1, 1, 0]);
        map.insert("o".to_string(), bitvec![u16, Lsb0; 0, 0, 1, 1, 1]);
        map.insert("r".to_string(), bitvec![u16, Lsb0; 0, 1, 0, 0, 0]);
        map.insert("s".to_string(), bitvec![u16, Lsb0; 0, 1, 0, 0, 1]);
        map.insert("d".to_string(), bitvec![u16, Lsb0; 0, 1, 0, 1, 0]);
        map.insert("l".to_string(), bitvec![u16, Lsb0; 0, 1, 0, 1, 1]);
        map.insert("-".to_string(), bitvec![u16, Lsb0; 0, 1, 1, 0, 0]);
        map.insert("\"".to_string(), bitvec![u16, Lsb0; 0, 1, 1, 0, 1]);
        map.insert("c".to_string(), bitvec![u16, Lsb0; 0, 1, 1, 1, 0]);
        map.insert("p".to_string(), bitvec![u16, Lsb0; 0, 1, 1, 1, 1]);
        map.insert("f".to_string(), bitvec![u16, Lsb0; 1, 0, 0, 0, 0]);
        map.insert(">".to_string(), bitvec![u16, Lsb0; 1, 0, 0, 0, 1]);
        map.insert("=".to_string(), bitvec![u16, Lsb0; 1, 0, 0, 1, 0]);
        map.insert(".".to_string(), bitvec![u16, Lsb0; 1, 0, 0, 1, 1]);
        map.insert("v".to_string(), bitvec![u16, Lsb0; 1, 0, 1, 0, 0]);
        map.insert("<".to_string(), bitvec![u16, Lsb0; 1, 0, 1, 0, 1]);
        map.insert("u".to_string(), bitvec![u16, Lsb0; 1, 0, 1, 1, 0]);
        map.insert("m".to_string(), bitvec![u16, Lsb0; 1, 0, 1, 1, 1]);
        map.insert(";".to_string(), bitvec![u16, Lsb0; 1, 1, 0, 0, 0]);
        map.insert("g".to_string(), bitvec![u16, Lsb0; 1, 1, 0, 0, 1]);
        map.insert(":".to_string(), bitvec![u16, Lsb0; 1, 1, 0, 1, 0]);
        map.insert("/".to_string(), bitvec![u16, Lsb0; 1, 1, 0, 1, 1]);
        map.insert("h".to_string(), bitvec![u16, Lsb0; 1, 1, 1, 0, 0]);
        map.insert("y".to_string(), bitvec![u16, Lsb0; 1, 1, 1, 0, 1]);
        map.insert("x".to_string(), bitvec![u16, Lsb0; 1, 1, 1, 1, 0]);
        map.insert(
            "b".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        );
        map.insert(
            "k".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 1, 0],
        );
        map.insert(
            ")".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
        );
        map.insert(
            "(".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        );
        map.insert(
            "w".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        );
        map.insert(
            "E".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 1, 0],
        );
        map.insert(
            "#".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
        );
        map.insert(
            "}".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 0, 0],
        );
        map.insert(
            "{".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 0, 1],
        );
        map.insert(
            "0".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );
        map.insert(
            "N".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 0, 1, 1],
        );
        map.insert(
            "A".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 0, 0],
        );
        map.insert(
            "2".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
        );
        map.insert(
            "R".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 1, 0],
        );
        map.insert(
            "1".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
        );
        map.insert(
            "T".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        );
        map.insert(
            "D".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 0, 1],
        );
        map.insert(
            "O".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
        );
        map.insert(
            "I".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        );
        map.insert(
            "S".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
        );
        map.insert(
            "_".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        );
        map.insert(
            "P".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 1, 0],
        );
        map.insert(
            "L".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
        );
        map.insert(
            "6".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        );
        map.insert(
            "4".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 0, 1],
        );
        map.insert(
            ",".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
        );
        map.insert(
            "z".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
        );
        map.insert(
            "M".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        );
        map.insert(
            "C".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        );
        map.insert(
            "B".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        );
        map.insert(
            "G".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        );
        map.insert(
            "%".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0],
        );
        map.insert(
            "j".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
        );
        map.insert(
            "3".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        );
        map.insert(
            "U".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        );
        map.insert(
            "8".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0],
        );
        map.insert(
            "*".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
        );
        map.insert(
            "5".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0],
        );
        map.insert(
            "9".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1],
        );
        map.insert(
            "+".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0],
        );
        map.insert(
            "F".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1],
        );
        map.insert(
            "|".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0],
        );
        map.insert(
            "W".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
        );
        map.insert(
            "V".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0],
        );
        map.insert(
            "@".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
        );
        map.insert(
            "q".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0],
        );
        map.insert(
            "'".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1],
        );
        map.insert(
            "Q".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0],
        );
        map.insert(
            "H".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1],
        );
        map.insert(
            "!".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0],
        );
        map.insert(
            "]".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1],
        );
        map.insert(
            "[".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0],
        );
        map.insert(
            "7".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1],
        );
        map.insert(
            "Z".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0],
        );
        map.insert(
            "Y".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1],
        );
        map.insert(
            "X".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
        );
        map.insert(
            "J".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
        );
        map.insert(
            "^".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
        );
        map.insert(
            "K".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1],
        );
        map.insert(
            "?".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
        );
        map.insert(
            "$".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1],
        );
        map.insert(
            "\\".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0],
        );
        map.insert(
            "~".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1],
        );
        map.insert(
            "`".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0],
        );
        map.insert(
            "&".to_string(),
            bitvec![u16, Lsb0; 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1],
        );
        map
    };
}
