extern crate midi2idim;
use bus::Bus;
// https://docs.rs/bus/latest/bus/

#[test]
fn test_bus() {
    let mut bus = Bus::new(10);
    let mut rx1 = bus.add_rx();
    let mut rx2 = bus.add_rx();
    bus.broadcast("Hello");
    assert_eq!(rx1.recv(), Ok("Hello"));
    assert_eq!(rx2.recv(), Ok("Hello"));
}

#[test]
fn test_send_discovery_msg() {
    let result = midi2idim::add(2, 2);
    assert_eq!(result, 4);
}
#[test]
fn discovery_msg() {
    let device = MidiDevice { device_id: 1 };
    let midimsg = device.compose_midi_ci_msg(
        SUB_ID_2_DISCOVERY_INQUIRY,
        BROADCAST_MUID,
        [0xFF, 0xFF, 0xFF, 0xFF],
    );

    assert_eq!(midimsg, 0xF07E7F0D70FFFFFFFFFF);
}
