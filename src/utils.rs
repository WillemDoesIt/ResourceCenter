use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

/// ### Description
/// Fetches the raw byte contents of a file from a path relative to directory served
///
/// ### Example
/// ```rust
/// // file is accessible via `localhost:8080/assets/image.png``
/// let data: Vec<u8> = get_file_bytes("assets/image.png").await;
/// ```
///
/// ### Warnings
/// Must called within async function
pub async fn get_file_bytes(path: &str) -> Vec<u8> {
    let window = web_sys::window().expect_log("No window found");
    let resp_value = JsFuture::from(window.fetch_with_str(path))
        .await
        .expect_log("Failed to get response from path and window");

    // `resp_value` is a `Response` object.
    let resp: Response = resp_value
        .dyn_into()
        .expect_log("Failed to get response from response value");

    // Convert response to an ArrayBuffer
    let promise = resp
        .array_buffer()
        .expect_log("Failed to Create array_buffer promise");
    let buffer = JsFuture::from(promise)
        .await
        .expect_log("Failed to get JsFuture (buffer) from promise");

    // Convert Buffer to Uint8Array to Vec and return it
    return js_sys::Uint8Array::new(&buffer).to_vec();
}

/// ### Description
/// Adjusts a string to a specified length by either truncating it or padding it with a filler character
///
/// - If the input string is longer than the desired length, it is truncated and ends with an ellipsis (`…`).
/// - If the input string is shorter, the filler character is appended to reach the desired length
///
/// ### Example
/// ```rust
/// let text = "Hello World!";
/// println!("{}", force_length(&text, 15, '-')); // Hello World!---
/// println!("{}", force_length(&text, 6, '-'));  // Hello…
/// ```
pub fn force_length(input: &str, length: usize, filler: char) -> String {
    let mut result = String::new();
    let mut char_count = 0;

    for char in input.chars() {
        if char_count >= length - 1 {
            break;
        }
        result.push(char);
        char_count += 1;
    }

    if input.chars().count() > length - 1 {
        result.push('…');
    } else {
        while char_count < length {
            result.push(filler);
            char_count += 1;
        }
    }

    result
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub trait ExpectLog<T> {
    fn expect_log(self, msg: &str) -> T;
}

fn log_and_panic(msg: &str) -> ! {
    let formatted = format!("[  ERROR  ] {}", msg);
    log(&formatted);
    panic!("{}", msg);
}

impl<T> ExpectLog<T> for Option<T> {
    fn expect_log(self, msg: &str) -> T {
        self.unwrap_or_else(|| log_and_panic(msg))
    }
}

impl<T, E> ExpectLog<T> for Result<T, E> {
    fn expect_log(self, msg: &str) -> T {
        self.unwrap_or_else(|_| log_and_panic(msg))
    }
}

