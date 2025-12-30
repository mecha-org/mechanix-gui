#!/usr/bin/env nu

# Debian Packaging Script for Flutter eLinux Apps
# Usage: nu package-deb.nu <app-name> <output-dir>

def main [
    app_name: string,
    output_dir: string
] {
    print $"[INFO] Starting Debian packaging for ($app_name)"

    # Load packaging metadata
    let metadata_file = "packaging-metadata.yaml"

    if not ($metadata_file | path exists) {
        error make { msg: $"Metadata file not found: ($metadata_file)" }
    }

    let metadata = open $metadata_file

    # Find app in metadata
    let app = ($metadata.applications | where name == $app_name | first)

    if ($app | is-empty) {
        error make { msg: $"App '($app_name)' not found in metadata" }
    }

    let app_folder = $app.folder
    let binary_name = $app.binary
    let app_maintainer = $app.maintainer
    let dependencies = ($app.dependencies | str join ", ")

    print $"[INFO] App: ($app_name)"
    print $"[INFO] Folder: ($app_folder)"
    print $"[INFO] Binary: ($binary_name)"

    # Read version from pubspec.yaml
    let pubspec_path = $"($app_folder)/pubspec.yaml"

    if not ($pubspec_path | path exists) {
        error make { msg: $"pubspec.yaml not found at ($pubspec_path)" }
    }

    let pubspec = open $pubspec_path
    let app_version = $pubspec.version
    let app_description = $pubspec.description

    print $"[INFO] Upstream Version: ($app_version)"

    # Get architecture
    let pkg_arch = (dpkg --print-architecture | str trim)
    print $"[INFO] Architecture: ($pkg_arch)"

    # Package name
    let pkg_name = $"mechanix-($app_name)"

    # Find Package resolver Script 
    let resolver_script = "../utils/resolve-next-version.nu" | path expand

    # Get version info from resolver and parse as JSON
    let version_data = try {
        # Using parenthesized external call. 
        # Since it's executable and has a shebang, we just call the path.
        (^$resolver_script 
            --format "deb" 
            --name $pkg_name 
            --upstream $app_version 
            --base-url "http://pkg.mecha.so" 
            | from json)
    } catch {
        print "[WARN] Resolver failed, defaulting to revision 1"
        { 
            full_version: $"($app_version)-1", 
            next_revision: 1, 
            upstream_version: $app_version 
        }
    }
    
    let pkg_version = $version_data.full_version
    let pkg_revision = $version_data.next_revision

    print $"[INFO] Full Package Version: ($pkg_version)"
    print $"[INFO] Package Revision: ($pkg_revision)"

    # Build directory
    let build_dir = $"($app_folder)/build/elinux/arm64/release/bundle"

    if not ($build_dir | path exists) {
        error make { msg: $"Build directory not found at ($build_dir). Run 'flutter-elinux build elinux --release' first." }
    }

    # Package structure
    let pkg_dir = "package"
    mkdir $pkg_dir
    mkdir $"($pkg_dir)/DEBIAN"
    mkdir $"($pkg_dir)/usr/bin"
    mkdir $"($pkg_dir)/usr/share/mechanix/($pkg_name)/data"
    mkdir $"($pkg_dir)/usr/share/mechanix/($pkg_name)/lib"
    mkdir $"($pkg_dir)/usr/lib/($pkg_name)"

    # DEBIAN/control
    let control_content = $"Package: ($pkg_name)
Version: ($pkg_version)
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

    if not ($binary_src | path exists) {
        error make { msg: $"Binary not found at ($binary_src)" }
    }

    cp $binary_src $binary_dest
    chmod 755 $binary_dest

    # Copy data
    let data_src = $"($build_dir)/data"
    if ($data_src | path exists) {
        for item in (ls $data_src) {
            let dest = $"($pkg_dir)/usr/share/mechanix/($pkg_name)/data/(($item.name | path basename))"
            cp -r $item.name $dest
        }
    }

    # Copy lib
    let lib_src = $"($build_dir)/lib"
    if ($lib_src | path exists) {
        for item in (ls $lib_src) {
            let dest = $"($pkg_dir)/usr/share/mechanix/($pkg_name)/lib/(($item.name | path basename))"
            cp -r $item.name $dest
        }
    }

    # Output dir
    mkdir $output_dir
    let deb_filename = $"($pkg_name)_($pkg_version)_($pkg_arch).deb"
    let deb_path = $"($output_dir)/($deb_filename)"
    ^dpkg-deb --build $pkg_dir $deb_path

    # Cleanup
    rm -rf $pkg_dir

    print $"[SUCCESS] ✅ Package created: ($deb_path)"
    print $deb_path
}