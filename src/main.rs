#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        let display_smile_0 = [
            [0, 1, 0, 0, 1],
            [1, 1, 0, 1, 1],
            [0, 0, 0, 0, 0],
            [1, 0, 0, 0, 1],
            [0, 1, 1, 1, 0],
        ];

        let display_smile_1 = [
            [1, 0, 0, 1, 0],
            [1, 1, 0, 1, 1],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 1, 0, 1],
        ];

        let display_smile_2 = [
            [1, 1, 0, 1, 1],
            [1, 0, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [1, 1, 0, 1, 1],
        ];

        let display_smile_3 = [
            [1, 1, 0, 1, 1],
            [0, 1, 0, 0, 1],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 0, 1, 1, 1],
        ];

        loop {
            display.show(&mut timer, display_smile_0, 100);
            display.show(&mut timer, display_smile_1, 100);
            display.show(&mut timer, display_smile_2, 100);
            display.show(&mut timer, display_smile_3, 100);
            // display.clear();
            // timer.delay_ms(250_u32);
        }
    }

    panic!("End");
}
