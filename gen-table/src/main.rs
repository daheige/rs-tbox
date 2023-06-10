mod engine;
mod sql_type;

use clap::{Arg, ArgAction, Command};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, welcome to gen-table");

    let matches = Command::new("clap demo")
        .version("0.1.0")
        .author("gen-table by daheige")
        .about("gen-table for mysql table structures convert rust code")
        .arg(
            Arg::new("dsn")
                .short('d')
                .long("dsn")
                .action(ArgAction::Set)
                .help("mysql dsn,eg:mysql://root:root1234@localhost/test")
                .required(true),
        )
        .arg(
            Arg::new("out_dir")
                .short('o')
                .long("out_dir")
                .help("gen code output dir")
                .default_value("src/model"),
        )
        .arg(
            Arg::new("table")
                .short('t')
                .long("table")
                .help("tables eg:orders,users")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("enable_tab_name")
                .short('e')
                .long("enable_tab_name")
                .help("whether to generate table_name method for struct")
                .default_value("true")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no_null_field")
                .short('n')
                .long("no_null")
                .help("whether to allow a field of null type")
                .default_value("false")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let dsn = matches.get_one::<String>("dsn").expect("dsn invalid");
    let out_dir = matches
        .get_one::<String>("out_dir")
        .expect("out_dir invalid");
    let enable_table_name = matches
        .get_one::<bool>("enable_tab_name")
        .expect("enable_tab_name invalid");
    let no_null_field = matches
        .get_one::<bool>("no_null_field")
        .expect("enable_tab_name invalid");
    let table = matches.get_one::<String>("table").expect("table invalid");

    println!(
        "tables:{} enable_table_name:{} no_null_field:{}",
        table, enable_table_name, no_null_field
    );

    let tables: Vec<&str> = table.split(",").collect();
    // fields are not allowed to be null
    let mut entry = engine::Engine::new(&dsn, &out_dir)
        .with_enable_tab_name(*enable_table_name)
        .with_no_null_field(*no_null_field);

    entry.gen_code(tables).await;

    Ok(())
}
