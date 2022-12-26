// #!/usr/bin/env rust

pub struct Midi2ProtocolType {
    protocol_type: u8, // midi2_ci_protocol_bytes
    version: u8,       // midi2_ci_protocol_values
    extensions: u8,    // midi2_ci_protocol_extensions (flags)
    reserved1: u8,
    reserved2: u8,
}

// impl MidiCiStandardFormat {
//     fn new() -> MidiCiStandardFormat {
//         sysex_start = MIDI_CI_MSG::SYSTEM_EXCLUSIVE_START;
//         universal_sysex = MIDI_CI_MSG::UNIVERSAL_SYSTEM_EXCLUSIVE;
//         sysex_sub_id_1 = MIDI_CI_MSG::SUB_ID_1_MIDI_CI;
//         sysex_end = MIDI_CI_MSG::END_UNIVERSAL_SYSTEM_EXCLUSIVE;
//     }
// }

// impl MidiCiDiscoveryInquiry for MidiCiStandardFormat {
//     fn discovery_message() -> MidiCiStandardFormat {
//         self.compose_midi_ci_msg(
//             SUB_ID_2_DISCOVERY_INQUIRY,
//             BROADCAST_MUID,
//             [0xFF, 0xFF, 0xFF, 0xFF],
//         );
//     }
// }
// pub mod midi {

pub mod midi_ci_msg {
    pub const SYSTEM_EXCLUSIVE_START: u8 = 0xF0;
    pub const UNIVERSAL_SYSTEM_EXCLUSIVE: u8 = 0x7E;
    pub const END_UNIVERSAL_SYSTEM_EXCLUSIVE: u8 = 0xF7;
    pub const TYPE_UTILITY: u8 = 0;
    pub const TYPE_SYSTEM: u8 = 1;
    pub const TYPE_MIDI_1_CHANNEL: u8 = 2;
    pub const TYPE_SYSEX7: u8 = 3;
    pub const TYPE_MIDI_2_CHANNEL: u8 = 4;
    pub const TYPE_SYSEX8_MDS: u8 = 5;
    pub const PROTOCOL_BYTES_TYPE: u8 = 1;
    pub const PROTOCOL_BYTES_VERSION: u8 = 2;
    pub const PROTOCOL_BYTES_EXTENSIONS: u8 = 3;
    pub const BROADCAST_MUID: u32 = 0xFFFFFF;
    pub const PROTOCOL_TYPE_MIDI1: u8 = 0x01;
    pub const PROTOCOL_TYPE_MIDI2: u8 = 0x02;
    pub const PROTOCOL_EXTENSIONS_JITTER: u8 = 1;
    pub const PROTOCOL_EXTENSIONS_LARGER: u8 = 2; // only for MIDI 1.0 compat UMP
    pub const SUB_ID_1_MIDI_CI: u8 = 0xD;
    pub const SUB_ID_2_DISCOVERY_INQUIRY: u8 = 0x70;
    pub const SUB_ID_2_DISCOVERY_REPLY: u8 = 0x71;
    pub const SUB_ID_2_INVALIDATE_MUID: u8 = 0x7E;
    pub const SUB_ID_2_NAK: u8 = 0x7F;
    pub const SUB_ID_2_PROTOCOL_NEGOTIATION_INQUIRY: u8 = 0x10;
    pub const SUB_ID_2_PROTOCOL_NEGOTIATION_REPLY: u8 = 0x11;
    pub const SUB_ID_2_SET_NEW_PROTOCOL: u8 = 0x12;
    pub const SUB_ID_2_TEST_NEW_PROTOCOL_I2R: u8 = 0x13;
    pub const SUB_ID_2_TEST_NEW_PROTOCOL_R2I: u8 = 0x14;
    pub const SUB_ID_2_CONFIRM_NEW_PROTOCOL_ESTABLISHED: u8 = 0x15;
    pub const SUB_ID_2_PROFILE_INQUIRY: u8 = 0x20;
    pub const SUB_ID_2_PROFILE_INQUIRY_REPLY: u8 = 0x21;
    pub const SUB_ID_2_SET_PROFILE_ON: u8 = 0x22;
    pub const SUB_ID_2_SET_PROFILE_OFF: u8 = 0x23;
    pub const SUB_ID_2_PROFILE_ENABLED_REPORT: u8 = 0x24;
    pub const SUB_ID_2_PROFILE_DISABLED_REPORT: u8 = 0x25;
    pub const SUB_ID_2_PROFILE_SPECIFIC_DATA: u8 = 0x2F;
    pub const SUB_ID_2_PROPERTY_CAPABILITIES_INQUIRY: u8 = 0x30;
    pub const SUB_ID_2_PROPERTY_CAPABILITIES_REPLY: u8 = 0x31;
    pub const SUB_ID_2_PROPERTY_HAS_DATA_INQUIRY: u8 = 0x32;
    pub const SUB_ID_2_PROPERTY_HAS_DATA_REPLY: u8 = 0x33;
    pub const SUB_ID_2_PROPERTY_GET_DATA_INQUIRY: u8 = 0x34;
    pub const SUB_ID_2_PROPERTY_GET_DATA_REPLY: u8 = 0x35;
    pub const SUB_ID_2_PROPERTY_SET_DATA_INQUIRY: u8 = 0x36;
    pub const SUB_ID_2_PROPERTY_SET_DATA_REPLY: u8 = 0x37;
    pub const SUB_ID_2_PROPERTY_SUBSCRIBE: u8 = 0x38;
    pub const SUB_ID_2_PROPERTY_SUBSCRIBE_REPLY: u8 = 0x39;
    pub const SUB_ID_2_PROPERTY_NOTIFY: u8 = 0x3F;
    pub const PROTOCOL_NEGOTIATION_SUPPORTED: u8 = 2;
    pub const PROFILE_CONFIGURATION_SUPPORTED: u8 = 4;
    pub const PROPERTY_EXCHANGE_SUPPORTED: u8 = 8;
    pub const MIDI_CI_VERSION_1_1: u8 = 0x01;
}