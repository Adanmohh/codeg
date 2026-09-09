#[cfg(feature = "tauri-runtime")]
#[path = "src/business_identity/native_acl.rs"]
#[allow(dead_code)]
mod native_acl;

fn main() {
    #[cfg(feature = "tauri-runtime")]
    {
        ensure_sidecar_placeholder();
        // An application manifest makes Tauri enforce ACL on custom commands.
        // Derive the host list from the actual registry so later registrations
        // cannot silently escape it. Generated files stay in this build output.
        let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"));
        let permissions = out.join("business-acl");
        std::fs::create_dir_all(&permissions).expect("create app ACL directory");
        let source = std::fs::read_to_string("src/lib.rs").expect("read command registry");
        std::fs::write(
            permissions.join("commands.toml"),
            native_acl::permissions(&source),
        )
        .expect("write app ACL");
        println!("cargo:rerun-if-changed=src/lib.rs");
        println!("cargo:rerun-if-changed=src/business_identity/native_acl.rs");
        let pattern = Box::leak(format!("{}/*.toml", permissions.display()).into_boxed_str());
        tauri_build::try_build(
            tauri_build::Attributes::new()
                .app_manifest(tauri_build::AppManifest::new().permissions_path_pattern(pattern)),
        )
        .expect("build app ACL");
        // Test the same generated manifests compiled into this app, isolated
        // from a different target's subsequent generated schema output.
        for name in ["acl-manifests.json", "capabilities.json"] {
            std::fs::copy(
                std::path::Path::new("gen/schemas").join(name),
                out.join(name),
            )
            .expect("snapshot generated ACL");
        }
    }
}

/// Tauri's bundler validates that every `bundle.externalBin` path resolves
/// to an existing file at build.rs time. The real `codeg-mcp` sidecar is
/// produced by `pnpm tauri:prepare-sidecars` (invoked from
/// `beforeBuildCommand` / `beforeDevCommand` and the CI release matrix) —
/// but plain `cargo check --features tauri-runtime` doesn't go through that
/// path, so without a backstop every contributor would hit
/// `resource path ... doesn't exist` on first compile.
///
/// We write a zero-byte placeholder when the sidecar is missing so
/// `cargo check` / clippy / rust-analyzer succeed. Production paths
/// overwrite the placeholder with the real binary before Tauri bundles it:
///   * `pnpm tauri build`  → `beforeBuildCommand` → `prepare-sidecars.mjs`
///   * release.yml         → explicit "Stage codeg-mcp sidecar" step
///   * `pnpm tauri dev`    → `beforeDevCommand` → `prepare-sidecars.mjs`
///
/// If you ever bypass those wrappers (e.g. invoking the Tauri CLI directly
/// without beforeBuildCommand) you'd ship the placeholder, so emit a
/// cargo:warning that surfaces in any compile log to make that loud.
#[cfg(feature = "tauri-runtime")]
fn ensure_sidecar_placeholder() {
    use std::fs;
    use std::path::PathBuf;

    let triple = std::env::var("TARGET").unwrap_or_default();
    if triple.is_empty() {
        return;
    }
    let ext = if triple.contains("windows") {
        ".exe"
    } else {
        ""
    };
    let dir = PathBuf::from("binaries");
    let path = dir.join(format!("codeg-mcp-{triple}{ext}"));

    println!("cargo:rerun-if-changed={}", path.display());

    let needs_placeholder = match fs::metadata(&path) {
        Ok(meta) => meta.len() == 0,
        Err(_) => true,
    };

    if needs_placeholder {
        if let Err(e) = fs::create_dir_all(&dir) {
            panic!("failed to create {}: {e}", dir.display());
        }
        if let Err(e) = fs::write(&path, b"") {
            panic!(
                "failed to write sidecar placeholder {}: {e}",
                path.display()
            );
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o755));
        }
        println!(
            "cargo:warning=codeg-mcp sidecar missing at {}; wrote 0-byte placeholder. \
             Run `pnpm tauri:prepare-sidecars` before `tauri build` to ship a working binary.",
            path.display()
        );
    }
}
