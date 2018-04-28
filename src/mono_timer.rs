use stm32f103xx::{TIM2, TIM3, TIM4};

#[cfg(feature = "time_units")]
use embedded_hal_time::Stopwatch;

use rcc::{APB1, Clocks};
use time::Hertz;

pub struct MonoTimer<TIM> {
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
            impl MonoTimer<$TIMX> {
                pub fn $timX<T>(tim: $TIMX, clocks: Clocks, apb1: &mut APB1) -> Self {
                    // enable and reset peripheral to a clean slate state
                    apb1.enr().modify(|_, w| w.$timXen().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    // Set overflow to max
                    tim.arr.write(|w| { w.arr().bits(0xffff) });

                    // Set prescaler
                    tim.psc.write(|w| w.psc().bits(0));

                    // Trigger an update event to load the prescaler value to the clock
                    tim.egr.write(|w| w.ug().set_bit());
                    // The above line raises an update event which will indicate
                    // that the timer is already finnished. Since this is not the case,
                    // it should be cleared
                    tim.sr.modify(|_, w| w.uif().clear_bit());

                    tim.cr1.modify(|_r, w| {
                        // Set clock division to 1, direction to up and enable
                        w
                            .ckd().no_div()
                            .dir().up()
                            .cen().set_bit()
                    });

                    MonoTimer { clocks, tim }
                }
            }

            impl Stopwatch<u16, Hertz> for MonoTimer<$TIMX> {
                fn ticks_passed(&self) -> u16 {
                    self.tim.cnt.read().cnt().bits()
                }

                fn frequency(&self) -> Hertz {
                    self.clocks.pclk1()
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
