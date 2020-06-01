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
use safe_transmute::{guarded_transmute_to_bytes_pod, guarded_transmute_pod};

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

        let mut serial = Serial::open_with_settings(port_name, settings)?;

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

    pub fn set_timeout(&mut self, timeout: u64) -> Result<()> {
        match self.serial.set_timeout(timeout) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::from(err))
        }
    }

    fn send_u8(&mut self, value: u8) -> Result<()> {
        let request = radix_string(&[value], &self.check_settings.input_format)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        return Ok(());
    }

    fn send_u16(&mut self, value: u16) -> Result<()> {
        let data = guarded_transmute_to_bytes_pod::<u16>(&value);
        let request = radix_string(data, &self.check_settings.input_format)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        return Ok(());
    }

    fn send_u32(&mut self, value: u32) -> Result<()> {
        let data = guarded_transmute_to_bytes_pod::<u32>(&value);
        let request = radix_string(data, &self.check_settings.input_format)?;
        self.serial.write_format(&request, self.check_settings.input_format)?;

        return Ok(());
    }

    fn send_x(&mut self) -> Result<()> {
        self.send_u8(self.extended_character)
    }

    fn recv(&mut self, length: usize) -> Result<Vec<u8>> {
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

    fn recv_u8(&mut self) -> Result<u8> {
        let data = self.recv(1)?;

        return Ok(data[0]);
    }

    fn recv_u16(&mut self) -> Result<u16> {
        let data = self.recv(2)?;
        let result = guarded_transmute_pod(&data).unwrap();

        return Ok(result);
    }

    fn recv_reply(&mut self) -> Result<P50XReply> {
        let data = self.recv_u8()?;

        return Ok(data.into());
    }

    fn xrecv(&mut self, valid_responses: &[P50XReply]) -> Result<P50XReply> {
        let response = self.recv_reply()?;

        if valid_responses.contains(&response) == false {
            return Err(Error::from(response));
        }

        return Ok(response);
    }

    fn xrecv_ok(&mut self) -> Result<P50XReply> {
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

    fn xpower_on(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xa7)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xhalt(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xa5)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xso_set(&mut self, special_option: u16, value: u8) -> Result<()> {
        self.send_x()?;
        self.send_u8(0xa3)?;
        self.send_u16(special_option)?;
        self.send_u8(value)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xso_get(&mut self, special_option: u16) -> Result<u8> {
        self.send_x()?;
        self.send_u8(0xa4)?;
        self.send_u16(special_option)?;

        self.xrecv_ok()?;
        let data = self.recv_u8()?;

        return Ok(data);
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

    fn xsensor(&mut self, module: u8) -> Result<[bool; 16]> {
        self.send_x()?;
        self.send_u8(0x98)?;
        self.send_u8(module)?;

        self.xrecv_ok()?;
        let data = self.recv_u16()?;

        let mut result = [false; 16];

        for i in 0..16 {
            if data & (1 << i) != 0 {
                result[i] = true;
            }
        }

        return Ok(result);
    }

    fn xsens_off(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0x99)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn x88p_get(&mut self, parameter: u8) -> Result<u8> {
        self.send_x()?;
        self.send_u8(0x9C)?;
        self.send_u8(parameter)?;

        self.xrecv_ok()?;
        let data = self.recv_u8()?;

        return Ok(data);
    }

    fn x88p_set(&mut self, parameter: u8, value: u8) -> Result<()> {
        self.send_x()?;
        self.send_u8(0x9D)?;
        self.send_u8(parameter)?;
        self.send_u8(value)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xs88_timer(&mut self, timer: u8, reset: bool) -> Result<u16> {
        self.send_x()?;
        self.send_u8(0x9E)?;

        let mut param = timer & 0x0F;
        if reset {
            param |= 0x80;
        }

        self.send_u8(param)?;

        self.xrecv_ok()?;
        let result = self.recv_u16()?;

        return Ok(result);
    }

    fn xs88_count(&mut self, timer: u8, reset: bool) -> Result<u16> {
        self.send_x()?;
        self.send_u8(0x9F)?;

        let mut param = timer & 0x0F;
        if reset {
            param |= 0x80;
        }

        self.send_u8(param)?;

        self.xrecv_ok()?;
        let result = self.recv_u16()?;

        return Ok(result);
    }

    fn xlok(&mut self, address: u16, speed: i8, options: XLokOptions) -> Result<()> {
        // get config byte from options
        let mut config: u8 = 0;

        if options.light {
            config |= 0x10;
        }

        if options.force {
            config |= 0x40;
        }

        if let Some(functions) = options.functions {
            config |= 0x80;

            for i in 0..4 {
                if functions[3 - i] {
                    config |= 1 << i;
                }
            }
        }

        // get actual speed value
        let speed_value: u8 = if options.emergency_stop {
            // speed 1 maps to emergency stop
            1
        } else {
            if speed >= 0 {
                speed as u8
            } else {
                config |= 0x20;

                (-speed) as u8
            }
        };

        self.send_x()?;
        self.send_u8(0x80)?;
        self.send_u16(address)?;
        self.send_u8(speed_value as u8)?;
        self.send_u8(config)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xlok_status(&mut self, address: u16) -> Result<XLokStatus> {
        self.send_x()?;
        self.send_u8(0x84)?;
        self.send_u16(address)?;

        self.xrecv_ok()?;
        let mut speed = self.recv_u8()? as i8;
        let config = self.recv_u8()?;
        let mut real_speed = self.recv_u8()? as i8;

        if config & 0x20 != 0 {
            speed = -speed;
            real_speed = -real_speed;
        }

        return Ok(XLokStatus {
            speed,
            real_speed,
            options: XLokOptions {
                emergency_stop: speed == 1,
                force: false,
                light: config & 0x10 != 0,
                functions: Some([config & 0x01 != 0, config & 0x02 != 0, config & 0x04 != 0, config & 0x08 != 0])
            }
        });
    }

    fn xlok_config(&mut self, address: u16) -> Result<XLokConfig> {
        self.send_x()?;
        self.send_u8(0x85)?;
        self.send_u16(address)?;

        self.xrecv_ok()?;
        let protocol = self.recv_u8()?;
        let speed_steps = self.recv_u8()?;
        let virtual_address_value = self.recv_u16()?;

        let virtual_address = if virtual_address_value == 0xFFFF {
            None
        } else {
            Some(virtual_address_value)
        };

        return Ok(XLokConfig {
            protocol: XProtocol::from(protocol),
            speed_steps,
            virtual_address
        });
    }

    fn xlok_dispatch(&mut self, address: u16) -> Result<Option<u8>> {
        self.send_x()?;
        self.send_u8(0x83)?;
        self.send_u16(address)?;

        if address & 0xFF00 != 0 {
            let result = self.recv_u8()?;

            return Ok(Some(result));
        } else {
            self.xrecv_ok()?;

            return Ok(None);
        }
    }

    fn xfunc(&mut self, address: u16, functions: [bool; 8]) -> Result<()> {
        let mut value: u8 = 0;

        for i in 0..8 {
            if functions[i] {
                value |= 1 << i;
            }
        }

        self.send_x()?;
        self.send_u8(0x88)?;
        self.send_u16(address)?;
        self.send_u8(value)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xfunc_status(&mut self, address: u16) -> Result<[bool; 8]> {
        self.send_x()?;
        self.send_u8(0x8C)?;
        self.send_u16(address)?;

        self.xrecv_ok()?;
        let data = self.recv_u8()?;
        let mut functions: [bool; 8] = [false; 8];

        for i in 0..8 {
            if data & (1 << i) != 0 {
                functions[i] = true;
            }
        }

        return Ok(functions);
    }

    fn xfuncx(&mut self, address: u16, functions: [bool; 8]) -> Result<()> {
        let mut value: u8 = 0;

        for i in 0..8 {
            if functions[i] {
                value |= 1 << i;
            }
        }

        self.send_x()?;
        self.send_u8(0x89)?;
        self.send_u16(address)?;
        self.send_u8(value)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xfuncx_status(&mut self, address: u16) -> Result<[bool; 8]> {
        self.send_x()?;
        self.send_u8(0x8D)?;
        self.send_u16(address)?;

        self.xrecv_ok()?;
        let data = self.recv_u8()?;
        let mut functions: [bool; 8] = [false; 8];

        for i in 0..8 {
            if data & (1 << i) != 0 {
                functions[i] = true;
            }
        }

        return Ok(functions);
    }

    fn xturnout(&mut self, address: u16, state: bool, options: XTurnoutOptions) -> Result<()> {
        let address_bytes = address.to_le_bytes();
        let mut data = address_bytes[1] & 0x07;

        if state {
            data |= 0x80;
        }

        if options.status {
            data |= 0x40;
        }

        if options.reserve {
            data |= 0x20;
        }

        if options.no_command {
            data |= 0x10;
        }

        self.send_x()?;
        self.send_u8(0x90)?;
        self.send_u8(address_bytes[0])?;
        self.send_u8(data)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xturnout_free(&mut self) -> Result<()> {
        self.send_x()?;
        self.send_u8(0x93)?;

        self.xrecv_ok()?;

        return Ok(());
    }

    fn xturnout_status(&mut self, address: u16) -> Result<XTurnoutStatus> {
        self.send_x()?;
        self.send_u8(0x94)?;
        self.send_u16(address)?;

        self.xrecv_ok()?;
        let data = self.recv_u8()?;

        let protocol = ((data & 0x01) << 1) | ((data & 0x08) >> 3);

        return Ok(XTurnoutStatus {
            protocol: XProtocol::from(protocol),
            reserved: data & 0x02 != 0,
            state: data & 0x04 != 0
        });
    }

    fn xturnout_group(&mut self, group_address: u8) -> Result<[(bool, bool); 8]> {
        self.send_x()?;
        self.send_u8(0x95)?;
        self.send_u8(group_address)?;

        self.xrecv_ok()?;
        let state = self.recv_u8()?;
        let reserved = self.recv_u8()?;

        let mut result = [(false, false); 8];

        for i in 0..8 {
            let current_state = state & (1 << i) != 0;
            let current_reserved = reserved & (1 << i) != 0;

            result[i] = (current_state, current_reserved);
        }

        return Ok(result);
    }
}
