# Initialize a Keyboard
kbd = Keyboard.new

# Initialize GPIO assign
kbd.init_pins(
  [ 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18 ], # row0, row1,... respectively
  [ 2, 3, 4, 5, 6, 7 ]  # col0, col1,... respectively
  # If you put USB port on the right side, use below instead
  # [ 12, 11, 10, 9, 8 ],
  # [ 7, 6, 5, 4, 3, 2 ]
)

# default layer should be added at first
kbd.add_layer :default, %i(
  TAB_ESC   KC_Q      KC_W      KC_E      KC_R      KC_T      KC_6      KC_7      KC_8      KC_9
  KC_LCTL   KC_A      KC_S      KC_D      KC_F      KC_G      KC_1      KC_2      KC_3      KC_4
  KC_LSFT   KC_Z      KC_X      KC_C      KC_V      KC_B      KC_LGUI   UP_RAISE  KC_LEFT   DOWN_LOWER

  KC_0      KC_MINUS  KC_EQUAL  KC_GRAVE  KC_Y      KC_U      KC_I      KC_O      KC_P      KC_BSPACE
  KC_5      KC_LBRC   KC_RBRC   KC_SPACE  KC_H      KC_J      KC_K      KC_L      KC_SCOLON KC_QUOTE
  UP_RAISE  KC_RIGHT  KC_RALT   KC_ENTER  KC_N      KC_M      KC_COMMA  KC_DOT    KC_SLASH  KC_BSLS
)
kbd.add_layer :raise, %i(
  XXXXXXX   KC_EXLM   KC_AT     KC_HASH   KC_DLR    KC_PERC   XXXXXXX   XXXXXXX   XXXXXXX   XXXXXXX
  XXXXXXX   KC_1      KC_2      KC_3      KC_4      KC_5      KC_DOWN   KC_UP     KC_RIGHT  KC_BSPACE
  XXXXXXX   KC_EXLM   KC_AT     KC_HASH   KC_DLR    KC_PERC   XXXXXXX   UP_RAISE  XXXXXXX   DOWN_LOWER


  XXXXXXX   XXXXXXX   XXXXXXX   XXXXXXX   KC_CIRC   KC_AMPR   KC_ASTER  KC_LPRN   KC_RPRN   XXXXXXX
  XXXXXXX   XXXXXXX   XXXXXXX   XXXXXXX   KC_6      KC_7      KC_8      KC_9      KC_10     KC_BSPACE
  UP_RAISE  XXXXXXX   XXXXXXX   XXXXXXX   XXXXXXX   XXXXXXX   KC_MUTE   KC_VOLDOWN KC_VOLUP XXXXXXX
)
kbd.add_layer :adjust, %i(
  XXXXXXX   KC_F2     KC_F3       KC_F4     KC_F5      KC_F6      KC_F7     KC_F8     KC_F9     KC_F10
  XXXXXXX   KC_F12    KC_QUOTE    KC_DQUO   KC_MINUS   KC_LEFT    KC_DOWN   KC_UP     KC_RIGHT  KC_DELETE
  XXXXXXX   KC_LGUI   KC_LALT     KC_LCTL   UNLOCK     UNLOCK     KC_RCTL   KC_RALT   KC_RGUI   DOWN_LOWER

  XXXXXXX   KC_F2     KC_F3       KC_F4     KC_F5      KC_F6      KC_F7     KC_F8     KC_F9     KC_F10
  XXXXXXX   KC_F12    KC_QUOTE    KC_DQUO   KC_MINUS   KC_LEFT    KC_DOWN   KC_UP     KC_RIGHT  KC_DELETE
  UP_RAISE  KC_LGUI   KC_LALT     KC_LCTL   UNLOCK     UNLOCK     KC_RCTL   KC_RALT   KC_RGUI   KC_ESCAPE
)
kbd.add_layer :lower, %i(
  XXXXXXX   KC_2      KC_3        KC_4      KC_5       KC_6       KC_7      KC_8      KC_9      KC_0
  XXXXXXX   KC_NO     KC_QUOTE    KC_DQUO   KC_MINUS   KC_GRAVE   KC_TILD   KC_PIPE   KC_COLON  KC_SCOLON
  XXXXXXX  KC_LGUI   KC_LALT     KC_LCTL   SPC_LOWER  ADJUST     KC_RCTL   KC_RALT   KC_RGUI   DOWN_LOWER

  XXXXXXX   KC_2      KC_3        KC_4      KC_5       KC_6       KC_7      KC_8      KC_9      KC_0
  XXXXXXX   KC_NO     KC_QUOTE    KC_DQUO   KC_MINUS   KC_GRAVE   KC_TILD   KC_PIPE   KC_COLON  KC_SCOLON
  UP_RAISE  KC_LGUI   KC_LALT     KC_LCTL   SPC_LOWER  ADJUST     KC_RCTL   KC_RALT   KC_RGUI   KC_RSFT
)
#
#                   Your custom    Keycode or                           Keycode (only modifiers)    Release time     Re-push time
#                   key name       Array of Keycode                     or Layer Symbol to be held  threshold(ms)    threshold(ms)
#                                  or Proc                              or Proc which will run      to consider as   to consider as
#                                  when you click                       while you keep press        `click the key`  `hold the key`
kbd.define_mode_key :TAB_ESC,    [ :KC_TAB,                             :KC_ESCAPE,                 150,             150 ]
kbd.define_mode_key :UP_RAISE,   [ :KC_UP,                              :raise,                     150,             150 ]
kbd.define_mode_key :DOWN_LOWER, [ :KC_DOWN,                            :lower,                     150,             150 ]
kbd.define_mode_key :ADJUST,     [ Proc.new { kbd.lock_layer :adjust }, :KC_LSFT,                   300,             nil ]
kbd.define_mode_key :UNLOCK,     [ Proc.new { kbd.unlock_layer },       :KC_LSFT,                   300,             nil ]

# Tip: You can also switch current layer by single click like this:
#   kbd.define_mode_key :RAISE, [ Proc.new { kbd.raise_layer }, :KC_LSFT, 150, nil ]
#   kbd.define_mode_key :LOWER, [ Proc.new { kbd.lower_layer }, :KC_LSFT, 150, nil ]

kbd.start!
