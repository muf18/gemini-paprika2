# Real-Time Cross-Platform Crypto Charting Client

This project is a self-contained, cross-platform real-time crypto price charting application, similar to TradingView. It is built with a Rust core for high-performance data processing and a Flutter UI for a responsive, cross-platform user experience.

## Features

- **Direct Exchange Connectivity**: Connects directly to public WebSocket endpoints of multiple exchanges.
- **Real-Time Aggregation**: Computes rolling VWAP and cumulative volume on-the-fly.
- **GPU-Accelerated Charting**: Renders responsive charts using Flutter and Skia.
- **Cross-Platform**: A single codebase for Android, iOS, Windows, macOS, and Linux.
- **CI/CD Ready**: Fully automated build, test, and packaging pipeline using GitHub Actions.

## Tech Stack

- **Core Logic**: Rust
- **UI**: Flutter
- **Concurrency**: Tokio
- **FFI Bridge**: `flutter_rust_bridge`
- **Data Schema**: Protocol Buffers

## How to Build

This project is configured for automated builds via the GitHub Actions workflow defined in `.github/workflows/build_and_test.yml`.

To build locally:

1.  **Install Prerequisites**:
    -   Rust toolchain
    -   Flutter SDK
    -   Protocol Buffers Compiler (`protoc`)
    -   `flutter_rust_bridge_codegen`: `cargo install flutter_rust_bridge_codegen`

2.  **Generate FFI Code**:
    ```bash
    flutter_rust_bridge_codegen --rust-input ./chart_core/src/api.rs --dart-output ./flutter_app/lib/src/rust/frb_generated.dart --c-output ./flutter_app/ios/Runner/generated.h
    ```

3.  **Run the App**:
    ```bash
    cd flutter_app
    flutter pub get
    flutter run
    ```