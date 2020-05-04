/*
 * File: device.rs
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

use serial_unit_testing::serial::*;
use serial_unit_testing::utils::TextFormat;

use crate::error::*;

pub struct Device {
    serial: Serial,
    check_settings: CheckSettings
}

impl Device {
    pub fn new(port_name: &str, baud_rate: u32) -> Result<Device> {
        let settings = settings::Settings {
            baud_rate,
            timeout: 1000,
            ..Default::default()
        };

        let mut serial = Serial::open_with_settings(port_name, &settings)?;

        // verify device is p50x device
        let check_settings = CheckSettings {
            ignore_case: true,
            input_format: TextFormat::Hex,
            output_format: TextFormat::Hex
        };

        let verified = Device::verify_connection(&mut serial, &check_settings)?;
        if verified == false {
            return Err(Error::UnknownDevice);
        }

        return Ok(Device {
            serial,
            check_settings
        });
    }

    fn verify_connection(serial: &mut Serial, check_settings: &CheckSettings) -> Result<bool> {
        let (result, _) = serial.check_with_settings("58C4", "00", check_settings)?;

        return Ok(result);
    }
}
