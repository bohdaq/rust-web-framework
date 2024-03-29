#[cfg(test)]
mod tests;

pub mod controller;

use crate::app::controller::favicon::FaviconController;
use crate::app::controller::file::initiate::FileUploadInitiateController;
use crate::app::controller::form::get_method::FormGetMethodController;
use crate::app::controller::form::multipart_enctype_post_method::FormMultipartEnctypePostMethodController;
use crate::app::controller::form::url_encoded_enctype_post_method::FormUrlEncodedEnctypePostMethodController;
use crate::app::controller::index::IndexController;
use crate::app::controller::not_found::NotFoundController;
use crate::app::controller::script::ScriptController;
use crate::app::controller::static_resource::StaticResourceController;
use crate::app::controller::style::StyleController;
use crate::application::Application;
use crate::controller::Controller;
use crate::core::New;
use crate::header::Header;

use crate::request::{Request};
use crate::response::{Response, STATUS_CODE_REASON_PHRASE};
use crate::server::ConnectionInfo;

#[derive(Copy, Clone)]
pub struct App {}

impl New for App {
    fn new() -> Self {
        App{}
    }
}

impl Application for App {
    fn execute(&self, request: &Request, connection: &ConnectionInfo) -> Result<Response, String> {
        let header_list = Header::get_header_list(&request);

        let mut response: Response = Response::get_response(
            STATUS_CODE_REASON_PHRASE.n501_not_implemented,
            Some(header_list),
            None
        );



        if IndexController::is_matching(&request, connection) {
            response = IndexController::process(&request, response, connection);
            return Ok(response)
        }

        if StyleController::is_matching(&request, connection) {
            response = StyleController::process(&request, response, connection);
            return Ok(response)
        }

        if ScriptController::is_matching(&request, connection) {
            response = ScriptController::process(&request, response, connection);
            return Ok(response)
        }

        if FileUploadInitiateController::is_matching(&request, connection) {
            response = FileUploadInitiateController::process(&request, response, connection);
            return Ok(response)
        }

        if FormUrlEncodedEnctypePostMethodController::is_matching(&request, connection) {
            response = FormUrlEncodedEnctypePostMethodController::process(&request, response, connection);
            return Ok(response)
        }

        if FormGetMethodController::is_matching(&request, connection) {
            response = FormGetMethodController::process(&request, response, connection);
            return Ok(response)
        }

        if FormMultipartEnctypePostMethodController::is_matching(&request, connection) {
            response = FormMultipartEnctypePostMethodController::process(&request, response, connection);
            return Ok(response)
        }

        if FaviconController::is_matching(&request, connection) {
            response = FaviconController::process(&request, response, connection);
            return Ok(response)
        }

        if StaticResourceController::is_matching(&request, connection) {
            response = StaticResourceController::process(&request, response, connection);
            return Ok(response)
        }

        if NotFoundController::is_matching(&request, connection) {
            response = NotFoundController::process(&request, response, connection);
            return Ok(response)
        }


        Ok(response)
    }
}

impl App {
    pub fn handle_request(request: Request) -> (Response, Request) {
        let header_list = Header::get_header_list(&request);

        let mut response: Response = Response::get_response(
            STATUS_CODE_REASON_PHRASE.n501_not_implemented,
            Some(header_list),
            None
        );



        if IndexController::is_matching_request(&request) {
            response = IndexController::process_request(&request, response);
            return (response, request)
        }

        if StyleController::is_matching_request(&request) {
            response = StyleController::process_request(&request, response);
            return (response, request)
        }

        if ScriptController::is_matching_request(&request) {
            response = ScriptController::process_request(&request, response);
            return (response, request)
        }

        if FileUploadInitiateController::is_matching_request(&request) {
            response = FileUploadInitiateController::process_request(&request, response);
            return (response, request)
        }

        if FormUrlEncodedEnctypePostMethodController::is_matching_request(&request) {
            response = FormUrlEncodedEnctypePostMethodController::process_request(&request, response);
            return (response, request)
        }

        if FormGetMethodController::is_matching_request(&request) {
            response = FormGetMethodController::process_request(&request, response);
            return (response, request)
        }

        if FormMultipartEnctypePostMethodController::is_matching_request(&request) {
            response = FormMultipartEnctypePostMethodController::process_request(&request, response);
            return (response, request)
        }

        if FaviconController::is_matching_request(&request) {
            response = FaviconController::process_request(&request, response);
            return (response, request)
        }

        if StaticResourceController::is_matching_request(&request) {
            response = StaticResourceController::process_request(&request, response);
            return (response, request)
        }

        if NotFoundController::is_matching_request(&request) {
            response = NotFoundController::process_request(&request, response);
            return (response, request)
        }


        (response, request)
    }

}