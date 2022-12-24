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
    // MIDI 2.0 UMP Section 3.
    pub const TYPE_UTILITY: u8 = 0;
    pub const TYPE_SYSTEM: u8 = 1;
    pub const TYPE_MIDI_1_CHANNEL: u8 = 2;
    pub const TYPE_SYSEX7: u8 = 3;
    pub const TYPE_MIDI_2_CHANNEL: u8 = 4;
    pub const TYPE_SYSEX8_MDS: u8 = 5;
    
    pub const PROTOCOL_BYTES_TYPE: u8 = 1;
    pub const PROTOCOL_BYTES_VERSION: u8 = 2;
    pub const PROTOCOL_BYTES_EXTENSIONS: u8 = 3;
    
    pub const BROADCAST_MUID: u32 = 0x7F7F7F7F;
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

pub mod midi_2_msg{

    pub const MIDI_2_STATUS_NOTE_OFF: u8 = 0x80;
    pub const MIDI_2_STATUS_NOTE_ON: u8 = 0x90;
    pub const MIDI_2_STATUS_PAF: u8 = 0xA0;
    pub const MIDI_2_STATUS_CC: u8 = 0xB0;
    pub const MIDI_2_STATUS_PROGRAM: u8 = 0xC0;
    pub const MIDI_2_STATUS_CAF: u8 = 0xD0;
    pub const MIDI_2_STATUS_PITCH_BEND: u8 = 0xE0;
    pub const MIDI_2_STATUS_PER_NOTE_RCC: u8 = 0x00;
    pub const MIDI_2_STATUS_PER_NOTE_ACC: u8 = 0x10;
    pub const MIDI_2_STATUS_RPN: u8 = 0x20;
    pub const MIDI_2_STATUS_NRPN: u8 = 0x30;
    pub const MIDI_2_STATUS_RELATIVE_RPN: u8 = 0x40;
    pub const MIDI_2_STATUS_RELATIVE_NRPN: u8 = 0x50;
    pub const MIDI_2_STATUS_PER_NOTE_PITCH_BEND: u8 = 0x60;
    pub const MIDI_2_STATUS_PER_NOTE_MANAGEMENT: u8 = 0xF0;
    pub const MIDI_2_CC_BANK_SELECT: u8 = 0x00;
    pub const MIDI_2_CC_MODULATION: u8 = 0x01;
    pub const MIDI_2_CC_BREATH: u8 = 0x02;
    pub const MIDI_2_CC_FOOT: u8 = 0x04;
    pub const MIDI_2_CC_PORTAMENTO_TIME: u8 = 0x05;
    pub const MIDI_2_CC_DTE_MSB: u8 = 0x06;
    pub const MIDI_2_CC_VOLUME: u8 = 0x07;
    pub const MIDI_2_CC_BALANCE: u8 = 0x08;
    pub const MIDI_2_CC_PAN: u8 = 0x0A;
    pub const MIDI_2_CC_EXPRESSION: u8 = 0x0B;
    pub const MIDI_2_CC_EFFECT_CONTROL_1: u8 = 0x0C;
    pub const MIDI_2_CC_EFFECT_CONTROL_2: u8 = 0x0D;
    pub const MIDI_2_CC_GENERAL_1: u8 = 0x10;
    pub const MIDI_2_CC_GENERAL_2: u8 = 0x11;
    pub const MIDI_2_CC_GENERAL_3: u8 = 0x12;
    pub const MIDI_2_CC_GENERAL_4: u8 = 0x13;
    pub const MIDI_2_CC_BANK_SELECT_LSB: u8 = 0x20;
    pub const MIDI_2_CC_MODULATION_LSB: u8 = 0x21;
    pub const MIDI_2_CC_BREATH_LSB: u8 = 0x22;
    pub const MIDI_2_CC_FOOT_LSB: u8 = 0x24;
    pub const MIDI_2_CC_PORTAMENTO_TIME_LSB: u8 = 0x25;
    pub const MIDI_2_CC_DTE_LSB: u8 = 0x26;
    pub const MIDI_2_CC_VOLUME_LSB: u8 = 0x27;
    pub const MIDI_2_CC_BALANCE_LSB: u8 = 0x28;
    pub const MIDI_2_CC_PAN_LSB: u8 = 0x2A;
    pub const MIDI_2_CC_EXPRESSION_LSB: u8 = 0x2B;
    pub const MIDI_2_CC_EFFECT1_LSB: u8 = 0x2C;
    pub const MIDI_2_CC_EFFECT2_LSB: u8 = 0x2D;
    pub const MIDI_2_CC_GENERAL_1_LSB: u8 = 0x30;
    pub const MIDI_2_CC_GENERAL_2_LSB: u8 = 0x31;
    pub const MIDI_2_CC_GENERAL_3_LSB: u8 = 0x32;
    pub const MIDI_2_CC_GENERAL_4_LSB: u8 = 0x33;
    pub const MIDI_2_CC_HOLD: u8 = 0x40;
    pub const MIDI_2_CC_PORTAMENTO_SWITCH: u8 = 0x41;
    pub const MIDI_2_CC_SOSTENUTO: u8 = 0x42;
    pub const MIDI_2_CC_SOFT_PEDAL: u8 = 0x43;
    pub const MIDI_2_CC_LEGATO: u8 = 0x44;
    pub const MIDI_2_CC_HOLD_2: u8 = 0x45;
    pub const MIDI_2_CC_SOUND_CONTROLLER_1: u8 = 0x46;
    pub const MIDI_2_CC_SOUND_CONTROLLER_2: u8 = 0x47;
    pub const MIDI_2_CC_SOUND_CONTROLLER_3: u8 = 0x48;
    pub const MIDI_2_CC_SOUND_CONTROLLER_4: u8 = 0x49;
    pub const MIDI_2_CC_SOUND_CONTROLLER_5: u8 = 0x4A;
    pub const MIDI_2_CC_SOUND_CONTROLLER_6: u8 = 0x4B;
    pub const MIDI_2_CC_SOUND_CONTROLLER_7: u8 = 0x4C;
    pub const MIDI_2_CC_SOUND_CONTROLLER_8: u8 = 0x4D;
    pub const MIDI_2_CC_SOUND_CONTROLLER_9: u8 = 0x4E;
    pub const MIDI_2_CC_SOUND_CONTROLLER_10: u8 = 0x4F;
    pub const MIDI_2_CC_GENERAL_5: u8 = 0x50;
    pub const MIDI_2_CC_GENERAL_6: u8 = 0x51;
    pub const MIDI_2_CC_GENERAL_7: u8 = 0x52;
    pub const MIDI_2_CC_GENERAL_8: u8 = 0x53;
    pub const MIDI_2_CC_PORTAMENTO_CONTROL: u8 = 0x54;
    pub const MIDI_2_CC_EFFECT_1: u8 = 0x5B;
    pub const MIDI_2_CC_EFFECT_2: u8 = 0x5C;
    pub const MIDI_2_CC_EFFECT_3: u8 = 0x5D;
    pub const MIDI_2_CC_EFFECT_4: u8 = 0x5E;
    pub const MIDI_2_CC_EFFECT_5: u8 = 0x5F;
    pub const MIDI_2_CC_DTE_INCREMENT: u8 = 0x60;
    pub const MIDI_2_CC_DTE_DECREMENT: u8 = 0x61;
    pub const MIDI_2_CC_NRPN_LSB: u8 = 0x62;
    pub const MIDI_2_CC_NRPN_MSB: u8 = 0x63;
    pub const MIDI_2_CC_RPN_LSB: u8 = 0x64;
    pub const MIDI_2_CC_RPN_MSB: u8 = 0x65;
    pub const MIDI_2_CC_ALL_SOUND_OFF: u8 = 0x78;
    pub const MIDI_2_CC_RESET_ALL_CONTROLLERS: u8 = 0x79;
    pub const MIDI_2_CC_LOCAL_CONTROL: u8 = 0x7A;
    pub const MIDI_2_CC_ALL_NOTES_OFF: u8 = 0x7B;
    pub const MIDI_2_CC_OMNI_MODE_OFF: u8 = 0x7C;
    pub const MIDI_2_CC_OMNI_MODE_ON: u8 = 0x7D;
    pub const MIDI_2_CC_POLY_MODE_ON_OFF: u8 = 0x7E;
    pub const MIDI_2_CC_POLY_MODE_ON: u8 = 0x7F;
    pub const MIDI_2_RPN_PITCH_BEND_SENSITIVITY: u8 = 0;
    pub const MIDI_2_RPN_FINE_TUNING: u8 = 1;
    pub const MIDI_2_RPN_COARSE_TUNING: u8 = 2;
    pub const MIDI_2_RPN_TUNING_PROGRAM: u8 = 3;
    pub const MIDI_2_RPN_TUNING_BANK_SELECT: u8 = 4;
    pub const MIDI_2_RPN_MODULATION_DEPTH: u8 = 5;
    pub const MIDI_2_META_SEQUENCE_NUMBER: u8 = 0x00;
    pub const MIDI_2_META_TEXT: u8 = 0x01;
    pub const MIDI_2_META_COPYRIGHT: u8 = 0x02;
    pub const MIDI_2_META_TRACK_NAME: u8 = 0x03;
    pub const MIDI_2_META_INSTRUMENT_NAME: u8 = 0x04;
    pub const MIDI_2_META_LYRIC: u8 = 0x05;
    pub const MIDI_2_META_MARKER: u8 = 0x06;
    pub const MIDI_2_META_CUE: u8 = 0x07;
    pub const MIDI_2_META_CHANNEL_PREFIX: u8 = 0x20;
    pub const MIDI_2_META_END_OF_TRACK: u8 = 0x2F;
    pub const MIDI_2_META_TEMPO: u8 = 0x51;
    pub const MIDI_2_META_SMPTE_OFFSET: u8 = 0x54;
    pub const MIDI_2_META_TIME_SIGNATURE: u8 = 0x58;
    pub const MIDI_2_META_KEY_SIGNATURE: u8 = 0x59;
    pub const MIDI_2_META_SEQUENCER_SPECIFIC: u8 = 0x7F;
    pub const MIDI_2_PER_NOTE_MANAGEMENT_RESET: u8 = 1;
    pub const MIDI_2_PER_NOTE_MANAGEMENT_DETACH: u8 = 2;
    pub const MIDI_2_ATTRIBUTE_TYPE_NONE: u8 = 0;
    pub const MIDI_2_ATTRIBUTE_TYPE_MANUFACTURER: u8 = 1;
    pub const MIDI_2_ATTRIBUTE_TYPE_PROFILE: u8 = 2;
    pub const MIDI_2_ATTRIBUTE_TYPE_PITCH7_9: u8 = 3;
    pub const MIDI_2_PROGRAM_CHANGE_OPTION_NONE: u8 = 0;
    pub const MIDI_2_PROGRAM_CHANGE_OPTION_BANK_VALID: u8 = 1;
    pub const MIDI_2_SYSEX_IN_ONE_UMP: u8 = 0;
    pub const MIDI_2_SYSEX_START: u8 = 0x10;
    pub const MIDI_2_SYSEX_CONTINUE: u8 = 0x20;
    pub const MIDI_2_SYSEX_END: u8 = 0x30;
    pub const MIDI_2_MIXED_DATA_STATUS_HEADER: u8 = 0x80;
    pub const MIDI_2_MIXED_DATA_STATUS_PAYLOAD: u8 = 0x90;
    pub const MIDI_2_SYSTEM_STATUS_MIDI_TIME_CODE: u8 = 0xF1;
    pub const MIDI_2_SYSTEM_STATUS_SONG_POSITION: u8 = 0xF2;
    pub const MIDI_2_SYSTEM_STATUS_SONG_SELECT: u8 = 0xF3;
    pub const MIDI_2_SYSTEM_STATUS_TUNE_REQUEST: u8 = 0xF6;
    pub const MIDI_2_SYSTEM_STATUS_TIMING_CLOCK: u8 = 0xF8;
    pub const MIDI_2_SYSTEM_STATUS_START: u8 = 0xFA;
    pub const MIDI_2_SYSTEM_STATUS_CONTINUE: u8 = 0xFB;
    pub const MIDI_2_SYSTEM_STATUS_STOP: u8 = 0xFC;
    pub const MIDI_2_SYSTEM_STATUS_ACTIVE_SENSING: u8 = 0xFE;
    pub const MIDI_2_SYSTEM_STATUS_RESET: u8 = 0xFF;
    pub const MIDI_2_JR_CLOCK: u8 = 0x10;
    pub const MIDI_2_JR_TIMESTAMP: u8 = 0x20;
}
