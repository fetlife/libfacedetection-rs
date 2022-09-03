generate_bindings:
	bindgen  wrapper.hpp --allowlist-function facedetect_cnn  --  -I libfacedetection/src/ -I ./ > src/bindings.rs
defalt: generate_bindings
