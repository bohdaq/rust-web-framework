use std::{env, fs};
use regex::Regex;


#[cfg(test)]
mod tests {
    use crate::CONSTANTS;
    use crate::constant::{HTTP_VERSIONS, REQUEST_METHODS, RESPONSE_STATUS_CODE_REASON_PHRASES};
    use crate::header::Header;
    use crate::mime_type::MimeType;
    use crate::request::Request;
    use crate::response::Response;
    use crate::server::Server;
    use super::*;

    #[test]
    fn detect_mime_type_for_mp4_file() {
        let expected_mime_type = MimeType::VIDEO_MP4;
        let request_uri = "/drahobrat_pt2/drahobrat_pt2_ver2.mp4";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_binary_file() {
        let expected_mime_type = MimeType::APPLICATION_OCTET_STREAM;
        let request_uri = "/rust-web-server/0.0.2/x86_64-unknown-linux-gnu/rws";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_text_file() {
        let expected_mime_type = MimeType::TEXT_PLAIN;
        let request_uri = "/dir/test.txt";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_css_file() {
        let expected_mime_type = MimeType::TEXT_CSS;
        let request_uri = "/dir/test.css";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_html_file() {
        let expected_mime_type = MimeType::TEXT_HTML;
        let request_uri = "/dir/test.html";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_js_file() {
        let expected_mime_type = MimeType::TEXT_JAVASCRIPT;
        let request_uri = "/dir/test.js";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_apng_file() {
        let expected_mime_type = MimeType::IMAGE_APNG;
        let request_uri = "/dir/test.apng";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_avif_file() {
        let expected_mime_type = MimeType::IMAGE_AVIF;
        let request_uri = "/dir/test.avif";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_gif_file() {
        let expected_mime_type = MimeType::IMAGE_GIF;
        let request_uri = "/dir/test.gif";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_jpg_file() {
        let expected_mime_type = MimeType::IMAGE_JPEG;
        let request_uri = "/dir/test.jpg";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_jpeg_file() {
        let expected_mime_type = MimeType::IMAGE_JPEG;
        let request_uri = "/dir/test.jpeg";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_jpe_file() {
        let expected_mime_type = MimeType::IMAGE_JPEG;
        let request_uri = "/dir/test.jpe";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_jif_file() {
        let expected_mime_type = MimeType::IMAGE_JPEG;
        let request_uri = "/dir/test.jif";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_jfif_file() {
        let expected_mime_type = MimeType::IMAGE_JPEG;
        let request_uri = "/dir/test.jfif";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_png_file() {
        let expected_mime_type = MimeType::IMAGE_PNG;
        let request_uri = "/dir/test.png";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_svg_file() {
        let expected_mime_type = MimeType::IMAGE_PNG;
        let request_uri = "/dir/test.png";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_webp_file() {
        let expected_mime_type = MimeType::IMAGE_WEBP;
        let request_uri = "/dir/test.webp";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_bmp_file() {
        let expected_mime_type = MimeType::IMAGE_BMP;
        let request_uri = "/dir/test.bmp";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_ico_file() {
        let expected_mime_type = MimeType::IMAGE_ICO;
        let request_uri = "/dir/test.ico";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_cur_file() {
        let expected_mime_type = MimeType::IMAGE_ICO;
        let request_uri = "/dir/test.cur";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_tif_file() {
        let expected_mime_type = MimeType::IMAGE_TIFF;
        let request_uri = "/dir/test.tif";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_image_tiff_file() {
        let expected_mime_type = MimeType::IMAGE_TIFF;
        let request_uri = "/dir/test.tiff";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_audio_aac_file() {
        let expected_mime_type = MimeType::AUDIO_AAC;
        let request_uri = "/dir/test.aac";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_audio_flac_file() {
        let expected_mime_type = MimeType::AUDIO_FLAC;
        let request_uri = "/dir/test.flac";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_audio_wav_file() {
        let expected_mime_type = MimeType::AUDIO_WAV;
        let request_uri = "/dir/test.wav";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_3gp_file() {
        let expected_mime_type = MimeType::VIDEO_3GP;
        let request_uri = "/dir/test.3gp";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_mpg_file() {
        let expected_mime_type = MimeType::VIDEO_MPEG;
        let request_uri = "/dir/test.mpg";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_mpeg_file() {
        let expected_mime_type = MimeType::VIDEO_MPEG;
        let request_uri = "/dir/test.mpeg";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_audio_oga_file() {
        let expected_mime_type = MimeType::AUDIO_OGG;
        let request_uri = "/dir/test.oga";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_mp4_file() {
        let expected_mime_type = MimeType::VIDEO_MP4;
        let request_uri = "/dir/test.mp4";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_m4v_file() {
        let expected_mime_type = MimeType::VIDEO_MP4;
        let request_uri = "/dir/test.m4v";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_ogg_file() {
        let expected_mime_type = MimeType::VIDEO_OGG;
        let request_uri = "/dir/test.ogg";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_ogv_file() {
        let expected_mime_type = MimeType::VIDEO_OGG;
        let request_uri = "/dir/test.ogv";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_m4p_file() {
        let expected_mime_type = MimeType::VIDEO_MP4;
        let request_uri = "/dir/test.m4p";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_quicktime_file() {
        let expected_mime_type = MimeType::VIDEO_QUICKTIME;
        let request_uri = "/dir/test.mov";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_video_webm_file() {
        let expected_mime_type = MimeType::VIDEO_WEBM;
        let request_uri = "/dir/test.webm";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn detect_mime_type_for_abiword_file() {
        let expected_mime_type = MimeType::APPLICATION_ABIWORD;
        let request_uri = "/dir/test.abw";

        let actual_mime_type = MimeType::detect_mime_type(request_uri);

        assert_eq!(expected_mime_type, actual_mime_type);
    }

    #[test]
    fn method_and_request_uri_and_http_version_regex() {
        let re = Regex::new(Request::METHOD_AND_REQUEST_URI_AND_HTTP_VERSION_REGEX).unwrap();
        let caps = re.captures("GET / HTTP/1.1").unwrap();

        assert_eq!(HTTP_VERSIONS.HTTP_VERSION_1_1, &caps["http_version"]);
        assert_eq!(REQUEST_METHODS.GET, &caps["method"]);
        assert_eq!(CONSTANTS.SLASH, &caps["request_uri"]);


        let re = Regex::new(Request::METHOD_AND_REQUEST_URI_AND_HTTP_VERSION_REGEX).unwrap();
        let caps = re.captures("GET /drahobrat_pt2/drahobrat_pt2_ver2.mp4 HTTP/1.1").unwrap();

        assert_eq!(HTTP_VERSIONS.HTTP_VERSION_1_1, &caps["http_version"]);
        assert_eq!(REQUEST_METHODS.GET, &caps["method"]);
        assert_eq!("/drahobrat_pt2/drahobrat_pt2_ver2.mp4", &caps["request_uri"]);

    }

    #[test]
    fn http_version_and_status_code_and_reason_phrase_regex() {
        let re = Regex::new(Response::HTTP_VERSION_AND_STATUS_CODE_AND_REASON_PHRASE_REGEX).unwrap();
        let caps = re.captures("HTTP/1.1 404 NOT FOUND").unwrap();

        assert_eq!(HTTP_VERSIONS.HTTP_VERSION_1_1, &caps["http_version"]);
        assert_eq!(RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.STATUS_CODE, &caps["status_code"]);
        assert_eq!(RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.REASON_PHRASE, &caps["reason_phrase"]);


        let re = Regex::new(Response::HTTP_VERSION_AND_STATUS_CODE_AND_REASON_PHRASE_REGEX).unwrap();
        let caps = re.captures("HTTP/1.1 200 OK").unwrap();

        assert_eq!(HTTP_VERSIONS.HTTP_VERSION_1_1, &caps["http_version"]);
        assert_eq!(RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE, &caps["status_code"]);
        assert_eq!(RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE, &caps["reason_phrase"]);

    }

    #[test]
    fn it_generates_successful_response_with_index_html() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = CONSTANTS.SLASH;
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE;
        let response_filepath = "index.html";
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_successful_response_with_static_file() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/static/test.txt";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();

        let response_filepath = [working_directory, request.request_uri.as_str()].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_not_found_page_for_absent_static_file() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/static/nonexistingfile";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1;
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();
        let not_found_page_path = "404.html";

        let response_filepath = [working_directory, CONSTANTS.SLASH, not_found_page_path].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port: usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_not_found_page_for_absent_route() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/nonexistingroute";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();
        let not_found_page_path = "404.html";

        let response_filepath = [working_directory, CONSTANTS.SLASH, not_found_page_path].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_not_found_page_for_static_directory() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/static/";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();
        let not_found_page_path = "404.html";

        let response_filepath = [working_directory, CONSTANTS.SLASH, not_found_page_path].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_not_found_page_for_static_subdirectory() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/static/subdir/";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N404_NOT_FOUND.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();
        let not_found_page_path = "404.html";

        let response_filepath = [working_directory, CONSTANTS.SLASH, not_found_page_path].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_successful_response_with_static_file_in_subdirectory() {
        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/static/test.txt";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();

        let response_filepath = [working_directory, request.request_uri.as_str()].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string()];


        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_successful_response_with_static_file_in_multiple_static_directories() {

        // 1st reading file from /static folder

        // request test data
        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/static/test.txt";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();

        let response_filepath = [working_directory, request.request_uri.as_str()].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string(), "/assets".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);






        // 2nd file read from /assets directory

        let request_host_header_name = "Host";
        let request_host_header_value = "localhost:7777";
        let request_method = REQUEST_METHODS.GET;
        let request_uri = "/assets/test.txt";
        let request_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();


        // request part
        let host = Header {
            header_name: request_host_header_name.to_string(),
            header_value: request_host_header_value.to_string()
        };

        let headers = vec![host];
        let request = Request {
            method: request_method.to_string(),
            request_uri: request_uri.to_string(),
            http_version: request_http_version.to_string(),
            headers
        };

        let raw_request = Request::generate_request(request);

        let request: Request = Request::parse_request(&raw_request);
        let host_header = request.get_header(request_host_header_name.to_string()).unwrap();

        assert_eq!(request_host_header_value.to_string(), host_header.header_value);
        assert_eq!(request_method.to_string(), request.method);
        assert_eq!(request_uri.to_string(), request.request_uri);
        assert_eq!(request_http_version.to_string(), request.http_version);

        // response part
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE;
        let response_filepath = &request.request_uri;

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();

        let response_filepath = [working_directory, request.request_uri.as_str()].join(CONSTANTS.EMPTY_STRING);
        let response_html_file= fs::read_to_string(response_filepath.to_string()).unwrap();
        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = response_html_file.len().to_string();

        let ip_addr= "127.0.0.1".to_string();
        let port : usize = "8787".parse().unwrap();
        let static_directories = vec!["/static".to_string(), "/assets".to_string()];

        let raw_response: String = Server::process_request(raw_request);
        let response = Response::parse_response(raw_response);
        let header = response.get_header(response_content_length_header_name.to_string()).unwrap();

        assert_eq!(response_content_length_header_value, header.header_value);
        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(response_html_file, response.message_body);
    }

    #[test]
    fn it_generates_successful_response_with_additional_headers() {
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = "401";
        let response_reason_phrase = "Unauthorized";
        let message_body = CONSTANTS.EMPTY_STRING;

        let response_user_agent_header_name = "User-Agent";
        let response_user_agent_value = "rws/0.0.1";

        let user_agent = Header {
            header_name: response_user_agent_header_name.to_string(),
            header_value: response_user_agent_value.to_string()
        };


        let headers = vec![user_agent];
        let response = Response {
            http_version: response_http_version.to_string(),
            status_code: response_status_code.to_string(),
            reason_phrase: response_reason_phrase.to_string(),
            headers,
            message_body: message_body.to_string()
        };


        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = message_body.len().to_string();


        let raw_response = Response::generate_response(response);
        let response = Response::parse_response(raw_response);


        let content_length_header = response.get_header(response_content_length_header_name.to_string()).unwrap();
        assert_eq!(response_content_length_header_value, content_length_header.header_value);

        let response_user_agent_header = response.get_header(response_user_agent_header_name.to_string()).unwrap();
        assert_eq!(response_user_agent_header.header_value, response_user_agent_value);


        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(message_body, response.message_body);


    }

    #[test]
    fn it_generates_successful_response_with_additional_headers_and_file() {
        let response_http_version = HTTP_VERSIONS.HTTP_VERSION_1_1.to_string();
        let response_status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.STATUS_CODE;
        let response_reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.N200_OK.REASON_PHRASE;
        let filepath = "/static/test.txt";

        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();

        let response_filepath = [working_directory, filepath].join(CONSTANTS.EMPTY_STRING);
        let message_body= fs::read_to_string(response_filepath.to_string()).unwrap();

        let response_user_agent_header_name = "User-Agent";
        let response_user_agent_value = "rws/0.0.1";

        let user_agent = Header {
            header_name: response_user_agent_header_name.to_string(),
            header_value: response_user_agent_value.to_string()
        };


        let headers = vec![user_agent];
        let response = Response {
            http_version: response_http_version.to_string(),
            status_code: response_status_code.to_string(),
            reason_phrase: response_reason_phrase.to_string(),
            headers,
            message_body: message_body.to_string()
        };


        let response_content_length_header_name = "Content-Length";
        let response_content_length_header_value = message_body.len().to_string();


        let raw_response = Response::generate_response(response);
        let response = Response::parse_response(raw_response);


        let content_length_header = response.get_header(response_content_length_header_name.to_string()).unwrap();
        assert_eq!(response_content_length_header_value, content_length_header.header_value);

        let response_user_agent_header = response.get_header(response_user_agent_header_name.to_string()).unwrap();
        assert_eq!(response_user_agent_header.header_value, response_user_agent_value);


        assert_eq!(response_http_version, response.http_version);
        assert_eq!(response_status_code, response.status_code);
        assert_eq!(response_reason_phrase, response.reason_phrase);
        assert_eq!(message_body, response.message_body);
    }

}
