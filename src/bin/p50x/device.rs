/*
 * File: device.rs
 * Date: 18.05.2020
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

use clap::{ArgMatches, App};
use p50x::P50XBinary;
use serial_unit_testing::utils::{radix_string, TextFormat};

use crate::utils::{command_group, common_command, run_command_with_result};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("status", Some(m)) => run_command_with_result(
            m,
            |device| device.xstatus(),
            |result| {
                Ok(format!(
                    "Stop: {}\nGo: {}\nHot: {}\nPower: {}\nHalt: {}\nExternal Central Unit: {}\nVoltage Regulation: {}",
                    result.stop_pressed,
                    result.go_pressed,
                    result.hot,
                    result.power,
                    result.halt,
                    result.external_central_unit,
                    result.voltage_regulation
                ))
            })?,
        ("version", Some(m)) => run_command_with_result(
            m,
            |device| device.xversion(),
            |result| {
                let mut version = String::new();
                let mut index = 0;

                loop {
                    let length = result[index] as usize;
                    if length == 0 {
                        break;
                    }

                    let data = &result[index + 1..index + 1 + length];
                    version.push_str("0x");

                    match radix_string(data, &TextFormat::Hex) {
                        Ok(hex_string) => version.push_str(&hex_string),
                        Err(err) => return Err(format!("Unable to format version: {}", err.to_string()))
                    };

                    index += 1 + length;

                    if index < result.len() - 1 {
                        version.push('\n');
                    }
                }

                return Ok(version);
            })?,
        _ => ()
    };

    return Ok(());
}

pub fn command<'a>() -> App<'a, 'a> {
    command_group(
        "device",
        "Get information about the device",
        vec![
            common_command("status", "Get device current status"),
            common_command("version", "Get device version")
        ]
    )
}
