use libc::size_t;

// The link attribute can take more than just the `name` parameter.


#[link(name = "verynice", kind = "static")]
extern "C" {
    pub fn give_number_pls() -> size_t;
}
