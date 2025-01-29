# Cargo Version Bump Script
# This script automatically bumps the patch version in Cargo.toml and publishes to crates.io

# Error handling preference
$ErrorActionPreference = "Stop"

try {
    # Read Cargo.toml and extract current version
    Write-Host "Reading Cargo.toml..."
    if (-not (Test-Path "Cargo.toml")) {
        throw "Cargo.toml not found in current directory"
    }

    $content = Get-Content -Path "Cargo.toml" -Raw
    if ($content -match 'version\s*=\s*"(.*)"') {
        $version = $matches[1]
        Write-Host "Current version: $version"
    } else {
        throw "Version not found in Cargo.toml"
    }

    # Split version and increment patch number
    $versionParts = $version -split '\.'
    if ($versionParts.Count -ne 3) {
        throw "Invalid version format. Expected 'major.minor.patch'"
    }

    $versionParts[2] = [int]$versionParts[2] + 1
    $newVersion = "$($versionParts[0]).$($versionParts[1]).$($versionParts[2])"
    Write-Host "New version: $newVersion"

    # Update Cargo.toml with new version
    Write-Host "Updating Cargo.toml..."
    $updatedContent = $content -replace "version\s*=\s*`"$version`"", "version = `"$newVersion`""
    Set-Content -Path "Cargo.toml" -Value $updatedContent

    # Git operations
    Write-Host "Committing changes..."
    git add .
    if ($LASTEXITCODE -ne 0) { throw "Git add failed" }

    git commit -m "bump version to $newVersion"
    if ($LASTEXITCODE -ne 0) { throw "Git commit failed" }

    Write-Host "Pushing to remote..."
    git push
    if ($LASTEXITCODE -ne 0) { throw "Git push failed" }

    # Publish to crates.io
    Write-Host "Publishing to crates.io..."
    cargo publish
    if ($LASTEXITCODE -ne 0) { throw "Cargo publish failed" }

    Write-Host "Successfully bumped version to $newVersion and published!"
} catch {
    Write-Error "Error: $_"
    exit 1
}