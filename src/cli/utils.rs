/*
 * File: commands.rs
 * Date: 12.05.2020
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

use std::error::Error;
use clap::{Arg, ArgMatches};
use p50x::Device;

pub fn common_args<'a>() -> Vec<Arg<'a, 'a>> {
    vec![
        Arg::with_name("port")
            .help("Serial port OS specific name")
            .required(true)
            .takes_value(true),
        Arg::with_name("baud")
            .long("baud")
            .short("b")
            .help("Serial port baud rate")
            .takes_value(true)
            .default_value("19200"),
        Arg::with_name("timeout")
            .long("timeout")
            .short("t")
            .help("Serial port timeout duration in ms")
            .takes_value(true)
            .default_value("1000")
    ]
}

pub fn get_device(matches: &ArgMatches) -> Result<Device, String> {
    // TODO: Set timeout
    let port_name = matches.value_of("port").unwrap();
    let baud_rate_arg = matches.value_of("baud").unwrap();

    let baud_rate: u32;

    if let Ok(value) = baud_rate_arg.parse::<u32>() {
        baud_rate = value;
    } else {
        return Err(format!("Invalid baud rate: {}", baud_rate_arg));
    }

    match Device::new(port_name, baud_rate) {
        Ok(device) => Ok(device),
        Err(err) => Err(err.description().to_string())
    }
}
