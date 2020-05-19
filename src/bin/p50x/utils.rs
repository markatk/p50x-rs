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

use clap::{Arg, SubCommand, App, ArgMatches, AppSettings};
use p50x::Device;

pub fn command_group<'a>(name: &str, description: &'a str, subcommands: Vec<App<'a, 'a>>) -> App<'a, 'a> {
    SubCommand::with_name(name)
        .about(description)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(subcommands)
}

pub fn common_command<'a>(name: &str, description: &'a str) -> App<'a, 'a> {
    SubCommand::with_name(name)
        .about(description)
        .args(&common_args())
}

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
            .default_value("1000"),
        Arg::with_name("quiet")
            .long("quiet")
            .short("q")
            .help("Prevent output to terminal")
    ]
}

pub fn run_command<F>(matches: &ArgMatches, callback: F) -> Result<(), String> where F: Fn(&mut Device) -> p50x::Result<()> {
    let mut device = get_device(matches)?;

    if let Err(err) = callback(&mut device) {
        return Err(err.to_string());
    }

    if matches.is_present("quiet") == false {
        println!("Ok");
    }

    return Ok(());
}

pub fn run_command_with_result<F, G, R>(
    matches: &ArgMatches,
    command_callback: F,
    result_callback: G
) -> Result<(), String> where F: Fn(&mut Device) -> p50x::Result<R>, G: Fn(R) -> Result<String, String> {
    let mut device = get_device(matches)?;

    let result = match command_callback(&mut device) {
        Ok(result) => result,
        Err(err) => return Err(err.to_string())
    };

    match result_callback(result) {
        Ok(output) => {
            if matches.is_present("quiet") == false {
                println!("{}", output);
            }

            Ok(())
        },
        Err(err) => Err(err.to_string())
    }
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
        Err(err) => Err(err.to_string())
    }
}

pub fn str_to_bool(value: &str) -> bool {
    match value.to_lowercase().as_str() {
        "true" | "t" | "1" | "one" | "yes" | "y"  => true,
        _ => false
    }
}
