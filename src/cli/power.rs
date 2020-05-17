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

use super::utils::{common_args, get_device};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let mut device = get_device(matches)?;

    match matches.subcommand() {
        ("on", _) => {
            device.xpower_on().unwrap();

            ()
        },
        ("off", _) => device.xpower_off().unwrap(),
        ("halt", _) => device.xhalt().unwrap(),
        _ => ()
    };

    return Ok(());
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("power")
        .about("Control power and related states")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .args(&common_args())
        .subcommand(SubCommand::with_name("on")
            .about("Set power on"))
        .subcommand(SubCommand::with_name("off")
            .about("Set power off"))
        .subcommand(SubCommand::with_name("halt")
            .about("Set halt mode"))
}
