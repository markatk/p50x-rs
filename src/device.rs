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
use serial_unit_testing::utils::{TextFormat, bytes_from_hex_string, radix_string};

use crate::error::*;
use crate::protocol::*;

pub struct Device {
    serial: Serial,
    check_settings: CheckSettings,
    extended_character: u8
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
            check_settings,
            extended_character: 0x58
        });
    }

    fn xsend(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        let request = self.build_request(data)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        let response = self.serial.read_str_with_format(self.check_settings.output_format)?;
        let response_data = bytes_from_hex_string(&response)?;

        return Ok(response_data);
    }

    fn xcommand(&mut self, data: &[u8], desired_data: &[u8]) -> Result<(bool, Vec<u8>)> {
        let request = self.build_request(data)?;
        let desired_response = radix_string(desired_data, &self.check_settings.input_format)?;

        let (result, actual_response) = self.serial.check_with_settings(&request, &desired_response, &self.check_settings)?;
        let actual_response_data = bytes_from_hex_string(&actual_response)?;

        return Ok((result, actual_response_data));
    }

    fn verify_connection(serial: &mut Serial, check_settings: &CheckSettings) -> Result<bool> {
        let (xresult, _) = serial.check_with_settings("58C4", "00", check_settings)?;
        let (result, _) = serial.check_with_settings("C4", "0000", check_settings)?;

        return Ok(result && xresult);
    }

    fn build_request(&self, data: &[u8]) -> Result<String> {
        let request = radix_string(&[&[self.extended_character], data].concat(), &self.check_settings.input_format)?;

        return Ok(request);
    }
}

impl P50X for Device {
    fn xpower_off(&mut self) -> Result<()> {
        let (result, response) = self.xcommand(&[0xa6], &[0x00])?;

        match result {
            true => Ok(()),
            false => {
                let response_str = radix_string(&response, &TextFormat::Hex)?;

                Err(Error::UnknownResponse(response_str))
            }
        }
    }

    fn xpower_on(&mut self) -> Result<bool> {
        let (result, response) = self.xcommand(&[0xa7], &[0x00])?;

        match result {
            true => Ok(true),
            false => {
                if  response.is_empty() == false && response[0] == 0x06 {
                    Ok(false)
                } else {
                    let response_str = radix_string(&response, &TextFormat::Hex)?;

                    Err(Error::UnknownResponse(response_str))
                }
            }
        }
    }

    fn xhalt(&mut self) -> Result<()> {
        let (result, response) = self.xcommand(&[0xa5], &[0x00])?;

        match result {
            true => Ok(()),
            false => {
                let response_str = radix_string(&response, &TextFormat::Hex)?;

                Err(Error::UnknownResponse(response_str))
            }
        }
    }

    fn xversion(&mut self) -> Result<Vec<u8>> {
        let data = self.xsend(&[0xa0])?;

        return Ok(data);
    }

    fn xstatus(&mut self) -> Result<DeviceStatus> {
        let data = self.xsend(&[0xa2])?;

        return Ok(DeviceStatus {
            stop_pressed: data[0] & 0x01 != 0,
            go_pressed: data[0] & 0x02 != 0,
            hot: data[0] & 0x04 != 0,
            power: data[0] & 0x08 != 0,
            halt: data[0] & 0x10 != 0,
            external_central_unit: data[0] & 0x20 != 0,
            voltage_regulation: data[0] & 0x40 != 0
        });
    }

    fn xnop(&mut self) -> Result<()> {
        let (result, response) = self.xcommand(&[0xc4], &[0x00])?;

        match result {
            true => Ok(()),
            false => {
                let response_str = radix_string(&response, &TextFormat::Hex)?;

                Err(Error::UnknownResponse(response_str))
            }
        }
    }
}
