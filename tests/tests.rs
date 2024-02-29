use gclient::{EventProcessor, GearApi, Result};
use gstd::{prelude::*, ActorId};
use gtest::{Log, Program, System};
use std::fs;
use hikikomori_io::*;

#[test]
fn test() {
    let system = System::new();

    system.init_logger();

    let state_binary = get_state_binary();
    let program = Program::current(&system);

    let mut result = program.send_bytes(2, []);

    assert!(!result.main_failed());

    result = program.send(2, HikikomoriAction::AddEnergy);

    assert!(!result.main_failed());

    // State reading

    // All state

    let mut expected_state = Hikikomori {
        device: ActorId::from(2),
        energy: 10,
    };

    let mut state: Hikikomori = program.read_state(b"").unwrap();

    // expected_state.sort_unstable();
    // state.sort_unstable();

    assert_eq!(state, expected_state);

    // Querying `StateQuery::Energy` from the `query` metafunction

    result = program.send(2, HikikomoriAction::AddEnergy);

    assert!(result.contains(&Log::builder().payload(20u32)));

    let StateQueryReply::Energy(energy) = program
        .read_state_using_wasm(
            b"",
            "query",
            state_binary.clone(),
            Some(StateQuery::Energy),
        )
        .unwrap()
    else {
        unreachable!()
    };

    assert_eq!(energy, 20);

    // Querying the state using the `device` metafunction

    let mut device: ActorId = program
        .read_state_using_wasm::<(), _, _>(b"", "device", state_binary, None)
        .unwrap();

    // pingers.sort_unstable();

    assert_eq!(
        expected_state.device,
        device,
    );
}

fn get_state_binary() -> Vec<u8> {
    fs::read("target/wasm32-unknown-unknown/debug/hikikomori_state.meta.wasm").unwrap()
}

const ALICE: [u8; 32] = [
    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
    76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];

// #[tokio::test]
// async fn gclient_test() -> Result<()> {
//     let wasm_binary = fs::read("target/wasm32-unknown-unknown/debug/hikikomori.opt.wasm").unwrap();
//     let client = GearApi::dev_from_path("target/tmp/gear").await?;
//     let mut listener = client.subscribe().await?;

//     let mut gas_limit = client
//         .calculate_upload_gas(None, wasm_binary.clone(), vec![], 0, true)
//         .await?
//         .min_limit;
//     let (mut message_id, program_id, _) = client
//         .upload_program_bytes(
//             wasm_binary,
//             gclient::now_micros().to_le_bytes(),
//             [],
//             gas_limit,
//             0,
//         )
//         .await?;

//     assert!(listener.message_processed(message_id).await?.succeed());

//     gas_limit = client
//         .calculate_handle_gas(None, program_id, PingPong::Ping.encode(), 0, true)
//         .await?
//         .min_limit;
//     (message_id, _) = client
//         .send_message(program_id, PingPong::Ping, gas_limit, 0)
//         .await?;

//     let (_, raw_reply, _) = listener.reply_bytes_on(message_id).await?;

//     assert_eq!(
//         PingPong::Pong,
//         Decode::decode(
//             &mut raw_reply
//                 .expect("action failed, received an error message instead of a reply")
//                 .as_slice()
//         )?
//     );

//     let state_binary = get_state_binary();

//     assert_eq!(
//         StateQueryReply::PingCount(1),
//         client
//             .read_state_using_wasm(
//                 program_id,
//                 vec![],
//                 "query",
//                 state_binary.clone(),
//                 Some(StateQuery::PingCount(ActorId::from(ALICE)))
//             )
//             .await?
//     );

//     assert_eq!(
//         StateQueryReply::Pingers(vec![ALICE.into()]),
//         client
//             .read_state_using_wasm(
//                 program_id,
//                 vec![],
//                 "query",
//                 state_binary,
//                 Some(StateQuery::Pingers)
//             )
//             .await?
//     );

//     Ok(())
// }
