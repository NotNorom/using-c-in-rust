fn main() {
    // specify the directories where our compiled libraries are in
    let extra_link_dirs = ["../is-c/out"];

    for dir in extra_link_dirs {
        // according to https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search
        // we can use "cargo:rustc-link-search=[KIND=]PATH" to specify which directory to add
        // to the library search path. This basically translates to a "-L PATH" flag for the compiler.
        //
        // and according to https://doc.rust-lang.org/rustc/command-line-arguments.html#option-l-search-path
        // 
        // The kind of search path can optionally be specified with the form -L KIND=PATH where KIND may be one of:
        //   dependency — Only search for transitive dependencies in this directory.
        //   crate — Only search for this crate's direct dependencies in this directory.
        //   native — Only search for native libraries in this directory.
        //   framework — Only search for macOS frameworks in this directory.
        //   all — Search for all library kinds in this directory. This is the default if KIND is not specified.

        // That means we could also just leave out the "=native" part and would be fine as well.
        // buuuut the rust team was kind enough to provide us with the options so why not use them :)
        println!("cargo:rustc-link-search=native={dir}");
    }
}
