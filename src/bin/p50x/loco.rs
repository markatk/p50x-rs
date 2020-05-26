/*
 * File: loco.rs
 * Date: 19.05.2020
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

use clap::{ArgMatches, App, Arg};
use p50x::{P50XBinary, Error, XLokOptions};

use crate::utils::{command_group, common_command, run_command, str_to_bool};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("set", Some(m)) => run_command(
            m,
            |device| {
                let address = match m.value_of("address").unwrap().parse::<u16>() {
                    Ok(so) => so,
                    Err(_) => return Err(Error::Other) // TODO: Proper handle parse error
                };

                let functions = if m.is_present("functions") {
                    let function_values: Vec<_> = m.values_of("functions").unwrap().collect();
                    let mut values: [bool; 4] = [false, false, false, false];

                    for i in 0..4 {
                        values[i] = str_to_bool(function_values[i]);
                    }

                    Some(values)
                } else {
                    None
                };

                let options = XLokOptions {
                    light: m.is_present("light"),
                    emergency_stop: m.is_present("emergency-stop"),
                    force: m.is_present("force"),
                    functions
                };

                match m.value_of("speed").unwrap().parse::<i8>() {
                    Ok(speed) => device.xlok(address, speed, options),
                    Err(_) => Err(Error::Other) // TODO: Proper handle parse error
                }
            })?,
        _ => ()
    };

    return Ok(());
}

pub fn command<'a>() -> App<'a, 'a> {
    command_group(
        "loco",
        "Control locomotives and get information about them",
        vec![
            common_command("set", "Set locomotive speed")
                .arg(Arg::with_name("address")
                    .help("Locomotive address")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("speed")
                    .help("Locomotive speed value")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("light")
                    .help("Enable lights")
                    .long("light")
                    .short("l"))
                .arg(Arg::with_name("emergency-stop")
                    .help("Send emergency stop signal")
                    .long("emergency-stop")
                    .short("e"))
                .arg(Arg::with_name("force")
                    .help("Force the command even if the loco is controlled elsewhere")
                    .long("force")
                    .short("f"))
                .arg(Arg::with_name("functions")
                    .help("Functions 1-4 to set")
                    .long("functions")
                    .short("F")
                    .takes_value(true)
                    .min_values(4)
                    .max_values(4))
        ]
    )
}
