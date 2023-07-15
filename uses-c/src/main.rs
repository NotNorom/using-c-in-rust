use uses_c;

fn main() {
    let number = unsafe { uses_c::give_number_pls() };

    println!("{number}");
}
