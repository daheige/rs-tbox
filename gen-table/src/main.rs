mod engine;
mod sql_type;

fn main() {
    println!("Hello, world!");
    println!(
        "mysql bigint for rust type:{}",
        sql_type::get_type("bigint")
    );

    let dsn = "mysql://root:root1234@localhost/test";
    let entry = engine::Engine::new(dsn, "src/model");
    entry.gen_code(vec!["orders"]);
}
