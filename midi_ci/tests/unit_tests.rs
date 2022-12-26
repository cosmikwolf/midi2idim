use std::string::ParseError;

// extern crate midi2idim;
// extern crate bincode;
// extern crate nom;
// use bincode::{config, Decode, Encode};
// use midi2idim::midi_ci::decode::hex_string_decode;
use midi2idim::midi_ci::defs::midi_ci_msg::*;
use midi2idim::midi_ci::MidiDevice;

#[macro_use]
extern crate nom;
use nom::IResult;
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
// use nom::bits::{bits, streaming::take};
use nom::bytes::{streaming::tag, streaming::take_until1};

#[derive(Debug, PartialEq)]
struct A<'a> {
    a: &'a [u8],
    b: &'a [u8],
}
fn extract_message_data_proto<'a>(i: &'a [u8], t: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    Ok((i, t))
}

fn parse_midi_message_proto<'a>(
    i: &'a [u8],
    start: &'a [u8],
    end: &'a [u8],
) -> IResult<&'a [u8], A<'a>> {
    let (i, _) = tag(start)(i)?;
    let (i, t) = take_until1(end)(i)?;
    let (i, b) = extract_message_data_proto(i, t)?;
    return Ok((i, A { a: start, b }));
}

#[test]
fn test_parse_midi_message_proto() {
    let original: &[u8] = &[0xF0, 0x01, 0x02, 0x03, 0xF7];
    let message_data: &[u8] = &[0x01, 0x02, 0x03];
    let start: &[u8] = &[0xF0];
    let end: &[u8] = &[0xF7];
    let result = parse_midi_message_proto(original, start, end);
    assert_eq!(
        result,
        Ok((
            end,
            A {
                a: start,
                b: message_data
            }
        ))
    );
}

// extern crate nom;
// use nom::{
//     // bytes::complete::tag;
//     bits::{bits, bytes},
//     bytes::streaming::{tag, take_until, take},
//     IResult
// };
// use nom::bits::bits;
// use nom::bits::bytes;

use nom::bytes::streaming;
// use nom::error::Error;
use nom::sequence::delimited;
const sysex_start: u8 = 0xF7;
const sysex_end_arr: &[u8] = &[0xF7];
const sysex_end: u8 = 0xF7;

// const end: &[u8] = &[sysex_end];

// fn parse(input: &[u8]) -> IResult<&[u8], (u8, u8, &[u8])> {}
// use midi2idim::midi_ci::midi_1::parse_midi_message;
fn parser<'a>(input: &'a [u8], start: &[u8], end: &[u8]) -> IResult<&'a [u8], &'a [u8]> {
    let message_data: &[u8] = &[0x01, 0x02, 0x03];
    delimited(
        streaming::tag(start),
        streaming::tag(message_data),
        streaming::tag(end),
    )(input)
}
#[test]
fn test_parser() {
    let original: &[u8] = &[0xF0, 0x01, 0x02, 0x03, 0xF7];
    let empty_bitseq: &[u8] = &[];
    let expected_result: &[u8] = &[0x01, 0x02, 0x03];
    let start: &[u8] = &[0xF0];
    let end: &[u8] = &[0xF7];
    // let result = parse_midi_message(original, start, end);
    assert_eq!(
        parser(original, start, end),
        Ok((empty_bitseq, expected_result))
    );
}

fn midiparser<'a>(input: &'a [u8], start: &[u8], end: &[u8]) -> IResult<&'a [u8], &'a [u8]> {
    delimited(
        streaming::tag(start),
        streaming::take_until(end),
        streaming::tag(end),
    )(input)
}
#[test]
fn test_parse_midi_message() {
    let original: &[u8] = &[0xF0, 0x01, 0x02, 0x03, 0xF7];
    let empty_bitseq: &[u8] = &[];
    let expected_result: &[u8] = &[0x01, 0x02, 0x03];
    let start: &[u8] = &[0xF0];
    let end: &[u8] = &[0xF7];
    // let result = parse_midi_message(original, start, end);
    assert_eq!(
        midiparser(original, start, end),
        Ok((empty_bitseq, expected_result))
    );
}
