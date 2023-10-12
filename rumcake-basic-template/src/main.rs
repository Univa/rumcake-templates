#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]
#![feature(return_position_impl_trait_in_trait)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::{build_layout, build_matrix, remap_matrix, ws2812_pin};

#[keyboard(usb, underglow = "ws2812_bitbang")]
pub struct {{ keyboard-name }};

// This keyboard's PCB uses a duplex matrix.
// `remap_matrix` creates a `remap` macro that allows us to have a better visual layout for the rest of configuration
// If your keyboard doesn't have a duplex matrix, you can remove this `remap_matrix` macro and remove the `remap!` wrapped around build_layout
remap_matrix! {
    {
        [ K00 K01 K02 K03 K04 K05 K06 K07 ]
        [ K08 K09 K10 K11 K12 K13 K14 #No ]
        [ K15 K16 K17 K18 K19 K20 K21 K22 ]
        [ K23 K24 K25 K26 K27 K28 K29 #No ]
        [ K30 K31 K32 K33 K34 K35 K36 K37 ]
        [ K38 K39 K40 K41 K42 K43 K44 #No ]
        [ K45 K46 K47 K48 K49 K50 K51 K52 ]
        [ K53 K54 K55 K56 K57 K58 K59 #No ]
        [ K60 K61 K62 K63 K64 K65 K66 K67 ]
        [ #No #No #No #No #No K68 K69 #No ]
    }
    {
        [ K00 K08 K01 K09 K02 K10 K03 K11 K04 K12 K05 K13 K06 K14 K07 K22 ]
        [ K15 K23 K16 K24 K17 K25 K18 K26 K19 K27 K20 K28 K21 K29 K37     ]
        [ K30 K38 K31 K39 K32 K40 K33 K41 K34 K42 K35 K43 K36 K44 K52     ]
        [ K45 K53 K46 K54 K47 K55 K48 K56 K49 K57 K50 K58 K51 K59 K67     ]
        [             K60 K61     K62 K63     K64 K65 K68 K66 K69         ]
    }
}

// Basic keyboard configuration
use rumcake::keyboard::Keyboard;
impl Keyboard for {{ keyboard-name }} {
    const MANUFACTURER: &'static str = ""; // TODO: Change this
    const PRODUCT: &'static str = "{{ keyboard-name }}";
    const SERIAL_NUMBER: &'static str = "1"; // TODO: Change this

}

// Layout configuration
use rumcake::keyboard::KeyboardLayout;
impl KeyboardLayout for {{ keyboard-name }} {
    // Use the remapping above to create the keyboard layout
    remap! {
        build_layout! {
            {
                [ Escape 1    2     3     4    5  6    7    8     9 0 -    =   '\\'   Delete Home ]
                [ Tab    Q    W     E     R    T  Y    U    I     O P '['  ']' BSpace PgUp   ]
                [ LCtrl  A    S     D     F    G  H    J    K     L ; '\'' No  Enter  PgDown ]
                [ LShift No   Z     X     C    V  B    N    M     , . /    Up  No     End    ]
                [ LGui   LAlt Space Space RAlt No Left Down Right ]
            }
        }
    }
}

// Matrix configuration
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for {{ keyboard-name }} {
    build_matrix! {
        { PB3 PB4 PA15 PB5 PA0 PA1 PB10 PB11 PA2 PA3 } // Rows
        { PB12 PB2 PB1 PB0 PA7 PA6 PA5 PA4 } // Columns
    }
}

// USB configuration
use rumcake::usb::USBKeyboard;
impl USBKeyboard for {{ keyboard-name }}  {
    const USB_VID: u16 = 0x0000; // TODO: Change this
    const USB_PID: u16 = 0x0000; // TODO:: Change this
}

// Via configuration
use rumcake::eeprom::KeyboardWithEEPROM;
impl KeyboardWithEEPROM for {{ keyboard-name }} {}
use rumcake::via::ViaKeyboard;
impl ViaKeyboard for {{ keyboard-name }} {}

// Underglow configuration + driver
use rumcake::underglow::UnderglowDevice;
impl UnderglowDevice for {{ keyboard-name }} {
    const NUM_LEDS: usize = 18; // TODO: Change this
}
use rumcake::drivers::ws2812_bitbang::underglow::WS2812BitbangUnderglowDriver;
impl WS2812BitbangUnderglowDriver for {{ keyboard-name }} {
    ws2812_pin! { PA10 } // TODO: Change this
}

