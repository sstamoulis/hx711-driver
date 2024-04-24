use core::{fmt::Debug, ops::Neg as _};

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
            let max_samples = max_samples.max(MIN_SAMPLES);
            let mut moving_average = 0;
            let mut last_moving_average = i32::MAX;
            for _ in 0..max_samples {
                if last_moving_average.abs_diff(moving_average) <= max_accuracy {
                    break;
                }
                last_moving_average = moving_average;
                let value = block!(self.read_signed(delay))?;
                moving_average += value.saturating_sub(moving_average).saturating_div(samples);
            }
            self.offset = moving_average.neg();
            Ok(())
        }
    }
}
