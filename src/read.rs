use core::{fmt::Debug, ops::Mul};

use hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};
use nb::block;

use crate::HX711;

impl<CLK, DT, E> HX711<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn read_raw<D: DelayUs<u32>>(&mut self, delay: &mut D) -> nb::Result<u32, E> {
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

    pub fn read_signed<D: DelayUs<u32>>(&mut self, delay: &mut D) -> nb::Result<i32, E> {
        let mut value = self.read_raw(delay)? as i32;
        // if the 24th bit is 1, then fill bits 25 to 32 with 1 to properly handle a negative number
        if value & 0b1 << 23 > 0 {
            value |= 0xFF00_0000_u32 as i32;
        }
        Ok(value)
    }

    pub fn read<D: DelayUs<u32>>(&mut self, delay: &mut D) -> nb::Result<f32, E> {
        let value = self.read_signed(delay)?;
        let value = value.saturating_add(self.offset) as f32;
        let value = value.mul(self.scale);
        Ok(value)
    }

    pub fn read_signed_average<D: DelayUs<u32>>(
        &mut self,
        samples: i32,
        delay: &mut D,
    ) -> nb::Result<i32, E> {
        let mut sum = self.read_signed(delay)?;
        let samples = samples.max(1);
        for _ in 1..samples {
            sum = sum.saturating_add(block!(self.read_signed(delay))?);
        }
        Ok(sum / samples)
    }
}
