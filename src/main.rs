extern crate portmidi as pm;
use std::thread;
use std::collections::HashSet;
use std::time::Duration;

#[derive(Debug)]
enum MidiError {
    SendError(pm::Error)
}

trait MidiWrapper {
    fn down(&mut self, note: u8, velocity: u8) -> Result<(), MidiError>;
    fn up(&mut self, note: u8) -> Result<(), MidiError>;
    fn get_down(&self) -> &HashSet<u8>;
}

struct PortMidiWrapper {
    output_port: pm::OutputPort,
    channel: u8,
    down: HashSet<u8>,
}

impl PortMidiWrapper {
    pub fn new() -> Result<Self, pm::Error> {
        let context = try!(pm::PortMidi::new());
        let device = try!(context.device(1));
        let output_port = try!(context.output_port(device, 1024));
        Ok(PortMidiWrapper {
            output_port: output_port,
            channel: 1,
            down: HashSet::new()
        })
    }

    fn send(&mut self, note: u8, velocity: u8, isdown: bool) -> Result<(), MidiError> {
        let status = if isdown { 0x90 } else { 0x80 };
        let message = pm::MidiMessage {
            status: status + self.channel,
            data1: note,
            data2: velocity // TODO: Check if velocity
        };
        match self.output_port.write_message(message) {
            Ok(_) => Ok(()),
            Err(e) => Err(MidiError::SendError(e)),
        }
    }
}

impl MidiWrapper for PortMidiWrapper {
    fn down(&mut self, note: u8, velocity: u8) -> Result<(), MidiError> {
        self.down.insert(note);
        self.send(note, velocity, true)
    }

    fn up(&mut self, note: u8) -> Result<(), MidiError> {
        self.down.remove(&note);
        self.send(note, 0, false)
    }

    fn get_down(&self) -> &HashSet<u8> {
        &self.down
    }
}

impl Drop for PortMidiWrapper {
    fn drop(&mut self) {
        for note in self.down.clone() {
            let _ = self.send(note.clone(), 0, false);
        }
    }
}

pub struct Cheatstrument {
    midi: MidiWrapper
}

#[test]
fn portmidiwrapper_test() {
    let mut midi = PortMidiWrapper::new().unwrap();
    midi.down(60, 100).unwrap();
    thread::sleep(Duration::from_millis(400));
    midi.up(60).unwrap();
}

#[test]
fn portmidi_drop_test() {
    let mut midi = PortMidiWrapper::new().unwrap();
    midi.down(70, 100).unwrap();
    thread::sleep(Duration::from_millis(400));
    // Should end tone by themselvs
}

fn main() {
}
