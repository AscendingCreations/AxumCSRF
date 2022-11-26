# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## 0.6.0 (26. November, 2022)
### Changed
- (Breaking) Removed Service
- (Breaking) Removed Layer
- Updated to Axum 0.6

## 0.5.0 (20. July, 2022)
### Changed
- Removed key from layer added to config instead.

### Removed
- Removed layer builder. Please change to use Config method instead.

### Added
- Several new configurations to better config the cookie.
- Added key switch to enable Private or Public cookies. Public is only for debugging!
- Starting this Changelog at 0.5.

### Fixed
- cookie not being saved due to cookie domain not getting set and SameSite set to Strict. Only affected linux?
