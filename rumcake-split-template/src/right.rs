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
use rumcake::keyboard::KeyboardMatrix;
use rumcake::{build_layout, build_matrix};

#[keyboard]
pub struct {{ keyboard-name }}Right;

// Since this is a peripheral device, this only needs a matrix
impl KeyboardMatrix for {{ keyboard-name }}Right {
    build_matrix! {
        { P0_22 P1_00 P0_11 P1_04 } // Rows
        { P0_09 P0_10 P1_11 P1_13 P1_15 P0_02 } // Columns
    }
}

// Split keyboard setup
use rumcake::split::drivers::nrf_ble::peripheral::NRFBLEPeripheralDevice;
impl NRFBLEPeripheralDevice for {{ keyboard-name }}Right {}
