#!/usr/bin/env nu

# ============================================================================
# Configuration Constants
# ============================================================================
const INSTALL_PREFIX = "/usr"
const SHARE_BASE = "/usr/share/mechanix"
const DESKTOP_DIR = "/usr/share/applications"

const REQUIRED_METADATA_FIELDS = ["name", "binary", "folder"]
const METADATA_FILE = "packaging-metadata.yaml"
const VERSION_RESOLVER = "../utils/resolve-next-version.nu"

# ============================================================================
# Helper Functions
# ============================================================================

# Validates architecture, strips symbols, and copies to destination
def safe-copy [src: path, dest_dir: path, target_arch: string] {
    if not ($src | path exists) { return }
    
    let info = (^file -b $src)
    let is_elf = ($info | str contains "ELF")
    
    if $is_elf {
        let arch_tag = (if $target_arch == "x86_64" { "x86-64" } else { "aarch64" })
        if ($info !~ $arch_tag) {
            print $"(ansi yellow)[SKIP] ($src | path basename) is wrong architecture ($info)(ansi reset)"
            return
        }
        if ($info !~ "stripped") {
            print $"[INFO] Stripping ($src | path basename)..."
            try { ^strip $src } catch { print "Strip failed (non-critical)" }
        }
    }
    if not ($dest_dir | path exists) { mkdir $dest_dir }
    cp -r $src $dest_dir
}

# Validates that all required fields exist in metadata
def validate-metadata [app: record] {
    let app_columns = ($app | columns)
    let missing = ($REQUIRED_METADATA_FIELDS | where { |field| 
        not ($field in $app_columns)
    })
    
    if ($missing | is-not-empty) {
        error make { 
            msg: $"Missing required metadata fields: ($missing | str join ', ')" 
        }
    }
    
    # Validate folder exists
    if not ($app.folder | path exists) {
        error make {
            msg: $"Application folder does not exist: ($app.folder)"
        }
    }
}

# Loads and validates metadata
def load-metadata [app_name: string] {
    if not ($METADATA_FILE | path exists) {
        error make { msg: $"Metadata file not found: ($METADATA_FILE)" }
    }
    
    let metadata = (open $METADATA_FILE)
    let app = ($metadata.applications | where name == $app_name | first)
    
    if ($app | is-empty) {
        let available = ($metadata.applications | get name | str join ", ")
        error make { 
            msg: $"App '($app_name)' not found. Available: ($available)" 
        }
    }
    
    validate-metadata $app
    $app
}

# Loads pubspec and validates version format
def load-pubspec [app_folder: path] {
    let pubspec_path = ($app_folder | path join "pubspec.yaml")
    
    if not ($pubspec_path | path exists) {
        error make { msg: $"pubspec.yaml not found in ($app_folder)" }
    }
    
    let pubspec = (open $pubspec_path)
    
    if not ("version" in ($pubspec | columns)) {
        error make { msg: "No version field in pubspec.yaml" }
    }
    
    $pubspec
}

# Resolves build directory with better error messages
def resolve-build-directory [app_folder: path, pkg_arch: string] {
    let default_p = ($app_folder | path join "build" "elinux" $pkg_arch "release" "bundle")
    let alt_arch_name = (if $pkg_arch == "aarch64" { "arm64" } else { "x64" })
    let alt_p = ($app_folder | path join "build" "elinux" $alt_arch_name "release" "bundle")
    
    if ($default_p | path exists) { 
        print $"[INFO] Using build directory: ($default_p)"
        return $default_p
    } else if ($alt_p | path exists) { 
        print $"[INFO] Using alternate build directory: ($alt_p)"
        return $alt_p
    } else {
        let tried = [$default_p, $alt_p]
        error make { 
            msg: $"Build directory not found. Tried:\n  ($tried | str join '\n  ')" 
        }
    }
}

# Resolves version with fallback
def resolve-version [pkg_name: string, upstream_version: string] {
    let resolver_path = ($VERSION_RESOLVER | path expand)
    
    if not ($resolver_path | path exists) {
        print $"(ansi yellow)[WARN] Version resolver not found at: ($resolver_path)(ansi reset)"
        print "[INFO] Using default version: 1"
        return {
            upstream_version: $upstream_version,
            next_revision: "1"
        }
    }
    
    print $"[INFO] Running version resolver: ($resolver_path)"
    
    let result = (do -i {
        ^nu $resolver_path --format "rpm" --name $pkg_name --upstream $upstream_version 
        | complete
    })
    
    if $result.exit_code != 0 {
        print $"(ansi yellow)[WARN] Version resolver failed with exit code ($result.exit_code)(ansi reset)"
        if ($result.stderr | is-not-empty) {
            print $"[DEBUG] Error output: ($result.stderr)"
        }
        if ($result.stdout | is-not-empty) {
            print $"[DEBUG] Stdout: ($result.stdout)"
        }
        print "[INFO] Using default version: 1"
        return {
            upstream_version: $upstream_version,
            next_revision: "1"
        }
    }
    
    try {
        let parsed = ($result.stdout | from json)
        print $"[INFO] Resolved version: ($parsed.upstream_version)-($parsed.next_revision)"
        $parsed
    } catch {
        print $"(ansi yellow)[WARN] Version resolver output is not valid JSON(ansi reset)"
        print $"[DEBUG] Raw output: ($result.stdout)"
        print "[INFO] Using default version: 1"
        {
            upstream_version: $upstream_version,
            next_revision: "1"
        }
    }
}

# Validates that binary exists in build directory
def validate-binary [build_dir: path, binary_name: string] {
    let binary_path = ($build_dir | path join $binary_name)
    
    if not ($binary_path | path exists) {
        error make {
            msg: $"Binary not found: ($binary_path)\nAvailable files: (ls $build_dir | get name | str join ', ')"
        }
    }
    
    $binary_path
}

# Collects and validates artifacts
def collect-artifacts [
    app: record,
    build_dir: path,
    rpm_root: path,
    pkg_name: string,
    pkg_arch: string
] {
    print "[INFO] Collecting artifacts..."
    
    # Binary (required)
    let binary_path = (validate-binary $build_dir $app.binary)
    safe-copy $binary_path $"($rpm_root)($INSTALL_PREFIX)/bin/" $pkg_arch
    
    # Desktop file (optional)
    let desktop_file = $"org.mechanix.($app.name).desktop"
    let desktop_src = ($app.folder | path join $desktop_file)
    let desktop_installed = if ($desktop_src | path exists) {
        print $"[INFO] Installing desktop file: ($desktop_file)"
        let desktop_dest = $"($rpm_root)($DESKTOP_DIR)/"
        mkdir $desktop_dest
        cp $desktop_src $desktop_dest
        true
    } else {
        print $"[INFO] No desktop file found: ($desktop_file)"
        false
    }
    
    # Libraries (only bundle app-specific libs, not Flutter runtime)
    let lib_src = ($build_dir | path join "lib")
    let lib_dest = $"($rpm_root)($SHARE_BASE)/($pkg_name)/lib/"
    
    if ($lib_src | path exists) {
        # Filter to only app-specific libraries (libapp.so, etc.)
        # Exclude Flutter runtime libraries as they come from dependencies
        let flutter_libs = ["libflutter_engine.so", "libflutter_elinux_wayland.so"]
        let lib_files = (ls $lib_src | where name !~ "(libflutter_engine|libflutter_elinux)")
        
        if ($lib_files | is-not-empty) {
            print $"[INFO] Installing ($lib_files | length) app-specific libraries"
            $lib_files | each { |item| safe-copy $item.name $lib_dest $pkg_arch }
        } else {
            print "[INFO] No app-specific libraries to bundle"
        }
        
        # Check if Flutter libraries exist (for informational purposes)
        let skipped = (ls $lib_src | where name =~ "(libflutter_engine|libflutter_elinux)")
        if ($skipped | is-not-empty) {
            let skipped_count = ($skipped | length)
            print $"[INFO] Skipping ($skipped_count) Flutter runtime libraries \(provided by dependencies\)"
        }
    } else {
        print $"(ansi yellow)[WARN] No lib directory found at ($lib_src)(ansi reset)"
    }
    
    # Data (optional)
    let data_src = ($build_dir | path join "data")
    if ($data_src | path exists) {
        let data_dest = $"($rpm_root)($SHARE_BASE)/($pkg_name)/data/"
        mkdir $data_dest
        let data_files = (ls $data_src)
        print $"[INFO] Installing ($data_files | length) data items"
        $data_files | each { |item| cp -r $item.name $data_dest }
    } else {
        print "[INFO] No data directory found"
    }
    
    { desktop_installed: $desktop_installed }
}

# Generates RPM spec file
def generate-spec [
    pkg_name: string,
    version_data: record,
    pkg_arch: string,
    app: record,
    pubspec: record,
    desktop_file: string,
    desktop_installed: bool
] {
    print "[INFO] Generating RPM spec file..."
    
    let dependencies = ($app | get -o dependencies | default [])
    let app_license = ($app | get -o license | default "MIT")
    let description = ($pubspec | get -o description | default "Mechanix Application")
    let maintainer = ($app | get -o maintainer | default "Mechanix Team <team@mecha.so>")
    
    mut spec_lines = [
        $"Name: ($pkg_name)"
        $"Version: ($version_data.upstream_version)"
        $"Release: ($version_data.next_revision)"
        $"Summary: ($description)"
        $"License: ($app_license)"
        $"Packager: ($maintainer)"
        "URL: https://mecha.so"
        $"BuildArch: ($pkg_arch)"
        "AutoReqProv: no"  # Disable automatic dependency detection - we manage dependencies manually
    ]
    
    # Add dependencies ONLY from metadata (not auto-detected)
    if ($dependencies | is-not-empty) {
        print $"[INFO] Adding ($dependencies | length) package dependencies from metadata"
        for dep in $dependencies {
            $spec_lines = ($spec_lines | append $"Requires: ($dep)")
        }
    } else {
        print $"(ansi yellow)[WARN] No dependencies specified in metadata(ansi reset)"
    }
    
    $spec_lines = ($spec_lines | append [
        ""
        "%description"
        $description
        ""
        "%post"
        "/usr/bin/update-desktop-database &> /dev/null || :"
        ""
        "%postun"
        "/usr/bin/update-desktop-database &> /dev/null || :"
        ""
        "%files"
        $"($INSTALL_PREFIX)/bin/($app.binary)"
    ])
    
    # Only add lib directory if app-specific libs exist
    # Check if directory will exist after filtering out Flutter libs
    $spec_lines = ($spec_lines | append $"%dir ($SHARE_BASE)/($pkg_name)")
    $spec_lines = ($spec_lines | append $"($SHARE_BASE)/($pkg_name)/*")
    
    if $desktop_installed {
        $spec_lines = ($spec_lines | append $"($DESKTOP_DIR)/($desktop_file)")
    }
    
    $spec_lines | str join "\n"
}

# Builds RPM package
def build-rpm [rpmbuild_root: path, rpm_root: path, pkg_name: string, spec_file: path] {
    print "[INFO] Building RPM package..."
    
    let build_res = (^rpmbuild 
        -ba 
        --define $"_topdir ($rpmbuild_root)" 
        --buildroot $rpm_root 
        $spec_file 
        | complete
    )
    
    if $build_res.exit_code != 0 {
        print $"(ansi red)[ERROR] RPM build failed:(ansi reset)"
        print $build_res.stderr
        error make { msg: "RPM build failed" }
    }
    
    print $build_res.stdout
}

# Exports built RPMs to output directory
def export-rpms [rpmbuild_root: path, output_dir: string] {
    let out = ($output_dir | path expand)
    if not ($out | path exists) { mkdir $out }
    
    let rpm_files = (glob $"($rpmbuild_root)/RPMS/**/*.rpm")
    
    if ($rpm_files | is-empty) {
        error make { msg: "No RPM files found after build" }
    }
    
    for f in $rpm_files {
        cp $f $out
        print $"\n(ansi green)✓ Created: ($f | path basename)(ansi reset)"
        print $"(ansi cyan)--- Package Contents ---(ansi reset)"
        ^rpm -qlp $f
    }
}

# ============================================================================
# Main Entry Point
# ============================================================================
def main [
    app_name: string,      # Application name from metadata
    output_dir: string,    # Output directory for built RPMs
    --clean-only           # Only clean build artifacts, don't build
] {
    print $"(ansi blue)========================================(ansi reset)"
    print $"(ansi blue)RPM Packaging for ($app_name)(ansi reset)"
    print $"(ansi blue)========================================(ansi reset)\n"
    
    # Setup rpmbuild root
    let rpmbuild_root = ($"($env.PWD)/rpmbuild" | path expand)
    
    if $clean_only {
        if ($rpmbuild_root | path exists) {
            print "[INFO] Cleaning rpmbuild directory..."
            rm -rf $rpmbuild_root
            print $"(ansi green)✓ Cleaned(ansi reset)"
        } else {
            print "[INFO] Nothing to clean"
        }
        return
    }
    
    # Clean previous builds
    if ($rpmbuild_root | path exists) {
        print "[INFO] Cleaning previous build..."
        rm -rf $rpmbuild_root
    }
    
    # Load and validate metadata
    print $"[INFO] Loading metadata for ($app_name)..."
    let app = (load-metadata $app_name)
    
    # Load pubspec
    let pubspec = (load-pubspec $app.folder)
    let upstream_version = ($pubspec.version | split row "+" | first)
    
    # Determine architecture
    let pkg_arch = (^uname -m | str trim)
    let pkg_name = $"mechanix-($app_name)"
    
    print $"[INFO] Package: ($pkg_name)"
    print $"[INFO] Architecture: ($pkg_arch)"
    print $"[INFO] Upstream version: ($upstream_version)"
    
    # Resolve build directory
    let build_dir = (resolve-build-directory $app.folder $pkg_arch)
    
    # Resolve version
    let version_data = (resolve-version $pkg_name $upstream_version)
    
    # Setup buildroot structure
    print "[INFO] Setting up buildroot..."
    mkdir $"($rpmbuild_root)/SPECS"
    let rpm_root = $"($rpmbuild_root)/BUILDROOT/($pkg_name)-($version_data.upstream_version)-($version_data.next_revision).($pkg_arch)"
    
    # Collect artifacts
    let artifacts = (collect-artifacts $app $build_dir $rpm_root $pkg_name $pkg_arch)
    
    # Generate spec file
    let desktop_file = $"org.mechanix.($app_name).desktop"
    let spec_content = (generate-spec 
        $pkg_name 
        $version_data 
        $pkg_arch 
        $app 
        $pubspec 
        $desktop_file 
        $artifacts.desktop_installed
    )
    
    let spec_file = $"($rpmbuild_root)/SPECS/($pkg_name).spec"
    $spec_content | save -f $spec_file
    print $"[INFO] Spec file written to: ($spec_file)"
    
    # Build RPM
    build-rpm $rpmbuild_root $rpm_root $pkg_name $spec_file
    
    # Export to output directory
    export-rpms $rpmbuild_root $output_dir
    
    # Cleanup
    print "\n[INFO] Cleaning up build directory..."
    rm -rf $rpmbuild_root
    
    print $"\n(ansi green)========================================(ansi reset)"
    print $"(ansi green)✓ Packaging complete!(ansi reset)"
    print $"(ansi green)========================================(ansi reset)"
}
