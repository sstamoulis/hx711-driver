/// The HX711 has different amplifier gain settings.
/// The choice of gain settings is controlled by writing a fixed number of
/// extra pulses after a read.
#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum GainMode {
    /// Amplification gain of 128 on channel A.
    A128 = 1, // extra pulses
    /// Amplification gain of 32 on channel B.
    B32 = 2,
    /// Amplification gain of 64 on channel A.
    A64 = 3,
}
