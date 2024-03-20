use tiny_blockchain::{
    generate_mock_blocks, seconds_now, Chain, TinyBlockchain, TinyBlockchainParams,
};

fn main() {
    // let init_transactions = vec![Transaction::new(
    //     U256::from_str_hex("0x1100100000000000000000000000000000000000000000000000000000000101")
    //         .unwrap(),
    //     U256::from_str_hex("0x2200200000000000000000000000000000000000000000000000000000000202")
    //         .unwrap(),
    //     999,
    // )];
    let init_chain = Chain {
        items: generate_mock_blocks(2016),
        last_update: seconds_now(),
    };
    let blockchain_params = TinyBlockchainParams {
        init_difficulty: 0x1dffffff,
        blocks_in_epoch: 2016,
        epoch: 2016 * 10,
    };
    let blockchain = TinyBlockchain::new(init_chain, blockchain_params);

    // println!("Result={:?}", init_chain.is_valid());

    // println!("Result: {}", format!("{:0width$x}", new_bits, width = 8));
    // println!(
    //     "Data: {}",
    //     format!(
    //         "{:0width$x}",
    //         compact_to_32_bits(
    //             U256::from_str_hex(
    //                 "0x00000000ffff0000000000000000000000000000000000000000000000000000"
    //             )
    //             .unwrap()
    //         ),
    //         width = 8
    //     )
    // );

    // let temp = "1100100000000000000000000000000000000000000000000000000000000101"
    //     .parse::<U256>()
    //     .unwrap();

    // assert_eq!(
    //     "1d00ffff",
    //     format!(
    //         "{:0width$x}",
    //         uncompact_to_256_bits(0x1d00ffff),
    //         // compact_to_32_bits(
    //         //     U256::from_str_hex(
    //         //         "0x00000000ffff0000000000000000000000000000000000000000000000000000"
    //         //     )
    //         //     .unwrap()
    //         // ),
    //         width = 64
    //     )
    // );

    // assert_eq!(
    //     "170ed0eb",
    //     format!(
    //         "{:0width$x}",
    //         compact_to_32_bits(
    //             U256::from_str_hex(
    //                 "0x0000000000000000000ed0eb0000000000000000000000000000000000000000"
    //             )
    //             .unwrap()
    //         ),
    //         width = 8
    //     )
    // );

    // assert_eq!(
    //     "181bc330",
    //     format!(
    //         "{:0width$x}",
    //         compact_to_32_bits(
    //             U256::from_str_hex(
    //                 "0x00000000000000001bc330000000000000000000000000000000000000000000"
    //             )
    //             .unwrap()
    //         ),
    //         width = 8
    //     )
    // );

    // assert_eq!(
    //     "20ffffff",
    //     format!(
    //         "{:0width$x}",
    //         compact_to_32_bits(
    //             U256::from_str_hex(
    //                 "0xffffff0000000000000000000000000000000000000000000000000000000000"
    //             )
    //             .unwrap()
    //         ),
    //         width = 8
    //     )
    // );

    // println!(
    //     "Result={:?}",
    //     blockchain.proof_of_work(&U256::from_i8(100).unwrap())
    // );
}
