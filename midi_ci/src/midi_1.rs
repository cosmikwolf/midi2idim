pub enum Type {
    InvalidType = 0x00,                     // For notifying errors
    NoteOff = 0x80,                         // Channel Message - Note Off
    NoteOn = 0x90,                          // Channel Message - Note On
    AfterTouchPoly = 0xA0,                  // Channel Message - Polyphonic AfterTouch
    ControlChange = 0xB0,                   // Channel Message - Control Change / Channel Mode
    ProgramChange = 0xC0,                   // Channel Message - Program Change
    AfterTouchChannel = 0xD0,               // Channel Message - Channel (monophonic) AfterTouch
    PitchBend = 0xE0,                       // Channel Message - Pitch Bend
    SystemExclusive = 0xF0,                 // System Exclusive
    SystemExclusiveStart = SystemExclusive, // System Exclusive Start
    TimeCodeQuarterFrame = 0xF1,            // System Common - MIDI Time Code Quarter Frame
    SongPosition = 0xF2,                    // System Common - Song Position Pointer
    SongSelect = 0xF3,                      // System Common - Song Select
    Undefined_F4 = 0xF4,
    Undefined_F5 = 0xF5,
    TuneRequest = 0xF6,        // System Common - Tune Request
    SystemExclusiveEnd = 0xF7, // System Exclusive End
    Clock = 0xF8,              // System Real Time - Timing Clock
    Undefined_F9 = 0xF9,
    Tick = Undefined_F9, // System Real Time - Timing Tick (1 tick = 10 milliseconds)
    Start = 0xFA,        // System Real Time - Start
    Continue = 0xFB,     // System Real Time - Continue
    Stop = 0xFC,         // System Real Time - Stop
    Undefined_FD = 0xFD,
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
    sample_dump_header = 0x01,
    sample_data_packet = 0x02,
    sample_dump_request = 0x03,
    midi_time_code = 0x04,
    sample_dump_extensions = 0x05,
    general_information = 0x06,
    file_dump = 0x07,
    midi_tuning_standard = 0x08,
    general_midi = 0x09,
    end_of_file = 0x7B,
    wait = 0x7C,
    cancel = 0x7D,
    nak = 0x7E,
    ack = 0x7F,
}

pub enum MidiTimeCodeSubId2 {
    special = 0x00,
    punch_in_points = 0x01,
    punch_out_points = 0x02,
    delete_punch_in_point = 0x03,
    delete_punch_out_point = 0x04,
    event_start_point = 0x05,
    event_stop_point = 0x06,
    event_start_points_with_additional_info = 0x07,
    event_stop_points_with_additional_info = 0x08,
    delete_event_start_point = 0x09,
    delete_event_stop_point = 0x0A,
    cue_points = 0x0B,
    cue_points_with_additional_inf = 0x0C,
    delete_cue_point = 0x0D,
    event_name_in_additional_info = 0x0E,
}

pub enum SampleDumpExtensionsSubId2 {
    multiple_loop_points = 0x01,
    loop_points_request = 0x02,
}

pub enum GeneralInformationSubId2 {
    identity_request = 0x01,
    identity_reply = 0x02,
}

pub enum FileDumpSubId2 {
    header = 0x01,
    data_packet = 0x02,
    request = 0x03,
}

pub enum MidiTuningStandardSubId2 {
    bulk_dump_request = 0x00,
    bulk_dump_reply = 0x01,
}

pub enum GeneralMidiSubId2 {
    general_midi_system_on = 0x01,
    general_midi_system_off = 0x02,
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

pub enum RegisteredParameterNumbers {
    PitchBendSensitivity = 0x0000 as u16,
    ChannelFineTuning = 0x0001 as u16,
    ChannelCoarseTuning = 0x0002 as u16,
    SelectTuningProgram = 0x0003 as u16,
    SelectTuningBank = 0x0004 as u16,
    ModulationDepthRange = 0x0005 as u16,
    NullFunction = (0x7f << 7) + 0x7f as u16,
}

pub fn generate_midi_10_message() {}
