#!/usr/bin/env nu

def main [
    --format: string,
    --name: string,
    --upstream: string,
    --base-url: string = "http://pkg.mecha.so"
] {

    if $format not-in ["deb", "rpm"] {
        error make { msg: "format must be 'deb' or 'rpm'" }
    }

    # Retrieve credentials from environment
    let username = ($env.MECHA_PULP_USERNAME? | default "")
    let password = ($env.MECHA_PULP_PASSWORD? | default "")

    let endpoint = if $format == "deb" {
        $"($base_url)/pulp/api/v3/content/deb/packages/?package=($name)"
    } else {
        $"($base_url)/pulp/api/v3/content/rpm/packages/?name=($name)"
    }

    # Prepare Headers
    let headers = if ($username != "" and $password != "") {
        {
            Authorization: $"Basic (($"($username):($password)" | encode base64))"
            Accept: "application/json"
        }
    } else {
        { Accept: "application/json" }
    }

    # Fetch data from Pulp
    let response = (http get --headers $headers $endpoint)
    let results = ($response.results? | default [])

    # Calculate Versions
    let versions = (
        $results
        | each { |pkg|
            if $format == "deb" { 
                $pkg.version 
            } else { 
                $"($pkg.version)-($pkg.release)" 
            }
        }
    )

    # Filter for the specific upstream version and find the highest revision
    let revisions = (
        $versions
        | where { |v| $v | str starts-with $"($upstream)-" }
        | each { |v| 
            let parts = ($v | split row "-")
            if ($parts | length) >= 2 {
                $parts | last | into int
            } else {
                0
            }
          }
    )

    let current_max_rev = if ($revisions | is-empty) { 0 } else { $revisions | math max }
    let next_rev = $current_max_rev + 1
    
    # Construct the JSON output
    let output = {
        package_name: $name
        format: $format
        upstream_version: $upstream
        current_revision: $current_max_rev
        next_revision: $next_rev
        full_version: $"($upstream)-($next_rev)"
    }

    # Output as JSON for the CI to consume
    $output | to json
}