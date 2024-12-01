use std::io::{self, Write, Read};
use std::net::TcpStream;

use dlt_format_parser::{ DltParse, dlt_analyze };

fn main() {
    println!("Hello, world!");

    let mut stream = TcpStream::connect("127.0.0.1:3490").unwrap();

    //stream.write_all(b"Hello, server!").unwrap();
    
    while true {
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).unwrap();
        let (header, header_extra, remaining_data) = buffer.dlt_parse();
        println!("{:?}", header);
        println!("{:?}", header_extra);
        println!("{:?}", header.get_htyp());
        println!("{}", header.get_version());
        // if ( header.get_htyp() == UEH_MASK)
        // {
        //     println!("{}", true);
        // }
        println!("ret val: {}", dlt_analyze(&header));
        header_extra.debug_print();
        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    }
    
}
