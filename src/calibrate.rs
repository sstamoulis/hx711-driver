mod state_finish;
mod state_weight1;
mod state_weight2;

use core::fmt::Debug;

use hal::digital::v2::{InputPin, OutputPin};

use crate::HX711;

use self::state_weight1::StateWeight1;

impl<CLK, DT, E> HX711<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn calibrate(self) -> StateWeight1<CLK, DT, E> {
        StateWeight1::new(self)
    }
}
