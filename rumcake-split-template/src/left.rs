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
use rumcake::{build_layout, build_matrix};

#[keyboard]
pub struct {{ keyboard-name }}Left;

// Basic keyboard configuration
use rumcake::keyboard::Keyboard;
impl Keyboard for {{ keyboard-name }}Left {
    const MANUFACTURER: &'static str = ""; // TODO: Change this
    const PRODUCT: &'static str = "{{ keyboard-name }}";

    // Use the remapping above to create the keyboard layout
    build_layout! {
        {
            [ Escape Q  W  E R      T    Y      U     I O  P  '['  ]
            [ Tab    A  S  D F      G    H      J     K L  ;  '\'' ]
            [ LCtrl  Z  X  C V      B    N      M     , .  /  ']'  ]
            [ No     No No 1 LShift LAlt BSpace Space 2 No No No   ]
        }
    }
}
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for {{ keyboard-name }}Left {
    build_matrix! {
        { P0_02 P1_13 P1_11 P0_10 } // Rows
        { P0_22 P0_24 P1_00 P0_11 P1_04 P1_06 } // Columns
    }
}

// Bluetooth configuration
use rumcake::nrf_ble::NRFBluetoothKeyboard;
impl NRFBluetoothKeyboard for {{ keyboard-name }}Left {
    const BLE_VID: u16 = 0x0000; // TODO: Change this
    const BLE_PID: u16 = 0x0000; // TODO: Change this
}

// Split keyboard setup
use rumcake::split::drivers::nrf_ble::central::NRFBLECentralDevice;
impl NRFBLECentralDevice for {{ keyboard-name }}Left {}

// USB configuration
use rumcake::usb::USBKeyboard;
impl USBKeyboard for {{ keyboard-name }}Left {
    const USB_VID: u16 = 0x0000; // TODO: Change this
    const USB_PID: u16 = 0x0000; // TODO: Change this
}
