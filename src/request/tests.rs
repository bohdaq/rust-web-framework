use regex::Regex;
use crate::constant::{HTTP_VERSIONS, REQUEST_METHODS};
use crate::{CONSTANTS, Request};

#[test]
fn method_and_request_uri_and_http_version_regex() {
    let re = Regex::new(Request::METHOD_AND_REQUEST_URI_AND_HTTP_VERSION_REGEX).unwrap();
    let caps = re.captures("GET / HTTP/1.1").unwrap();

    assert_eq!(HTTP_VERSIONS.http_version_1_1, &caps["http_version"]);
    assert_eq!(REQUEST_METHODS.get, &caps["method"]);
    assert_eq!(CONSTANTS.slash, &caps["request_uri"]);


    let re = Regex::new(Request::METHOD_AND_REQUEST_URI_AND_HTTP_VERSION_REGEX).unwrap();
    let caps = re.captures("GET /draho-brat_pt2/drahobrat_pt2_ver2.mp4 HTTP/1.1").unwrap();

    assert_eq!(HTTP_VERSIONS.http_version_1_1, &caps["http_version"]);
    assert_eq!(REQUEST_METHODS.get, &caps["method"]);
    assert_eq!("/draho-brat_pt2/drahobrat_pt2_ver2.mp4", &caps["request_uri"]);

}

#[test]
fn test_request_ok() {
    let method = REQUEST_METHODS.get;
    let request_uri = CONSTANTS.slash;
    let http_version = HTTP_VERSIONS.http_version_1_1;

    let request_data = [method, request_uri, http_version].join(" ");
    let raw_request = [request_data, CONSTANTS.new_line_separator.to_string()].join("");

    let request = Request::parse_request(raw_request.as_bytes()).unwrap();

    assert_eq!(method, request.method);
    assert_eq!(request_uri, request.request_uri);
    assert_eq!(http_version, request.http_version);
}

#[test]
fn test_request_ok_with_special_characters() {
    let method = REQUEST_METHODS.get;
    let special_characters = "_:;.,/\"'?!(){}[]@<>=-+*#$&`|~^%";
    let request_uri = [CONSTANTS.slash, special_characters].join("");
    let http_version = HTTP_VERSIONS.http_version_1_1;


    let request_data = [method, request_uri.as_str(), http_version].join(" ");
    let raw_request = [request_data, CONSTANTS.new_line_separator.to_string()].join("");

    let request = Request::parse_request(raw_request.as_bytes()).unwrap();

    assert_eq!(method, request.method);
    assert_eq!(request_uri, request.request_uri);
    assert_eq!(http_version, request.http_version);
}

#[test]
fn test_request_ok_with_ukrainian_characters() {
    let method = REQUEST_METHODS.get;
    let ukrainian_characters = "АаБбВвГгҐґДдЕеЄєЖжЗзИиІіЇїЙйКкЛлМмНнОоПпРрСсТтУуФфХхЦцЧчШшЩщЬьЮюЯя";
    let request_uri = [CONSTANTS.slash, ukrainian_characters].join("");
    let http_version = HTTP_VERSIONS.http_version_1_1;


    let request_data = [method, request_uri.as_str(), http_version].join(" ");
    let raw_request = [request_data, CONSTANTS.new_line_separator.to_string()].join("");

    let request = Request::parse_request(raw_request.as_bytes()).unwrap();

    assert_eq!(method, request.method);
    assert_eq!(request_uri, request.request_uri);
    assert_eq!(http_version, request.http_version);
}

#[test]
fn test_request_not_ok() {
    let method = REQUEST_METHODS.get;
    let request_uri = [CONSTANTS.slash, CONSTANTS.whitespace, CONSTANTS.hyphen].join("");
    let http_version = HTTP_VERSIONS.http_version_1_1;

    let request_data = [method, request_uri.as_str(), http_version].join(" ");
    let raw_request = [request_data, CONSTANTS.new_line_separator.to_string()].join("");

    let boxed_request = Request::parse_request(raw_request.as_bytes());
    assert_eq!(true, boxed_request.is_err());

    let error_message = format!("Unable to parse method, request uri and http version: {}", raw_request);
    assert_eq!(error_message, boxed_request.err().unwrap());
}

#[test]
fn test_request_not_ok_empty_request() {
    let boxed_request = Request::parse_request(b"");
    assert_eq!(true, boxed_request.is_err());

    let error_message = format!("Unable to parse method, request uri and http version: ");
    assert_eq!(error_message, boxed_request.err().unwrap());
}

#[test]
fn test_request_not_ok_dummy_not_valid_request() {
    let dummy_request = "some dummy not valid request";
    let boxed_request = Request::parse_request(dummy_request.as_bytes());
    assert_eq!(true, boxed_request.is_err());

    let error_message = format!("Unable to parse method, request uri and http version: {}", dummy_request);
    assert_eq!(error_message, boxed_request.err().unwrap());
}