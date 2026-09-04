/*****************************************************************************
 * Copyright (c) 2022, Nations Technologies Inc.
 *
 * All rights reserved.
 * ****************************************************************************
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 * this list of conditions and the disclaimer below.
 *
 * Nations' name may not be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * DISCLAIMER: THIS SOFTWARE IS PROVIDED BY NATIONS "AS IS" AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
 * DISCLAIMED. IN NO EVENT SHALL NATIONS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA,
 * OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 * EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 * ****************************************************************************/

// The code is based on official library examples. So there is the copyright notice.
#![no_std]
#![no_main]

use core::ptr::read_volatile;

use cortex_m::asm::{delay, nop};
use defmt::{dbg, info};
use defmt_rtt as _;
use pac::{FLASH, RCC};
use panic_halt as _;

use cortex_m_rt::entry;
extern crate n32l40x_pac as pac;

#[cfg(feature = "app")]
#[used]
#[unsafe(link_section = ".start_block")]
static BOOT_SIG: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF];

#[entry]
fn main() -> ! {
    RCC.RCC_CTRL().modify(|v| v.set_HSIEN(true));
    delay(1000);
    // is write protection status bit per page
    let wrp = FLASH.FLASH_WRP().read().0;
    info!("wrp: {:b}", wrp);
    let rp1 = FLASH.FLASH_OB().read().RDPRT1();
    let rp2 = FLASH.FLASH_OB().read().RDPRT2();
    dbg!(rp1, rp2);
    // Unlock FLASH
    info!("Will unlock FLASH");
    FLASH.FLASH_KEY().write(|v| v.set_FKEY(0x45670123));
    FLASH.FLASH_KEY().write(|v| v.set_FKEY(0xCDEF89AB));
    flash_clear_sts_flag();
    wait_for_last_opt(0x2000).unwrap();
    FLASH.FLASH_OPTKEY().write(|v| v.set_OPTKEY(0x45670123));
    FLASH.FLASH_OPTKEY().write(|v| v.set_OPTKEY(0xCDEF89AB));
    // wait_for_last_opt(0x2000).unwrap();
    // info!("Unlocked");
    FLASH.FLASH_CTRL().modify(|v| v.set_OPTER(true));
    // wait_for_last_opt(0x2000).unwrap();
    // info!("Set for erase");
    FLASH.FLASH_CTRL().modify(|v| v.set_START(true));
    wait_for_last_opt(0xB0000).unwrap();
    flash_clear_sts_flag();
    FLASH.FLASH_CTRL().modify(|v| v.set_OPTER(false));
    info!("Erased OB");
    FLASH.FLASH_CTRL().modify(|v| v.set_OPTPG(true));
    unsafe {
        let ob1 = read_volatile(0x1FFF_F800 as *mut u32);
        core::ptr::write_volatile(0x1FFF_F800 as *mut u32, (ob1 & 0xFFFF_0000) | 0x5AA5)
    };
    // wait_for_last_opt(0x2000).unwrap();
    FLASH.FLASH_WRP().write(|v| v.set_WRPT(0));
    wait_for_last_opt(0x2000).unwrap();
    info!("Write protection disbaled! Reset now.");
    FLASH.FLASH_CTRL().modify(|v| v.set_OPTPG(false));
    loop {
        info!("Amongus");
        delay(500_000);
    }
}

fn flash_clear_sts_flag() {
    FLASH.FLASH_STS().modify(|v| {
        v.set_PGERR(false);
        v.set_PVERR(false);
        v.set_WRPERR(false);
        v.set_EOP(false);
        v.set_EVERR(false);
        //v.set_ECCERR(false);
    });
}

fn wait_for_last_opt(timeout: u32) -> Result<(), ()> {
    nop();
    nop();
    nop();
    nop();
    for _ in 0..timeout {
        if !flash_is_busy() {
            return Ok(());
        }
    }
    Err(())
}

fn flash_is_busy() -> bool {
    FLASH.FLASH_STS().read().BUSY()
}
