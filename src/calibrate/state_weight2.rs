use core::fmt::Debug;

use hal::{blocking::delay::DelayUs, digital::v2::{InputPin, OutputPin}};
use nb::block;

use crate::HX711;

use super::state_finish::StateFinish;

pub struct StateWeight2<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    hx711: HX711<CLK, DT, E>,
    first_measure: f32,
}

impl<CLK, DT, E> StateWeight2<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn new(hx711: HX711<CLK, DT, E>, first_measure: f32) -> Self {
        Self {
            hx711,
            first_measure,
        }
    }

    pub fn measure_2nd<D>(self, delay: &mut D) -> StateFinish<CLK, DT, E>
    where
        D: DelayUs<u32>,
    {
        let Self { mut hx711, first_measure } = self;
        let second_measure = block!(hx711.read_signed_average(10, delay)).unwrap();
        let second_measure = second_measure as f32;
        StateFinish::new(hx711, first_measure, second_measure)
    }
}
