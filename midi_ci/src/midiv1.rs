pub mod message_type {
    pub const INVALID_TYPE: u8 = 0x00;                     // For notifying errors
    // pub const UNIVERSAL_SYSTEM_EXCLUSIVE: u8 = 0x80;                         // Channel Message - Note Off
    pub const NOTE_OFF: u8 = 0x80;                         // Channel Message - Note Off
    pub const NOTE_ON: u8 = 0x90;                          // Channel Message - Note On
    pub const AFTER_TOUCH_POLY: u8 = 0xA0;                  // Channel Message - Polyphonic AfterTouch
    pub const CONTROL_CHANGE: u8 = 0xB0;                   // Channel Message - Control Change / Channel Mode
    pub const PROGRAM_CHANGE: u8 = 0xC0;                   // Channel Message - Program Change
    pub const AFTER_TOUCH_CHANNEL: u8 = 0xD0;               // Channel Message - Channel (monophonic) AfterTouch
    pub const PITCH_BEND: u8 = 0xE0;                       // Channel Message - Pitch Bend
    pub const SYSTEM_EXCLUSIVE: u8 = 0xF0;                 // System Exclusive
    pub const SYSEX_START: u8 = SYSTEM_EXCLUSIVE;                 // System Exclusive
    pub const TIME_CODE_QUARTER_FRAME: u8 = 0xF1;            // System Common - MIDI Time Code Quarter Frame
    pub const SONG_POSITION: u8 = 0xF2;                    // System Common - Song Position Pointer
    pub const SONG_SELECT: u8 = 0xF3;                      // System Common - Song Select
    pub const UNDEFINED_F4: u8 = 0xF4;
    pub const UNDEFINED_F5: u8 = 0xF5;
    pub const TUNE_REQUEST: u8 = 0xF6;        // System Common - Tune Request
    pub const SYSTEM_EXCLUSIVE_END: u8 = 0xF7; // System Exclusive End
    pub const SYSEX_END: u8 = SYSTEM_EXCLUSIVE_END; // System Exclusive End
    pub const CLOCK: u8 = 0xF8;              // System Real Time - Timing Clock
    pub const SRT_TICK: u8 = 0xF9; // System Real Time - Timing Tick (1 tick = 10 milliseconds)
    pub const SRT_START: u8 = 0xFA;        // System Real Time - Start
    pub const SRT_CONTINUE: u8 = 0xFB;     // System Real Time - Continue
    pub const SRT_STOP: u8 = 0xFC;         // System Real Time - Stop
    pub const UNDEFINED_FD: u8 = 0xFD;
    pub const ACTIVE_SENSING: u8 = 0xFE; // System Real Time - Active Sensing
    pub const SYSTEM_RESET: u8 = 0xFF;   // System Real Time - System Reset
}

pub mod thru_mode {
    pub const OFF: u8 = 0;              // Thru disabled (nothing passes through).
    pub const FULL: u8 = 1;             // Fully enabled Thru (every incoming message is sent back).
    pub const SAME_CHANNEL: u8 = 2;      // Only the messages on the Input Channel will be sent back.
    pub const DIFFERENT_CHANNEL: u8 = 3; // All the messages but the ones on the Input Channel will be sent back.
}

pub mod sub_id1 {
    pub const SAMPLE_DUMP_HEADER: u8 = 0x01;
    pub const SAMPLE_DATA_PACKET: u8 = 0x02;
    pub const SAMPLE_DUMP_REQUEST: u8 = 0x03;
    pub const MIDI_TIME_CODE: u8 = 0x04;
    pub const SAMPLE_DUMP_EXTENSIONS: u8 = 0x05;
    pub const GENERAL_INFORMATION: u8 = 0x06;
    pub const FILE_DUMP: u8 = 0x07;
    pub const MIDI_TUNING_STANDARD: u8 = 0x08;
    pub const GENERAL_MIDI: u8 = 0x09;
    pub const END_OF_FILE: u8 = 0x7B;
    pub const WAIT: u8 = 0x7C;
    pub const CANCEL: u8 = 0x7D;
    pub const NAK: u8 = 0x7E;
    pub const ACK: u8 = 0x7F;
}

pub mod midi_time_code_sub_id2 {
    pub const SPECIAL: u8 = 0x00;
    pub const PUNCH_IN_POINTS: u8 = 0x01;
    pub const PUNCH_OUT_POINTS: u8 = 0x02;
    pub const DELETE_PUNCH_IN_POINT: u8 = 0x03;
    pub const DELETE_PUNCH_OUT_POINT: u8 = 0x04;
    pub const EVENT_START_POINT: u8 = 0x05;
    pub const EVENT_STOP_POINT: u8 = 0x06;
    pub const EVENT_START_POINTS_WITH_ADDITIONAL_INFO: u8 = 0x07;
    pub const EVENT_STOP_POINTS_WITH_ADDITIONAL_INFO: u8 = 0x08;
    pub const DELETE_EVENT_START_POINT: u8 = 0x09;
    pub const DELETE_EVENT_STOP_POINT: u8 = 0x0A;
    pub const CUE_POINTS: u8 = 0x0B;
    pub const CUE_POINTS_WITH_ADDITIONAL_INF: u8 = 0x0C;
    pub const DELETE_CUE_POINT: u8 = 0x0D;
    pub const EVENT_NAME_IN_ADDITIONAL_INFO: u8 = 0x0E;
}

pub mod sample_dump_extensions_sub_id2 {
    pub const MULTIPLE_LOOP_POINTS: u8 = 0x01;
    pub const LOOP_POINTS_REQUEST: u8 = 0x02;
}

pub mod general_information_sub_id2 {
    pub const IDENTITY_REQUEST: u8 = 0x01;
    pub const IDENTITY_REPLY: u8 = 0x02;
}

pub mod file_dump_sub_id2 {
    pub const HEADER: u8 = 0x01;
    pub const DATA_PACKET: u8 = 0x02;
    pub const REQUEST: u8 = 0x03;
}

pub mod midi_tuning_standard_sub_id2 {
    pub const BULK_DUMP_REQUEST: u8 = 0x00;
    pub const BULK_DUMP_REPLY: u8 = 0x01;
}

pub mod general_midi_sub_id2 {
    pub const GENERAL_MIDI_SYSTEM_ON: u8 = 0x01;
    pub const GENERAL_MIDI_SYSTEM_OFF: u8 = 0x02;
}
/* brief Enumeration of Control Change command numbers.
See the detailed controllers numbers & description here:
http://www.somascape.org/midi/tech/spec.html#ctrlnums
*/
pub mod cc {
    // High resolution Continuous Controllers MSB (+32 for LSB) ----------------
    pub const BANK_SELECT: u8 = 0;
    pub const MODULATION_WHEEL: u8 = 1;
    pub const BREATH_CONTROLLER: u8 = 2;
    pub const FOOT_CONTROLLER: u8 = 4;
    pub const PORTAMENTO_TIME: u8 = 5;
    pub const DATA_ENTRY_MSB: u8 = 6;
    pub const CHANNEL_VOLUME: u8 = 7;
    pub const BALANCE: u8 = 8;
    pub const PAN: u8 = 10;
    pub const EXPRESSION_CONTROLLER: u8 = 11;
    pub const EFFECT_CONTROL1: u8 = 12;
    pub const EFFECT_CONTROL2: u8 = 13;
    pub const GENERAL_PURPOSE_CONTROLLER1: u8 = 16;
    pub const GENERAL_PURPOSE_CONTROLLER2: u8 = 17;
    pub const GENERAL_PURPOSE_CONTROLLER3: u8 = 18;
    pub const GENERAL_PURPOSE_CONTROLLER4: u8 = 19;
    pub const BANK_SELECT_LSB: u8 = 32;
    pub const MODULATION_WHEEL_LSB: u8 = 33;
    pub const BREATH_CONTROLLER_LSB: u8 = 34;
    pub const FOOT_CONTROLLER_LSB: u8 = 36;
    pub const PORTAMENTO_TIME_LSB: u8 = 37;
    pub const DATA_ENTRY_LSB: u8 = 38;
    pub const CHANNEL_VOLUME_LSB: u8 = 39;
    pub const BALANCE_LSB: u8 = 40;
    pub const PAN_LSB: u8 = 42;
    pub const EXPRESSION_CONTROLLER_LSB: u8 = 43;
    pub const EFFECT_CONTROL1LSB: u8 = 44;
    pub const EFFECT_CONTROL2LSB: u8 = 45;
    pub const SUSTAIN: u8 = 64;
    pub const PORTAMENTO: u8 = 65;
    pub const SOSTENUTO: u8 = 66;
    pub const SOFT_PEDAL: u8 = 67;
    pub const LEGATO: u8 = 68;
    pub const HOLD: u8 = 69;
    pub const SOUND_CONTROLLER1: u8 = 70;  // Synth: Sound Variation   FX: Exciter On/Off
    pub const SOUND_CONTROLLER2: u8 = 71;  // Synth: Harmonic Content  FX: Compressor On/Off
    pub const SOUND_CONTROLLER3: u8 = 72;  // Synth: Release Time      FX: Distortion On/Off
    pub const SOUND_CONTROLLER4: u8 = 73;  // Synth: Attack Time       FX: EQ On/Off
    pub const SOUND_CONTROLLER5: u8 = 74;  // Synth: Brightness        FX: Expander On/Off
    pub const SOUND_CONTROLLER6: u8 = 75;  // Synth: Decay Time        FX: Reverb On/Off
    pub const SOUND_CONTROLLER7: u8 = 76;  // Synth: Vibrato Rate      FX: Delay On/Off
    pub const SOUND_CONTROLLER8: u8 = 77;  // Synth: Vibrato Depth     FX: Pitch Transpose On/Off
    pub const SOUND_CONTROLLER9: u8 = 78;  // Synth: Vibrato Delay     FX: Flange/Chorus On/Off
    pub const SOUND_CONTROLLER10: u8 = 79; // Synth: Undefined         FX: Special Effects On/Off
    pub const GENERAL_PURPOSE_CONTROLLER5: u8 = 80;
    pub const GENERAL_PURPOSE_CONTROLLER6: u8 = 81;
    pub const GENERAL_PURPOSE_CONTROLLER7: u8 = 82;
    pub const GENERAL_PURPOSE_CONTROLLER8: u8 = 83;
    pub const PORTAMENTO_CONTROL: u8 = 84;
    pub const EFFECTS1: u8 = 91; // Reverb send level
    pub const EFFECTS2: u8 = 92; // Tremolo depth
    pub const EFFECTS3: u8 = 93; // Chorus send level
    pub const EFFECTS4: u8 = 94; // Celeste depth
    pub const EFFECTS5: u8 = 95; // Phaser depth
    pub const DATA_INCREMENT: u8 = 96;
    pub const DATA_DECREMENT: u8 = 97;
    pub const NRPNLSB: u8 = 98; // Non-Registered Parameter Number (LSB)
    pub const NRPNMSB: u8 = 99; // Non-Registered Parameter Number (MSB)
    pub const RPNLSB: u8 = 100; // Registered Parameter Number (LSB)
    pub const RPNMSB: u8 = 101; // Registered Parameter Number (MSB)
    pub const ALL_SOUND_OFF: u8 = 120;
    pub const RESET_ALL_CONTROLLERS: u8 = 121;
    pub const LOCAL_CONTROL: u8 = 122;
    pub const ALL_NOTES_OFF: u8 = 123;
    pub const OMNI_MODE_OFF: u8 = 124;
    pub const OMNI_MODE_ON: u8 = 125;
    pub const MONO_MODE_ON: u8 = 126;
    pub const POLY_MODE_ON: u8 = 127;
}

pub mod midi {
    pub const   V1_CHAN_1_NOTE_OFF: u8 = 0x80; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_2_NOTE_OFF: u8 = 0x81; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_3_NOTE_OFF: u8 = 0x82; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_4_NOTE_OFF: u8 = 0x83; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_5_NOTE_OFF: u8 = 0x84; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_6_NOTE_OFF: u8 = 0x85; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_7_NOTE_OFF: u8 = 0x86; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_8_NOTE_OFF: u8 = 0x87; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_9_NOTE_OFF: u8 = 0x88; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_10_NOTE_OFF: u8 = 0x89; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_11_NOTE_OFF: u8 = 0x8A; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_12_NOTE_OFF: u8 = 0x8B; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_13_NOTE_OFF: u8 = 0x8C; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_14_NOTE_OFF: u8 = 0x8D; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_15_NOTE_OFF: u8 = 0x8E; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_16_NOTE_OFF: u8 = 0x8F; 	        // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_1_NOTE_ON: u8 = 0x90; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_2_NOTE_ON: u8 = 0x91; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_3_NOTE_ON: u8 = 0x92; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_4_NOTE_ON: u8 = 0x93; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_5_NOTE_ON: u8 = 0x94; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_6_NOTE_ON: u8 = 0x95; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_7_NOTE_ON: u8 = 0x96; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_8_NOTE_ON: u8 = 0x97; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_9_NOTE_ON: u8 = 0x98; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_10_NOTE_ON: u8 = 0x99; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_11_NOTE_ON: u8 = 0x9A; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_12_NOTE_ON: u8 = 0x9B; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_13_NOTE_ON: u8 = 0x9C; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_14_NOTE_ON: u8 = 0x9D; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_15_NOTE_ON: u8 = 0x9E; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_16_NOTE_ON: u8 = 0x9F; 	            // Note Number (0-127) 	Note Velocity (0-127)
    pub const   V1_CHAN_1_POLYPHONIC: u8 = 0xA0;            // Aftertouch 	Note Number (0-127) 	Pressure (0-127)
    pub const   V1_CHAN_2_POLYPHONIC: u8 = 0xA1;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_3_POLYPHONIC: u8 = 0xA2;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_4_POLYPHONIC: u8 = 0xA3;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_5_POLYPHONIC: u8 = 0xA4;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_6_POLYPHONIC: u8 = 0xA5;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_7_POLYPHONIC: u8 = 0xA6;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_8_POLYPHONIC: u8 = 0xA7;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_9_POLYPHONIC: u8 = 0xA8;            // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_10_POLYPHONIC: u8 = 0xA9;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_11_POLYPHONIC: u8 = 0xAA;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_12_POLYPHONIC: u8 = 0xAB;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_13_POLYPHONIC: u8 = 0xAC;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_14_POLYPHONIC: u8 = 0xAD;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_15_POLYPHONIC: u8 = 0xAE;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_16_POLYPHONIC: u8 = 0xAF;           // Aftertouch 	Note Number (0-127 	Pressure (0-127)
    pub const   V1_CHAN_1_CONTROL_MODE: u8 = 0xB0;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_2_CONTROL_MODE: u8 = 0xB1;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_3_CONTROL_MODE: u8 = 0xB2;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_4_CONTROL_MODE: u8 = 0xB3;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_5_CONTROL_MODE: u8 = 0xB4;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_6_CONTROL_MODE: u8 = 0xB5;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_7_CONTROL_MODE: u8 = 0xB6;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_8_CONTROL_MODE: u8 = 0xB7;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_9_CONTROL_MODE: u8 = 0xB8;          // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_10_CONTROL_MODE: u8 = 0xB9;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_11_CONTROL_MODE: u8 = 0xBA;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_12_CONTROL_MODE: u8 = 0xBB;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_13_CONTROL_MODE: u8 = 0xBC;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_14_CONTROL_MODE: u8 = 0xBD;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_15_CONTROL_MODE: u8 = 0xBE;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_16_CONTROL_MODE: u8 = 0xBF;         // Change 	see Table 3 	see Table 3
    pub const   V1_CHAN_1_PROGRAM_CHANGE: u8 = 0xC0; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_2_PROGRAM_CHANGE: u8 = 0xC1; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_3_PROGRAM_CHANGE: u8 = 0xC2; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_4_PROGRAM_CHANGE: u8 = 0xC3; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_5_PROGRAM_CHANGE: u8 = 0xC4; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_6_PROGRAM_CHANGE: u8 = 0xC5; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_7_PROGRAM_CHANGE: u8 = 0xC6; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_8_PROGRAM_CHANGE: u8 = 0xC7; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_9_PROGRAM_CHANGE: u8 = 0xC8; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_10_PROGRAM_CHANGE: u8 = 0xC9; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_11_PROGRAM_CHANGE: u8 = 0xCA; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_12_PROGRAM_CHANGE: u8 = 0xCB; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_13_PROGRAM_CHANGE: u8 = 0xCC; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_14_PROGRAM_CHANGE: u8 = 0xCD; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_15_PROGRAM_CHANGE: u8 = 0xCE; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_16_PROGRAM_CHANGE: u8 = 0xCF; 	    // Program # (0-127) 	none
    pub const   V1_CHAN_1_CHANNEL_AFTERTOUCH: u8 = 0xD0; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_2_CHANNEL_AFTERTOUCH: u8 = 0xD1; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_3_CHANNEL_AFTERTOUCH: u8 = 0xD2; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_4_CHANNEL_AFTERTOUCH: u8 = 0xD3; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_5_CHANNEL_AFTERTOUCH: u8 = 0xD4; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_6_CHANNEL_AFTERTOUCH: u8 = 0xD5; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_7_CHANNEL_AFTERTOUCH: u8 = 0xD6; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_8_CHANNEL_AFTERTOUCH: u8 = 0xD7; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_9_CHANNEL_AFTERTOUCH: u8 = 0xD8; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_10_CHANNEL_AFTERTOUCH: u8 = 0xD9; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_11_CHANNEL_AFTERTOUCH: u8 = 0xDA; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_12_CHANNEL_AFTERTOUCH: u8 = 0xDB; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_13_CHANNEL_AFTERTOUCH: u8 = 0xDC; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_14_CHANNEL_AFTERTOUCH: u8 = 0xDD; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_15_CHANNEL_AFTERTOUCH: u8 = 0xDE; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_16_CHANNEL_AFTERTOUCH: u8 = 0xDF; 	// Pressure (0-127) 	none
    pub const   V1_CHAN_1_PITCH_BEND_CHANGE: u8 = 0xE0; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_2_PITCH_BEND_CHANGE: u8 = 0xE1; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_3_PITCH_BEND_CHANGE: u8 = 0xE2; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_4_PITCH_BEND_CHANGE: u8 = 0xE3; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_5_PITCH_BEND_CHANGE: u8 = 0xE4; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_6_PITCH_BEND_CHANGE: u8 = 0xE5; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_7_PITCH_BEND_CHANGE: u8 = 0xE6; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_8_PITCH_BEND_CHANGE: u8 = 0xE7; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_9_PITCH_BEND_CHANGE: u8 = 0xE8; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_10_PITCH_BEND_CHANGE: u8 = 0xE9; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_11_PITCH_BEND_CHANGE: u8 = 0xEA; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_12_PITCH_BEND_CHANGE: u8 = 0xEB; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_13_PITCH_BEND_CHANGE: u8 = 0xEC; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_14_PITCH_BEND_CHANGE: u8 = 0xED; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_15_PITCH_BEND_CHANGE: u8 = 0xEE; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_CHAN_16_PITCH_BEND_CHANGE: u8 = 0xEF; 	// Pitch Bender LSB (0-127) 	Pitch Bender MSB (0-127)
    pub const   V1_SYSTEM_EXCLUSIVE: u8 = 0xF0;
    pub const   V1_MIDI_TIME_CODE_QTR_FRAME: u8 = 0xF1;     // 	-see spec- 	-see spec-
    pub const   V1_SONG_POSITION_POINTER: u8 = 0xF2;        // 	LSB 	MSB
    pub const   V1_SONG_SELECT: u8 = 0xF3;                  //  (Song #) 	(0-127) 	none
    // pub const   V1_UNDEFINED: u8 = 0xF4;                    //      (Reserved) 	--- 	---
    // pub const   V1_UNDEFINED: u8 = 0xF5;                    //       (Reserved) 	--- 	---
    pub const   V1_TUNE_REQUEST: u8 = 0xF6; 	            //  none 	none
    pub const   V1_END_OF_SYS_EX: u8 = 0xF7;                //      (EOX) 	none 	none
    pub const   V1_TIMING_CLOCK: u8 = 0xF8;                 //     	none 	none
    pub const   V1_UNDEFINED: u8 = 0xF9;                    // #(Reserved) 	--- 	---
    pub const   V1_START: u8 = 0xFA; 	                    //  #none 	none
    pub const   V1_CONTINUE: u8 = 0xFB; 	                //      #none 	none
    pub const   V1_STOP: u8 = 0xFC; 	                    //  #none 	none
    // pub const   V1_UNDEFINED: u8 = 0xFD;                    // #(Reserved) 	--- 	---
    pub const   V1_ACTIVE_SENSING: u8 = 0xFE;               // #	none 	none
    pub const   V1_SYSTEM_RESET: u8 = 0xFF; 	            //  #none 	none
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
