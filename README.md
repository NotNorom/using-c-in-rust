# Using some C code in Rust

The `is-c` folder contains c code that we want to use in rust. The `uses-c` folder contains a rust a library that uses the c code. Both statically and dynamically compiled libraries can be used.

## How to use (statically)

1. Have a rust library aptly names `uses-c
2. Have an `extern` block somewhere in the rust code, linking to a static c library. For example:
```rs
use libc::size_t;

#[link(name = "verynice", kind = "static")]
extern "C" {
    pub fn give_number_pls() -> size_t;
}

```

As we are linking against C code it might happen that we need to use C types in our rust code :shocking:.
That's why in your `Cargo.toml` you will need the `libc` crate and use the appropriate types.

3. Have a c library with the same name somewhere that contains a function with the same signature as we just specified in the rust code. For example:
```c
#include <stddef.h>

size_t give_number_pls() {
    return 69;
}

```

__nice__.

4. Compile the C code using `gcc -c verynice.c -o .verynice.o` to get an object file.

5. For whatever reason we cannot use an object file directly so now, we put it into an archive!
This is done using `ar rcs libverynice.a verynice.o`. An important step just happend: we added "lib" to the beginning of the file. Because reasons a library file does not start with "lib" will not be found in the linking step later. Why? I don't know.

6. With our c library now finally being done, let's help rust with finding it.
In the rust directory, right next to the `Cargo.toml` we create a `build.rs` file and put this in:
```rs
fn main() {
    println!("cargo:rustc-link-search=native=./path_to/dir_dir/with_the/compiled_libraries");
}

```

7. Finally we can compile the rust code using `cargo build` and even __run__ it using `cargo run`.
8. Doooone

## Provided makefiles
This repo has a few makefiles. If you run `make run` at the repository root first the c library is built, then the rust library and finally it is also run.
`make clean` exists to get rid of both the c and rust artifacts.