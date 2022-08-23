use libfacedetection_sys::facedetect_cnn as facedetect_cnn_sys;
use std::alloc::{self, Layout};
use std::os::raw::c_int;
use thiserror::Error;

// DO NOT CHANGE!
const BUF_SIZE: usize = 0x20000;

#[derive(Error, Debug)]
pub enum LibfacedetectionError {
    #[error("allocation error")]
    AllocError(#[from] alloc::LayoutError),
    #[error("error from the facedetection lib")]
    FaceDetectionError,
}

/// A detected face description
#[derive(Debug)]
pub struct Face {
    /// confidence level 0-100
    pub confidence: u16,
    /// x coordinate
    pub x: u16,
    /// y coordinate
    pub y: u16,
    /// width
    pub width: u16,
    /// height
    pub height: u16,
    /// landmarks (nose, eyes, mouth, etc..)
    pub landmarks: [(u16, u16); 5],
}

impl Face {
    unsafe fn from_ptr(data: *const u16) -> Self {
        let confidence = *data.offset(0);
        let x = *data.offset(1);
        let y = *data.offset(2);
        let width = *data.offset(3);
        let height = *data.offset(4);
        let mut landmarks = [(0, 0); 5];
        for idx in 0..5 {
            let landmark_x = *data.offset(5 + idx * 2);
            let landmark_y = *data.offset(5 + idx * 2 + 1);
            let landmark = (landmark_x, landmark_y);
            landmarks[idx as usize] = landmark;
        }
        Face {
            confidence,
            x,
            y,
            width,
            height,
            landmarks,
        }
    }
}

#[derive(Debug)]
pub struct DetectionResult {
    pub faces: Vec<Face>,
}

/// Detect faces in an image using libfacedetection
pub fn facedetect_cnn(
    bgr_image_data: *const u8,
    width: i32,
    height: i32,
    step: u32,
) -> Result<DetectionResult, LibfacedetectionError> {
    let layout = Layout::from_size_align(BUF_SIZE, 32)?;
    let result_buffer = unsafe { alloc::alloc(layout) };

    let result = unsafe {
        facedetect_cnn_sys(
            result_buffer,
            bgr_image_data as *mut u8,
            width as c_int,
            height as c_int,
            step as c_int,
        ) as *const i32
    };
    if result.is_null() {
        return Err(LibfacedetectionError::FaceDetectionError);
    }
    let faces_detected = unsafe { *result };

    let mut faces = Vec::with_capacity(faces_detected as usize);
    for idx in 0..faces_detected {
        let p = unsafe { (result.offset(1) as *const u16).offset(142 * idx as isize) };
        let face = unsafe { Face::from_ptr(p) };
        faces.push(face);
    }
    unsafe {
        alloc::dealloc(result_buffer, layout);
    }
    let detection_result = DetectionResult { faces };
    Ok(detection_result)
}

#[cfg(test)]
mod tests {
    use super::facedetect_cnn;
    use opencv::{imgcodecs::IMREAD_COLOR, prelude::MatTraitConst};
    use opencv::prelude::*;

    #[test]
    fn test_detection() {
        let img = opencv::imgcodecs::imread("tests/face.jpg", IMREAD_COLOR)
            .expect("failed to load image");
        let faces = facedetect_cnn(
            img.ptr(0).unwrap(),
            img.cols(),
            img.rows(),
            img.mat_step().get(0) as u32,
        )
        .expect("failed to detect faces");
        assert_eq!(faces.faces.len(), 1);
    }
}
