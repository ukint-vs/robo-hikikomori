#![no_std]

use gstd::{prelude::*, ActorId};
use hikikomori_io::*;

#[gmeta::metawasm]
pub mod metafns {
    pub type State = hikikomori_io::State;

    pub fn query(state: State, query: StateQuery) -> StateQueryReply {
        match query {
            StateQuery::Energy => StateQueryReply::Energy(state.energy),
            StateQuery::Device => StateQueryReply::Device(state.device),
        }
    }

    pub fn device(state: State) -> ActorId {
        state.device
    }

    // pub fn ping_count(state: State, actor: ActorId) -> u128 {
    //     state
    //         .iter()
    //         .find_map(|(some_actor, count)| (some_actor == &actor).then_some(count))
    //         .copied()
    //         .unwrap_or_default()
    // }
}
