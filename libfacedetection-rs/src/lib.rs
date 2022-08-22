use libfacedetection_sys::facedetect_cnn as facedetect_cnn_sys;
use std::os::raw::c_int;
use std::alloc::{self, Layout};
use thiserror::Error;

// DO NOT CHANGE!
const BUF_SIZE: usize = 0x20000;

#[derive(Error, Debug)]
pub enum LibFacedetectionError {
    #[error("allocation error")]
    AllocError(#[from] alloc::LayoutError),
    #[error("error from the facedetection lib")]
    FaceDetectionError,
}

#[derive(Debug)]
pub struct Face {
    pub confidence: u16,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub landmarks: [u16; 10],
}

impl Face {
    unsafe fn from_ptr(data: *const u16) -> Self {
        let confidence = *data.offset(0);
        let x = *data.offset(1);
        let y = *data.offset(2);
        let w = *data.offset(3);
        let h = *data.offset(4);
        let mut landmarks = [0; 10];
        for idx in 0..10 {
            let landmark = *data.offset(5+idx);
            landmarks[idx as usize] = landmark;
        }
        Face {
            confidence,
            x,
            y,
            w,
            h,
            landmarks
        }
    }
}

#[derive(Debug)]
pub struct DetectionResult {
    pub faces: Vec<Face>
}

pub fn facedetect_cnn(
    bgr_image_data: *const u8,
    width: i32,
    height: i32,
    step: u32,
) -> Result<DetectionResult, LibFacedetectionError> {
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
        return Err(LibFacedetectionError::FaceDetectionError);
    }
    let faces_detected = unsafe { *result };

    let mut faces = Vec::with_capacity(faces_detected as usize);
    for idx in 0..faces_detected {
        let p = unsafe { (result.offset(1) as *const u16).offset(142*idx as isize) };
        let face = unsafe { Face::from_ptr(p) };
        faces.push(face);
    }
    unsafe { alloc::dealloc(result_buffer, layout); }
    let detection_result = DetectionResult { faces: faces };
    Ok(detection_result)
}
