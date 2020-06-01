/*
 * File: turnout.rs
 * Date: 01.06.2020
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
use p50x::{P50XBinary, Error, XTurnoutOptions};

use crate::utils::{command_group, common_command, run_command, run_command_with_result, str_to_bool};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("set", Some(m)) => run_command(
            m,
            |device| {
                let state = str_to_bool(m.value_of("state").unwrap());
                let options = XTurnoutOptions {
                    status: m.is_present("status-off"),
                    reserve: m.is_present("reserved"),
                    no_command: m.is_present("no-command")
                };

                match m.value_of("address").unwrap().parse::<u16>() {
                    Ok(address) => device.xturnout(address, state, options),
                    Err(_) => return Err(Error::Other) // TODO: Proper handle parse error
                }
            }
        )?,
        ("free", Some(m)) => run_command(m, |device| device.xturnout_free())?,
        ("status", Some(m)) => run_command_with_result(
            m,
            |device| {
                match m.value_of("address").unwrap().parse::<u16>() {
                    Ok(address) => device.xturnout_status(address),
                    Err(_) => return Err(Error::Other) // TODO: Proper handle parse error
                }
            },
            |result| Ok(result.to_string())
        )?,
        ("group", Some(m)) => run_command_with_result(
            m,
            |device| {
                match m.value_of("group-address").unwrap().parse::<u8>() {
                    Ok(address) => device.xturnout_group(address),
                    Err(_) => return Err(Error::Other) // TODO: Proper handle parse error
                }
            },
            |data| {
                let mut result = "State Reserved\n".to_string();

                for i in 0..8 {
                    if data[i].0 {
                        result += "On    ";
                    } else {
                        result += "Off   ";
                    }

                    if data[i].1 {
                        result += "On\n";
                    } else {
                        result += "Off\n";
                    }
                }

                return Ok(result);
            }
        )?,
        _ => ()
    };

    return Ok(());
}

pub fn command<'a>() -> App<'a, 'a> {
    command_group(
        "turnout",
        "Control turnouts and get information about them",
        vec![
            common_command("set", "Set tunrout state and other properties")
                .arg(Arg::with_name("address")
                    .help("Turnout address")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("state")
                    .help("Turnout state")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("status-off")
                    .help("Coil status off")
                    .long("status-off")
                    .short("s"))
                .arg(Arg::with_name("reserved")
                    .help("Reserve exclusive control")
                    .long("reserved")
                    .short("r"))
                .arg(Arg::with_name("no-command")
                    .help("Do not send the actual track command, just update the internal state")
                    .long("no-command")
                    .short("n")),
            common_command("free", "Free all reserved turnouts"),
            common_command("status", "Get the status of a turnout")
                .arg(Arg::with_name("address")
                    .help("Turnout address")
                    .required(true)
                    .takes_value(true)),
            common_command("group", "Get the state and reserved state of a turnout group")
                .arg(Arg::with_name("group-address")
                    .help("Turnout group address")
                    .required(true)
                    .takes_value(true)),
        ]
    )
}
