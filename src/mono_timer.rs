use stm32f103xx::{TIM2, TIM3, TIM4};

#[cfg(feature = "time_units")]
use embedded_hal_time::Stopwatch;

use rcc::{APB1, Clocks};
use time::Hertz;

struct MonoTimer<TIM> {
    tim: TIM,
    clocks: Clocks
}

/// Interrupt events
pub enum Event {
    /// Timer timed out / count down ended
    Update,
}


macro_rules! hal {
    ($($TIMX:ident: ($timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(
            impl MonoTimer<$timX> {
                pub fn $timX<T>(tim: $TIMX, clocks: Clocks, apb1: &mut APB1) -> Self {
                    // enable and reset peripheral to a clean slate state
                    apb1.enr().modify(|_, w| w.$timXen().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    let result = MonoTimer { clocks, tim };
                }
            }

            impl Stopwatch<u16, Hertz> for MonoTimer<$timX> {
                fn ticks_passed(&self) -> u16 {
                    self.tim.arr.read()
                }
            }
        )+
    }
}


hal! {
    TIM2: (tim2, tim2en, tim2rst),
    TIM3: (tim3, tim3en, tim3rst),
    TIM4: (tim4, tim4en, tim3rst),
}
