pub mod parser;
pub mod decode;
// use hex_string_decode;
use crate::midi_ci::defs::midi_ci_msg::*;
use bincode::{Decode, Encode, config};
use rand::{thread_rng, Rng};
pub mod defs;


#[derive(Debug)]
pub struct MidiDevice {
    pub muid: u32,
    device_id: u8,
    version_format: u8,
    device_manufacturer: u32,
    device_family: u16,
    device_family_model: u16,
    software_revision_level: u32,
    receivable_maximum_sysex_message_size: u32,
}

#[derive(Encode, Decode, Debug, PartialEq)]
pub struct MidiCiMessage {
    sysex_start: u8,
    universal_sysex: u8,
    device_id: u8,
    sysex_sub_id_1: u8,
    sysex_sub_id_2: u8,
    version_format: u8,
    source_muid: u32,
    destination_muid: u32,
    data: Payload,
    sysex_end: u8,
}
// https://stackoverflow.com/questions/67594909/multiple-possible-types-for-a-serializable-structs-field

#[derive(Encode, Decode, Debug, PartialEq)]
pub enum Payload {
    DiscoveryPayload(DiscoveryPayload),
    NakMsgPayload(NakMsgPayload),
}

#[derive(Encode, Decode, Debug, PartialEq)]
pub struct NakMsgPayload;

#[derive(Encode, Decode, Debug, PartialEq)]
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
            protocol_negotiation_supported: false,
            profile_configuration_supported: false,
            property_exchange_supported: false,
            bit_5_reserved: false,
            bit_6_reserved: false,
            bit_7_zero: false,
        }
    }
}
#[derive(Encode, Decode, Debug, PartialEq)]
pub struct DiscoveryPayload {
    device_manufacturer: u32,
    device_family: u16,
    device_family_model: u16,
    software_revision_level: u32,
    capability_inquiry_category_supported: CiCategory,
    receivable_maximum_sysex_message_size: u32,
}

impl DiscoveryPayload {
    pub fn new(device: &MidiDevice, ci_category: CiCategory) -> Self {
        DiscoveryPayload {
            device_manufacturer: device.device_manufacturer,
            device_family: device.device_family,
            device_family_model: device.device_family_model,
            software_revision_level: device.software_revision_level,
            capability_inquiry_category_supported: ci_category,
            receivable_maximum_sysex_message_size: device.receivable_maximum_sysex_message_size,
        }
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
        data: Payload,
    ) -> Self {
        return MidiCiMessage {
            sysex_start: SYSTEM_EXCLUSIVE_START,
            universal_sysex: UNIVERSAL_SYSTEM_EXCLUSIVE,
            device_id,
            sysex_sub_id_1: SUB_ID_1_MIDI_CI,
            sysex_sub_id_2,
            version_format,
            source_muid,
            destination_muid,
            data,
            sysex_end: END_UNIVERSAL_SYSTEM_EXCLUSIVE,
        };

    }
}
trait MidiCodec {
    fn encode(&self) -> Vec<u8>;
    // fn decode(&mut self, midibytes: Vec<u8>) -> MidiCiMessage;
}
impl MidiCodec for MidiCiMessage{
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
        data: Payload,
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
        let data = DiscoveryPayload::new(self, ci_category);
        return self.build_midi_ci_msg(
            SUB_ID_2_DISCOVERY_INQUIRY,
            BROADCAST_MUID,
            Payload::DiscoveryPayload(data),
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
