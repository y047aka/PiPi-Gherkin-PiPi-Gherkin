#![no_std]
#![no_main]

use core::convert::Infallible;

use cortex_m::{asm, prelude::*};
use cortex_m_rt::entry;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_time::duration::Extensions as _;
use heapless::Deque;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    self, clocks::init_clocks_and_plls, pac, sio::Sio, usb::UsbBus, watchdog::Watchdog,
};

use usb_device as usbd;
use usbd::{
    class_prelude::UsbBusAllocator,
    device::{UsbDeviceBuilder, UsbVidPid},
};

use usbd_hid::{
    descriptor::{KeyboardReport, SerializedDescriptor},
    hid_class::{
        HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
    },
};

mod keycode;
use keycode::Keycode::{self, *};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);

    let bus = UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );
    let bus_allocator = UsbBusAllocator::new(bus);
    let vid_pid = UsbVidPid(0x6666, 0x0470);
    let mut hid = HIDClass::new_with_settings(
        &bus_allocator,
        KeyboardReport::desc(),
        10,
        HidClassSettings {
            subclass: HidSubClass::NoSubClass,
            protocol: HidProtocol::Keyboard,
            config: ProtocolModeConfig::ForceReport,
            locale: HidCountryCode::NotSupported,
        },
    );
    let mut dev = UsbDeviceBuilder::new(&bus_allocator, vid_pid)
        .manufacturer("y047aka")
        .product("PiPi-Gherkin-PiPi-Gherkin")
        .serial_number("001")
        .build();

    let sio = Sio::new(pac.SIO);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let col0 = pins.gpio2.into_pull_up_input();
    let col1 = pins.gpio3.into_pull_up_input();
    let col2 = pins.gpio4.into_pull_up_input();
    let col3 = pins.gpio5.into_pull_up_input();
    let col4 = pins.gpio6.into_pull_up_input();
    let col5 = pins.gpio7.into_pull_up_input();
    let cols: &[Column; 6] = &[&col0, &col1, &col2, &col3, &col4, &col5];

    let mut row0 = pins.gpio8.into_push_pull_output();
    let mut row1 = pins.gpio9.into_push_pull_output();
    let mut row2 = pins.gpio10.into_push_pull_output();
    let mut row3 = pins.gpio11.into_push_pull_output();
    let mut row4 = pins.gpio12.into_push_pull_output();
    let mut row5 = pins.gpio13.into_push_pull_output();
    let mut row6 = pins.gpio14.into_push_pull_output();
    let mut row7 = pins.gpio15.into_push_pull_output();
    let mut row8 = pins.gpio16.into_push_pull_output();
    let mut row9 = pins.gpio17.into_push_pull_output();
    let rows: &mut [Row; 10] = &mut [
        &mut row0, &mut row1, &mut row2, &mut row3, &mut row4, &mut row5, &mut row6, &mut row7,
        &mut row8, &mut row9,
    ];

    let mut scan_countdown = timer.count_down();
    scan_countdown.start(10.milliseconds());

    let mut led_pin = pins.led.into_push_pull_output();

    let mut macro_queue = Deque::<KeyboardReport, 32>::new();
    let mut is_macro_pressed = false;

    for i in 1..100_000_000 {
        dev.poll(&mut [&mut hid]);

        if scan_countdown.wait().is_ok() {
            if let Some(report) = macro_queue.pop_front() {
                hid.push_input(&report).ok();
            } else {
                let state = scan_keys(rows, cols);
                if false {
                    if !is_macro_pressed {
                        for report in MACRO_SEQUENCE_Y047AKA {
                            macro_queue.push_back(report.clone()).ok();
                        }
                    }
                    is_macro_pressed = true;
                } else {
                    is_macro_pressed = false;

                    let report = build_report(&state);
                    hid.push_input(&report).ok();
                }
            }
        }
        // drop received data
        hid.pull_raw_output(&mut [0; 64]).ok();

        if i % 5_000 == 0 {
            led_pin.set_high().unwrap();
        }
        if i % 10_000 == 0 {
            led_pin.set_low().unwrap();
        }
    }

    // Reboot back into USB mode (no activity, both interfaces enabled)
    bsp::hal::rom_data::reset_to_usb_boot(0, 0);

    // In case the reboot fails
    loop {
        cortex_m::asm::nop();
    }
}

pub type Column<'a> = &'a dyn InputPin<Error = Infallible>;
pub type Row<'a> = &'a mut dyn OutputPin<Error = Infallible>;
pub type StateMatrix = [[bool; 6]; 10];

fn scan_keys(rows: &mut [Row], cols: &[Column]) -> StateMatrix {
    let mut matrix = [[false; 6]; 10];
    for (row_pin, row_state) in rows.iter_mut().zip(matrix.iter_mut()) {
        row_pin.set_low().unwrap();
        asm::delay(10);
        for (col_pin, col_state) in cols.iter().zip(row_state.iter_mut()) {
            *col_state = col_pin.is_low().unwrap();
        }
        row_pin.set_high().unwrap();
        asm::delay(10);
    }
    matrix
}

fn build_report(matrix: &StateMatrix) -> KeyboardReport {
    let mut keycodes = [0u8; 6];
    let mut keycode_count = 0;
    let mut push_key = |keycode: Keycode| {
        keycodes[keycode_count] = keycode::to_u8(keycode);
        keycode_count += 1;
    };
    let mut modifier = 0;

    if matrix[0][0] {
        push_key(KC_TAB);
    }
    if matrix[0][1] {
        push_key(KC_QUOTE);
    }
    if matrix[0][2] {
        push_key(KC_COMMA);
    }
    if matrix[0][3] {
        push_key(KC_DOT);
    }
    if matrix[0][4] {
        push_key(KC_P);
    }
    if matrix[0][5] {
        push_key(KC_Y);
    }
    if matrix[1][0] {
        push_key(KC_6);
    }
    if matrix[1][1] {
        push_key(KC_7);
    }
    if matrix[1][2] {
        push_key(KC_8);
    }
    if matrix[1][3] {
        push_key(KC_9);
    }

    if matrix[1][4] {
        modifier += 0x01;
    }
    if matrix[1][5] {
        push_key(KC_A);
    }
    let kcs_2: [Keycode; 6] = [KC_O, KC_E, KC_U, KC_I, KC_1, KC_2];
    let mut count = 0;
    for col in kcs_2 {
        if matrix[2][count] {
            push_key(col);
        }
        count += 1;
    }
    if matrix[3][0] {
        push_key(KC_3);
    }
    if matrix[3][1] {
        push_key(KC_4);
    }

    if matrix[3][2] {
        modifier += 0x02;
    }
    if matrix[3][3] {
        push_key(KC_SEMICOLON);
    }
    if matrix[3][4] {
        push_key(KC_Q);
    }
    if matrix[3][5] {
        push_key(KC_J);
    }
    if matrix[4][0] {
        push_key(KC_C);
    }
    if matrix[4][1] {
        push_key(KC_X);
    }
    if matrix[4][2] {
        modifier += 0x08;
    }
    if matrix[4][3] {
        push_key(BOOTSEL);
    }
    if matrix[4][4] {
        push_key(KC_LEFT);
    }
    if matrix[4][5] {
        push_key(KC_DOWN);
    }

    let kcs_5: [Keycode; 6] = [KC_0, KC_GRAVE, KC_EQUAL, KC_SLASH, KC_F, KC_G];
    let mut count = 0;
    for col in kcs_5 {
        if matrix[5][count] {
            push_key(col);
        }
        count += 1;
    }
    if matrix[6][0] {
        push_key(KC_K);
    }
    if matrix[6][1] {
        push_key(KC_R);
    }
    if matrix[6][2] {
        push_key(KC_L);
    }
    if matrix[6][3] {
        push_key(KC_BACKSPACE);
    }

    if matrix[6][4] {
        push_key(KC_5);
    }
    if matrix[6][5] {
        push_key(KC_LEFT_BRACKET);
    }
    let kcs_7: [Keycode; 6] = [KC_RIGHT_BRACKET, KC_SPACE, KC_D, KC_H, KC_T, KC_N];
    let mut count = 0;
    for col in kcs_7 {
        if matrix[7][count] {
            push_key(col);
        }
        count += 1;
    }
    if matrix[8][0] {
        push_key(KC_S);
    }
    if matrix[8][1] {
        push_key(KC_MINUS);
    }

    if matrix[8][2] {
        push_key(KC_UP);
    }
    if matrix[8][3] {
        push_key(KC_RIGHT);
    }
    if matrix[8][4] {
        modifier += 0x40;
    }
    if matrix[8][5] {
        push_key(KC_ENTER);
    }
    let kcs_9: [Keycode; 6] = [KC_B, KC_M, KC_W, KC_V, KC_Z, KC_BACKSLASH];
    let mut count = 0;
    for col in kcs_9 {
        if matrix[9][count] {
            push_key(col);
        }
        count += 1;
    }

    KeyboardReport {
        modifier,
        reserved: 0,
        leds: 0,
        keycodes,
    }
}

const MACRO_SEQUENCE_Y047AKA: &[KeyboardReport] = &[
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x1C, 0, 0, 0, 0, 0], // y
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x27, 0, 0, 0, 0, 0], // 0
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x21, 0, 0, 0, 0, 0], // 4
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x24, 0, 0, 0, 0, 0], // 7
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x04, 0, 0, 0, 0, 0], // a
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x0e, 0, 0, 0, 0, 0], // k
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x04, 0, 0, 0, 0, 0], // a
    },
    KeyboardReport {
        modifier: 0,
        reserved: 0,
        leds: 0,
        keycodes: [0x0, 0, 0, 0, 0, 0],
    },
];

// End of file
