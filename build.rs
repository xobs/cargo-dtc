use std::{fs::File, io::Write, str::FromStr};

const DTC_FILES: &[&str] = &[
    "dtc/src/checks.c",
    "dtc/src/data.c",
    "dtc/src/dtc.c",
    "dtc/src/flattree.c",
    "dtc/src/fstree.c",
    "dtc/src/livetree.c",
    "dtc/src/srcpos.c",
    "dtc/src/treesource.c",
    "dtc/src/util.c",
    "dtc/src/dtc-lexer.lex.c",
    "dtc/src/dtc-parser.tab.c",
];

fn main() {
    let build_target_dir = std::env::var("OUT_DIR").unwrap();
    let mut version_file = std::path::PathBuf::from_str(build_target_dir.as_str()).unwrap();
    version_file.push("version_gen.h");
    let mut version_file = File::create(&version_file).unwrap();
    writeln!(version_file, "#define DTC_VERSION \"{} (rust)\"", env!("CARGO_PKG_VERSION")).unwrap();
    drop(version_file);

    let mut build = cc::Build::new();

    for c_src in DTC_FILES {
        println!("cargo:rerun-if-changed={}", c_src);
    }

    let include_dir = std::fs::read_dir("dtc/include").unwrap();
    for entry in include_dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
        } else if path.is_dir() {
            let dir = std::fs::read_dir(path).unwrap();
            for entry in dir {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
                }
            }
        }
    }

    build
        .include("dtc/include/libfdt")
        .include("dtc/include")
        .include("target")
        .files(DTC_FILES)
        .warnings(false)
        .define("NO_YAML", None);

    if cfg!(target_env = "msvc") {
        build.define("strcasecmp", Some("_stricmp"));
        build.file("dtc/src/getopt.c");
        build.include("dtc/include/win");
    } else {
        build.define("main(argc,argv)", Some("dtc_main(argc,argv)"));
    }

    build.compile("dtc");
}
