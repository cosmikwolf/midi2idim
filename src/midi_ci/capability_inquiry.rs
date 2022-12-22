struct MidiDevice {
    muid: u32,
    device_id: u8,
    version_format: u8,
    device_manufacturer: u24,
    device_family: u16,
    device_family_model: u16,
    software_revision_level: u32,
    receivable_maximum_sysex_message_size: u32,
}

trait CapabilityInquiry {
    use nom::bytes::complete::tag;

    fn new(device_id:u8,
           device_manufacturer: u24, 
           device_family: u16,
           device_family_model: u16,
           software_revision_level: u32)-> MidiDevice {
        MidiDevice {
            device_id,
            generate_muid(),
            MIDI_CI_VERSION_1_1
        }
    }
    fn parse_midi_ci_start(s: &str) -> IResult<&str, &str> {
      tag(&[SYSTEM_EXCLUSIVE_START, UNIVERSAL_SYSTEM_EXCLUSIVE])
    }
    
    // 3.2.1 Generating a MUID
    fn generate_muid() {
        use rand::Rng;
        self.muid = rng.gen::<u32>() && 0x0FFFFFFF; // 28 byte muid
    }

    fn build_midi_ci_msg( &self, _sysex_sub_id_2: u8, _destination_muid: u32,
        _data: <u8>) -> midi_ci_msg_format {
        midi_ci_msg_format {
            MIDI_CI_MSG::SYSTEM_EXCLUSIVE_START,
            MIDI_CI_MSG::UNIVERSAL_SYSTEM_EXCLUSIVE,
            device_id = self.device_id,
            sysex_sub_id_1 = MIDI_CI_MSG::SUB_ID_1_MIDI_CI,
            sysex_sub_id_2 = _sysex_sub_id_2,
            version_format = self.version_format,
            source_muid = self.muid,
            destination_muid = _destination_muid,
            data = _data,
            sysex_end = MIDI_CI_MSG::END_UNIVERSAL_SYSTEM_EXCLUSIVE,
        }
    }


    // 5.5 Discovery Message
    fn send_discovery_message(&self) {
        compose_ci_msg(&self, SUB_ID_2_DISCOVERY_INQUIRY, BROADCAST_MUID, [])
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


