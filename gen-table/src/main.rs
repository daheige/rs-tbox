mod sql_type;

fn main() {
    println!("Hello, world!");
    println!(
        "mysql bigint for rust type:{}",
        sql_type::get_type("bigint")
    )
}
