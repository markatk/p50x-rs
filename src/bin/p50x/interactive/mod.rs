/*
 * File: mod.rs
 * Date: 16.01.2022
 * Author: MarkAtk
 *
 * MIT License
 *
 * Copyright (c) 2022 MarkAtk
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

use clap::{ArgMatches, App, SubCommand, Arg};
use std::io::Write;
use std::vec::Vec;

use crate::utils::run_command;

mod commands;

use commands::*;

pub fn run(matches: &ArgMatches) -> Result<(), String> {
    let commands = register_commands();

    return run_command(matches, |device| {
        // interactive loop
        loop {
            let input = prompt("> ");

            // handle special commands
            if input == "exit" {
                break;
            }

            if input == "help" {
                let mut keys: Vec<&String> = commands.keys().collect();
                keys.sort();

                for key in keys {
                    println!("{}", key);
                }

                continue;
            }

            // handle registered commands
            if let Some(callback) = commands.get(input.as_str()) {
                match callback(&input, device) {
                    Ok(_) => println!("Ok"),
                    Err(err) => return Err(err)
                };
            } else {
                println!("Unknown command: {}", input);
            }
        }

        return Ok(());
    });
}

pub fn command<'a>() -> App<'a, 'a> {
    SubCommand::with_name("interactive")
        .about("Interactive mode to send multiple commands to device")
        .args(&vec![
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
        ])
}

fn prompt(name: &str) -> String {
    let mut line = String::new();

    print!("{}", name);

    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");

    return line.trim().to_string()
}


