mod state_finish;
mod state_tare;
mod state_weight1;
mod state_weight2;

use core::fmt::Debug;

use hal::digital::v2::{InputPin, OutputPin};

use crate::HX711;

use self::{state_tare::StateTare, state_weight1::StateWeight1};

impl<CLK, DT, E> HX711<CLK, DT, E>
where
    CLK: OutputPin<Error = E>,
    DT: InputPin<Error = E>,
    E: Debug,
{
    pub fn calibrate(self) -> StateTare<CLK, DT, E> {
        StateTare::new(self)
    }

    pub fn calibrate_with_current_offset(self) -> StateWeight1<CLK, DT, E> {
        StateWeight1::new(self)
    }
}