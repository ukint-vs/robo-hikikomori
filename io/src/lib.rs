#![no_std]

use gmeta::{InOut, Metadata, Out};
use gstd::{prelude::*, ActorId};

/// The contract metadata. Used by frontend apps & for describing the types of messages that can be
/// sent in contract's entry points. See also [`Metadata`].
pub struct ContractMetadata;

/// `()` means the contract doesn't process & reply messages at the above written entry point or
/// doesn't implement it.
impl Metadata for ContractMetadata {
    /// I/O types for the `init()` entry point.
    type Init = ();
    /// I/O types for the `handle()` entry point.
    ///
    /// Here the [`PingPong`] type is used for both incoming and outgoing messages.
    type Handle = InOut<HikikomoriAction, u32>;
    /// Types for miscellaneous scenarios.
    type Others = ();
    /// The input type for the `handle_reply()` entry point.
    type Reply = ();
    /// The output type for the `handle_signal()` entry point.
    type Signal = ();
    /// I/O types for the `state()` entry point.
    ///
    /// You can also specify just an output ([`Out`]) or input ([`In`](gmeta::In)) type, if both
    /// ([`InOut`]) are expected like here.
    type State = Out<State>;
}

#[derive(Debug, Default, Encode, Decode, TypeInfo, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct Hikikomori {
    pub device: ActorId,
    pub energy: u32,
}

pub type State = Hikikomori;

#[derive(Encode, Decode, TypeInfo, Debug, PartialEq, Eq)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum HikikomoriAction {
    AddEnergy,
}

/// Queries the contract state.
///
/// Used in the `state` crate.
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQuery {
    /// Gets the energy of the hikikomori.
    ///
    /// Returns [`StateQueryReply::Energy`].
    Energy,
    Device,
}

/// The result of successfully processed [`StateQuery`].
///
/// Used in the `state` crate.
#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum StateQueryReply {
    /// Returned from [`StateQuery::Energy`].
    Energy(u32),
    /// Returned from [`StateQuery::Device`].
    Device(ActorId),
}
