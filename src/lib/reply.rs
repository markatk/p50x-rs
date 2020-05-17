/*
 * File: reply.ts
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

use std::convert::From;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum P50XReply {
    Ok = 0x00,
    BadCommand = 0x01,
    BadParameter = 0x02,
    PowerOff = 0x06,
    FullTurnoutCommandStack = 0x09,
    NoData = 0x0A,
    NoSlot = 0x0B,
    BadLokParameter = 0x0C,
    BadTurnoutParameter = 0x0E,
    BadSpecialOptionValue = 0x0F,
    LowTurnoutCommandStackSpace = 0x40,
    LokHalt = 0x41,
    LokPowerOff = 0x42,
    Busy = 0x80,
    Unknown
}

impl From<u8> for P50XReply {
    fn from(value: u8) -> P50XReply {
        match value {
            0x00 => P50XReply::Ok,
            0x01 => P50XReply::BadCommand,
            0x02 => P50XReply::BadParameter,
            0x06 => P50XReply::PowerOff,
            0x09 => P50XReply::FullTurnoutCommandStack,
            0x0A => P50XReply::NoData,
            0x0B => P50XReply::NoSlot,
            0x0C => P50XReply::BadLokParameter,
            0x0E => P50XReply::BadTurnoutParameter,
            0x0F => P50XReply::BadSpecialOptionValue,
            0x40 => P50XReply::LowTurnoutCommandStackSpace,
            0x41 => P50XReply::LokHalt,
            0x42 => P50XReply::LokPowerOff,
            0x80 => P50XReply::Busy,
            _ => P50XReply::Unknown
        }
    }
}
