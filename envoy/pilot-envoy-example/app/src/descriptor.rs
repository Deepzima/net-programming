pub fn file_descriptor_set() -> &'static [u8] {
    include_bytes!("../envoy_descriptor.bin")
}
