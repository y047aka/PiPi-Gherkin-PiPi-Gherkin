
#PiPi-GHERKIN - Raspberry Pi PICO
import board
from kmk.keys import KC
from kmk.kmk_keyboard import KMKKeyboard
from kmk.matrix import DiodeOrientation
from kmk.hid import HIDModes
from kmk.modules.layers import Layers
from kmk.modules.modtap import ModTap

gherkin = KMKKeyboard()
gherkin.modules.append(Layers())
gherkin.modules.append(ModTap())

gherkin.col_pins = (board.GP2, board.GP3, board.GP4, board.GP5, board.GP6, board.GP7)
gherkin.row_pins = (board.GP8, board.GP9, board.GP10, board.GP11, board.GP12, board.GP13, board.GP14, board.GP15, board.GP16, board.GP17, board.GP18)

#Rotate gherkin so USB on Right Side. Not needed if USB on Left Side.
gherkin.col_pins = tuple(gherkin.col_pins)
gherkin.row_pins = tuple(gherkin.row_pins)

gherkin.diode_orientation = DiodeOrientation.COLUMNS

gherkin.debug_enabled = True

_______ = KC.TRNS

gherkin.keymap = [
    [
        KC.MT(KC.TAB, KC.ESC),     KC.Q,    KC.W,    KC.E,    KC.R,    KC.T,    KC.N6,   KC.N7,   KC.N8,   KC.N9,
        KC.LCTRL,KC.A,    KC.S,    KC.D,    KC.F,    KC.G,    KC.N1,   KC.N2,   KC.N3,   KC.N4,
        KC.LSFT, KC.Z,    KC.X,    KC.C,    KC.V,    KC.B,    KC.LGUI, KC.MO(1),KC.LEFT, KC.DOWN,

        KC.N0,   KC.MINS, KC.EQL,  KC.GRV,  KC.Y,    KC.U,    KC.I,    KC.O,    KC.P,    KC.MT(KC.BSPC, KC.ENT),
        KC.N5,   KC.LBRC, KC.RBRC, KC.SPC,  KC.H,    KC.J,    KC.K,    KC.L,    KC.SCLN, KC.QUOT,
        KC.UP,   KC.RGHT, KC.BSLS, KC.ENT,  KC.N,    KC.M,    KC.COMM, KC.DOT,  KC.SLSH, KC.SPC,
    ],
    [
        _______, KC.EXLM, KC.AT,   KC.HASH, KC.DLR,  KC.PERC, _______, _______, _______, _______,
        _______, KC.N1,   KC.N2,   KC.N3,   KC.N4,   KC.N5,   _______, _______, _______, _______,
        _______, KC.EXLM, KC.AT,   KC.HASH, KC.DLR,  KC.PERC, _______, _______, _______, _______,

        _______, _______, _______, _______, KC.CIRC, KC.AMPR, KC.ASTR, KC.LPRN, KC.RPRN, _______,
        _______, _______, _______, _______, KC.N6,   KC.N7,   KC.N8,   KC.N9,   KC.N0,   KC.MINS,
        _______, _______, _______, _______, KC.MPLY, KC.MFFD, KC.MUTE, KC.VOLD, KC.VOLU, _______,
    ],
    [
        KC.EXLM, KC.AT,   KC.HASH, KC.DLR,  KC.PERC, KC.CIRC, KC.AMPR, KC.ASTR, KC.LPRN, KC.RPRN,
        KC.F11,  KC.F12,  _______, _______, _______, _______, _______, _______, _______, KC.GRV,
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,

        KC.EXLM, KC.AT,   KC.HASH, KC.DLR,  KC.PERC, KC.CIRC, KC.AMPR, KC.ASTR, KC.LPRN, KC.RPRN,
        KC.F11,  KC.F12,  _______, _______, _______, _______, _______, _______, _______, KC.GRV,
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,
    ],
    [
        _______, _______, _______, _______, _______, KC.MINS, KC.EQL,  KC.LBRC, KC.RBRC, KC.BSLS,
        KC.TAB,  _______, _______, _______, _______, KC.COMM, KC.DOT,  KC.SLSH, KC.SCLN, KC.QUOT,
        _______, _______, _______, _______, _______, _______, KC.LEFT, KC.DOWN, KC.UP,   KC.RGHT,

        _______, _______, _______, _______, _______, KC.MINS, KC.EQL,  KC.LBRC, KC.RBRC, KC.BSLS,
        KC.TAB,  _______, _______, _______, _______, KC.COMM, KC.DOT,  KC.SLSH, KC.SCLN, KC.QUOT,
        _______, _______, _______, _______, _______, _______, KC.LEFT, KC.DOWN, KC.UP,   KC.RGHT,
    ],
    [
        _______, _______, _______, _______, _______, KC.UNDS, KC.PLUS, KC.LCBR, KC.RCBR, KC.PIPE,
        KC.TAB,  _______, _______, _______, _______, KC.LABK, KC.RABK, KC.QUES, KC.COLN, KC.DQUO,
        _______, _______, _______, _______, _______, _______, KC.HOME, KC.PGDN, KC.PGUP, KC.END,

        _______, _______, _______, _______, _______, KC.UNDS, KC.PLUS, KC.LCBR, KC.RCBR, KC.PIPE,
        KC.TAB,  _______, _______, _______, _______, KC.LABK, KC.RABK, KC.QUES, KC.COLN, KC.DQUO,
        _______, _______, _______, _______, _______, _______, KC.HOME, KC.PGDN, KC.PGUP, KC.END,
    ],
    [
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,

        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,
        _______, _______, _______, _______, _______, _______, _______, _______, _______, _______,
    ],
]

if __name__ == '__main__':
    gherkin.go(hid_type=HIDModes.USB) #Wired USB enable
