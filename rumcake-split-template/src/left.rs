#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::{build_layout, build_standard_matrix};

#[keyboard(
    usb,
    bluetooth,
    split_central(
        driver_setup_fn = setup_nrf_ble,
        driver_type = "nrf-ble"
    )
)]
pub struct {{ keyboard-name }}Left;

// Basic keyboard configuration
use rumcake::keyboard::Keyboard;
impl Keyboard for {{ keyboard-name }}Left {
    const MANUFACTURER: &'static str = ""; // TODO: Change this
    const PRODUCT: &'static str = "{{ keyboard-name }}";
}

// Layout configuration
use rumcake::bluetooth::BluetoothCommand::*;
use rumcake::keyberon::action::{Action::Custom, Action::*};
use rumcake::keyboard::{KeyboardLayout, Keycode::*};
impl KeyboardLayout for {{ keyboard-name }}Left {
    build_layout! {
        {
            [ Escape Q  W  E R      T    Y      U     I O  P  '['  ]
            [ Tab    A  S  D F      G    H      J     K L  ;  '\'' ]
            [ LCtrl  Z  X  C V      B    N      M     , .  /  ']'  ]
            [ No     No No 1 LShift LAlt BSpace Space 2 No No No   ]
        }
        {
            [ LGui                              F1 F2 F3 F4 F5 F6      F7     F8   F9    F10 F11 ]
            [ t                                 t  t  t  t  t  Left    Down   Up   Right t   t   ]
            [ {Custom(Bluetooth(ToggleOutput))} t  t  t  t  t  Home    PgDown PgUp End   t   F12 ]
            [ t                                 t  t  t  t  t  PScreen Enter  t    t     t   t   ]
        }
        {
            [ t   1 2 3 4 5      6 7 8 9 0    '(' ]
            [ t   t t t t t      - = t t t    t   ]
            [ '`' t t t t t      t t t t '\\' ')' ]
            [ t   t t t t Delete t t t t t    t   ]
        }
    }
}

// Matrix configuration
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for {{ keyboard-name }}Left {
    type Layout = Self; // send matrix events from this half's matrix to the layout

    build_standard_matrix! {
        rows: [ P0_02 P1_13 P1_11 P0_10 ], // Rows
        cols: [ P0_22 P0_24 P1_00 P0_11 P1_04 P1_06 ] // Columns
    }
}

// Bluetooth configuration
use rumcake::hw::mcu::BluetoothDevice;
impl BluetoothDevice for {{ keyboard-name }}Left {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // TODO: Change this
}

use rumcake::bluetooth::BluetoothKeyboard;
impl BluetoothKeyboard for {{ keyboard-name }}Left {
    const BLE_VID: u16 = 0x0000; // TODO: Change this
    const BLE_PID: u16 = 0x0000; // TODO: Change this
}

// Split keyboard setup
use rumcake::split::central::{CentralDevice, CentralDeviceDriver};
use rumcake::drivers::nrf_ble::central::setup_nrf_ble_split_central;
impl CentralDevice for {{ keyboard-name }}Left {
    type Layout = Self; // send matrix events from peripherals to the implemented layout
}
async fn setup_nrf_ble() -> (impl CentralDeviceDriver, &'static [[u8; 6]]) {
    setup_nrf_ble_split_central! {
        peripheral_addresses: [[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]], // TODO: Change this, must contain the address for the right half.
    }
}

// USB configuration
use rumcake::usb::USBKeyboard;
impl USBKeyboard for {{ keyboard-name }}Left {
    const USB_VID: u16 = 0x0000; // TODO: Change this
    const USB_PID: u16 = 0x0000; // TODO: Change this
}
