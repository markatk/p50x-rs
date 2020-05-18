/*
 * File: power.rs
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

use clap::{ArgMatches, SubCommand, App, AppSettings};
use p50x::P50XBinary;

use crate::utils::{common_command, run_command, run_command_with_result};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("on", Some(m)) => {
            run_command_with_result(m, |device| device.xpower_on(), |result| {
                match result {
                    true => Ok("Ok".to_string()),
                    false => Err("Power could not be turned on".to_string())
                }
            })?;
        },
        ("off", Some(m)) => run_command(m, |device| device.xpower_off())?,
        ("halt", Some(m)) => run_command(m, |device| device.xhalt())?,
        _ => ()
    };

    return Ok(());
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("power")
        .about("Control power and related states")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(vec![
            common_command("on", "Set power on"),
            common_command("off", "Set power off"),
            common_command("halt", "Set halt mode")
        ])
}
