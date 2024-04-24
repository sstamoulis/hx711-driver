use core::fmt::Debug;

use hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};
use nb::block;

use crate::HX711;

use super::state_weight2::StateWeight2;

pub struct StateWeight1<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    hx711: HX711<CLK, DT, E>,
}

impl<CLK, DT, E> StateWeight1<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn new(hx711: HX711<CLK, DT, E>) -> Self {
        Self { hx711 }
    }

    pub fn measure_1st<D>(self, delay: &mut D) -> StateWeight2<CLK, DT, E>
    where
        D: DelayUs<u32>,
    {
        let Self { mut hx711 } = self;
        let first_measure = block!(hx711.read_signed_average(10, delay)).unwrap();
        let first_measure = first_measure as f32;
        StateWeight2::new(hx711, first_measure)
    }
}
