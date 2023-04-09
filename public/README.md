This is a library for using Cloud Vision in your flow function for [flows.network](https://flows.network).

## Usage example
```rust
use cloud_vision_flows::text_detection;
use lambda_flows::{request_received, send_response};

#[no_mangle]
pub fn run() {
    request_received(|_qry, body| {
        let text = text_detection(String::from_utf8(body).unwrap());
        match text {
            Ok(r) => send_response(
                200,
                vec![(
                    String::from("content-type"),
                    String::from("text/plain; charset=UTF-8"),
                )],
                r.as_bytes().to_vec(),
            ),
            Err(e) => send_response(
                500,
                vec![(
                    String::from("content-type"),
                    String::from("text/plain; charset=UTF-8"),
                )],
                e.as_bytes().to_vec(),
            ),
        }
    });
}
```

The raw body of the request of this lambda function will be passed to [`text_detection`](https://docs.rs/cloud-vision-flows/latest/cloud_vision_flows/fn.text_detection.html) then the function respond with the detected text.

The whole document is [here](https://docs.rs/cloud-vision-flows).

