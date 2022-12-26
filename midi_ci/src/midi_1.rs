#[derive(PartialEq, PartialOrd)]
pub enum Status {
    InvalidType = 0x00,                     // For notifying errors
    NoteOff = 0x80,                         // Channel Message - Note Off
    NoteOn = 0x90,                          // Channel Message - Note On
    AfterTouchPoly = 0xA0,                  // Channel Message - Polyphonic AfterTouch
    ControlChange = 0xB0,                   // Channel Message - Control Change / Channel Mode
    ProgramChange = 0xC0,                   // Channel Message - Program Change
    AfterTouchChannel = 0xD0,               // Channel Message - Channel (monophonic) AfterTouch
    PitchBend = 0xE0,                       // Channel Message - Pitch Bend
    SystemExclusive = 0xF0,                 // System Exclusive
    TimeCodeQuarterFrame = 0xF1,            // System Common - MIDI Time Code Quarter Frame
    SongPosition = 0xF2,                    // System Common - Song Position Pointer
    SongSelect = 0xF3,                      // System Common - Song Select
    UndefinedF4 = 0xF4,
    UndefinedF5 = 0xF5,
    TuneRequest = 0xF6,        // System Common - Tune Request
    SystemExclusiveEnd = 0xF7, // System Exclusive End
    Clock = 0xF8,              // System Real Time - Timing Clock
    SrtTick = 0xF9, // System Real Time - Timing Tick (1 tick = 10 milliseconds)
    SrtStart = 0xFA,        // System Real Time - Start
    SrtContinue = 0xFB,     // System Real Time - Continue
    SrtStop = 0xFC,         // System Real Time - Stop
    UndefinedFd = 0xFD,
    ActiveSensing = 0xFE, // System Real Time - Active Sensing
    SystemReset = 0xFF,   // System Real Time - System Reset
    End,
}

pub enum ThruMode {
    Off = 0,              // Thru disabled (nothing passes through).
    Full = 1,             // Fully enabled Thru (every incoming message is sent back).
    SameChannel = 2,      // Only the messages on the Input Channel will be sent back.
    DifferentChannel = 3, // All the messages but the ones on the Input Channel will be sent back.
}

pub enum SubId1 {
    SampleDumpHeader = 0x01,
    SampleDataPacket = 0x02,
    SampleDumpRequest = 0x03,
    MidiTimeCode = 0x04,
    SampleDumpExtensions = 0x05,
    GeneralInformation = 0x06,
    FileDump = 0x07,
    MidiTuningStandard = 0x08,
    GeneralMidi = 0x09,
    EndOfFile = 0x7B,
    Wait = 0x7C,
    Cancel = 0x7D,
    Nak = 0x7E,
    Ack = 0x7F,
}

pub enum MidiTimeCodeSubId2 {
    Special = 0x00,
    PunchInPoints = 0x01,
    PunchOutPoints = 0x02,
    DeletePunchInPoint = 0x03,
    DeletePunchOutPoint = 0x04,
    EventStartPoint = 0x05,
    EventStopPoint = 0x06,
    EventStartPointsWithAdditionalInfo = 0x07,
    EventStopPointsWithAdditionalInfo = 0x08,
    DeleteEventStartPoint = 0x09,
    DeleteEventStopPoint = 0x0A,
    CuePoints = 0x0B,
    CuePointsWithAdditionalInf = 0x0C,
    DeleteCuePoint = 0x0D,
    EventNameInAdditionalInfo = 0x0E,
}

pub enum SampleDumpExtensionsSubId2 {
    MultipleLoopPoints = 0x01,
    LoopPointsRequest = 0x02,
}

pub enum GeneralInformationSubId2 {
    IdentityRequest = 0x01,
    IdentityReply = 0x02,
}

pub enum FileDumpSubId2 {
    Header = 0x01,
    DataPacket = 0x02,
    Request = 0x03,
}

pub enum MidiTuningStandardSubId2 {
    BulkDumpRequest = 0x00,
    BulkDumpReply = 0x01,
}

pub enum GeneralMidiSubId2 {
    GeneralMidiSystemOn = 0x01,
    GeneralMidiSystemOff = 0x02,
}
/* brief Enumeration of Control Change command numbers.
See the detailed controllers numbers & description here:
http://www.somascape.org/midi/tech/spec.html#ctrlnums
*/
pub enum CC {
    // High resolution Continuous Controllers MSB (+32 for LSB) ----------------
    BankSelect = 0,
    ModulationWheel = 1,
    BreathController = 2,
    FootController = 4,
    PortamentoTime = 5,
    DataEntryMSB = 6,
    ChannelVolume = 7,
    Balance = 8,
    Pan = 10,
    ExpressionController = 11,
    EffectControl1 = 12,
    EffectControl2 = 13,
    // CC15 undefined
    GeneralPurposeController1 = 16,
    GeneralPurposeController2 = 17,
    GeneralPurposeController3 = 18,
    GeneralPurposeController4 = 19,
    // CC20 to CC31 undefined
    BankSelectLSB = 32,
    ModulationWheelLSB = 33,
    BreathControllerLSB = 34,
    // CC35 undefined
    FootControllerLSB = 36,
    PortamentoTimeLSB = 37,
    DataEntryLSB = 38,
    ChannelVolumeLSB = 39,
    BalanceLSB = 40,
    // CC41 undefined
    PanLSB = 42,
    ExpressionControllerLSB = 43,
    EffectControl1LSB = 44,
    EffectControl2LSB = 45,
    // CC46 to CC63 undefined
    Sustain = 64,
    Portamento = 65,
    Sostenuto = 66,
    SoftPedal = 67,
    Legato = 68,
    Hold = 69,
    // Low resolution continuous controllers -----------------------------------
    SoundController1 = 70,  // Synth: Sound Variation   FX: Exciter On/Off
    SoundController2 = 71,  // Synth: Harmonic Content  FX: Compressor On/Off
    SoundController3 = 72,  // Synth: Release Time      FX: Distortion On/Off
    SoundController4 = 73,  // Synth: Attack Time       FX: EQ On/Off
    SoundController5 = 74,  // Synth: Brightness        FX: Expander On/Off
    SoundController6 = 75,  // Synth: Decay Time        FX: Reverb On/Off
    SoundController7 = 76,  // Synth: Vibrato Rate      FX: Delay On/Off
    SoundController8 = 77,  // Synth: Vibrato Depth     FX: Pitch Transpose On/Off
    SoundController9 = 78,  // Synth: Vibrato Delay     FX: Flange/Chorus On/Off
    SoundController10 = 79, // Synth: Undefined         FX: Special Effects On/Off
    GeneralPurposeController5 = 80,
    GeneralPurposeController6 = 81,
    GeneralPurposeController7 = 82,
    GeneralPurposeController8 = 83,
    PortamentoControl = 84,
    // CC85 to CC90 undefined
    Effects1 = 91, // Reverb send level
    Effects2 = 92, // Tremolo depth
    Effects3 = 93, // Chorus send level
    Effects4 = 94, // Celeste depth
    Effects5 = 95, // Phaser depth
    DataIncrement = 96,
    DataDecrement = 97,
    NRPNLSB = 98, // Non-Registered Parameter Number (LSB)
    NRPNMSB = 99, // Non-Registered Parameter Number (MSB)
    RPNLSB = 100, // Registered Parameter Number (LSB)
    RPNMSB = 101, // Registered Parameter Number (MSB)
    // CC102 to CC119 undefined
    // Channel Mode messages ---------------------------------------------------
    AllSoundOff = 120,
    ResetAllControllers = 121,
    LocalControl = 122,
    AllNotesOff = 123,
    OmniModeOff = 124,
    OmniModeOn = 125,
    MonoModeOn = 126,
    PolyModeOn = 127,
}

// pub enum RegisteredParameterNumbers {
//     PitchBendSensitivity = 0x0000u16,
//     ChannelFineTuning = 0x0001u16,
//     ChannelCoarseTuning = 0x0002u16,
//     SelectTuningProgram = 0x0003u16,
//     SelectTuningBank = 0x0004u16,
//     ModulationDepthRange = 0x0005u16,
//     NullFunction = (0x7f << 7) + 0x7f,
// }

// pub struct DataBytes {
    
// }
// pub struct MidiMessage<'a> {
//     status: u8,
//     data: &'a [u8],
// }
// fn extract_message_data<'a>(i: &'a [u8], t: &'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
//     Ok((i, t))
// }


// fn detect_midi_start(i: &[u8]) -> IResult<&[u8], &[u8]> {
//     tag(&[sysex_start])(i)
// }

// fn extract_status_byte<'a>(i: &'a [u8], o: &'a [u8]) -> IResult<&'a [u8], u8> {
//     Ok((i, o[0]))
// }

// extern crate nom;
// use nom::{
//     // bytes::complete::tag;
//     bits::{bits, bytes},
//     bytes::streaming::{tag, take_until, take},
//     IResult
// };
// let mut parser = delimited(tag("("), tag("abc"), tag(")"));


// const sysex_start:u8 = 0xF7;
// const sysex_end_arr: &[u8] = &[0xF7];
// const sysex_end: u8 = 0xF7;

// const end: &[u8] = &[sysex_end];

// fn parse_midi_message( i: &[u8] ) -> IResult<&[u8], (&[u8], &[u8])> {
//     bits::<_, _, Error<(&[u8], usize)>, _, _> map_res(
//         tag(&[sysex_start]),
//         take_until::<&[u8], &[u8], Err::Error>(end),
//     )(i)
//     // let (i, _) = tag(&[sysex_start])(i)?;

//     // let (i, o) = take(1u8)(i)?;
//     // let (i, status) = extract_status_byte(i, o)?;
//     // let end: &[u8] = &[sysex_end]; 
//     // let (i, t) = take_until((&[u8])&[sysex_end])(i)?;
//     // let (i, b) = extract_message_data(i, t)?;

//     // return Ok((i, MidiMessage { status:status[0], data:&[0xFF]}));
// }
