#!/usr/bin/env nu

# Debian Packaging Script for Flutter eLinux Apps
# Usage: nu package-deb.nu <app-name> <output-dir>

# Function to get next Debian revision by querying APT
def get_next_debian_revision_apt [
    pkg_name: string,
    app_version: string,
    repo_url: string = "http://pkg.mecha.so/debian"
] {
    print $"[INFO] Updating APT cache to get latest package information..."

    # Check if sudo command exists
    let sudo_exists = (which sudo | is-not-empty)

    # Determine if we should use sudo
    let use_sudo = if $sudo_exists {
        # Check if we can run sudo without password prompt
        let can_sudo = (^sudo -n true | complete | get exit_code) == 0
        $can_sudo
    } else {
        false
    }

    if $use_sudo {
        print "[INFO] Running: sudo apt update"
        let update_result = (^sudo apt update | complete)

        if $update_result.exit_code != 0 {
            print "[WARN] apt update failed, proceeding with cached data"
            print $"[WARN] ($update_result.stderr)"
        } else {
            print "[INFO] APT cache updated successfully"
        }
    } else {
        # Run apt update without sudo
        if $sudo_exists {
            print "[INFO] No sudo access, running: apt update"
        } else {
            print "[INFO] sudo not available, running: apt update"
        }

        let update_result = (^apt update | complete)

        if $update_result.exit_code != 0 {
            print "[WARN] apt update failed, proceeding with cached data"
            print $"[WARN] ($update_result.stderr)"
        } else {
            print "[INFO] APT cache updated successfully"
        }
    }

    print $"[INFO] Querying APT for ($pkg_name) version ($app_version) in repo ($repo_url)"

    try {
        # Get all available versions from APT
        let madison_output = (^apt-cache madison $pkg_name | complete)

        if $madison_output.exit_code != 0 {
            print "[INFO] Package not found in APT, starting with revision 1"
            return 1
        }

        # Parse apt-cache madison output and filter by repository
        let versions = ($madison_output.stdout
            | lines
            | where { |line| ($line | str trim) != "" }
            | where { |line| $line | str contains $repo_url }
            | each { |line|
                let parts = ($line | split row "|")
                if ($parts | length) >= 2 {
                    $parts | get 1 | str trim
                } else {
                    null
                }
            }
            | where { |v| $v != null }
        )

        print $"[DEBUG] Versions found in ($repo_url): ($versions)"

        # Filter for our upstream version with Debian revision
        let matching_versions = ($versions
            | where { |v| $v | str starts-with $"($app_version)-" }
        )

        if ($matching_versions | is-empty) {
            print $"[INFO] No Debian revisions found for ($app_version), starting with -1"
            return 1
        }

        # Extract revision numbers
        let revisions = ($matching_versions
            | each { |v|
                let rev = ($v | str replace $"($app_version)-" "")
                $rev | into int
            }
        )

        let max_revision = ($revisions | math max)
        let next_revision = $max_revision + 1

        print $"[INFO] Found existing revisions for ($app_version) in ($repo_url): ($revisions | str join ', ')"
        print $"[INFO] Next revision will be: -($next_revision)"

        return $next_revision

    } catch {
        print $"[WARN] Error querying APT: ($in)"
        print "[WARN] Defaulting to revision 1"
        return 1
    }
}

def main [
    app_name: string,         # App name from metadata (e.g., "files", "camera")
    output_dir: string,       # Output directory for .deb file
    --repo-url: string = "http://pkg.mecha.so/debian"  # Repository URL to query
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
    let app_description = $pubspec.description

    print $"[INFO] Upstream Version: ($app_version)"

    # Get architecture
    let pkg_arch = (dpkg --print-architecture | str trim)
    print $"[INFO] Architecture: ($pkg_arch)"

    # Package name from metadata
    let pkg_name = $"mechanix-($app_name)"

    # Get next Debian revision from APT
    let deb_revision = (get_next_debian_revision_apt $pkg_name $app_version $repo_url)
    let pkg_version = $"($app_version)-($deb_revision)"

    print $"[INFO] Debian Revision: ($deb_revision)"
    print $"[INFO] Full Package Version: ($pkg_version)"

    # Define build directory
    let build_dir = $"($app_folder)/build/elinux/arm64/release/bundle"

    if not ($build_dir | path exists) {
        print $"[ERROR] Build directory not found at ($build_dir)"
        print "[ERROR] Make sure you've run 'flutter-elinux build elinux --release' first"
        exit 1
    }

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
    let deb_filename = $"($pkg_name)_($pkg_version)_($pkg_arch).deb"
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