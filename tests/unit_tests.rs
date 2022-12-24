// extern crate midi2idim;
// extern crate bincode;
// extern crate nom;
use bincode::{config, Decode, Encode};
use midi2idim::midi_ci::decode::hex_string_decode;
use midi2idim::midi_ci::MidiDevice;
use midi2idim::midi_ci::constants_definitions::midi_ci_msg::*;
use bus::Bus;

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
        TEST_DEVICE_ID, TEST_DEVICE_MFG,
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

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
  }

#[test]
fn test_bus() {
    let mut bus = Bus::new(10);
    let mut rx1 = bus.add_rx();
    let mut rx2 = bus.add_rx();
    bus.broadcast("Hello");
    assert_eq!(rx1.recv(), Ok("Hello"));
    assert_eq!(rx2.recv(), Ok("Hello"));
}

const DISCOVERY_MESSAGE_INQUIRY_LITERAL:&str = "0xF07E7F0D70FFFFFFFFFF";
#[test]
fn test_discovery_msg() {
    let device = init_test_midi_device();
    let discovery_msg = device.build_discovery_inquiry_message();
    // assert_eq!(encoded, hex_string_decode(DISCOVERY_MESSAGE_INQUIRY_LITERAL));
}   

#[test]
fn test_as_bytes() {
    let string_val = "0xFF";
    let result = hex_string_decode(string_val);
    assert_eq!(result, 255);

}
#[test]
fn discovery_msg() {
    // let midimsg = 

    // assert_eq!(midimsg, 0xF07E7F0D70FFFFFFFFFF);
}

