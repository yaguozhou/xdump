/*
 * Copyright (c) 2020 Yaguo Zhou
 * xdump is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *          http://license.coscl.org.cn/MulanPSL2
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

const SUBSTR_LEN_HEX: usize = 16;
const SUBSTR_LEN_OCT: usize = 14;
const SUBSTR_LEN_BINARY: usize = 7;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("display_type")
                .short("t")
                .long("type")
                .help("a|h|d|o|b, display file contents in all|hexadecimal|decimal|octal|binary")
                .default_value("a")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("full_path_of_file")
                .required(true)
                .help("full path of file"),
        )
        .get_matches();

    let full_path_of_file = matches.value_of("full_path_of_file").unwrap();
    let display_type = matches.value_of("display_type").unwrap();
    let path = Path::new(full_path_of_file);
    if !path.is_file() {
        eprint!("{} is NOT a file", full_path_of_file);
        exit(1);
    }
    let mut s = String::new();

    File::open(&path)
        .expect("open error")
        .read_to_string(&mut s)
        .expect("read failed");

    match display_type {
        "a" => dump_all(&s),
        "h" => dump(&s, Type::HEX),
        "d" => dump(&s, Type::DEC),
        "o" => dump(&s, Type::OCT),
        "b" => dump(&s, Type::BIN),
        _ => eprintln!("display_type should be one of a|h|d|o|b"),
    }
}

fn dump_all(s: &str) {
    dump(s, Type::HEX);
    dump(s, Type::DEC);
    dump(s, Type::OCT);
    dump(s, Type::BIN);
}

fn dump(s: &str, t: Type) {
    println!("{}\n{}", t.value(), "-".repeat(t.value().len()));
    let len = match t {
        Type::BIN => SUBSTR_LEN_BINARY,
        Type::HEX => SUBSTR_LEN_HEX,
        _ => SUBSTR_LEN_OCT,
    };
    split(s, len).iter().for_each(|x| {
        dump_line(x, &t, len);
    });
    println!();
}

fn split(s: &str, size: usize) -> Vec<String> {
    let mut result = Vec::new();

    let len = s.len();

    if len <= size {
        result.push(s.to_owned());
    } else {
        let parts = len / size;
        for i in 0..parts {
            result.push(s[i * size..(i + 1) * size].to_owned());
        }
        result.push(s[parts * size..].to_owned());
    }

    result
}

enum Type {
    OCT,
    DEC,
    HEX,
    BIN,
}

impl Type {
    fn value(&self) -> &str {
        match self {
            Type::HEX => "hex",
            Type::DEC => "dec",
            Type::OCT => "oct",
            Type::BIN => "bin",
        }
    }
}

fn dump_line(s: &str, t: &Type, len: usize) {
    let bytes = s.as_bytes();
    match t {
        Type::HEX => {
            bytes.iter().for_each(|x| {
                print!("{:>02x} ", x);
            });
            print!("{}", " ".repeat((2 + 1) * (len - s.len())));
        }
        Type::DEC => {
            bytes.iter().for_each(|x| {
                print!("{:>3} ", x);
            });
            print!("{}", " ".repeat((3 + 1) * (len - s.len())));
        }
        Type::OCT => {
            bytes.iter().for_each(|x| {
                print!("{:>03o} ", x,);
            });
            print!("{}", " ".repeat((3 + 1) * (len - s.len())));
        }
        Type::BIN => {
            bytes.iter().for_each(|x| {
                print!("{:>08b} ", x);
            });
            print!("{}", " ".repeat((8 + 1) * (len - s.len())));
        }
    }
    println!("  |{}|", s.replace("\n", "."));
}
