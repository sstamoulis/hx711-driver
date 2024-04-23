#![no_std]

extern crate embedded_hal as hal;
extern crate nb;
use core::fmt::Debug;

use hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};
use nb::block;

#[derive(Clone, Copy)]
pub enum GainMode {
    A128 = 25,
    B32 = 26,
    A64 = 27,
}

pub struct HX711<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    clock: CLK,
    data: DT,
    gain_mode: GainMode,
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
        };
        me.reset(delay)?;
        Ok(me)
    }

    pub fn is_ready(&self) -> Result<bool, E> {
        self.data.is_low()
    }

    pub fn reset<D: DelayUs<u32>>(&mut self, delay: &mut D) -> Result<(), E> {
        self.clock.set_low()?;
        delay.delay_us(1);
        self.clock.set_high()?;
        delay.delay_us(70);
        self.clock.set_low()?;
        Ok(())
    }

    pub fn read_raw<D: DelayUs<u32>>(&mut self, delay: &mut D) -> nb::Result<i32, E> {
        if !self.is_ready()? {
            Err(nb::Error::WouldBlock)
        } else {
            let mut value = 0;
            critical_section::with(|_cs| -> Result<(), E> {
                for i in (0..24).rev() {
                    delay.delay_us(1);
                    self.clock.set_high()?;
                    delay.delay_us(1);
                    if self.data.is_high()? {
                        value |= 0b1 << i;
                    }
                    self.clock.set_low()?;
                }
                for _ in 0..self.gain_mode as usize {
                    delay.delay_us(1);
                    self.clock.set_high()?;
                    delay.delay_us(1);
                    self.clock.set_low()?;
                }
                Ok(())
            })?;
            Ok(value)
        }
    }

    pub fn tare<D: DelayUs<u32>>(
        &mut self,
        delay: &mut D,
        samples: u16,
        max_samples: usize,
        max_accuracy: u32,
    ) -> nb::Result<(), E> {
        if !self.is_ready()? {
            Err(nb::Error::WouldBlock)
        } else {
            const MIN_SAMPLES: usize = 3;
            let samples = samples
                .try_into()
                .unwrap_or(i32::MAX)
                .max(MIN_SAMPLES as i32);
            let max_samples = max_samples.min(MIN_SAMPLES);
            let mut moving_average = 0;
            let mut last_moving_average = i32::MAX;
            for _ in 0..max_samples {
                if last_moving_average.abs_diff(moving_average) <= max_accuracy {
                    break;
                }
                last_moving_average = moving_average;
                let value = block!(self.read_raw(delay))?;
                moving_average += value.saturating_sub(moving_average).saturating_div(samples);
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{GainMode, HX711};
    use embedded_hal_mock::eh0::{delay, pin};

    #[test]
    fn test_new() {
        let clock_expectations = [
            pin::Transaction::set(pin::State::Low),
            pin::Transaction::set(pin::State::High),
            pin::Transaction::set(pin::State::Low),
        ];
        let hx711 = {
            let clock = pin::Mock::new(&clock_expectations);
            let data = pin::Mock::new([]);
            let gain_mode = GainMode::B32;
            let mut delay = delay::NoopDelay::new();
            HX711::new(clock, data, gain_mode, &mut delay).unwrap()
        };
        let HX711 { mut clock, mut data, .. } = hx711;
        clock.done();
        data.done();
    }
}
