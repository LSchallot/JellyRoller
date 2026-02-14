# Change Log
All notable changes to this project will be documented in this file.
 
## [1.1.0] - In progress
Special thanks to @JKamsker or their contributions to this release!

### Added
- Added ability to create Reports for Series and BoxSets
- Added ability to enable/disable a library

### Fixed
- Corrected issue with Movie report generation

### Change
- Consolidation of image handling code

### Maintenance
- Dependency updates

## [1.0.0] - 2025-11-03
I am happy to present JellyRoller 1.0.0.  Thank you to everyone that has used this software and special thanks to everyone that has submitted issues, feature requests, and contributions.

### DEPRECATION NOTICE
- The "--json" flag has been removed.  This can be replace with the "--output-format" flag.

### Added
- Added ability to generate and restore from backup via new Jellyfin API.
- Added ability to configure new server instance via a configuration file.
- Added ability to change a user's profile image.
- Integrated prop_reader for easy property file management.

### Maintenance
- Code reorganization.
- Dependency updates.

## [0.8.0] - 2025-03-11
Special thanks to @jkellz-dev for their contributions to this release!

### DEPRECATION NOTICE
- The "--json" flag will be removed as of the 1.0.0 release.  I am not sure exactly when that will be, but please make efforts to move away from the "--json" boolean and move towards the "--output_format" flag

### Added
- Added ability to generate auto-completions (@jkellz-dev)
- Added ability to perform a silent configuration

### Changed
- Changed hierarchy of download archive

### Maintenance
- Clippy cleanup
- Updated documentation
- Updated dependencies based on dependabot and cargo-audit

## [0.7.0] - 2025-01-12
Special thanks to @xeoneox for their contributions to this release!

### DEPRECATION NOTICE
- The "--json" flag will be removed as of the 1.0.0 release.  I am not sure exactly when that will be, but please make efforts to move away from the "--json" boolean and move towards the "--output_format" flag

### Added
- Added applicationt to Homebrew (@xeoneox)
- Added new artifact for Homebrew downloads
- Added output_format to arguments that previously only had the "--json" flag

### Changed
- Changed ordering of help menu and various functions in the code
- Help menu can now be shown prior to initial configuration

### Maintenance
- Clippy cleanup
- Updated documentation
- Formatted code via rustfmt

## [0.6.0] - 2025-01-04
Special thanks to @jamesread for his contributions to this release!

### Added
- Customizable output columns on search, and CSV support (@jamesread)
- Search on parentid and output in json format, optionally with filename (@jamesread)
- New commands added
    + get-repositories
    + register-repository
    + get-packages
    + install-package

## [0.5.0] - 2024-12-15

### Added
- Bug report generation (planned for future use)
- Added ability to use local jellyroller.config
- Added additional binaries for ARMv7 and ARMv8
- Extended library scanning capabilities
- New commands added
    + Update item metadata
    + Add a new library

### Changed
- Dependency updates
- Documentation updates
- Clippy clenup

### Fixed
- Help text verbiage

## [0.4.1] - 2024-07-05

### Fixed
- Fixed the reset-password body information to prevent users from being passwordless

## [0.4.0] - 2024-06-30

### Added
- Ability to edit images by name or id

### Changed
- Migrated away from using x-emby-authentication header values
- Updated dependencies
- Updated documentation

## [0.3.1] - 2023-07-09

### Changed
- Dependency updates

## [0.3.0] - 2023-07-07

### Added
- New commands added
    + Export activity
    + Export Movies
    + Search Media
- Added api key support
    + All existing instances will be migrated on next execution

### Fixed
- Dependency updates
- Issue with executing tasks while background tasks were running
- Documentation clarity

## [0.2.0] - 2022-09-12
 
### Added
- Project repository structure corrections
    + Updated README.md
    + Added .gitignore
    + Added CHANGELOG.md
- New commands added
    + Jellyfin restart
    + Jellyfin shutdown
    + Library information display
    + Plugin information display
    + Added ability to list tasks
    + Added ability to execute tasks by name
    + Added ability to export all user information excluding passwords
    + Added ability to import users from a file.

### Changed
- Output displays
- Project code structure reworked
- Small changes to implement best practices

### Fixed
- Fixed issue where some user details were being reverted to default values upon a policy update
 
## [0.1.0] - 2022-08-23
Initial release
