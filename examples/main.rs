extern crate serial_raw;
use serial_raw::tty;

use std::env;
use std::path::Path;
use std::io::{Read, Write};



fn main() {
    let args: Vec<String> = env::args().collect();

    let dev_name = &args[1];
    let baudrate = &args[2].parse::<i32>().unwrap();

    println!("open {} with baudrate {}", dev_name, baudrate);

    let mut port = tty::TTYPort::open(Path::new(dev_name), *baudrate).unwrap();

    println!("write data");
    port.write(b"abcdefg").unwrap();

    loop {
        let mut buf:[u8;128] = [0;128];
        println!("buf len is {}", buf.len());
        let ret = port.read(&mut buf).unwrap();

        if ret > 0 {
            println!("read over: {}", ret);
        }

        let ret = port.write(&buf).unwrap();
        if ret > 0 {
            let mut r = Vec::new();
            for i in 0..ret {
                r.push(buf[i]);
            }
            println!("{}", String::from_utf8(r).unwrap());
        }
}