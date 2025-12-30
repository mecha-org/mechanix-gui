#!/usr/bin/env nu

# RPM Packaging Script for Flutter eLinux Apps
# Usage: nu package-rpm.nu <app-name> <output-dir>

def main [
    app_name: string,
    output_dir: string
] {
    print $"[INFO] Starting RPM packaging for ($app_name)"

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

    let dependencies = (
        ($app | get dependencies | default [])
        | each { |it| $it | str replace -r '\s*\(' ' ' | str replace -r '\)\s*' '' }
        | str join ", "
    )
    
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

    # Architecture
    let pkg_arch = (^uname -m | str trim)
    print $"[INFO] Architecture: ($pkg_arch)"

    # Package name
    let pkg_name = $"mechanix-($app_name)"

    let resolver_script = "../utils/resolve-next-version.nu" | path expand

    # Get version info from resolver
    let version_data = try {
        let resolver_cmd = [
            "nu"  # Call nu directly
            $resolver_script
            "--format" "rpm"
            "--name" $pkg_name
            "--upstream" $app_version
            "--base-url" "http://pkg.mecha.so"
        ]
        # Run the command and immediately parse the JSON output
        ^$resolver_cmd.0 ...($resolver_cmd | drop 1) | from json
    } catch {
        print "[WARN] Resolver failed, defaulting to revision 1"
        { next_revision: 1, upstream_version: $app_version }
    }
    
    let pkg_version = $version_data.upstream_version
    let pkg_release = ($version_data.next_revision | save - | into string) # Ensure it's a string
    
    print $"[INFO] RPM Version: ($pkg_version)"
    print $"[INFO] RPM Release: ($pkg_release)"

    # Build directory
    let build_dir = $"($app_folder)/build/elinux/arm64/release/bundle"
    if not ($build_dir | path exists) {
        error make { msg: $"Build directory not found at ($build_dir). Run 'flutter-elinux build elinux --release' first." }
    }

    # RPM build structure
    let rpmbuild_root = "rpmbuild"
    let buildroot_base = $"($rpmbuild_root)/BUILDROOT"
    let rpm_root = $"($buildroot_base)/($pkg_name)-($pkg_version)-($pkg_release).($pkg_arch)"

    print $"[INFO] Creating RPM build structure in ($rpmbuild_root)"

    mkdir $"($rpmbuild_root)/SPECS"
    mkdir $"($rpmbuild_root)/BUILD"
    mkdir $"($rpmbuild_root)/RPMS"
    mkdir $"($rpmbuild_root)/SOURCES"
    mkdir $"($rpmbuild_root)/SRPMS"
    mkdir $buildroot_base

    mkdir $"($rpm_root)/usr/bin"
    mkdir $"($rpm_root)/usr/share/mechanix/($pkg_name)/data"
    mkdir $"($rpm_root)/usr/share/mechanix/($pkg_name)/lib"
    mkdir $"($rpm_root)/usr/lib/($pkg_name)"

    # Copy binary
    let binary_src = $"($build_dir)/($binary_name)"
    let binary_dest = $"($rpm_root)/usr/bin/($binary_name)"
    if not ($binary_src | path exists) {
        error make { msg: $"Binary not found at ($binary_src)" }
    }
    cp $binary_src $binary_dest
    chmod 755 $binary_dest

    # Copy data
    let data_src = $"($build_dir)/data"
    if ($data_src | path exists) {
        for item in (ls $data_src) {
            let dest = $"($rpm_root)/usr/share/mechanix/($pkg_name)/data/(($item.name | path basename))"
            cp -r $item.name $dest
        }
    }

    # Copy lib
    let lib_src = $"($build_dir)/lib"
    if ($lib_src | path exists) {
        for item in (ls $lib_src) {
            let dest = $"($rpm_root)/usr/share/mechanix/($pkg_name)/lib/(($item.name | path basename))"
            cp -r $item.name $dest
        }
    }

    # Generate RPM spec
    let spec_content = $"Name:           ($pkg_name)
Version:        ($pkg_version)
Release:        ($pkg_release)
Summary:        ($app_description)

License:        Proprietary
URL:            https://mecha.so
BuildArch:      ($pkg_arch)

Requires:       ($dependencies)

%description
($app_description)

%files
/usr/bin/($binary_name)
/usr/share/mechanix/($pkg_name)/*
/usr/lib/($pkg_name)
"
    $spec_content | save -f $"($rpmbuild_root)/SPECS/($pkg_name).spec"

    # Create output directory
    let output_path = ($output_dir | path expand)
    mkdir $output_path

    # Build RPM
    print $"[INFO] Building RPM package..."
    let rpmbuild_result = (^rpmbuild -ba 
        --define $"_topdir ($rpmbuild_root | path expand)"
        --buildroot $"($rpm_root | path expand)"
        $"($rpmbuild_root)/SPECS/($pkg_name).spec"
        | complete
    )

    if $rpmbuild_result.exit_code != 0 {
        print "[ERROR] RPM build failed:"
        print $rpmbuild_result.stderr
        rm -rf $rpmbuild_root
        exit 1
    }

    print "[INFO] RPM build successful, locating built packages..."

    # Find and move all built RPMs - try the specific arch directory first
    let rpm_arch_dir = $"($rpmbuild_root)/RPMS/($pkg_arch)"
    
    let built_rpms = if ($rpm_arch_dir | path exists) {
        try {
            ls $rpm_arch_dir | where type == "file" and name =~ ".rpm$" | get name
        } catch {
            []
        }
    } else {
        []
    }
    
    if ($built_rpms | is-empty) {
        print "[ERROR] No RPM files found after build"
        print $"[DEBUG] Checked directory: ($rpm_arch_dir)"
        print "[DEBUG] RPM directory contents:"
        try {
            ls $"($rpmbuild_root)/RPMS"
        }
        rm -rf $rpmbuild_root
        exit 1
    }

    for rpm_path in $built_rpms {
        let rpm_name = ($rpm_path | path basename)
        cp $rpm_path $"($output_path)/($rpm_name)"
        print $"[INFO] Copied: ($rpm_name)"
    }

    # Cleanup
    rm -rf $rpmbuild_root

    print $"[SUCCESS] ✅ RPM package created in ($output_path)"
}