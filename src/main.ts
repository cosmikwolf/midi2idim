import { Omnibus } from "@hypersphere/omnibus";

interface MIDI_CI_Message{
    sysex_start: 0xF0
    universal_sysex_header: 0x7E
    device_id: number
    sysex_subid_1: number
    sysex_subid_2: number
    midi_ci_msg_version: 1
    source_muid: number
    destination_muid: number
    data: Uint8Array
    sysex_end: 0xF7
}
interface MidiEvents {
    "midi_ci": [msg: MIDI_CI_Message],
};

const bus = new Omnibus<MidiEvents>();

bus.on("midi_ci", (msg:MIDI_CI_Message) => {
    console.log("MIDI CI message " + msg.device_id)
});

const message = <MIDI_CI_Message>{
    device_id: 69
}
// TypeScript will also make sure those are provided properly.
bus.trigger("midi_ci", message);

