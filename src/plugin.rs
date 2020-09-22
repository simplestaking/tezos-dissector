// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use wireshark_definitions::FieldDescriptor;
use wireshark_epan_adapter::{Plugin, NameDescriptor, PrefFilenameDescriptor, DissectorDescriptor};
use tezos_messages::p2p::encoding::{
    ack::AckMessage, metadata::MetadataMessage, peer::PeerMessageResponse,
    connection::ConnectionMessage,
};
use tezos_conversation::TezosEncoded;
use super::dissector::TezosDissector;

#[no_mangle]
static plugin_version: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

#[no_mangle]
static plugin_want_major: i32 = wireshark_epan_adapter::PLUGIN_WANT_MAJOR;

#[no_mangle]
static plugin_want_minor: i32 = wireshark_epan_adapter::PLUGIN_WANT_MINOR;

#[no_mangle]
extern "C" fn plugin_register() {
    if cfg!(debug_assertions) {
        let file = env!("PWD")
            .parse::<std::path::PathBuf>()
            .unwrap()
            .join("target/log.txt");
        simple_logging::log_to_file(file, log::LevelFilter::Info).unwrap();
    }

    Plugin::new(
        DissectorDescriptor {
            display_name: "Tezos\0",
            short_name: "tezos_tcp\0",
        },
        NameDescriptor {
            name: "Tezos Protocol\0",
            short_name: "tezos\0",
            filter_name: "tezos\0",
        },
        // all the fields that might appear on the tree UI should be declared here
        &[
            &[
                FieldDescriptor::String {
                    name: "Conversation\0",
                    abbrev: "tezos.conversation_id\0",
                },
                FieldDescriptor::String {
                    name: "Source\0",
                    abbrev: "tezos.source\0",
                },
                FieldDescriptor::String {
                    name: "Decryption error\0",
                    abbrev: "tezos.decryption_error\0",
                },
                FieldDescriptor::String {
                    name: "Decoding error\0",
                    abbrev: "tezos.decoding_error\0",
                },
                FieldDescriptor::String {
                    name: "Messages\0",
                    abbrev: "tezos.messages\0",
                },
            ],
            // chunk
            &[
                FieldDescriptor::Int64Dec {
                    name: "Chunk\0",
                    abbrev: "tezos.chunk\0",
                },
                FieldDescriptor::Int64Dec {
                    name: "Chunk length\0",
                    abbrev: "tezos.chunk.length\0",
                },
                FieldDescriptor::String {
                    name: "Decrypted data\0",
                    abbrev: "tezos.chunk.data\0",
                },
                FieldDescriptor::String {
                    // only for first pass
                    name: "Buffering\0",
                    abbrev: "tezos.chunk.buffering\0",
                },
                FieldDescriptor::String {
                    name: "Message authentication code\0",
                    abbrev: "tezos.chunk.mac\0",
                },
            ],
        ],
        &[PrefFilenameDescriptor {
            name: "identity_json_file\0",
            title: "Identity JSON file\0",
            description: "JSON file with node identity information\0",
        }],
    )
    // declare fields needed for presenting types
    .register_type::<TezosEncoded<ConnectionMessage>>()
    .register_type::<TezosEncoded<MetadataMessage>>()
    .register_type::<TezosEncoded<AckMessage>>()
    .register_type::<TezosEncoded<PeerMessageResponse>>()
    // before this line we just create static structure that does nothing,
    // this line registers the plugin in the wireshark, o wireshark can call our dissector
    .register(Box::new(TezosDissector::new()))
}
