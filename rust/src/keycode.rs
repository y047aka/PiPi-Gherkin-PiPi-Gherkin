use rp_pico as bsp;

pub enum Keycode {
    BOOTSEL,

    KC_NO, // Reserved (no event indicated)

    KC_A, // a and A
    KC_B, // b and B
    KC_C, // c and C
    KC_D, // d and D
    KC_E, // e and E
    KC_F, // f and F
    KC_G, // g and G
    KC_H, // h and H
    KC_I, // i and I
    KC_J, // j and J
    KC_K, // k and K
    KC_L, // l and L
    KC_M, // m and M
    KC_N, // n and N
    KC_O, // o and O
    KC_P, // p and P
    KC_Q, // q and Q
    KC_R, // r and R
    KC_S, // s and S
    KC_T, // t and T
    KC_U, // u and U
    KC_V, // v and V
    KC_W, // w and W
    KC_X, // x and X
    KC_Y, // y and Y
    KC_Z, // z and Z
}

pub fn to_u8(keycode: Keycode) -> u8 {
    use Keycode::*;

    match keycode {
        BOOTSEL => {
            // Reboot back into USB mode (no activity, both interfaces enabled)
            bsp::hal::rom_data::reset_to_usb_boot(0, 0);

            0x00
        }

        KC_NO => 0x00,

        KC_A => 0x04,
        KC_B => 0x05,
        KC_C => 0x06,
        KC_D => 0x07,
        KC_E => 0x08,
        KC_F => 0x09,
        KC_G => 0x0A,
        KC_H => 0x0B,
        KC_I => 0x0C,
        KC_J => 0x0D,
        KC_K => 0x0E,
        KC_L => 0x0F,
        KC_M => 0x10,
        KC_N => 0x11,
        KC_O => 0x12,
        KC_P => 0x13,
        KC_Q => 0x14,
        KC_R => 0x15,
        KC_S => 0x16,
        KC_T => 0x17,
        KC_U => 0x18,
        KC_V => 0x19,
        KC_W => 0x1A,
        KC_X => 0x1B,
        KC_Y => 0x1C,
        KC_Z => 0x1D,
    }
}
