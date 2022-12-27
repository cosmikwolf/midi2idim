pub mod decode;
pub mod parser;

use serde::{Deserialize, Serialize};
// use hex_string_decode;
use crate::midi_ci::defs::MidiCi;
use bincode::{config, Decode, Encode};
use rand::{thread_rng, Rng};
pub mod defs;

#[derive(Debug)]
pub struct MidiDevice {
    pub muid: u32,
    pub device_id: u8,
    version_format: u8,
    device_manufacturer: u32,
    device_family: u16,
    device_family_model: u16,
    software_revision_level: u32,
    receivable_maximum_sysex_message_size: u32,
}

pub fn u32_to_u7lsb(input: u32) -> [u8;4] {
    let nib1 = input & 0x7f;
    let nib2 = (input & (0x7f << 7) ) << 1;
    let nib3 = (input & (0x7f << 14)) << 2;
    let nib4 = (input & (0x7f << 21)) << 3;
    (nib1 + nib2 + nib3 + nib4).to_le_bytes()
}

pub fn u16_to_u7lsb(input: u16) -> [u8;2] {
    let nib1 = input & 0x7f;
    let nib2 = (input & (0x7f << 7))>>7;
    [nib1 as u8, nib2 as u8]
}

#[derive(Encode, Decode, Debug, PartialEq)]
pub struct MidiCiMessage {
    status_byte: u8,
    sysex_id: u8,
    device_id: u8,
    sysex_sub_id_1: u8,
    sysex_sub_id_2: u8,
    version_format: u8,
    source_muid: u32,
    destination_muid: u32,
    data: Vec<u8>,
}

impl MidiCiMessage {
    pub fn header(&self) -> [u8;14] {
        [   self.status_byte,
            self.sysex_id,
            self.device_id,
            self.sysex_sub_id_1,
            self.sysex_sub_id_2,
            self.version_format,
            u32_to_u7lsb(self.source_muid)[0],
            u32_to_u7lsb(self.source_muid)[1],
            u32_to_u7lsb(self.source_muid)[2],
            u32_to_u7lsb(self.source_muid)[3],
            u32_to_u7lsb(self.destination_muid)[0],
            u32_to_u7lsb(self.destination_muid)[1],
            u32_to_u7lsb(self.destination_muid)[2],
            u32_to_u7lsb(self.destination_muid)[3] ]
    }
    pub fn serialize(&self) -> Vec<u8> {
        // https://stackoverflow.com/questions/40792801/best-way-to-concatenate-vectors-in-rust
        self.header().to_vec().iter().cloned().chain(self.data.iter().cloned()).collect() // Cloned
    }
}
// struct Payload {
//     fn parse_data(self: &Self, data: &[u8]) -> Self;
// }
// impl Payload for GenericPayload {
//     fn parse_data(data: &[u8]) -> Self {
//         GenericPayload {
//             data: data.to_vec()
//         }
//     }
//  }
#[derive(Encode, Decode, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericPayload {
    data: Vec<u8>,

}

impl GenericPayload {
    pub fn new(&self, data:Vec<u8>) -> Self {
        GenericPayload {
            data
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        use core::mem::size_of;
        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding()
            .write_fixed_array_length();
        let mut slice: [u8; size_of::<Self>()] =
            [0u8; size_of::<Self>()];
        bincode::encode_into_slice(&self, &mut slice, config).unwrap();
        slice.to_vec()
    }
}
#[derive(Encode, Decode, Debug, PartialEq, Serialize, Deserialize)]
pub struct NakMsgPayload;

#[derive(Encode, Decode, Debug, PartialEq, Serialize, Deserialize)]
pub struct CiCategory {
    bit_0_reserved: bool,
    protocol_negotiation_supported: bool,
    profile_configuration_supported: bool,
    property_exchange_supported: bool,
    bit_5_reserved: bool,
    bit_6_reserved: bool,
    bit_7_zero: bool,
}

impl Default for CiCategory {
    fn default() -> CiCategory {
        CiCategory {
            bit_0_reserved: false,
            protocol_negotiation_supported: true,
            profile_configuration_supported: false,
            property_exchange_supported: false,
            bit_5_reserved: false,
            bit_6_reserved: false,
            bit_7_zero: false,
        }
    }
}

impl CiCategory {
    fn serialize(&self) -> u8 {
        (1 & self.protocol_negotiation_supported as u8) << 1
        + (1 & self.profile_configuration_supported as u8) << 2 
        + (1 & self.property_exchange_supported as u8) << 3 
    }
}
#[derive(Encode, Decode, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryMessagePayload {
    device_manufacturer: u32,
    device_family: u16,
    device_family_model: u16,
    software_revision_level: u32,
    capability_inquiry_category_supported: CiCategory,
    receivable_maximum_sysex_message_size: u32,
}

impl DiscoveryMessagePayload {
    pub fn new(device: &MidiDevice, ci_category: CiCategory) -> Self {
        DiscoveryMessagePayload {
            device_manufacturer: device.device_manufacturer,
            device_family: device.device_family,
            device_family_model: device.device_family_model,
            software_revision_level: device.software_revision_level,
            capability_inquiry_category_supported: ci_category,
            receivable_maximum_sysex_message_size: device.receivable_maximum_sysex_message_size,
        }
    }
    pub fn serialize(&self) -> [u8;17] {
        [   u32_to_u7lsb(self.device_manufacturer)[0],
            u32_to_u7lsb(self.device_manufacturer)[1],
            u32_to_u7lsb(self.device_manufacturer)[2],
            u32_to_u7lsb(self.device_manufacturer)[3],
            u16_to_u7lsb(self.device_family)[0],
            u16_to_u7lsb(self.device_family)[1],
            u16_to_u7lsb(self.device_family_model)[0],
            u16_to_u7lsb(self.device_family_model)[1],
            u32_to_u7lsb(self.software_revision_level)[0],
            u32_to_u7lsb(self.software_revision_level)[1],
            u32_to_u7lsb(self.software_revision_level)[2],
            u32_to_u7lsb(self.software_revision_level)[3],
            self.capability_inquiry_category_supported.serialize(),
            u32_to_u7lsb(self.receivable_maximum_sysex_message_size)[0],
            u32_to_u7lsb(self.receivable_maximum_sysex_message_size)[1],
            u32_to_u7lsb(self.receivable_maximum_sysex_message_size)[2],
            u32_to_u7lsb(self.receivable_maximum_sysex_message_size)[3]
        ]
    }
}

// 3.2.1 Generating a MUID
fn generate_muid() -> u32 {
    let mut rng = thread_rng();
    return rng.gen_range(0..0x0FFFFFFF);
}

impl MidiCiMessage {
    pub fn new(
        device_id: u8,
        sysex_sub_id_2: u8,
        version_format: u8,
        source_muid: u32,
        destination_muid: u32,
        data: Vec<u8>,
    ) -> Self {
        return MidiCiMessage {
            status_byte: MidiCi::SYSTEM_EXCLUSIVE_START,
            sysex_id: MidiCi::UNIVERSAL_SYSTEM_EXCLUSIVE,
            device_id,
            sysex_sub_id_1: MidiCi::SUB_ID_1_MIDI_CI,
            sysex_sub_id_2,
            version_format,
            source_muid,
            destination_muid,
            data,
        };
    }
}
trait MidiCodec {
    fn encode(&self) -> Vec<u8>;
    // fn decode(&mut self, midibytes: Vec<u8>) -> MidiCiMessage;
}
impl MidiCodec for MidiCiMessage {
    fn encode(&self) -> Vec<u8> {
        let config = config::standard()
        // // pick one of:
        // .with_big_endian()
        // .with_little_endian()
        // // pick one of:
        // .with_variable_int_encoding()
        // .with_fixed_int_encoding()
        // // pick one of:
        // .skip_fixed_array_length()
        // .write_fixed_array_length();
        ;
        return bincode::encode_to_vec(&self, config).unwrap();
    }
}

impl MidiDevice {
    pub fn new(
        device_id: u8,
        device_manufacturer: u32,
        version_format: u8,
        device_family: u16,
        device_family_model: u16,
        software_revision_level: u32,
        receivable_maximum_sysex_message_size: u32,
    ) -> Self {
        MidiDevice {
            device_id,
            device_manufacturer,
            device_family,
            version_format,
            device_family_model,
            software_revision_level,
            muid: generate_muid(),
            receivable_maximum_sysex_message_size,
        }
    }
    pub fn build_midi_ci_msg(
        &self,
        sysex_sub_id_2: u8,
        destination_muid: u32,
        data: Vec<u8>,
    ) -> MidiCiMessage {
        return MidiCiMessage::new(
            self.device_id,
            sysex_sub_id_2,
            self.version_format,
            self.muid,
            destination_muid,
            data,
        );
    }

    // 5.5 Discovery Message
    pub fn build_discovery_inquiry_message(&self) -> MidiCiMessage {
        let mut ci_category: CiCategory = CiCategory::default();
        ci_category.protocol_negotiation_supported = true;
        let data = DiscoveryMessagePayload::new(self, ci_category);
        return self.build_midi_ci_msg(
            MidiCi::SUB_ID_2_DISCOVERY_INQUIRY,
            MidiCi::BROADCAST_MUID,
            data.serialize().to_vec(),
        );
    }
}
/*

impl CapabilityInquiry for MidiDevice {

    fn parse_midi_ci_start(s: &str) -> IResult<&str, &str> {
      tag(&[SYSTEM_EXCLUSIVE_START, UNIVERSAL_SYSTEM_EXCLUSIVE])
    }



    // 5.6 Reply to Discovery Message
    fn send_reply_to_discovery_message(&self) {}

    // 5.7 Invalidate MUID Message
    fn send_invalidate_muid_message(&self) {}

    // 5.8 NAK MIDI - CI Message
    fn send_nak_midi_ci_message(&self) {}

    // 6.1 Protocol Types Supported
    // 6.2 Universal MIDI Packet Required
    // 6.3 Protocol Inquiry and Negotiation Mechanism
    // 6.4 Initiate Protocol Negotiation Message
    // 6.5 Reply to Initiate Protocol Negotiation Message
    // 6.6 Set New Protocol Message
    // 6.7 Test New Protocol Initiator to Responder Message
    // 6.8 Test New Protocol Responder to Initiator Message
    // 6.9 Confirmation New Protocol Established Message
    // 6.10 Subsequent Protocol Negotiation
    // 7.0 PROFILE CONFIGURATION
    // 7.1 Profile Configuration Mechanism
    // 7.2 Profile Inquiry Message
    // 7.3 Reply to Profile Inquiry Message
    // 7.4 Set Profile On Message
    // 7.5 Set Profile Off Message
    // 7.6 Profile Enabled Report Message
    // 7.7 Profile Disabled Report Message
    // 7.8 Profile Specific Data Message
    // 8. PROPERTY EXCHANGE
    // 8.1 Property Inquiry and Negotiation Mechanism
    // 8.2 Property Data May Be Sent in Multiple Chunks
    // 8.2.1 No Chunking of Header Data
    // 8.3 Multiple Simultaneous Inquiries and Request ID
    // 8.4 Inquiry: Property Exchange Capabilities
    // 8.5 Reply to Property Exchange Capabilities
    // 8.6 Inquiry: Has Property Data
    // 8.7 Reply to Has Property Data
    // 8.8 Inquiry: Get Property Data
    // 8.9 Reply to Get Property Data
    // 8.10 Inquiry: Set Property Data
    // 8.11 Reply to Set Property Data
    // 8.12 Subscription
    // 8.13 Reply to Subscription
    // 8.14 Notify Message
    // Appendix A: Minimum Requirements
    // Appendix B: Avoiding Collisions of MUID
    // Appendix C: MIDI Chaining Limitation
    // Appendix D: List of all MIDI-CI Messages
}


*/
