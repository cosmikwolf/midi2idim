// this will be a midi CLI that will allow you to
//  - send midi clock
//  - send and receive sysex messages
//  - record midi to a file
//  - playback midi files
// use midi2idim::midi_definitions::midi2_cc;

#[no_mangle]
pub extern "C" fn root() {
    println!("test");
    println!("{:?}", MIDI2_CC_ALL_NOTES_OFF);
}

pub fn main() {
    root()
}
