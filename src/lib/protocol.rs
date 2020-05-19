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

use super::error::Result;
use super::reply::P50XReply;

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

#[derive(Debug, Copy, Clone, Default)]
pub struct XLokOptions {
    pub emergency_stop: bool,
    pub force: bool,
    pub light: bool,
    pub functions: Option<[bool; 4]>
}

pub trait P50XBinary {
    fn xpower_off(&mut self) -> Result<()>;
    fn xpower_on(&mut self) -> Result<bool>;
    fn xhalt(&mut self) -> Result<()>;
    fn xso_set(&mut self, special_option: u16, value: u8) -> Result<P50XReply>;
    fn xso_get(&mut self, special_option: u16) -> Result<Option<u8>>;
    fn xversion(&mut self) -> Result<Vec<u8>>;
    fn xp50xch(&mut self, extended_character: u8) -> Result<()>;
    fn xstatus(&mut self) -> Result<DeviceStatus>;
    fn xnop(&mut self) -> Result<()>;

    fn xlok(&mut self, address: u16, speed: i8, options: XLokOptions) -> Result<P50XReply>;
}
