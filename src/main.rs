#![no_std]
#![no_main]

use cortex_m::asm::nop;
use defmt::info;
use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;
extern crate n32l40x_pac as pac;

#[used]
#[unsafe(link_section = ".start_block")]
static BOOT_SIG: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF];

#[entry]
fn main() -> ! {
    loop {
        info!("Amongus");
        for _ in 0..500_000 {
            nop();
        }
    }
}
