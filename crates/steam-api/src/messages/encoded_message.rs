//! EMSG (Encoded Message)
//!
//! When steam sends any packet through the socket, it first need to be decoded.
//! We call this message a EMsg. We take the raw packet data to check what message it is
//! sending to us.
//!
//! Here we take care of stripping the message apart to see what is being sent.
//!
//! The next step would be in the message module, after the message has been decoded.
//!
//! Check link below for more info:
//! https://github.com/SteamRE/SteamKit/blob/master/SteamKit2/SteamKit2/Steam/CMClient.cs#L423

use crate::messages::enums::{EMsg, EUniverse};
use num::FromPrimitive;

const PROTOMASK: u32 = 0x8000_0000;
const EMSGMASK: u32 = !PROTOMASK;

impl EMsg {
    /// Creates an `EMsg` from a uint32
    pub fn from_uint(message: u32) -> Self {
        match EMsg::from_u32(message) {
            Some(value) => value,
            None => panic!("ABORT"),
        }
    }

    /// Creates an `EMsg` from a raw data packet.
    pub fn from_raw_message(message: &[u8]) -> Result<Self, EMsgError> {
        // an error should be throw if the message doesnt have 4 bytes of length
        let extracted_varint: u32 = Self::extract_varint(message);

        match EMsg::from_u32(extracted_varint) {
            Some(value) => Ok(value),
            None => Err(EMsgError::ValueNotFound("A value was not found.")),
        }
    }

    /// Strips protobuf message flag out and and returns it
    pub fn strip_protobuf_flag(message: u32) -> u32 {
        message & PROTOMASK
    }

    /// Strips Emsg and returns
    pub fn strip_message(message: &[u8]) -> &[u8] {
        &message[4..]
    }

    /// Checks if a message is flagged as a protobuf
    pub fn is_protobuf(message: &[u8]) -> bool {
        Self::strip_protobuf_flag(Self::extract_varint(message)) > 0
    }

    /// Extract varint from data
    pub fn extract_varint(message: &[u8]) -> u32 {
        u32::from_le_bytes(*array_ref!(message, 0, 4))
    }
}

#[derive(Debug)]
pub enum EMsgError {
    MessageNotLongEnough(&'static str),
    ValueNotFound(&'static str),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// ChannelEncryptRequest
    fn get_channel_encrypt_request() -> [u8; 44] {
        let on_connection_packet: [u8; 44] = [
            23, 5, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 1, 0, 0, 0, 1, 0, 0, 0, 66, 126, 251, 245, 88, 122, 243, 123, 102, 163, 11, 54,
            151, 145, 31, 54,
        ];
        on_connection_packet
    }

    /// ClientChatEnter, EMsg(807)
    fn get_example_message() -> [u8; 353] {
        let struct_msg_data: [u8; 353] = [
            0x27, 0x03, 0x00, 0x00, 0x24, 0x02, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0xAC, 0x15, 0x89, 0x00,
            0x01, 0x00, 0x10, 0x01, 0x8E, 0x56, 0x11, 0x00, 0xBC, 0x4E, 0x2A, 0x00, 0x00, 0x00,
            0x88, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0xBC, 0x4E, 0x2A, 0x00, 0x00, 0x00, 0x70, 0x01, 0xBC, 0x4E, 0x2A, 0x00, 0x00, 0x00,
            0x70, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x53, 0x61, 0x78,
            0x74, 0x6F, 0x6E, 0x20, 0x48, 0x65, 0x6C, 0x6C, 0x00, 0x00, 0x4D, 0x65, 0x73, 0x73,
            0x61, 0x67, 0x65, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x00, 0x07, 0x73, 0x74, 0x65,
            0x61, 0x6D, 0x69, 0x64, 0x00, 0xAC, 0x15, 0x89, 0x00, 0x01, 0x00, 0x10, 0x01, 0x02,
            0x70, 0x65, 0x72, 0x6D, 0x69, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x73, 0x00, 0x7B, 0x03,
            0x00, 0x00, 0x02, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6C, 0x73, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x08, 0x08, 0x00, 0x4D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4F, 0x62, 0x6A,
            0x65, 0x63, 0x74, 0x00, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6D, 0x69, 0x64, 0x00, 0x00,
            0x28, 0x90, 0x00, 0x01, 0x00, 0x10, 0x01, 0x02, 0x70, 0x65, 0x72, 0x6D, 0x69, 0x73,
            0x73, 0x69, 0x6F, 0x6E, 0x73, 0x00, 0x08, 0x00, 0x00, 0x00, 0x02, 0x44, 0x65, 0x74,
            0x61, 0x69, 0x6C, 0x73, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0x08, 0x00, 0x4D, 0x65,
            0x73, 0x73, 0x61, 0x67, 0x65, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x00, 0x07, 0x73,
            0x74, 0x65, 0x61, 0x6D, 0x69, 0x64, 0x00, 0xB0, 0xDC, 0x5B, 0x04, 0x01, 0x00, 0x10,
            0x01, 0x02, 0x70, 0x65, 0x72, 0x6D, 0x69, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x73, 0x00,
            0x08, 0x00, 0x00, 0x00, 0x02, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6C, 0x73, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x08, 0x08, 0x00, 0x4D, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x4F,
            0x62, 0x6A, 0x65, 0x63, 0x74, 0x00, 0x07, 0x73, 0x74, 0x65, 0x61, 0x6D, 0x69, 0x64,
            0x00, 0x39, 0xCB, 0x77, 0x05, 0x01, 0x00, 0x10, 0x01, 0x02, 0x70, 0x65, 0x72, 0x6D,
            0x69, 0x73, 0x73, 0x69, 0x6F, 0x6E, 0x73, 0x00, 0x1A, 0x03, 0x00, 0x00, 0x02, 0x44,
            0x65, 0x74, 0x61, 0x69, 0x6C, 0x73, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x08, 0xE8,
            0x03, 0x00, 0x00,
        ];
        struct_msg_data
    }

    #[test]
    fn from_raw_data() {
        let on_connection_packet = get_channel_encrypt_request();
        let emsg = EMsg::from_raw_message(&on_connection_packet).unwrap();

        assert_eq!(emsg, EMsg::ChannelEncryptRequest)
    }

    #[test]
    fn from_raw_data_another() {
        let packet = get_example_message();
        let emsg = EMsg::from_raw_message(&packet).unwrap();

        assert_eq!(emsg, EMsg::ClientChatEnter)
    }

    #[test]
    fn check_if_not_protobuf() {
        let on_connection_packet = get_channel_encrypt_request();
        assert!(!EMsg::is_protobuf(&on_connection_packet))
    }
}
