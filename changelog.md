# Trotter Changelog

## 2023.11.11 - 0.1.0
- Initial commit 🥳

## 2023.11.13 - 0.2.0
- Added `trot` binary
- Added `Actor::input`
- Added `Response::save_to_path`
- Added tcp timeout

## 2023.11.13 - 0.3.0
- Added `--pretty-print` option to `trot`
- Added `parse` module for parsing gemtext into symbols.
- Added `Response::is_gemtext`

## 2023.11.15 - 0.4.0
- `trot`: Decided to remove `-i`, `--input` and instead
  capture all remaining arguments as input because it feels
  better to use.
- Added `Response::certificate_pem` function
- Added `Response::certificate_info` function
- Added domain name validation.
- Fixed queries being stripped from urls when an input function
  isn't used.
- Fixed scenario where client would read more than 1024
  bytes in header.

## 2023.11.16 - 0.5.0
- `trot`: Added `--cert-pem` and `--cert-info` options
- Implemented `Into<u8>` for `Status`, and removed panic.
