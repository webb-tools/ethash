use byteorder::ByteOrder;
use ethash::types;

// this test is used as a playground
#[test]
fn proofs() {
    let rlp_encoded = include_str!("fixtures/2.rlp");
    let rlp_encoded = hex::decode(rlp_encoded.trim()).unwrap();
    let header: types::BlockHeader = rlp::decode(&rlp_encoded).unwrap();
    let header_hash = ethash::seal_header(&types::BlockHeaderSeal::from(header.clone()));
    assert_eq!(
        header_hash.as_bytes(),
        hex::decode("d9a38e294d953b1e735e8e71025a1855ed7f2139e13ff8a19bb7e82383576c47").unwrap()
    );

    let dag = ethash::LightDAG::<ethash::EthereumPatch>::new(header.number);
    let (mix_hash, result) = dag.hashimoto(header_hash, header.nonce);
    assert_eq!(
        result.as_bytes(),
        hex::decode("000000003a0a4fb7f886bad18226a47fb09767ac8c0c87141083443ac5cfdf59").unwrap()
    );
    assert_eq!(mix_hash, header.mix_hash);

    let indices = ethash::get_indices(header_hash, header.nonce, dag.full_size, |i| {
        let raw_data = ethash::calc_dataset_item(&dag.cache, i);
        let mut data = [0u32; 16];
        for (i, b) in data.iter_mut().enumerate() {
            *b = byteorder::LE::read_u32(&raw_data[(i * 4)..]);
        }
        data
    });

    assert_eq!(
        indices,
        &[
            4990688, 6987316, 1807929, 2596874, 3359925, 3073025, 3519380, 5337872, 2175509,
            4172374, 1572107, 5437761, 4861897, 5627685, 4991962, 2554186, 3290547, 6561417,
            7089885, 7073632, 786997, 3378685, 6185265, 5283049, 4273209, 3161257, 5030708,
            5274872, 3725170, 202134, 5492399, 6895738, 5696426, 6626457, 2345861, 262304, 2658959,
            7286807, 547777, 5472769, 7664032, 1035384, 2671289, 4103686, 8347077, 2322872,
            6754122, 2654051, 4610695, 65291, 3601125, 1821797, 5122957, 5336515, 7610054, 652865,
            375080, 5367006, 2543741, 2475727, 341558, 5858560, 7361407, 3569253
        ]
    );
}
