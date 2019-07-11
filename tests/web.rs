//! Test suite for the Web and headless browsers.
// NOTE: Other tests are within the relevant source files

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use base64::decode;
use image_worker::process_image;
use wasm_bindgen::JsValue;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
enum TestImage {
    Jpeg,
    Png,
}

#[cfg(test)]
impl TestImage {
    fn get_vec(&self) -> Vec<u8> {
        decode(match self {
            TestImage::Jpeg => "/9j/4AAQSkZJRgABAQAASABIAAD/4QDKRXhpZgAATU0AKgAAAAgABwESAAMAAAABAAEAAAEaAAUAAAABAAAAYgEbAAUAAAABAAAAagEoAAMAAAABAAIAAAExAAIAAAARAAAAcgEyAAIAAAAUAAAAhIdpAAQAAAABAAAAmAAAAAAAAABIAAAAAQAAAEgAAAABUGl4ZWxtYXRvciAzLjguNQAAMjAxOTowNzoxMSAxMjowNzo0MgAAA6ABAAMAAAABAAEAAKACAAQAAAABAAAAyKADAAQAAAABAAAAyAAAAAD/4QmSaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLwA8P3hwYWNrZXQgYmVnaW49Iu+7vyIgaWQ9Ilc1TTBNcENlaGlIenJlU3pOVGN6a2M5ZCI/PiA8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA1LjQuMCI+IDxyZGY6UkRGIHhtbG5zOnJkZj0iaHR0cDovL3d3dy53My5vcmcvMTk5OS8wMi8yMi1yZGYtc3ludGF4LW5zIyI+IDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiIHhtbG5zOnhtcD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLyIgeG1wOkNyZWF0b3JUb29sPSJQaXhlbG1hdG9yIDMuOC41IiB4bXA6TW9kaWZ5RGF0ZT0iMjAxOS0wNy0xMVQxMjowNzo0MiIvPiA8L3JkZjpSREY+IDwveDp4bXBtZXRhPiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIDw/eHBhY2tldCBlbmQ9InciPz4A/+0AOFBob3Rvc2hvcCAzLjAAOEJJTQQE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",
            TestImage::Png => "iVBORw0KGgoAAAANSUhEUgAAAMgAAADICAIAAAAiOjnJAAAAAXNSR0IArs4c6QAAAAlwSFlzAAALEwAACxMBAJqcGAAAA6ppVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IlhNUCBDb3JlIDUuNC4wIj4KICAgPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4KICAgICAgPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIKICAgICAgICAgICAgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIgogICAgICAgICAgICB4bWxuczp0aWZmPSJodHRwOi8vbnMuYWRvYmUuY29tL3RpZmYvMS4wLyIKICAgICAgICAgICAgeG1sbnM6ZXhpZj0iaHR0cDovL25zLmFkb2JlLmNvbS9leGlmLzEuMC8iPgogICAgICAgICA8eG1wOk1vZGlmeURhdGU+MjAxOS0wNy0xMVQxMjowNzo0NTwveG1wOk1vZGlmeURhdGU+CiAgICAgICAgIDx4bXA6Q3JlYXRvclRvb2w+UGl4ZWxtYXRvciAzLjguNTwveG1wOkNyZWF0b3JUb29sPgogICAgICAgICA8dGlmZjpPcmllbnRhdGlvbj4xPC90aWZmOk9yaWVudGF0aW9uPgogICAgICAgICA8dGlmZjpDb21wcmVzc2lvbj4wPC90aWZmOkNvbXByZXNzaW9uPgogICAgICAgICA8dGlmZjpSZXNvbHV0aW9uVW5pdD4yPC90aWZmOlJlc29sdXRpb25Vbml0PgogICAgICAgICA8dGlmZjpZUmVzb2x1dGlvbj43MjwvdGlmZjpZUmVzb2x1dGlvbj4KICAgICAgICAgPHRpZmY6WFJlc29sdXRpb24+NzI8L3RpZmY6WFJlc29sdXRpb24+CiAgICAgICAgIDxleGlmOlBpeGVsWERpbWVuc2lvbj4yMDA8L2V4aWY6UGl4ZWxYRGltZW5zaW9uPgogICAgICAgICA8ZXhpZjpDb2xvclNwYWNlPjE8L2V4aWY6Q29sb3JTcGFjZT4KICAgICAgICAgPGV4aWY6UGl4ZWxZRGltZW5zaW9uPjIwMDwvZXhpZjpQaXhlbFlEaW1lbnNpb24+CiAgICAgIDwvcmRmOkRlc2NyaXB0aW9uPgogICA8L3JkZjpSREY+CjwveDp4bXBtZXRhPgr0M5ySAAAEs0lEQVR4Ae3UwYlVQRRFUZVOSOhsBCMwrc5G6JAUpzXYHw71Ri5nj/NvgYtNf/3z89cX/wjcFvh2+0HvEfgnICwdPCIgrEdYPSosDTwiIKxHWD0qLA08IiCsR1g9KiwNPCIgrEdYPSosDTwi8Pby1d8/vr/8jR/8bwLvH5/9X/YXq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so4CwRjhnLSCs9rGOAsIa4Zy1gLDaxzoKCGuEc9YCwmof6yggrBHOWQsIq32so8Dby7v3j8+Xv/EDAoeAv1gHiM87AsK64+iVQ0BYB4jPOwLCuuPolUNAWAeIzzsCwrrj6JVDQFgHiM87AsK64+iVQ0BYB4jPOwLCuuPolUPgL+NCCAXjUgXAAAAAAElFTkSuQmCC",
        }).unwrap()
    }
}

#[derive(Serialize)]
struct ProcessImageParams {
    bg: Vec<u8>,
    dx: f32,
    dy: f32,
    format: String,
    height: u32,
    mode: String,
    quality: u8,
    scale: f32,
    width: u32,
}

#[wasm_bindgen_test]
fn process_jpeg_image_in_browser() {
    let data = TestImage::Jpeg.get_vec();

    assert_eq!(data.len(), 6391);

    let mut array: [u8; 6391] = [0; 6391];
    let bytes = &data[..array.len()];
    array.copy_from_slice(bytes);

    process_image(
        &array,
        JsValue::from_serde(&ProcessImageParams {
            bg: vec![],
            dx: 0.0,
            dy: 0.0,
            format: "jpeg".to_string(),
            height: 100,
            mode: "fill".to_string(),
            quality: 90,
            scale: 1.0,
            width: 50,
        })
        .unwrap(),
    )
    .unwrap();
}

#[wasm_bindgen_test]
fn process_png_image_in_browser() {
    let data = TestImage::Png.get_vec();

    assert_eq!(data.len(), 2244);

    let mut array: [u8; 2244] = [0; 2244];
    let bytes = &data[..array.len()];
    array.copy_from_slice(bytes);

    process_image(
        &array,
        JsValue::from_serde(&ProcessImageParams {
            bg: vec![],
            dx: 0.0,
            dy: 0.0,
            format: "png".to_string(),
            height: 100,
            mode: "fill".to_string(),
            quality: 90,
            scale: 1.0,
            width: 50,
        })
        .unwrap(),
    )
    .unwrap();
}