use std::string::ParseError;

// extern crate midi2idim;
// extern crate bincode;
// extern crate nom;
// use bincode::{config, Decode, Encode};
// use midi2idim::midi_ci::decode::hex_string_decode;
use midi2idim::midi_ci::defs::MidiCi;
use midi2idim::midi_ci::parser::parse_u7lsb_to_u32;
use midi2idim::midi_ci::{MidiDevice, u32_to_u7lsb};

#[macro_use]
extern crate nom;
use nom::{bits, IResult};
// use midi2idim::midi_1::
// use bus::Bus;
// https://docs.rs/bus/latest/bus/
const TEST_DEVICE_ID: u8 = 0x42;
const TEST_DEVICE_MFG: u32 = 0x69;
const TEST_DEVICE_FAMILY: u16 = 0xAB;
const TEST_DEVICE_FAMILY_MODEL: u16 = 0xBC;
const TEST_SOFTWARE_REVISION_LEVEL: u32 = 0xBADAF007;
const TEST_VERSION_FORMAT: u8 = MidiCi::MIDI_CI_VERSION_1_1;
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
use nom::branch::alt;
use nom::sequence::delimited;
use nom::sequence::tuple;
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

fn test_midi_parser_1<'a>(
    input: &'a [u8],
    start: &[u8],
    end: &[u8],
) -> IResult<&'a [u8], &'a [u8]> {
    delimited(
        streaming::tag(start),
        streaming::take_until(end),
        streaming::tag(end),
    )(input)
}

fn test_midi_parser_2<'a>(
    input: &'a [u8],
    start: &[u8],
    end: &[u8],
) -> IResult<&'a [u8], (&'a [u8], &'a [u8])> {
    tuple((streaming::tag(start), streaming::take_until(end)))(input)
}
fn test_midi_parser_3<'a>(
    input: &'a [u8],
    start: &[u8],
    end: &[u8],
) -> IResult<&'a [u8], (&'a [u8], &'a [u8], &'a [u8])> {
    tuple((
        streaming::tag(start),
        streaming::take(1u8),       // status byte
        streaming::take_until(end), // message data
    ))(input)
}

use midi2idim::midi_ci::MidiCiMessage;
use midi2idim::midiv1::message_type;

fn parse_midi_message_type<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], (&'a [u8], &'a [u8], &'a [u8])> {
    tuple((
        streaming::tag(&[message_type::SYSEX_START]),
        streaming::take(1u8), // status byte
        streaming::take_until(&[message_type::SYSEX_END] as &[u8]), // message data
    ))(input)
}
const msg: &[u8] = &[message_type::SYSEX_START];

#[test]
fn test_parse_midi_message() {
    let original: &[u8] = &[0xF0, 0x01, 0x02, 0x03, 0xF7];
    let empty_bitseq: &[u8] = &[];
    let expected_result: &[u8] = &[0x01, 0x02, 0x03];
    let start: &[u8] = &[0xF0];
    let end: &[u8] = &[0xF7];
    // let result = parse_midi_message(original, start, end);
    assert_eq!(
        test_midi_parser_1(original, start, end),
        Ok((empty_bitseq, expected_result))
    );
    assert_eq!(
        test_midi_parser_2(original, start, end),
        Ok((end, (start, expected_result)))
    );
    assert_eq!(
        test_midi_parser_3(original, start, end),
        Ok((end, (start, &expected_result[0..1], &expected_result[1..])))
    );
    assert_eq!(
        parse_midi_message_type(original),
        Ok((end, (start, &expected_result[0..1], &expected_result[1..])))
    );
}
#[test]
fn test_u16_u7lsb_conversion() {
    use midi2idim::midi_ci::parser::parse_u7lsb_to_u16;
    use midi2idim::midi_ci::u16_to_u7lsb;
    let u7lsb_1: &[u8] = &[0x00, 0x00];
    let u16_1: u16 = 0x0000;
    let u7lsb_2: &[u8] = &[0x7F, 0x00];
    let u16_2: u16 = 0x7F;
    let u7lsb_3: &[u8] = &[0x7f, 0x7f];
    let u16_3: u16 = 0x3FFF;
    let u7lsb_4: &[u8] = &[0x00, 0x01];
    let u16_4: u16 = 0x80;
    let u7lsb_5: &[u8] = &[0x01, 0x00];
    let u16_5: u16 = 0x01;
    assert_eq!(parse_u7lsb_to_u16(u7lsb_1),Ok((u7lsb_1,{u16_1})));
    assert_eq!(parse_u7lsb_to_u16(u7lsb_2),Ok((u7lsb_2,{u16_2})));
    assert_eq!(parse_u7lsb_to_u16(u7lsb_3),Ok((u7lsb_3,{u16_3})));
    assert_eq!(parse_u7lsb_to_u16(u7lsb_4),Ok((u7lsb_4,{u16_4})));
    assert_eq!(parse_u7lsb_to_u16(u7lsb_5),Ok((u7lsb_5,{u16_5})));
    assert_eq!(&u16_to_u7lsb(u16_1),u7lsb_1 );
    assert_eq!(&u16_to_u7lsb(u16_2),u7lsb_2 );
    assert_eq!(&u16_to_u7lsb(u16_3),u7lsb_3 );
    assert_eq!(&u16_to_u7lsb(u16_4),u7lsb_4 );
    assert_eq!(&u16_to_u7lsb(u16_5),u7lsb_5 );
}
#[test]
fn test_u32_to_u7lsb() {
    use midi2idim::midi_ci::u32_to_u7lsb;
    let u7lsb_1: &[u8] = &[0x7f, 0x7f, 0x7f, 0x7f];
    let u32_1: u32 = MidiCi::BROADCAST_MUID;
    let u7lsb_2: &[u8] = &[0x7f, 0x00, 0x00, 0x00];
    let u32_2: u32 = 0x7F;
    let u7lsb_3: &[u8] = &[0x7f, 0x7f, 0x00, 0x00];
    let u32_3: u32 = 0x3FFF;
    let u7lsb_4: &[u8] = &[0x00, 0x01, 0x00, 0x00];
    let u32_4: u32 = 0x80;
    let u7lsb_5: &[u8] = &[0x7f, 0x7f, 0x7f, 0x00];
    let u32_5: u32 = 0x1FFFFF;
    assert_eq!(&u32_to_u7lsb(u32_1),u7lsb_1 );
    assert_eq!(&u32_to_u7lsb(u32_2),u7lsb_2 );
    assert_eq!(&u32_to_u7lsb(u32_3),u7lsb_3 );
    assert_eq!(&u32_to_u7lsb(u32_4),u7lsb_4 );
    assert_eq!(&u32_to_u7lsb(u32_5),u7lsb_5 );
}

#[test]
fn test_parse_u7lsb_to_u32() {
    let u7lsb_1: &[u8] = &[0x7f, 0x7f, 0x7f, 0x7f];
    let u32_1 = MidiCi::BROADCAST_MUID;
    let u7lsb_2: &[u8] = &[0x7f, 0x00, 0x00, 0x00];
    let u32_2 = 0x7F;
    let u7lsb_3: &[u8] = &[0x7f, 0x7f, 0x00, 0x00];
    let u32_3 = 0x3FFF;
    let u7lsb_4: &[u8] = &[0x00, 0x01, 0x00, 0x00];
    let u32_4 = 0x80;
    let u7lsb_5: &[u8] = &[0x7f, 0x7f, 0x7f, 0x00];
    let u32_5 = 0x1FFFFF;
    use midi2idim::midi_ci::parser::parse_u7lsb_to_u32;
    assert_eq!(parse_u7lsb_to_u32(u7lsb_1), Ok((u7lsb_1, { u32_1 })));
    assert_eq!(parse_u7lsb_to_u32(u7lsb_2), Ok((u7lsb_2, { u32_2 })));
    assert_eq!(parse_u7lsb_to_u32(u7lsb_3), Ok((u7lsb_3, { u32_3 })));
    assert_eq!(parse_u7lsb_to_u32(u7lsb_4), Ok((u7lsb_4, { u32_4 })));
    assert_eq!(parse_u7lsb_to_u32(u7lsb_5), Ok((u7lsb_5, { u32_5 })));
}
#[test]
fn test_universal_sysex_header() {
    let device = init_test_midi_device();
    let discovery_inquiry_header: &[u8] = &[
        MidiCi::SYSTEM_EXCLUSIVE_START,
        MidiCi::UNIVERSAL_SYSTEM_EXCLUSIVE,
        device.device_id,
        MidiCi::SUB_ID_1_MIDI_CI,
        MidiCi::SUB_ID_2_DISCOVERY_INQUIRY,
        MidiCi::MIDI_CI_VERSION_1_1,
        u32_to_u7lsb(device.muid)[0],
        u32_to_u7lsb(device.muid)[1],
        u32_to_u7lsb(device.muid)[2],
        u32_to_u7lsb(device.muid)[3],
    ];

    assert_eq!(
        &device.build_discovery_inquiry_message().serialize()[0..10],
        discovery_inquiry_header);

    // parse_universal_sysex
}

fn parse_universal_sysex<'a>(input: &'a [u8]) -> IResult<&'a [u8], MidiCiMessage> {
    let (
        input,
        (
            _start,
            device_id,
            _sysex_sub_id_1,
            sysex_sub_id_2,
            version_format,
            source_muid,
            destination_muid,
            data,
        ),
    ) = tuple((
        streaming::tag(&[
            message_type::SYSEX_START,
            MidiCi::UNIVERSAL_SYSTEM_EXCLUSIVE,
        ]),
        streaming::take(1u8),                                       // device ID
        streaming::take(1u8),                                       // SubID#1
        streaming::take(1u8),                                       // SubID#2
        streaming::take(1u8),                                       // Message Format
        streaming::take(1u8),                                       // Source MUID
        streaming::take(1u8),                                       // Destination MUID
        streaming::take_until(&[message_type::SYSEX_END] as &[u8]), // message data bytes
    ))(input)?;

    let parsed_source_muid = parse_u7lsb_to_u32(source_muid);
    let parsed_source_muid = parsed_source_muid.unwrap().1;
    let parsed_destination_muid = parse_u7lsb_to_u32(destination_muid);
    let parsed_destination_muid = parsed_destination_muid.unwrap().1;
    Ok((
        input,
        MidiCiMessage::new(
            device_id[0],
            sysex_sub_id_2[0],
            version_format[0],
            parsed_source_muid,
            parsed_destination_muid,
            data.to_vec()
        ),
    ))
}


#[test]
fn test_parse_universal_sysex(){
    assert!(false)
}
// #[test]
// fn test_discovery_msg() {
//     let device2 = init_test_midi_device();
//     assert_eq!(device.muid, device2.muid);
// let discovery_msg = device.build_discovery_inquiry_message();
// assert_eq!(encoded, hex_string_decode(DISCOVERY_MESSAGE_INQUIRY_LITERAL));
// }

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
