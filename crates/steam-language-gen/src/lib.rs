#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate steam_language_gen_derive;

use downcast_rs::{impl_downcast, Downcast};
use enum_dispatch::enum_dispatch;
use serde::Serialize;

use crate::generated::headers::{ExtendedMessageHeader, StandardMessageHeader};
use steam_protobuf::steam::steammessages_base::CMsgProtoBufHeader;
use steam_protobuf::Message;

pub mod generated;
#[cfg(feature = "generator")]
pub mod generator;
#[cfg(feature = "generator")]
pub mod parser;

#[enum_dispatch]
#[derive(Clone, Debug, Serialize)]
/// This wraps our headers so we can be generic over them over a Msg type.
pub enum MessageHeaderWrapper {
    Std(StandardMessageHeader),
    Proto(CMsgProtoBufHeader),
    Ext(ExtendedMessageHeader),
}

/// Every implementation has to implement bincode::serialize and deserialize
pub trait SerializableBytes: Downcast {
    fn to_bytes(&self) -> Vec<u8>;
}
impl_downcast!(SerializableBytes);

/// delegate serialization to inner type
impl SerializableBytes for MessageHeaderWrapper {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            MessageHeaderWrapper::Std(hdr) => hdr.to_bytes(),
            MessageHeaderWrapper::Ext(hdr) => hdr.to_bytes(),
            MessageHeaderWrapper::Proto(hdr) => hdr.write_to_bytes().expect("Error writing protobuf"),
        }
    }
}

pub trait DeserializableBytes {
    fn from_bytes(packet_data: &[u8]) -> Self;
}

#[enum_dispatch(MessageHeaderWrapper)]
pub trait MessageHeader: Downcast {
    fn set_target(&mut self, new_target: u64);
    fn set_source(&mut self, new_source: u64);
    fn source(&self) -> u64;
    fn target(&self) -> u64;
}
impl_downcast!(MessageHeader);

// facilities around headers
pub trait MessageHeaderExt: Downcast {
    fn create() -> Self;
    /// Returns header on the left, rest on the right
    fn split_from_bytes(data: &[u8]) -> (&[u8], &[u8]);
}

pub trait MessageBodyExt: Downcast {
    fn split_from_bytes(data: &[u8]) -> (&[u8], &[u8]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Element {
    File,
    Head,
    Type,
    Member,
}

#[derive(PartialEq, Eq)]
pub struct Token<'a> {
    value: String,
    default: Option<&'a str>,
}

impl<'a> Token<'a> {
    fn get_value(&self) -> &String {
        &self.value
    }
    fn get_default(&self) -> Option<&'a str> {
        self.default
    }
}
