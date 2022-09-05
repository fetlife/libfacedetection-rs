TARGET_TRIPLET := $(shell rustc --version --verbose | grep host | cut -d ' ' -f 2)

generate_bindings:
	bindgen wrapper.hpp --allowlist-function facedetect_cnn  --  -I libfacedetection/src/ -I ./ > bindings/libfacedetection_$(TARGET_TRIPLET).rs
defalt: generate_bindings
