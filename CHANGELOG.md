# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## 0.7.2 (13. July, 2023)
### Added
- dor.rs cargo entries so layer options will appear in documents.

## 0.7.1 (13. July, 2023)
### Added
- Layer Feature to allow getting CsrfTokens using a service.
- Example for middleware usage.

## 0.7.0 (12. July, 2023)
### Changed
- Replaced Bcrypt with Argon2.
- authenticity_token now returns an error instead of unwrapping.
- Added Error type to give slightly better error return messages.

## 0.6.1 (30. March, 2023)
### Changed
- Updated Base64 and cookie.

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
