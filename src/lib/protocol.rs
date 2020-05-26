/*
 * File: protocol.rs
 * Date: 04.05.2020
 * Author: MarkAtk
 *
 * MIT License
 *
 * Copyright (c) 2020 MarkAtk
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use std::convert::From;
use std::fmt;

use super::error::Result;
use super::utils::bool_arr_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LokProtocol {
    Motorola = 0,
    Selectrix = 1,
    DCC = 2,
    FMZ = 3
}

impl From<u8> for LokProtocol {
    fn from(value: u8) -> LokProtocol {
        match value {
            1 => LokProtocol::Selectrix,
            2 => LokProtocol::DCC,
            3 => LokProtocol::FMZ,
            0 | _ => LokProtocol::Motorola
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DeviceStatus {
    pub stop_pressed: bool,
    pub go_pressed: bool,
    pub hot: bool,
    pub power: bool,
    pub halt: bool,
    pub external_central_unit: bool,
    pub voltage_regulation: bool
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stop: {}\nGo: {}\nHot: {}\nPower: {}\nHalt: {}\nExternal Central Unit: {}\nVoltage Regulation: {}",
            self.stop_pressed,
            self.go_pressed,
            self.hot,
            self.power,
            self.halt,
            self.external_central_unit,
            self.voltage_regulation
        )
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct XLokOptions {
    pub emergency_stop: bool,
    pub force: bool,
    pub light: bool,
    pub functions: Option<[bool; 4]>
}

#[derive(Debug, Copy, Clone)]
pub struct XLokStatus {
    pub speed: i8,
    pub real_speed: i8,
    pub options: XLokOptions,
}

impl fmt::Display for XLokStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Speed: {}\nReal speed: {}\nEmergency stop: {}\nForce: {}\nLight: {}\nFunctions: {}",
            self.speed,
            self.real_speed,
            self.options.emergency_stop,
            self.options.force,
            self.options.light,
            bool_arr_to_string(&self.options.functions.unwrap())
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct XLokConfig {
    pub protocol: LokProtocol,
    pub speed_steps: u8,
    pub virtual_address: Option<u16>
}

impl fmt::Display for XLokConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Protocol: {:?}\nSpeed steps: {}\nVirtual address: {:?}", self.protocol, self.speed_steps, self.virtual_address)
    }
}

pub trait P50XBinary {
    fn xpower_off(&mut self) -> Result<()>;
    fn xpower_on(&mut self) -> Result<()>;
    fn xhalt(&mut self) -> Result<()>;
    fn xso_set(&mut self, special_option: u16, value: u8) -> Result<()>;
    fn xso_get(&mut self, special_option: u16) -> Result<u8>;
    fn xversion(&mut self) -> Result<Vec<u8>>;
    fn xp50xch(&mut self, extended_character: u8) -> Result<()>;
    fn xstatus(&mut self) -> Result<DeviceStatus>;
    fn xnop(&mut self) -> Result<()>;

    fn xlok(&mut self, address: u16, speed: i8, options: XLokOptions) -> Result<()>;
    fn xlok_status(&mut self, address: u16) -> Result<XLokStatus>;
    fn xlok_config(&mut self, address: u16) -> Result<XLokConfig>;
    fn xlok_dispatch(&mut self, address: u16) -> Result<Option<u8>>;
    fn xfunc(&mut self, address: u16, functions: [bool; 8]) -> Result<()>;
    fn xfunc_status(&mut self, address: u16) -> Result<[bool; 8]>;
    fn xfuncx(&mut self, address: u16, functions: [bool; 8]) -> Result<()>;
    fn xfuncx_status(&mut self, address: u16) -> Result<[bool; 8]>;
}
