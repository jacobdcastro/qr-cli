# qr-cli

A fast and simple command-line tool to generate and display QR codes for URLs and other strings.

## Features

- Generate QR codes from text or URLs
- Save QR codes as SVG or PNG files
- Automatically open generated QR codes in system default viewer
- Cross-platform support (Windows, macOS, Linux)

## Installation

```sh
cargo install qr-cli
```

## Usage

Generate a QR code from a URL and open it in the default viewer:

```sh
qr "https://example.com"
```

Generate a QR code from a string and save it as a PNG file:

```sh
qr "https://example.com" --output qr.png
```
