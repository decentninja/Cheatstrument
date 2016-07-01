extern crate portmidi as pm;

use std::thread;
use std::time::Duration;

static CHANNEL: u8 = 0;

struct Cheatstrument {
    output_port: pm::OutputPort,
    CHANNEL: u8,
    device_recives: bool,
    down: Vec<u8>
}

impl Drop for Cheatstrument {
    fn drop(&mut self) {
        let mut clone = self.down.clone();
        for midi in clone {
            self.up(midi);
        }
    }
}

impl Cheatstrument {

    pub fn new() -> Result<Self, pm::Error> {
        // TODO: Wrap error handling by replacing pm::Error by new CheatError that implements FROMs.
        let context = try!(pm::PortMidi::new());
        let device = try!(context.device(1));
        let output_port = try!(context.output_port(device, 1024));
        Ok(Cheatstrument {
            output_port: output_port,
            CHANNEL: CHANNEL,
            device_recives: true,
            down: vec!()
        })
    }

    fn send(&mut self, midi: u8, status: u8) {
        let message = pm::MidiMessage {
            status: status + self.CHANNEL,
            data1: midi,
            data2: 100, // Velocity?
        };
        let result = self.output_port.write_message(message);
        self.device_recives = result.is_ok();
    }

    fn down(&mut self, midi: u8) {
        self.send(midi, 0x90);
    }

    fn up(&mut self, midi: u8) {
        self.send(midi, 0x80);
    }

}

#[test]
fn portmidi_test() {
    let context = pm::PortMidi::new().expect("Could not connect to PortMidi.");
    assert!(true);
    let mut out_port = context.device(1)
                          .and_then(|dev| context.output_port(dev, 1024))
                          .expect("Could not connect to device");
    let note_on = pm::MidiMessage {
        status: 0x90 + CHANNEL,
        data1: 60,
        data2: 100,
    };
    out_port.write_message(note_on).expect("Could not send message");
    thread::sleep(Duration::from_millis(400));
    let note_off = pm::MidiMessage {
        status: 0x80 + CHANNEL,
        data1: 60,
        data2: 100,
    };
    out_port.write_message(note_off).expect("Could not send message");
}

fn main() {
    Cheatstrument::new();
}
