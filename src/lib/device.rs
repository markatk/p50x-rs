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
use safe_transmute::guarded_transmute_to_bytes_pod;

use super::error::*;
use super::protocol::*;
use super::reply::P50XReply;

pub struct Device {
    serial: Serial,
    check_settings: CheckSettings,
    extended_character: u8,
    read_buffer: Vec<u8>
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
            extended_character: 0x58,
            read_buffer: Vec::new()
        });
    }

    pub fn send_u8(&mut self, value: u8) -> Result<()> {
        let request = radix_string(&[value], &self.check_settings.input_format)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        return Ok(());
    }

    pub fn send_u16(&mut self, value: u16) -> Result<()> {
        let data = guarded_transmute_to_bytes_pod::<u16>(&value);
        let request = radix_string(data, &self.check_settings.input_format)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        return Ok(());
    }

    pub fn send_u32(&mut self, value: u32) -> Result<()> {
        let data = guarded_transmute_to_bytes_pod::<u32>(&value);
        let request = radix_string(data, &self.check_settings.input_format)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        return Ok(());
    }

    pub fn send_x(&mut self) -> Result<()> {
        self.send_u8(self.extended_character)
    }

    pub fn recv(&mut self, length: usize) -> Result<Vec<u8>> {
        // if requested data is already completely read
        if self.read_buffer.len() >= length {
            let mut data: Vec<u8> = Vec::new();

            for _ in 0..length {
                data.push(self.read_buffer[0]);
                self.read_buffer.remove(0);
            }

            return Ok(data);
        }

        let actual_length = length - self.read_buffer.len();

        let response = self.serial.read_min_str_with_format(actual_length, self.check_settings.output_format)?;
        let mut data = bytes_from_hex_string(&response)?;

        // insert read_buffer
        if self.read_buffer.is_empty() && data.len() > length {
            self.read_buffer = data.split_off(length);
        } else if self.read_buffer.is_empty() == false {
            for i in 0..self.read_buffer.len() {
                data.insert(i, self.read_buffer[i]);
            }
        }

        return Ok(data);
    }

    pub fn recv_u8(&mut self) -> Result<u8> {
        let data = self.recv(1)?;

        return Ok(data[0]);
    }

    pub fn recv_reply(&mut self) -> Result<P50XReply> {
        let data = self.recv_u8()?;

        return Ok(data.into());
    }

    pub fn xrecv(&mut self, valid_responses: &[P50XReply]) -> Result<P50XReply> {
        let response = self.recv_reply()?;

        if valid_responses.contains(&response) == false {
            return Err(Error::from(response));
        }

        return Ok(response);
    }

    pub fn xrecv_ok(&mut self) -> Result<P50XReply> {
        self.xrecv(&[P50XReply::Ok])
    }

    fn verify_connection(serial: &mut Serial, check_settings: &CheckSettings) -> Result<bool> {
        let (xresult, _) = serial.check_with_settings("58C4", "00", check_settings)?;
        let (result, _) = serial.check_with_settings("C4", "0000", check_settings)?;

        return Ok(result && xresult);
    }
}

impl P50XBinary for Device {
    fn xpower_off(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xa6)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xpower_on(&mut self) -> Result<bool> {
        self.send_x()?;
        self.send_u8(0xa7)?;

        let response = self.xrecv(&[P50XReply::Ok, P50XReply::PowerOff])?;

        return Ok(response == P50XReply::Ok);
    }

    fn xhalt(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xa5)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xso_set(&mut self, special_option: u16, value: u8) -> Result<P50XReply> {
        self.send_x()?;
        self.send_u8(0xa3)?;
        self.send_u16(special_option)?;
        self.send_u8(value)?;

        let response = self.xrecv(&[P50XReply::Ok, P50XReply::BadCommand, P50XReply::BadParameter, P50XReply::BadSpecialOptionValue])?;

        return Ok(response);
    }

    fn xso_get(&mut self, special_option: u16) -> Result<Option<u8>> {
        self.send_x()?;
        self.send_u8(0xa4)?;
        self.send_u16(special_option)?;

        let response = self.xrecv(&[P50XReply::Ok, P50XReply::BadParameter])?;
        if response == P50XReply::BadParameter {
            return Ok(None);
        }

        let data = self.recv_u8()?;

        return Ok(Some(data));
    }

    fn xversion(&mut self) -> Result<Vec<u8>> {
        self.send_x()?;
        self.send_u8(0xa0)?;

        let mut data: Vec<u8> = Vec::new();

        loop {
            let length = self.recv_u8()?;
            data.push(length);

            if length == 0 {
                break;
            }

            let mut chunk = self.recv(length as usize)?;
            data.append(&mut chunk);
        }

        return Ok(data);
    }

    fn xp50xch(&mut self, extended_character: u8) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xa1)?;
        self.send_u8(extended_character)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xstatus(&mut self) -> Result<DeviceStatus> {
        self.send_x()?;
        self.send_u8(0xa2)?;

        let data = self.recv_u8()?;

        return Ok(DeviceStatus {
            stop_pressed: data & 0x01 != 0,
            go_pressed: data & 0x02 != 0,
            hot: data & 0x04 != 0,
            power: data & 0x08 != 0,
            halt: data & 0x10 != 0,
            external_central_unit: data & 0x20 != 0,
            voltage_regulation: data & 0x40 != 0
        });
    }

    fn xnop(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xc4)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    // fn xlok(&mut self, address: u16, speed: i8) -> Result<()> {
    //     self.send_x()?;
    //     self.send_u8(0x80)?;
    //     self.send_u16(address)?;

    // }
}
