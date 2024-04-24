use core::fmt::Debug;

use hal::digital::v2::{InputPin, OutputPin};

use crate::HX711;

pub struct StateFinish<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    hx711: HX711<CLK, DT, E>,
    first_measure: f32,
    second_measure: f32,
}

impl<CLK, DT, E> StateFinish<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn new(hx711: HX711<CLK, DT, E>, first_measure: f32, second_measure: f32) -> Self {
        Self {
            hx711,
            first_measure,
            second_measure,
        }
    }

    pub fn finish(self, first_known: f32, second_known: f32) -> HX711<CLK, DT, E> {
        let Self {
            mut hx711,
            first_measure,
            second_measure,
        } = self;
        // known_weight = scale * measured_weight + offset
        hx711.scale = (second_known - first_known) / (second_measure - first_measure);
        hx711.offset = (second_known - hx711.scale * second_measure) as i32;
        hx711
    }
}
