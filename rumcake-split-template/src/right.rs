#![no_main]
#![no_std]
#![feature(macro_metavar_expr)]
#![feature(type_alias_impl_trait)]
#![feature(generic_const_exprs)]

use defmt_rtt as _;
use panic_probe as _;
use rumcake::keyberon;
use rumcake::keyboard;
use rumcake::keyboard::KeyboardMatrix;
use rumcake::{build_layout, build_matrix};

#[keyboard(split_peripheral(driver = "ble"))]
pub struct {{ keyboard-name }}Right;

use rumcake::keyboard::Keyboard;
impl Keyboard for {{ keyboard-name }}Right {
    // Needed for advertising data (Bluetooth GAP)
    const MANUFACTURER: &'static str = ""; // TODO: Change this
    const PRODUCT: &'static str = "{{ keyboard-name }}";
}

// Since this is a peripheral device, this only needs a matrix
impl KeyboardMatrix for {{ keyboard-name }}Right {
    build_matrix! {
        { P0_22 P1_00 P0_11 P1_04 } // Rows
        { P0_09 P0_10 P1_11 P1_13 P1_15 P0_02 } // Columns
    }

    fn remap_to_layout(row: u8, col: u8) -> (u8, u8) {
        // Since the layout is stored on the central device, we need to remap the matrix events
        // to the proper coordinates on the layout

        (row, 6 + col)
    }
}

// Bluetooth configuration
use rumcake::hw::mcu::BluetoothDevice;
impl BluetoothDevice for {{ keyboard-name }}Right {
    const BLUETOOTH_ADDRESS: [u8; 6] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // TODO: Change this
}

// Split keyboard setup
use rumcake::drivers::nrf_ble::peripheral::NRFBLEPeripheralDevice;
impl NRFBLEPeripheralDevice for {{ keyboard-name }}Right {
    const CENTRAL_ADDRESS: [u8; 6] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // TODO: Change this, must match the left half's BLUETOOTH_ADDRESS
}
