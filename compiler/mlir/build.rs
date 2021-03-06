extern crate cc;
extern crate walkdir;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::{DirEntry, WalkDir};

const ENV_LLVM_PREFIX: &'static str = "LLVM_PREFIX";
const ENV_LLVM_BUILD_STATIC: &'static str = "LLVM_BUILD_STATIC";
const ENV_LLVM_LINK_LLVM_DYLIB: &'static str = "LLVM_LINK_LLVM_DYLIB";
const ENV_LUMEN_LLVM_LTO: &'static str = "LUMEN_LLVM_LTO";

fn main() {
    let cwd = env::current_dir().unwrap();
    let llvm_prefix = detect_llvm_prefix();

    println!("cargo:rerun-if-env-changed={}", ENV_LLVM_PREFIX);
    println!("cargo:rerun-if-env-changed={}", ENV_LLVM_BUILD_STATIC);
    println!("cargo:rerun-if-env-changed={}", ENV_LUMEN_LLVM_LTO);

    rerun_if_changed_anything_in_dir(&cwd.join("c_src"));

    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut cfg = cc::Build::new();
    cfg.warnings(false);

    // Compile our MLIR shims with the same flags as LLVM
    let cxxflags = env::var("DEP_LUMEN_LLVM_CORE_CXXFLAGS").unwrap();
    for flag in cxxflags.split(";") {
        cfg.flag(flag);
    }

    match env::var_os(ENV_LUMEN_LLVM_LTO) {
        Some(val) if val == "ON" => {
            cfg.flag("-flto=thin");
        }
        _ => {}
    }

    if env::var_os("LLVM_NDEBUG").is_some() {
        cfg.define("NDEBUG", None);
        cfg.debug(false);
    }

    let include_dir = outdir.join("include");
    let include_mlir_dir = include_dir.join("lumen/mlir");
    fs::create_dir_all(include_mlir_dir.as_path()).unwrap();
    for entry in fs::read_dir(cwd.join("c_src/include/lumen/mlir")).unwrap() {
        let entry = entry.unwrap();
        let file = entry.path();
        let basename = entry.file_name();
        fs::copy(file, include_mlir_dir.join(basename)).unwrap();
    }

    println!("cargo:include={}", include_dir.display());

    let lumen_llvm_include_dir = env::var("DEP_LUMEN_LLVM_CORE_INCLUDE").unwrap();
    cfg.file("c_src/MLIR.cpp")
       .file("c_src/Diagnostics.cpp")
       .file("c_src/ModuleReader.cpp")
       .file("c_src/ModuleWriter.cpp")
       .file("c_src/ConvertToLLVM.cpp")
       .include(llvm_prefix.join("include"))
       .include(lumen_llvm_include_dir)
       .include(include_dir)
       .cpp(true)
       .cpp_link_stdlib(None) // we handle this below
       .compile("lumen_mlir_core");

    link_libs(&[
        "MLIRAnalysis",
        "MLIRAffineOps",
        "MLIRCallInterfaces",
        "MLIRControlFlowInterfaces",
        "MLIRCopyOpInterface",
        "MLIRDerivedAttributeOpInterface",
        "MLIRDialect",
        "MLIREDSC",
        "MLIRIR",
        "MLIRInferTypeOpInterface",
        "MLIRLLVMIR",
        "MLIRLLVMIRTransforms",
        "MLIRLoopAnalysis",
        "MLIRLoopLikeInterface",
        "MLIROpenMP",
        "MLIRParser",
        "MLIRPass",
        "MLIRSideEffectInterfaces",
        "MLIRStandardOps",
        "MLIRStandardOpsTransforms",
        "MLIRStandardToLLVM",
        "MLIRSupport",
        "MLIRTargetLLVMIR",
        "MLIRTargetLLVMIRModuleTranslation",
        "MLIRTransformUtils",
        "MLIRTransforms",
        "MLIRTranslation",
    ]);

    let ldflags = env::var("DEP_LUMEN_LLVM_CORE_LDFLAGS").unwrap();
    for flag in ldflags.split(";") {
        println!("cargo:rustc-link-search=native={}", flag);
    }
}

pub fn output(cmd: &mut Command) -> String {
    let output = match cmd.stderr(Stdio::inherit()).output() {
        Ok(status) => status,
        Err(e) => fail(&format!(
            "failed to execute command: {:?}\nerror: {}",
            cmd, e
        )),
    };
    if !output.status.success() {
        panic!(
            "command did not execute successfully: {:?}\n\
             expected success, got: {}",
            cmd, output.status
        );
    }
    String::from_utf8(output.stdout).unwrap()
}

fn rerun_if_changed_anything_in_dir(dir: &Path) {
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !ignore_changes(e)) {
        let entry = entry.unwrap();
        if !entry.file_type().is_dir() {
            let path = entry.path();
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

fn ignore_changes(entry: &DirEntry) -> bool {
    let ty = entry.file_type();
    if ty.is_dir() {
        return false;
    }
    let path = entry.path();
    if path.starts_with(".") {
        return true;
    }
    false
}

fn link_libs(libs: &[&str]) {
    match env::var_os(ENV_LLVM_BUILD_STATIC) {
        Some(val) if val == "ON" => link_libs_static(libs),
        _ => link_libs_dylib(libs),
    }
}

#[inline]
fn link_libs_static(libs: &[&str]) {
    for lib in libs {
        link_lib_static(lib);
    }
}

#[inline]
fn link_libs_dylib(libs: &[&str]) {
    let llvm_link_llvm_dylib = env::var(ENV_LLVM_LINK_LLVM_DYLIB).unwrap_or("OFF".to_owned());
    if llvm_link_llvm_dylib == "ON" {
        link_lib_dylib("MLIR");
    } else {
        for lib in libs {
            link_lib_dylib(lib);
        }
    }
}

#[inline]
fn link_lib_static(lib: &str) {
    println!("cargo:rustc-link-lib=static={}", lib);
}

#[inline]
fn link_lib_dylib(lib: &str) {
    println!("cargo:rustc-link-lib=dylib={}", lib);
}

fn detect_llvm_prefix() -> PathBuf {
    if let Ok(prefix) = env::var(ENV_LLVM_PREFIX) {
        return PathBuf::from(prefix);
    }

    if let Ok(llvm_config) = which::which("llvm-config") {
        let mut cmd = Command::new(llvm_config);
        cmd.arg("--prefix");
        return PathBuf::from(output(&mut cmd));
    }

    let mut llvm_prefix = env::var("XDG_DATA_HOME")
        .map(|s| PathBuf::from(s))
        .unwrap_or_else(|_| {
            let mut home = PathBuf::from(env::var("HOME").expect("HOME not defined"));
            home.push(".local/share");
            home
        });
    llvm_prefix.push("llvm");
    if llvm_prefix.exists() {
        // Make sure its actually the prefix and not a root
        let llvm_bin = llvm_prefix.as_path().join("bin");
        if llvm_bin.exists() {
            return llvm_prefix;
        }
        let lumen = llvm_prefix.as_path().join("lumen");
        if lumen.exists() {
            return lumen.to_path_buf();
        }
    }

    fail("LLVM_PREFIX is not defined and unable to locate LLVM to build with");
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
