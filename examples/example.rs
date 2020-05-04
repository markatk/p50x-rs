/*
 * File: example.rs
 * Date: 04.05.2020
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

use p50x::*;

fn main() {
    let port_name = "/dev/ttyACM1";
    let baud_rate = 9600;

    println!("Connecting to {} with {} baud...", port_name, baud_rate);

    let mut device = Device::new(port_name, baud_rate).expect("Device could not be created");

    println!("P50X device detected");

    let version = device.xversion().unwrap();
    println!("Version: {:?}", version);

    let status = device.xstatus().unwrap();
    println!("Device status {:?}", status);

    device.xpower_on().unwrap();

    let status = device.xstatus().unwrap();
    println!("Device status {:?}", status);

    device.xpower_off().unwrap();

    let status = device.xstatus().unwrap();
    println!("Device status {:?}", status);
}
