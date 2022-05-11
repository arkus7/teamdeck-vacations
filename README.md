# teamdeck-vacations

# Build tools

Tools used:
- [`just`](https://github.com/casey/just) - Handy way to save and run project-specific commands (similar to `make`)
- [`fpm`](https://github.com/jordansissel/fpm) - Effing package management! Build packages for multiple platforms (deb, rpm, etc) with great ease and sanity.

# Packaging
This project uses [`fpm`](https://fpm.readthedocs.io/en/latest/) for packaging the binary into macOS `.pkg` installer.

## Creating `.app` bundle for macOS

Guide: https://stackoverflow.com/a/3251285/4856711

Icons: 
- https://stackoverflow.com/a/20703594/4856711
- https://stackoverflow.com/a/32490491/4856711