extern crate portmidi as pm;

use std::thread;
use std::time::Duration;
use pm::MidiMessage;

static CHANNEL: u8 = 0;

fn print_devices() {
    let context = pm::PortMidi::new().expect("Could not connect to PortMidi");
    for dev in context.devices().expect("No devices available") {
        println!("{}", dev);
    }
}


#[test]
fn portmidi_test() {
    let context = pm::PortMidi::new().expect("Could not connect to PortMidi.");
    assert!(true);
    print_devices();
    let mut out_port = context.device(1)
                          .and_then(|dev| context.output_port(dev, 1024))
                          .expect("Could not connect to device");
    let note_on = MidiMessage {
        status: 0x90 + CHANNEL,
        data1: 60,
        data2: 100,
    };
    out_port.write_message(note_on).expect("Could not send message");
    thread::sleep(Duration::from_millis(400));
    let note_off = MidiMessage {
        status: 0x80 + CHANNEL,
        data1: 60,
        data2: 100,
    };
    out_port.write_message(note_off).expect("Could not send message");
}

fn main() {
    print_devices();
}
