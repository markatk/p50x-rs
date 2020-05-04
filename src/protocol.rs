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

use crate::Result;

#[derive(Debug)]
pub struct DeviceStatus {
    pub stop_pressed: bool,
    pub go_pressed: bool,
    pub hot: bool,
    pub power: bool,
    pub halt: bool,
    pub external_central_unit: bool,
    pub voltage_regulation: bool
}

pub trait P50X {
    fn xpower_off(&mut self) -> Result<()>;
    fn xpower_on(&mut self) -> Result<bool>;
    fn xhalt(&mut self) -> Result<()>;
    fn xversion(&mut self) -> Result<Vec<u8>>;
    fn xstatus(&mut self) -> Result<DeviceStatus>;
    fn xnop(&mut self) -> Result<()>;
}
