use cortex_m::interrupt;
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin, prelude::*};

mod raw_state;
mod raw_status;

#[derive(Debug, Default)]
pub struct GamecubeControllerState {}

pub struct GamecubeController<P, D> {
    pin: P,
    delay: D,
    state: GamecubeControllerState,
}

impl<Pin, Delay> GamecubeController<Pin, Delay>
where
    Pin: OutputPin,
    Delay: DelayUs<u32>,
{
    pub fn new(pin: Pin, delay: Delay) -> Self {
        let mut pin = pin;
        pin.set_low();

        GamecubeController {
            pin,
            delay,
            state: GamecubeControllerState::default(),
        }
    }

    fn send(&self, data: &[u8]) {
        pin.set_high();
    }

    pub fn init(&self) {
        let _cmd = [0x00];
        interrupt::free(move |_| {
            // TODO: write command to pin

        });
    }
}
