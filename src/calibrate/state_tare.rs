use core::fmt::Debug;

use hal::{
    blocking::delay::DelayUs,
    digital::v2::{InputPin, OutputPin},
};

use crate::HX711;

use super::state_weight1::StateWeight1;

pub struct StateTare<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    hx711: HX711<CLK, DT, E>,
}

impl<CLK, DT, E> StateTare<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn new(hx711: HX711<CLK, DT, E>) -> Self {
        Self { hx711 }
    }
    
    pub fn tare<D: DelayUs<u32>>(self, delay: &mut D) -> nb::Result<StateWeight1<CLK, DT, E>, E> {
        let Self { mut hx711 } = self;
        hx711.tare(delay, 7, usize::MAX, 1)?;
        Ok(StateWeight1::new(hx711))
    }
}
