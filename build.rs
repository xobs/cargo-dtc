use std::{fs::File, io::Write};

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
    let mut version_file = File::create("dtc/version_gen.h").unwrap();
    writeln!(version_file, "#define DTC_VERSION \"DTC rust\"").unwrap();
    drop(version_file);

    let mut build = cc::Build::new();

    build
        .include("dtc/include/libfdt")
        .include("dtc/include")
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
