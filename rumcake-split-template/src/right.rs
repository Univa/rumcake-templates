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

#[keyboard(split_peripheral(driver_setup_fn = setup_nrf_ble, driver_type = "nrf-ble"))]
pub struct {{ keyboard-name }}Right;

use rumcake::keyboard::Keyboard;
impl Keyboard for {{ keyboard-name }}Right {
    // Needed for advertising data (Bluetooth GAP)
    const MANUFACTURER: &'static str = ""; // TODO: Change this
    const PRODUCT: &'static str = "{{ keyboard-name }}";
}

// Since this is a peripheral device, this only needs a matrix
use rumcake::keyboard::KeyboardMatrix;
impl KeyboardMatrix for {{ keyboard-name }}Right {
    type PeripheralDeviceType = Self; // send matrix events to the central device via the peripheral device's driver

    build_standard_matrix! {
        rows: [ P0_22 P1_00 P0_11 P1_04 ], // Rows
        cols: [ P0_09 P0_10 P1_11 P1_13 P1_15 P0_02 ] // Columns
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
use ::rumcake::split::peripheral::{PeripheralDevice, PeripheralDeviceDriver};
use rumcake::drivers::nrf_ble::peripheral::setup_nrf_ble_split_peripheral;
impl PeripheralDevice for {{ keyboard-name }}Right {}
async fn setup_nrf_ble() -> (impl PeripheralDeviceDriver, [u8; 6]) {
    setup_nrf_ble_split_central! {
        peripheral_addresses: [[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]], // TODO: Change this, must match the left half's BLUETOOTH_ADDRESS
    }
}
