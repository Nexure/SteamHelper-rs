// This file is generated by rust-protobuf 2.20.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `steammessages_remoteclient_service.steamclient.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_20_0;

static file_descriptor_proto_data: &'static [u8] = b"\
    \n4steammessages_remoteclient_service.steamclient.proto\x1a,steammessage\
    s_unified_base.steamclient.proto\x1a1steammessages_remoteclient_service_\
    messages.proto2\xd4\x0c\n\x0cRemoteClient\x12\x88\x01\n\x0eGetPairingInf\
    o\x12%.CRemoteClient_GetPairingInfo_Request\x1a&.CRemoteClient_GetPairin\
    gInfo_Response\"'\x82\xb5\x18#Get\x20pairing\x20info\x20for\x20an\x20ent\
    ered\x20PIN\x12~\n\x0cNotifyOnline\x12\".CRemoteClient_Online_Notificati\
    on\x1a\x0b.NoResponse\"=\x82\xb5\x189Let\x20the\x20service\x20know\x20we\
    're\x20available\x20for\x20status\x20listeners\x12n\n\x11NotifyReplyPack\
    et\x12'.CRemoteClient_ReplyPacket_Notification\x1a\x0b.NoResponse\"#\x82\
    \xb5\x18\x1fSend\x20a\x20reply\x20to\x20a\x20remote\x20client\x12\x9f\
    \x01\n\x12AllocateTURNServer\x12).CRemoteClient_AllocateTURNServer_Reque\
    st\x1a*.CRemoteClient_AllocateTURNServer_Response\"2\x82\xb5\x18.Allocat\
    e\x20a\x20TURN\x20server\x20for\x20a\x20streaming\x20session\x12\xa7\x01\
    \n\x13AllocateRelayServer\x12*.CRemoteClient_AllocateRelayServer_Request\
    \x1a+.CRemoteClient_AllocateRelayServer_Response\"7\x82\xb5\x183Allocate\
    \x20a\x20UDP\x20relay\x20server\x20for\x20a\x20streaming\x20session\x12}\
    \n\x0bAllocateSDR\x12\".CRemoteClient_AllocateSDR_Request\x1a#.CRemoteCl\
    ient_AllocateSDR_Response\"%\x82\xb5\x18!Allocate\x20SDR\x20resources\
    \x20for\x20an\x20app\x12\x83\x01\n\x18SendSteamBroadcastPacket\x12*.CRem\
    oteClient_SteamBroadcast_Notification\x1a\x0b.NoResponse\".\x82\xb5\x18*\
    Broadcast\x20a\x20packet\x20to\x20remote\x20Steam\x20clients\x12{\n\x16S\
    endSteamToSteamPacket\x12(.CRemoteClient_SteamToSteam_Notification\x1a\
    \x0b.NoResponse\"*\x82\xb5\x18&Send\x20a\x20packet\x20to\x20a\x20remote\
    \x20Steam\x20client\x12\xa8\x01\n\x1cSendRemotePlaySessionStarted\x12#.C\
    RemotePlay_SessionStarted_Request\x1a$.CRemotePlay_SessionStarted_Respon\
    se\"=\x82\xb5\x189Let\x20the\x20server\x20know\x20that\x20we\x20started\
    \x20a\x20Remote\x20Play\x20session\x12\x94\x01\n\x1cSendRemotePlaySessio\
    nStopped\x12(.CRemotePlay_SessionStopped_Notification\x1a\x0b.NoResponse\
    \"=\x82\xb5\x189Let\x20the\x20server\x20know\x20that\x20we\x20stopped\
    \x20a\x20Remote\x20Play\x20session\x12\x88\x01\n\x1cSendRemotePlayTogeth\
    erPacket\x12!.CRemotePlayTogether_Notification\x1a\x0b.NoResponse\"8\x82\
    \xb5\x184Send\x20a\x20Remote\x20Play\x20Together\x20packet\x20to\x20a\
    \x20Steam\x20client\x1a.\x82\xb5\x18*Methods\x20for\x20Steam\x20remote\
    \x20client\x20operations2\x94\x07\n\x17RemoteClientSteamClient\x12\x90\
    \x01\n\x1aNotifyRegisterStatusUpdate\x120.CRemoteClient_RegisterStatusUp\
    date_Notification\x1a\x0b.NoResponse\"3\x82\xb5\x18/Register\x20for\x20s\
    tatus\x20updates\x20with\x20a\x20Steam\x20client\x12\x96\x01\n\x1cNotify\
    UnregisterStatusUpdate\x122.CRemoteClient_UnregisterStatusUpdate_Notific\
    ation\x1a\x0b.NoResponse\"5\x82\xb5\x181Unregister\x20for\x20status\x20u\
    pdates\x20with\x20a\x20Steam\x20client\x12p\n\x12NotifyRemotePacket\x12(\
    .CRemoteClient_RemotePacket_Notification\x1a\x0b.NoResponse\"#\x82\xb5\
    \x18\x1fSend\x20a\x20packet\x20to\x20a\x20Steam\x20client\x12\x85\x01\n\
    \x1aNotifySteamBroadcastPacket\x12*.CRemoteClient_SteamBroadcast_Notific\
    ation\x1a\x0b.NoResponse\".\x82\xb5\x18*Broadcast\x20a\x20packet\x20to\
    \x20remote\x20Steam\x20clients\x12\x91\x01\n\x18NotifySteamToSteamPacket\
    \x12(.CRemoteClient_SteamToSteam_Notification\x1a\x0b.NoResponse\">\x82\
    \xb5\x18:Send\x20a\x20packet\x20to\x20a\x20Steam\x20client\x20from\x20a\
    \x20remote\x20Steam\x20client\x12\x8a\x01\n\x1eNotifyRemotePlayTogetherP\
    acket\x12!.CRemotePlayTogether_Notification\x1a\x0b.NoResponse\"8\x82\
    \xb5\x184Send\x20a\x20Remote\x20Play\x20Together\x20packet\x20to\x20a\
    \x20Steam\x20client\x1a2\x82\xb5\x18*Methods\x20for\x20Steam\x20remote\
    \x20client\x20operations\xc0\xb5\x18\x02B\x03\x80\x01\x01\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
