use std::env;
use std::process::Command;
use std::str;

// List of cfgs this build script is allowed to set. The list is needed to support check-cfg, as we
// need to know all the possible cfgs that this script will set. If you need to set another cfg
// make sure to add it to this list as well.
const ALLOWED_CFGS: &'static [&'static str] = &[
    "emscripten_new_stat_abi",
    "freebsd10",
    "freebsd11",
    "freebsd12",
    "freebsd13",
    "freebsd14",
    "freebsd15",
    "gnu_time64_abi",
    "libc_const_extern_fn",
    "libc_const_extern_fn_unstable",
    "libc_deny_warnings",
    "libc_thread_local",
];

// Extra values to allow for check-cfg.
const CHECK_CFG_EXTRA: &'static [(&'static str, &'static [&'static str])] = &[
    ("target_os", &["switch", "aix", "ohos", "hurd"]),
    ("target_env", &["illumos", "wasi", "aix", "ohos"]),
    (
        "target_arch",
        &["loongarch64", "mips32r6", "mips64r6", "csky"],
    ),
];

fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");

    let (rustc_minor_ver, is_nightly) = rustc_minor_nightly();
    let rustc_dep_of_std = env::var("CARGO_FEATURE_RUSTC_DEP_OF_STD").is_ok();
    let libc_ci = env::var("LIBC_CI").is_ok();
    let libc_check_cfg = env::var("LIBC_CHECK_CFG").is_ok();
    let const_extern_fn_cargo_feature = env::var("CARGO_FEATURE_CONST_EXTERN_FN").is_ok();

    // The ABI of libc used by std is backward compatible with FreeBSD 12.
    // The ABI of libc from crates.io is backward compatible with FreeBSD 11.
    //
    // On CI, we detect the actual FreeBSD version and match its ABI exactly,
    // running tests to ensure that the ABI is correct.
    match which_freebsd() {
        Some(10) if libc_ci => set_cfg("freebsd10"),
        Some(11) if libc_ci => set_cfg("freebsd11"),
        Some(12) if libc_ci || rustc_dep_of_std => set_cfg("freebsd12"),
        Some(13) if libc_ci => set_cfg("freebsd13"),
        Some(14) if libc_ci => set_cfg("freebsd14"),
        Some(15) if libc_ci => set_cfg("freebsd15"),
        Some(_) | None => set_cfg("freebsd11"),
    }

    match emcc_version_code() {
        Some(v) if (v >= 30142) => set_cfg("emscripten_new_stat_abi"),
        // Non-Emscripten or version < 3.1.42.
        Some(_) | None => (),
    }

    // Some ABIs need to redirect time related symbols to their time64 equivalents.
    if is_gnu_time64_abi() {
        set_cfg("gnu_time64_abi");
    }

    // On CI: deny all warnings
    if libc_ci {
        set_cfg("libc_deny_warnings");
    }

    // #[thread_local] is currently unstable
    if rustc_dep_of_std {
        set_cfg("libc_thread_local");
    }

    // Rust >= 1.62.0 allows to use `const_extern_fn` for "Rust" and "C".
    if rustc_minor_ver >= 62 {
        set_cfg("libc_const_extern_fn");
    } else {
        // Rust < 1.62.0 requires a crate feature and feature gate.
        if const_extern_fn_cargo_feature {
            if !is_nightly || rustc_minor_ver < 40 {
                panic!("const-extern-fn requires a nightly compiler >= 1.40");
            }
            set_cfg("libc_const_extern_fn_unstable");
            set_cfg("libc_const_extern_fn");
        }
    }

    // check-cfg is a nightly cargo/rustc feature to warn when unknown cfgs are used across the
    // codebase. libc can configure it if the appropriate environment variable is passed. Since
    // rust-lang/rust enforces it, this is useful when using a custom libc fork there.
    //
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#check-cfg
    if libc_check_cfg {
        for cfg in ALLOWED_CFGS {
            if rustc_minor_ver >= 75 {
                println!("cargo:rustc-check-cfg=cfg({})", cfg);
            } else {
                println!("cargo:rustc-check-cfg=values({})", cfg);
            }
        }
        for &(name, values) in CHECK_CFG_EXTRA {
            let values = values.join("\",\"");
            if rustc_minor_ver >= 75 {
                println!("cargo:rustc-check-cfg=cfg({},values(\"{}\"))", name, values);
            } else {
                println!("cargo:rustc-check-cfg=values({},\"{}\")", name, values);
            }
        }
    }
}

fn rustc_minor_nightly() -> (u32, bool) {
    macro_rules! otry {
        ($e:expr) => {
            match $e {
                Some(e) => e,
                None => panic!("Failed to get rustc version"),
            }
        };
    }

    let rustc = otry!(env::var_os("RUSTC"));
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .ok()
        .expect("Failed to get rustc version");
    if !output.status.success() {
        panic!(
            "failed to run rustc: {}",
            String::from_utf8_lossy(output.stderr.as_slice())
        );
    }

    let version = otry!(str::from_utf8(&output.stdout).ok());
    let mut pieces = version.split('.');

    if pieces.next() != Some("rustc 1") {
        panic!("Failed to get rustc version");
    }

    let minor = pieces.next();

    // If `rustc` was built from a tarball, its version string
    // will have neither a git hash nor a commit date
    // (e.g. "rustc 1.39.0"). Treat this case as non-nightly,
    // since a nightly build should either come from CI
    // or a git checkout
    let nightly_raw = otry!(pieces.next()).split('-').nth(1);
    let nightly = nightly_raw
        .map(|raw| raw.starts_with("dev") || raw.starts_with("nightly"))
        .unwrap_or(false);
    let minor = otry!(otry!(minor).parse().ok());

    (minor, nightly)
}

fn which_freebsd() -> Option<i32> {
    let output = std::process::Command::new("freebsd-version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    match &stdout {
        s if s.starts_with("10") => Some(10),
        s if s.starts_with("11") => Some(11),
        s if s.starts_with("12") => Some(12),
        s if s.starts_with("13") => Some(13),
        s if s.starts_with("14") => Some(14),
        s if s.starts_with("15") => Some(15),
        _ => None,
    }
}

fn emcc_version_code() -> Option<u64> {
    let output = std::process::Command::new("emcc")
        .arg("-dumpversion")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let version = String::from_utf8(output.stdout).ok()?;

    // Some Emscripten versions come with `-git` attached, so split the
    // version string also on the `-` char.
    let mut pieces = version.trim().split(|c| c == '.' || c == '-');

    let major = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let minor = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);
    let patch = pieces.next().and_then(|x| x.parse().ok()).unwrap_or(0);

    Some(major * 10000 + minor * 100 + patch)
}

fn set_cfg(cfg: &str) {
    if !ALLOWED_CFGS.contains(&cfg) {
        panic!("trying to set cfg {}, but it is not in ALLOWED_CFGS", cfg);
    }
    println!("cargo:rustc-cfg={}", cfg);
}

fn is_gnu_time64_abi() -> bool {
    match env::var("RUST_LIBC_TIME_BITS") {
        Ok(time_bits) => {
            if time_bits == "64" {
                return true;
            }
            if time_bits == "32" {
                return false;
            }
            if time_bits != "default" {
                panic!("Invalid value for RUST_LIBC_TIME_BITS");
            }
        }
        Err(_) => {}
    }
    match env::var("CARGO_CFG_TARGET_ENV") {
        Ok(target_env) => {
            if target_env != "gnu" {
                return false;
            }
        }
        Err(_) => panic!("CARGO_CFG_TARGET_ENV not set"),
    }
    match env::var("CARGO_CFG_TARGET_OS") {
        Ok(target_os) => {
            if target_os != "linux" {
                return false;
            }
        }
        Err(_) => panic!("CARGO_CFG_TARGET_OS not set"),
    }
    match env::var("CARGO_CFG_TARGET_POINTER_WIDTH") {
        Ok(bits) => {
            if bits == "64" {
                return false;
            }
            bits
        }
        Err(_) => panic!("CARGO_CFG_TARGET_POINTER_WIDTH not set"),
    };
    // At this point, we _know_ it is *-*-linux-gnu* with 32 bit
    // pointers. Some 64 bit arch still have 32 bit pointers though.
    match env::var("TARGET") {
        Ok(target) => {
            // x86_64-unknown-linux-gnux32 and similar
            if target.contains("x86_64") && target.contains("x32") {
                return false;
            }
        }
        Err(_) => panic!("TARGET not set"),
    }
    return true;
}
