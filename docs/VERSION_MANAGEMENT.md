# Version Management

This document describes the version management process for the Rust EtherNet/IP library.

## Version Scheme

This project follows [Semantic Versioning](https://semver.org/) (SemVer):

- **MAJOR.MINOR.PATCH** (e.g., 0.2.0)
- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality in a backwards compatible manner
- **PATCH**: Backwards compatible bug fixes

## Current Version: 0.2.0

### Version History

| Version | Release Date | Status | Notes |
|---------|-------------|--------|-------|
| 0.2.0   | 2025-01-15  | Current | Enhanced C# FFI, multi-PLC support, UDT support |
| 0.1.0   | 2025-01-01  | Legacy  | Initial release |

## Files That Contain Version Information

The following files contain version information and must be updated when releasing a new version:

### Core Rust Files
- `Cargo.toml` - Main package version
- `src/version.rs` - Version constants
- `VERSION` - Simple version file

### C# Project Files
- `csharp/RustEtherNetIp/RustEtherNetIp.csproj` - Main C# library
- `examples/WpfExample/WpfExample.csproj` - WPF example
- `examples/WinFormsExample/WinFormsExample.csproj` - WinForms example
- `examples/AspNetExample/AspNetExample.csproj` - ASP.NET example

### Documentation Files
- `CHANGELOG.md` - Release notes and version history
- `README.md` - Quick start dependency version

## Automated Version Management

### Using the Update Script

Use the PowerShell script to automatically update versions across all files:

```powershell
# Update version only
.\scripts\update-version.ps1 -Version "0.3.0"

# Update version and add changelog entry
.\scripts\update-version.ps1 -Version "0.3.0" -UpdateChangelog
```

### Manual Version Update Process

If you prefer to update manually:

1. **Update Cargo.toml**
   ```toml
   version = "0.3.0"
   ```

2. **Update src/version.rs**
   ```rust
   pub const MAJOR_VERSION: u32 = 0;
   pub const MINOR_VERSION: u32 = 3;
   pub const PATCH_VERSION: u32 = 0;
   ```

3. **Update VERSION file**
   ```
   0.3.0
   ```

4. **Update C# project files**
   ```xml
   <Version>0.3.0</Version>
   <AssemblyVersion>0.3.0.0</AssemblyVersion>
   <FileVersion>0.3.0.0</FileVersion>
   ```

5. **Update CHANGELOG.md**
   Add new version entry with release date and changes.

## Release Process

### 1. Pre-Release Checklist

- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Performance benchmarks are current
- [ ] Examples work with new version
- [ ] CHANGELOG.md is updated with all changes

### 2. Version Update

```powershell
# Update version across all files
.\scripts\update-version.ps1 -Version "X.Y.Z" -UpdateChangelog

# Review and edit CHANGELOG.md with actual changes
# Edit the generated changelog entry to include real changes
```

### 3. Build and Test

```bash
# Build Rust library
cargo build --release

# Test Rust library
cargo test

# Build C# examples
dotnet build csharp/RustEtherNetIp/RustEtherNetIp.csproj
dotnet build examples/WpfExample/WpfExample.csproj
dotnet build examples/WinFormsExample/WinFormsExample.csproj
dotnet build examples/AspNetExample/AspNetExample.csproj

# Run integration tests
cargo test --test integration_tests
```

### 4. Commit and Tag

```bash
# Commit version changes
git add .
git commit -m "Release version X.Y.Z"

# Create and push tag
git tag vX.Y.Z
git push origin main
git push origin vX.Y.Z
```

### 5. Publish (when ready)

```bash
# Publish to crates.io
cargo publish

# Publish C# package to NuGet (if configured)
dotnet pack csharp/RustEtherNetIp/RustEtherNetIp.csproj
dotnet nuget push bin/Release/RustEtherNetIp.X.Y.Z.nupkg
```

## Version Planning

### Upcoming Versions

#### v0.3.0 (Q2 2025)
- Program Scope Tags support
- Real-time subscriptions
- Advanced connection pooling
- ControlLogix support

#### v0.4.0 (Q3 2025)
- Security features
- Advanced diagnostics
- Cloud integration capabilities

#### v0.5.0 (Q4 2025)
- Advanced analytics
- Multi-PLC coordination
- Production-ready release

## Version Compatibility

### Rust Library
- **0.2.x**: Compatible with Rust 1.70+
- **0.1.x**: Compatible with Rust 1.70+

### C# Bindings
- **0.2.x**: Compatible with .NET 6.0+, .NET 9.0+
- **0.1.x**: Compatible with .NET 6.0+

### PLC Compatibility
- **All versions**: CompactLogix L1x-L5x series
- **0.3.0+**: ControlLogix L6x-L7x series (planned)

## Breaking Changes Policy

### Major Version (X.0.0)
- API breaking changes allowed
- Migration guide provided
- Deprecation warnings in previous minor versions

### Minor Version (0.X.0)
- New features only
- Backwards compatible
- May deprecate features (with warnings)

### Patch Version (0.0.X)
- Bug fixes only
- Backwards compatible
- No new features

## Support Policy

- **Current version**: Full support
- **Previous minor version**: Security fixes only
- **Older versions**: Community support only

## Troubleshooting Version Issues

### Common Issues

1. **Version mismatch between Rust and C#**
   - Ensure all project files have the same version
   - Rebuild all projects after version update

2. **Git tag conflicts**
   - Check existing tags: `git tag -l`
   - Delete conflicting tag: `git tag -d vX.Y.Z`

3. **Build failures after version update**
   - Clean build: `cargo clean && dotnet clean`
   - Rebuild: `cargo build --release && dotnet build`

### Verification Commands

```bash
# Check current version in all files
grep -r "0\.2\.0" . --include="*.toml" --include="*.csproj" --include="*.rs" --include="*.md"

# Verify git tags
git tag -l | grep "v0"

# Check build status
cargo check && dotnet build --no-restore
``` 