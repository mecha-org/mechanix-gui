#!/usr/bin/env nu

# Debian Packaging Script for Flutter eLinux Apps
# Usage: nu package-deb.nu <app-name> <output-dir>

def main [
    app_name: string,         # App name from metadata (e.g., "files", "camera")
    output_dir: string        # Output directory for .deb file
] {
    print $"[INFO] Starting Debian packaging for ($app_name)"

    # Load packaging metadata
    let metadata_file = "packaging-metadata.yaml"

    if not ($metadata_file | path exists) {
        print $"[ERROR] ($metadata_file) not found in current directory"
        print "[ERROR] Make sure you're running this from the apps/ directory"
        exit 1
    }

    let metadata = open $metadata_file

    # Find app in metadata
    let app = ($metadata.applications | where name == $app_name | first)

    if ($app | is-empty) {
        print $"[ERROR] App '($app_name)' not found in metadata"
        print "[ERROR] Available apps:"
        $metadata.applications | select name folder | print
        exit 1
    }

    let app_folder = $app.folder
    let binary_name = $app.binary
    let app_description = $app.description
    let app_maintainer = $app.maintainer
    let dependencies = ($app.dependencies | str join ", ")

    print $"[INFO] App: ($app_name)"
    print $"[INFO] Folder: ($app_folder)"
    print $"[INFO] Binary: ($binary_name)"

    # Read version from pubspec.yaml
    let pubspec_path = $"($app_folder)/pubspec.yaml"

    if not ($pubspec_path | path exists) {
        print $"[ERROR] pubspec.yaml not found at ($pubspec_path)"
        exit 1
    }

    let pubspec = open $pubspec_path
    let app_version = $pubspec.version

    print $"[INFO] Version: ($app_version)"

    # Get architecture
    let pkg_arch = (dpkg --print-architecture | str trim)
    print $"[INFO] Architecture: ($pkg_arch)"

    # Define build directory
    let build_dir = $"($app_folder)/build/elinux/arm64/release/bundle"

    if not ($build_dir | path exists) {
        print $"[ERROR] Build directory not found at ($build_dir)"
        print "[ERROR] Make sure you've run 'flutter-elinux build elinux --release' first"
        exit 1
    }

    # Package name from metadata
    let pkg_name = $"mechanix-($app_name)"

    # Create package directory structure
    let pkg_dir = "package"

    print $"[INFO] Creating package structure in ($pkg_dir)"

    mkdir $pkg_dir
    mkdir $"($pkg_dir)/DEBIAN"
    mkdir $"($pkg_dir)/usr/bin"
    mkdir $"($pkg_dir)/usr/share/mechanix/($pkg_name)/data"
    mkdir $"($pkg_dir)/usr/share/mechanix/($pkg_name)/lib"
    mkdir $"($pkg_dir)/usr/lib/($pkg_name)"

    # Generate debian/control file
    print "[INFO] Generating DEBIAN/control file"

    let control_content = $"Package: ($pkg_name)
Version: ($app_version)
Section: utils
Priority: optional
Architecture: ($pkg_arch)
Maintainer: ($app_maintainer)
Depends: ($dependencies)
Description: ($app_description)
"

    $control_content | save -f $"($pkg_dir)/DEBIAN/control"

    # Copy binary
    let binary_src = $"($build_dir)/($binary_name)"
    let binary_dest = $"($pkg_dir)/usr/bin/($binary_name)"

    print $"[INFO] Copying binary: ($binary_src) -> ($binary_dest)"

    if not ($binary_src | path exists) {
        print $"[ERROR] Binary not found at ($binary_src)"
        exit 1
    }

    cp $binary_src $binary_dest
    chmod 755 $binary_dest

    # Copy data assets if they exist
    let data_src = $"($build_dir)/data"
    if ($data_src | path exists) {
        print $"[INFO] Copying data assets from ($data_src)"
        let data_items = (ls $data_src)
        if not ($data_items | is-empty) {
            for item in $data_items {
                let dest = $"($pkg_dir)/usr/share/mechanix/($pkg_name)/data/(($item.name | path basename))"
                cp -r $item.name $dest
            }
        }
    } else {
        print "[INFO] No data directory found, skipping"
    }

    # Copy lib directory if it exists
    let lib_src = $"($build_dir)/lib"
    if ($lib_src | path exists) {
        print $"[INFO] Copying libraries from ($lib_src)"
        let lib_items = (ls $lib_src)
        if not ($lib_items | is-empty) {
            for item in $lib_items {
                let dest = $"($pkg_dir)/usr/share/mechanix/($pkg_name)/lib/(($item.name | path basename))"
                cp -r $item.name $dest
            }
        }
    } else {
        print "[INFO] No lib directory found, skipping"
    }

    # Create output directory
    mkdir $output_dir

    # Build .deb package
    let deb_filename = $"($pkg_name)_($app_version)_($pkg_arch).deb"
    let deb_path = $"($output_dir)/($deb_filename)"

    print $"[INFO] Building .deb package: ($deb_path)"

    ^dpkg-deb --build $pkg_dir $deb_path

    # Cleanup
    print "[INFO] Cleaning up temporary files"
    rm -rf $pkg_dir

    print $"[SUCCESS] ✅ Package created: ($deb_path)"

    # Return the path for use in CI
    print $deb_path
}