/*
 * File: main.rs
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

#[macro_use]
extern crate clap;

use std::process::exit;
use clap::{App, ArgMatches, AppSettings};

mod utils;
mod power;
mod device;
mod so;
mod loco;
mod turnout;
mod interactive;

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("power", Some(m)) => power::run(m),
        ("device", Some(m)) => device::run(m),
        ("so", Some(m)) => so::run(m),
        ("loco", Some(m)) => loco::run(m),
        ("turnout", Some(m)) => turnout::run(m),
        ("interactive", Some(m)) => interactive::run(m),
        _ => Ok(())
    }
}

fn main() {
    let matches = App::new("p50x")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(crate_version!())
        .version_short("v")
        .about("P50X command-line utility")
        .subcommands(vec![
            power::command(),
            device::command(),
            so::command(),
            loco::command(),
            turnout::command(),
            interactive::command()
        ])
        .get_matches();

    if let Err(e) = run(matches) {
        eprintln!("{}", e);

        // TODO: Output p50x result code
        exit(1);
    }
}
