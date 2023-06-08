use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Engine {
    dsn: String,
    is_output_cmd: bool,
    enable_table_name_fn: bool,
    no_null_field: bool,
    out_dir: String,
}

impl Engine {
    pub fn new(dsn: &str, out_dir: &str) -> Self {
        let s = Self {
            dsn: dsn.to_string(),
            out_dir: out_dir.to_string(),
            ..Default::default()
        };

        s.check_out_dir();
        s.connect_db();

        s
    }

    // create out_dir
    fn check_out_dir(&self) {
        println!("{}", self.out_dir);
        let out_dir = Path::new(self.out_dir.as_str());
        if !out_dir.is_dir() {
            let _ = fs::create_dir_all(out_dir).expect("create out_dir failed");
        }
    }

    fn connect_db(&self) {
        println!("{}", self.dsn);
    }

    /// gen rust model entity for tables
    pub fn gen_code(&self, tables: Vec<&str>) {
        println!("gen tables:{:?} rust code", tables);
    }
}
