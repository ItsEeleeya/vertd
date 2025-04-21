fn main() {
    // swift-rs has a minimum of macOS 10.13
    // Ensure the same minimum supported macOS version is specified as in your `Package.swift` file.
    swift_rs::SwiftLinker::new("13.3")
        .with_package("VertSwiftLib", "../swift-lib/")
        .link();

    tauri_build::build()
}
