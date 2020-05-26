/*
 * File: so.rs
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

use clap::{ArgMatches, App, Arg};
use p50x::{P50XBinary, Error};

use crate::utils::{command_group, common_command, run_command, run_command_with_result};

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("get", Some(m)) => run_command_with_result(
            m,
            |device| {
                match m.value_of("special_option").unwrap().parse::<u16>() {
                    Ok(so) => device.xso_get(so),
                    Err(_) => Err(Error::Other) // TODO: Proper handle parse error
                }
            },
            |result| Ok(result.to_string()))?,
        ("set", Some(m)) => run_command(
            m,
            |device| {
                let so = match m.value_of("special_option").unwrap().parse::<u16>() {
                    Ok(so) => so,
                    Err(_) => return Err(Error::Other) // TODO: Proper handle parse error
                };

                match m.value_of("value").unwrap().parse::<u8>() {
                    Ok(value) => device.xso_set(so, value),
                    Err(_) => Err(Error::Other) // TODO: Proper handle parse error
                }
            })?,
        _ => ()
    };

    return Ok(());
}

pub fn command<'a>() -> App<'a, 'a> {
    command_group(
        "so",
        "Get or set special options",
        vec![
            common_command("get", "Get a special option")
                .arg(Arg::with_name("special_option")
                    .help("Special option number")
                    .required(true)
                    .takes_value(true)),
            common_command("set", "Set a special option")
                .arg(Arg::with_name("special_option")
                    .help("Special option number")
                    .required(true)
                    .takes_value(true))
                .arg(Arg::with_name("value")
                    .help("Special option value")
                    .required(true)
                    .takes_value(true))
        ]
    )
}
