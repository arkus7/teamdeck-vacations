APP_VERSION := `cargo pkgid | cut -d# -f2 | cut -d: -f2`
ICONS_OUTPUT_DIR := "dist/AppIcon.iconset"
ICNS_OUTPUT_FILE := "dist/AppIcon.icns"

default:
    just --list

_check:
    cargo check

build: _check
    cargo build --release

package name="teamdeck-vacations": build package_assets
    fpm -p dist/{{name}}-{{APP_VERSION}}.pkg

clean:
    rm -rf dist

_create_dist:
    mkdir -p dist

package_assets: _create_dist iconset info_plist

iconset:
    mkdir -p {{ICONS_OUTPUT_DIR}}
    sips -z 16 16     assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_16x16.png
    sips -z 32 32     assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_16x16@2x.png
    sips -z 32 32     assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_32x32.png
    sips -z 64 64     assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_32x32@2x.png
    sips -z 128 128   assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_128x128.png
    sips -z 256 256   assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_128x128@2x.png
    sips -z 256 256   assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_256x256.png
    sips -z 512 512   assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_256x256@2x.png
    sips -z 512 512   assets/teamdeck-vacations-logo.png --out {{ICONS_OUTPUT_DIR}}/icon_512x512.png
    cp assets/teamdeck-vacations-logo.png {{ICONS_OUTPUT_DIR}}/icon_512x512@2x.png
    iconutil -c icns {{ICONS_OUTPUT_DIR}} -o {{ICNS_OUTPUT_FILE}}
    rm -R {{ICONS_OUTPUT_DIR}}

info_plist:
    cp assets/Info.plist dist/Info.plist