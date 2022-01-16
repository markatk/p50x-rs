/*
 * File: commands.rs
 * Author: Markus Grigull
 * Date: 16.01.22
 *
 * Copyright (c) 2022 Markus Grigull
 */

use std::collections::HashMap;
use p50x::{Device, P50XBinary};

pub fn register_commands() -> HashMap<String, fn(&str, &mut Device) -> Result<(), p50x::Error>> {
    let mut commands: HashMap<String, fn(&str, &mut Device) -> Result<(), p50x::Error>> = HashMap::new();

    commands.insert("power on".into(), |_, device| device.xpower_on());
    commands.insert("power off".into(), |_, device| device.xpower_off());
    commands.insert("halt".into(), |_, device| device.xhalt());

    return commands;
}
