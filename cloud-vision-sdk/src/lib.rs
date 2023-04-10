//! Google Cloud Vision integration for [Flows.network](https://flows.network)
//!
//! # Quick Start
//!
//! To get started, let's write a very tiny flow function.
//!
//! ```rust
//! use cloud_vision_flows::text_detection;
//! use lambda_flows::{request_received, send_response};
//!
//! #[no_mangle]
//! pub fn run() {
//!     request_received(|_qry, body| {
//!         let text = text_detection(String::from_utf8(body).unwrap());
//!         match text {
//!             Ok(r) => send_response(
//!                 200,
//!                 vec![(
//!                     String::from("content-type"),
//!                     String::from("text/plain; charset=UTF-8"),
//!                 )],
//!                 r.as_bytes().to_vec(),
//!             ),
//!             Err(e) => send_response(
//!                 500,
//!                 vec![(
//!                     String::from("content-type"),
//!                     String::from("text/plain; charset=UTF-8"),
//!                 )],
//!                 e.as_bytes().to_vec(),
//!             ),
//!         }
//!     });
//! }
//!
//! ```
//!
//! When the Lambda request is received, detect the text using [text_detection].

use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref CLOUD_VISION_API_PREFIX: String = String::from(
        std::option_env!("CLOUD_VISION_API_PREFIX")
            .unwrap_or("https://cloud-vision-flows-integration.vercel.app/api")
    );
}

extern "C" {
    fn get_flows_user(p: *mut u8) -> i32;
    fn get_flow_id(p: *mut u8) -> i32;
    fn set_error_log(p: *const u8, len: i32);
}

/// Detect text for the given image.
///
/// `image_base64` is the base64 encoded image content.
///
/// Return the detected text.
///
/// Example:
///
/// Take a picture from Internet, such as
/// "https://bullzeyedesign.com/wp-content/uploads/famous_textbased_logos.jpg".
/// If we pass the base64 encoded content of this image,
/// then the result should be:
/// Ok("Coca-Cola DISNEY Google\nFedEx ebay Kleenex")
/// If the content can't be recognized as an image by Cloud Vision,
/// then the result would be something like:
/// Err("Error: 13 INTERNAL: Request message serialization failure: invalid encoding")
///
pub fn text_detection(image_base64: String) -> Result<String, String> {
    unsafe {
        let mut flows_user = Vec::<u8>::with_capacity(100);
        let c = get_flows_user(flows_user.as_mut_ptr());
        flows_user.set_len(c as usize);
        let flows_user = String::from_utf8(flows_user).unwrap();

        let mut flow_id = Vec::<u8>::with_capacity(100);
        let c = get_flow_id(flow_id.as_mut_ptr());
        if c == 0 {
            panic!("Failed to get flow id");
        }
        flow_id.set_len(c as usize);
        let flow_id = String::from_utf8(flow_id).unwrap();

        let mut writer = Vec::new();
        let uri = format!(
            "{}/{}/{}/text_detection",
            CLOUD_VISION_API_PREFIX.as_str(),
            flows_user,
            flow_id,
        );

        let uri = Uri::try_from(uri.as_str()).unwrap();
        let body = image_base64.as_bytes();
        match Request::new(&uri)
            .method(Method::POST)
            .header("Content-Type", "text/plain")
            .header("Content-Length", &body.len())
            .body(body)
            .send(&mut writer)
        {
            Ok(res) => match res.status_code().is_success() {
                true => Ok(String::from_utf8_lossy(&writer).into_owned()),
                false => {
                    set_error_log(writer.as_ptr(), writer.len() as i32);
                    Err(String::from_utf8_lossy(&writer).into_owned())
                }
            },
            Err(e) => {
                let e = e.to_string();
                set_error_log(e.as_ptr(), e.len() as i32);
                return Err(e);
            }
        }
    }
}
