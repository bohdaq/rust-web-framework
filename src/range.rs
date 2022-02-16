use std::io::prelude::*;
use std::net::TcpStream;
use std::{env, fs, io};
use std::borrow::Borrow;
use std::char::MAX;
use std::fs::{File, metadata};
use std::io::BufReader;

use crate::request::Request;
use crate::response::Response;
use crate::app::App;
use crate::{CONSTANTS, Server};
use crate::constant::{HTTP_VERSIONS, REQUEST_METHODS, RESPONSE_STATUS_CODE_REASON_PHRASES};
use crate::header::Header;


pub struct Range {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub struct ContentRange {
    pub(crate) unit: String,
    pub(crate) range: Range,
    pub(crate) size: String
}


impl Range {

    pub(crate) fn parse_range(filelength: usize, range_str: &str) -> Range {
        const START_INDEX: usize = 0;
        const END_INDEX: usize = 1;

        let mut range = Range { start: 0, end: filelength };
        let parts: Vec<&str> = range_str.split(CONSTANTS.HYPHEN).collect();
        for (i, part) in parts.iter().enumerate() {
            let num = part.trim();
            let length = num.len();
            if i == START_INDEX && length != 0 {
                range.start = num.parse().unwrap();
            }
            if i == END_INDEX && length != 0 {
                range.end = num.parse().unwrap();
            }
            if i == END_INDEX && length != 0 && range.start == 0 {
                let num_usize : usize = num.parse().unwrap();
                range.start = filelength - num_usize;
                range.end = filelength;
            }
        }
        range
    }

    pub(crate) fn parse_content_range(filelength: usize, raw_range_value: &str) {
        const INDEX_AFTER_UNIT_DECLARATION : usize = 1;
        let mut content_range_list: Vec<ContentRange> = vec![];

        println!("raw_range_value: {}", raw_range_value);
        let split_raw_range_value: Vec<&str> = raw_range_value.split(CONSTANTS.EQUALS).collect();
        let raw_bytes = split_raw_range_value.get(INDEX_AFTER_UNIT_DECLARATION).unwrap();
        println!("split_raw_range_value: {}", raw_bytes);

        let bytes: Vec<&str> = raw_bytes.split(CONSTANTS.COMMA).collect();
        for byte in bytes {
            let range = Range::parse_range(filelength, byte);
            let content_range = ContentRange {
                unit: CONSTANTS.BYTES.to_string(),
                range,
                size: filelength.to_string()
            };
            println!("unit: {} range: {} - {} size: {}", content_range.unit, content_range.range.start, content_range.range.end, content_range.size);
            content_range_list.push(content_range);
        }
    }

    pub(crate) fn get_exact_start_and_end_of_file(request_uri: &str, range: &Header) -> (usize, usize, usize) {
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut length: usize = 0;

        let mut contents = Vec::new();
        let static_filepath = Server::get_static_filepath(request_uri);
        let boxed_file = File::open(&static_filepath);
        if boxed_file.is_ok()  {
            let md = metadata(&static_filepath).unwrap();
            if md.is_file() {
                let mut file = boxed_file.unwrap();


                length = md.len() as usize;
                let raw_range_value = &range.header_value;
                let content_range = Range::parse_content_range(length, raw_range_value);




                file.read_to_end(&mut contents).expect("Unable to read");
            }
        }



        return (start, end, length)
    }
}


