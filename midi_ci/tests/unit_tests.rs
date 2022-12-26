// extern crate midi2idim;
// extern crate bincode;
// extern crate nom;
// use bincode::{config, Decode, Encode};
// use midi2idim::midi_ci::decode::hex_string_decode;
use midi2idim::midi_ci::defs::midi_ci_msg::*;
use midi2idim::midi_ci::MidiDevice;

use nom::{
    bytes::complete::{
        tag
    },
    IResult,
};
// use midi2idim::midi_1::
// use bus::Bus;
// https://docs.rs/bus/latest/bus/
const TEST_DEVICE_ID: u8 = 0x42;
const TEST_DEVICE_MFG: u32 = 0x69;
const TEST_DEVICE_FAMILY: u16 = 0xAB;
const TEST_DEVICE_FAMILY_MODEL: u16 = 0xBC;
const TEST_SOFTWARE_REVISION_LEVEL: u32 = 0xBADAF007;
const TEST_VERSION_FORMAT: u8 = MIDI_CI_VERSION_1_1;
const TEST_RECEIVABLE_MAXIMUM_SYSEX_MESSAGE_SIZE: u32 = 128;

fn init_test_midi_device() -> MidiDevice {
    return MidiDevice::new(
        TEST_DEVICE_ID,
        TEST_DEVICE_MFG,
        TEST_VERSION_FORMAT,
        TEST_DEVICE_FAMILY,
        TEST_DEVICE_FAMILY_MODEL,
        TEST_SOFTWARE_REVISION_LEVEL,
        TEST_RECEIVABLE_MAXIMUM_SYSEX_MESSAGE_SIZE,
    );
}

// fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
//     Ok(u8::from_str_radix(input, 16));
//   }

// fn is_hex_digit(c: char) -> bool {
//     c.is_digit(16)
//   }

// const DISCOVERY_MESSAGE_INQUIRY_LITERAL:&str = "0xF07E7F0D70FFFFFFFFFF";
// #[test]
fn test_discovery_msg() {
    let device = init_test_midi_device();
    let device2 = init_test_midi_device();
    assert_eq!(device.muid, device2.muid);
    // let discovery_msg = device.build_discovery_inquiry_message();
    // assert_eq!(encoded, hex_string_decode(DISCOVERY_MESSAGE_INQUIRY_LITERAL));
}

// pub fn parse_midi_bytes(i: &[u8]) -> IResult<&[u8], &[u8]> {
//     // tag([0xF0])(i)
//     Ok(i);
// }

#[test]
fn test_as_bytes() {
    use crate::midi_ci::parser::universal_sysex;
    const broadcast_device_inquery: &[u8] = &[0xF0, 0x7E, 0xF7, 0x06, 0x01, 0xF7];
    const expected_remaining_message: &[u8] = &[0x7E, 0xF7, 0x06, 0x01];
    assert_eq!(broadcast_device_inquery, &[0xF0, 0x7E, 0xF7, 0x06, 0x01, 0xF7])
    assert_eq!(universal_sysex(broadcast_device_inquery), Ok((expected_remaining_message, broadcast_device_inquery)))
    // let string_val = "0xFF";
    // let result = hex_string_decode(string_val);
    // assert_eq!(result, 255);
}

// fn generate_midi_1_

// MIDI 1.0 GENERAL MIDI SYSTEM ON MESSAGE
// Pagee 52 M1_v4-2-1_MIDI_1-0_Detailed_Specification_96-1-4.pdf
//
// F0 7E <device ID> 09 01 F7
//
// F0 7E           Universal Non-Real Time SysEx header
// <device ID>     ID of target device (suggest using 7F ‘All Call’)
// 09              sub-ID#1 = General MIDI message
// 01              sub-ID#2 = General MIDI On
// F7              EOX
//
// MIDI 1.0 GENERAL MIDI SYSTEM OFF
// F0 7E <device ID> 09 02 F7
