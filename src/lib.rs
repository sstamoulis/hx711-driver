#![no_std]

extern crate embedded_hal as hal;
extern crate nb;

mod gain_mode;
pub use gain_mode::GainMode;
mod read;
mod tare;

use core::fmt::Debug;

use hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};

pub struct HX711<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    clock: CLK,
    data: DT,
    gain_mode: GainMode,
    offset: i32,
    scale: f32,
}

impl<CLK, DT, E> HX711<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn new<D: DelayUs<u32>>(
        clock: CLK,
        data: DT,
        gain_mode: GainMode,
        delay: &mut D,
    ) -> Result<Self, E> {
        let mut me = Self {
            clock,
            data,
            gain_mode,
            offset: 0,
            scale: 1.0,
        };
        me.reset(delay)?;
        Ok(me)
    }

    pub fn is_ready(&self) -> Result<bool, E> {
        self.data.is_low()
    }

    pub fn get_offset(&self) -> i32 {
        self.offset
    }

    pub fn reset<D: DelayUs<u32>>(&mut self, delay: &mut D) -> Result<(), E> {
        self.clock.set_low()?;
        delay.delay_us(1);
        self.clock.set_high()?;
        delay.delay_us(70);
        self.clock.set_low()?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::{GainMode, HX711};
    use embedded_hal_mock::eh0::{delay, pin, MockError};

    fn init() -> HX711<pin::Mock, pin::Mock, MockError> {
        let clock_expectations = [
            pin::Transaction::set(pin::State::Low),
            pin::Transaction::set(pin::State::High),
            pin::Transaction::set(pin::State::Low),
        ];
        let clock = pin::Mock::new(&clock_expectations);
        let data = pin::Mock::new([]);
        let gain_mode = GainMode::B32;
        let mut delay = delay::NoopDelay::new();
        HX711::new(clock, data, gain_mode, &mut delay).unwrap()
    }

    #[test]
    fn test_new() {
        let hx711 = init();
        let HX711 {
            mut clock,
            mut data,
            ..
        } = hx711;
        clock.done();
        data.done();
    }
}
