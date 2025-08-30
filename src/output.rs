use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, Debug)]
enum Outport {
    Stdout = 1,
    Stderr = 2,
}

static OUTPORT: AtomicU8 = AtomicU8::new(Outport::Stdout as u8);

pub fn set_outport(outport: Outport) {
    OUTPORT.store(outport as u8, Ordering::Relaxed);
}

pub fn get_outport() -> Outport {
    match OUTPORT.load(Ordering::Relaxed) {
        1 => Outport::Stdout,
        2 => Outport::Stderr,
        _ => unreachable!()
    }
}
