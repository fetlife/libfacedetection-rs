# libfacedetection-rs

Rust bindings to [libfacedetection](https://github.com/ShiqiYu/libfacedetection)

## Installing

You need to have  [libclang](https://rust-lang.github.io/rust-bindgen/requirements.html) and [cmake](https://github.com/rust-lang/cmake-rs) installed to generate bindings, no other lib is required, although it's convenient to use opencv-rust to pass in image data.

## Usage

```rust
use libfacedetection_rs::facedetect_cnn;
use opencv_core::Mat;

// load your own image instead of using `Mat::default()`
let frame = Mat::default();

let facedetect_result = facedetect_cnn(
    frame.ptr(0)?,
    frame.cols(),
    frame.rows(),
    frame.mat_step().get(0) as u32,
);
match facedetect_result {
    Ok(detection_result) => {
        for face in detection_result.faces {
            println!("Found face: {:?}", face);
            println!("Confidence: {:?}", face.confidence);
            println!("Bounding box: {},{},{},{}", face.x, face.y, face.width, face.height);
            println!("Landmarks: {:?}", face.landmarks);
        }
    }
    Err(e) => {
        println!("Error in libfacedetection_rs: {:?}", e);
    }
}
```
