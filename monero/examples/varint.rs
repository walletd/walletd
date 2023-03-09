extern crate walletd_monero;
use std::fmt::Display;

use walletd_monero::{varint, VarInt, VarIntEncoding};

// Comparing to an example of key offsets, checking what the varint bytes are
pub fn main() {
    let example_vector_1: Vec<u64> = vec![
        1519, 166, 1765, 128, 571, 336, 614, 252, 55, 168, 168, 29, 173, 12, 13, 92,
    ];
    println!("example_vector_1: {:?}", example_vector_1);
    print_convert_vector_to_varint(&example_vector_1);

    let example_vector_2: Vec<u8> = vec![
        1, 194, 71, 125, 36, 146, 45, 164, 134, 137, 13, 109, 146, 75, 4, 189, 83, 132, 116, 49,
        225, 216, 113, 192, 226, 250, 65, 241, 190, 110, 74, 113, 223, 2, 9, 1, 60, 29, 61, 59,
        234, 68, 186, 193,
    ];

    println!("example_vector_2: {:?}", example_vector_2);
    print_convert_vector_to_varint(&example_vector_2);

    let example_vector_3: Vec<u8> = vec![
        1, 9, 249, 108, 245, 132, 103, 18, 43, 196, 128, 169, 186, 203, 246, 220, 156, 52, 111, 1,
        180, 179, 208, 132, 130, 230, 189, 247, 27, 149, 119, 152, 253,
    ];
    println!("example_vector_3: {:?}", example_vector_3);
    print_convert_vector_to_varint(&example_vector_3);

    let example_vector_4: Vec<u64> = vec![
        6474877, 57361, 36970, 4456, 8077, 3305, 1473, 1706, 753, 49, 163, 128, 384, 169, 9, 52,
    ];
    println!("example_vector_4: {:?}", example_vector_4);
    print_convert_vector_to_varint(&example_vector_4);

    let example_5: Vec<u64> = vec![1903200000];
    println!("example_5: {:?}", example_5);
    print_convert_vector_to_varint(&example_5);

    let example_6: Vec<u64> = vec![164120000];
    println!("example_6: {:?}", example_6);
    print_convert_vector_to_varint(&example_6);

    let example_vector_7: Vec<u64> = vec![
        5014406, 1039327, 527419, 743, 3048, 2078, 268, 1564, 96, 260, 179, 694, 16, 129, 71, 174,
    ];
    println!("example_vector_7: {:?}", example_vector_7);
    print_convert_vector_to_varint(&example_vector_7);

    let example_vector_8: Vec<u64> = vec![
        3217843, 3294983, 47015, 3327, 19705, 2406, 2533, 1152, 233, 914, 15, 214, 11, 20, 19, 82,
    ];
    println!("example_vector_8: {:?}", example_vector_8);
    print_convert_vector_to_varint(&example_vector_8);

    let example_vector_9: Vec<u8> = vec![
        1, 238, 233, 163, 155, 216, 165, 112, 246, 52, 126, 151, 162, 98, 156, 56, 71, 46, 244,
        233, 7, 155, 11, 252, 199, 182, 98, 9, 103, 118, 195, 17, 37, 2, 9, 1, 107, 228, 184, 46,
        11, 211, 77, 52,
    ];
    println!("example_vector_9: {:?}", example_vector_9);
    print_convert_vector_to_varint(&example_vector_9);

    let example_vector_10: Vec<u8> = vec![
        1, 218, 234, 125, 186, 222, 118, 25, 165, 98, 213, 104, 195, 171, 66, 158, 240, 173, 99,
        65, 155, 81, 138, 134, 225, 106, 22, 251, 120, 254, 25, 244, 121,
    ];
    println!("example_vector_10: {:?}", example_vector_10);
    print_convert_vector_to_varint(&example_vector_10);

    let example_11: Vec<u64> = vec![174460000];
    println!("example_11: {:?}", example_11);
    print_convert_vector_to_varint(&example_11);

    let example_12: Vec<u64> = vec![164120000];
    println!("example_12: {:?}", example_12);
    print_convert_vector_to_varint(&example_12);
}

pub fn print_convert_vector_to_varint<T>(example: &Vec<T>)
where
    T: varint::UnsignedInt,
{
    let mut vec_as_bytes = Vec::new();
    for num in example {
        let varint = VarInt(*num);
        let encoded_bytes = varint.encode_to_bytes();

        println!(
            "num: {} represented as varint bytes: {:?}, as hex: {}",
            num,
            &encoded_bytes,
            hex::encode(&encoded_bytes)
        );
        vec_as_bytes.extend(&encoded_bytes);
    }
    println!("vec_as_hex: {}", hex::encode(vec_as_bytes));
}
