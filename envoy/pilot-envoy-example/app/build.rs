fn main() {
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path("envoy_descriptor.bin")
        .compile(&["./proto/ads.proto"], &["../proto"])
        .expect("Errore nella compilazione dei proto");
}
