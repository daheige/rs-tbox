use crate::sql_type;
use futures::TryStreamExt;
use sqlx;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

const LICENSE_HEADER: &str = "// code generated by gen-table. DO NOT EDIT!!!\n";

#[derive(Debug, Default)]
pub struct Engine {
    dsn: String,
    max_connections: u32, // Maximum number of connections. The default value is 50
    min_connections: u32, // Minimum number of connections. 6 by default
    max_lifetime: Duration, // Maximum life cycle, default 1800s

    // Life cycle of the idle connection. The default value is 600 seconds
    idle_timeout: Duration,
    connect_timeout: Duration, // The connection times out. The default value is 10 seconds

    enable_table_name_fn: bool, // whether to generate table_name method for struct,default:true
    no_null_field: bool,        // whether to allow a field of null type,default:false
    out_dir: String,            // rust file out_dir,default: src/model
}

#[derive(Debug, Default)]
pub struct ColumnEntry {
    pub table_name: String,
    pub field: String,
    pub data_type: String,
    pub field_desc: String,
    pub field_key: String,
    pub order_by: u64,
    pub is_nullable: String,
    pub max_length: Option<u64>,
    pub numeric_prec: Option<u64>,
    pub numeric_scale: Option<u64>,
    pub extra: String,
    pub field_comment: String,
}

fn capit(s: &str) -> String {
    format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
}

fn camel_case(s: &str) -> String {
    let arr: Vec<&str> = s.split("_").collect();
    let mut res: Vec<String> = Vec::new();
    for s in arr {
        let cur_str = capit(s);
        res.push(cur_str);
    }

    res.join("").to_string()
}

#[test]
fn it_works() {
    println!("{}", capit("news"));
    println!("{}", camel_case("news_topics"));
}

impl Engine {
    pub fn new(dsn: &str, out_dir: &str) -> Self {
        let mut s = Self {
            dsn: dsn.to_string(),
            enable_table_name_fn: true,
            max_connections: 50,
            min_connections: 6,
            max_lifetime: Duration::from_secs(1800),
            idle_timeout: Duration::from_secs(600),
            connect_timeout: Duration::from_secs(10),
            out_dir: out_dir.to_string(),
            ..Default::default()
        };
        if s.out_dir.is_empty() {
            s.out_dir = "src/model".to_string();
        }

        // create out_dir
        s.create_out_dir();

        // create mod.rs
        s.create_mod_file();

        s
    }

    pub fn with_enable_tab_name(mut self, enable: bool) -> Self {
        self.enable_table_name_fn = enable;
        self
    }

    pub fn with_no_null_field(mut self, no_null_field: bool) -> Self {
        self.no_null_field = no_null_field;
        self
    }

    /// gen rust model entity for tables
    pub async fn gen_code(&mut self, tables: Vec<&str>) {
        if tables.is_empty() {
            println!("No tables require code generation");
            return;
        }

        println!("gen tables:{:?} rust code", tables);

        // create mysql connection pool
        let pool = self
            .init_pool()
            .await
            .expect("mysql pool connection failed");
        if !self.check_table_exist(&pool, &tables).await {
            return;
        }

        let out_dir = Path::new(self.out_dir.as_str());
        // open mod.rs by append way
        let mut mod_file = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(out_dir.join("mod.rs"))
            .expect("create mod.rs failed");

        // write header for mod.rs
        mod_file
            .write(LICENSE_HEADER.as_bytes())
            .expect("write header failed");
        // gen code for table info
        for table in &tables {
            println!("gen code for table:{}", table);
            self.gen_table_code(&pool, &out_dir, &mut mod_file, table)
                .await;
            println!("gen code for table:{} finish", table);
        }
    }

    async fn check_table_exist(&self, pool: &sqlx::MySqlPool, tables: &Vec<&str>) -> bool {
        for table in tables {
            let records = self
                .get_columns(&pool, table)
                .await
                .expect("get table columns failed");
            if records.is_empty() {
                println!("current table:{} has no fields", table);
                return false;
            }
        }

        true
    }

    // create out_dir
    fn create_out_dir(&self) {
        let out_dir = Path::new(self.out_dir.as_str());
        if !out_dir.is_dir() {
            let _ = fs::create_dir_all(out_dir).expect("create out_dir failed");
        }
    }

    fn create_mod_file(&mut self) {
        let out_dir = Path::new(self.out_dir.as_str());
        fs::File::create(out_dir.join("mod.rs")).expect("create mod.rs failed");
    }

    fn get_query_fields(&self) -> Vec<&str> {
        let fields = vec![
            "TABLE_NAME as table_name",
            "COLUMN_NAME as field",
            "DATA_TYPE as data_type",
            "COLUMN_TYPE as field_desc",
            "COLUMN_KEY as field_key",
            "ORDINAL_POSITION as order_by",
            "IS_NULLABLE as is_nullable",
            "CHARACTER_MAXIMUM_LENGTH as max_length",
            "NUMERIC_PRECISION as numeric_prec",
            "NUMERIC_SCALE as numeric_scale",
            "EXTRA as extra",
            "COLUMN_COMMENT as field_comment",
        ];
        fields
    }

    async fn get_columns(
        &self,
        pool: &sqlx::MySqlPool,
        table: &str,
    ) -> Result<Vec<ColumnEntry>, sqlx::Error> {
        let fields = self.get_query_fields().join(",");
        let sql = format!(
            "SELECT {} FROM information_schema.COLUMNS WHERE table_schema = DATABASE() AND TABLE_NAME = ?",
            fields
        );

        let mut rows = sqlx::query(&sql).bind(table).fetch(pool);
        let mut records = Vec::new();
        while let Some(row) = rows.try_next().await? {
            let record = ColumnEntry {
                table_name: row.get("table_name"),
                field: row.get("field"),
                data_type: row.get("data_type"),
                field_desc: row.get("field_desc"),
                field_key: row.get("field_key"),
                order_by: row.get("order_by"),
                is_nullable: row.get("is_nullable"),
                max_length: row.get("max_length"),
                numeric_prec: row.get("numeric_prec"),
                numeric_scale: row.get("numeric_scale"),
                extra: row.get("extra"),
                field_comment: row.get("field_comment"),
            };
            records.push(record);
        }

        Ok(records)
    }

    // init mysql pool
    async fn init_pool(&self) -> Result<sqlx::MySqlPool, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(self.max_connections)
            .min_connections(self.min_connections)
            .max_lifetime(self.max_lifetime)
            .idle_timeout(self.idle_timeout)
            .acquire_timeout(self.connect_timeout)
            .connect(&self.dsn)
            .await?;
        Ok(pool)
    }

    fn get_no_null_fields(&self) -> Vec<String> {
        let v = vec![
            "i32".to_string(),
            "i64".to_string(),
            "f64".to_string(),
            "f32".to_string(),
            "String".to_string(),
        ];

        v
    }

    async fn gen_table_code(
        &self,
        pool: &sqlx::MySqlPool,
        out_dir: &Path,
        mod_file: &mut fs::File,
        table: &str,
    ) {
        // gen mod.rs
        mod_file
            .write(format!("pub mod {};\n", table).as_bytes())
            .unwrap();

        // create table eg:xxx.rs
        let mut file = fs::File::create(out_dir.join(table.to_string() + ".rs"))
            .expect("create mod.rs failed");

        // gen table module header
        file.write(format!("{}// gen code for {} table.\n", LICENSE_HEADER, table).as_bytes())
            .expect("write content failed");

        let records = self
            .get_columns(pool, table)
            .await
            .expect("get table columns failed");

        // import use std::time::Duration for time
        if self.check_import_duration(&records) {
            file.write(format!("{}", "use std::time::Duration;\n").as_bytes())
                .expect("import std::time::Duration failed");
        }
        file.write(format!("{}", "use serde::{Deserialize, Serialize};\n\n").as_bytes())
            .expect("import serde failed");

        let tab_upper = table.to_uppercase();
        // gen table const name
        file.write(
            format!(
                "// {}_TABLE for {} table\nconst {}_TABLE: &str = \"{}\";\n\n",
                tab_upper, table, tab_upper, table,
            )
            .as_bytes(),
        )
        .expect("write content failed");

        // gen struct code
        // struct start
        let table_entity_name = camel_case(table);
        file.write(format!("// {}Entity for {} table\n", table_entity_name, table).as_bytes())
            .expect("write content failed");

        file.write(format!("{}", "#[derive(Debug, Default, Serialize, Deserialize)]\n").as_bytes())
            .expect("gen struct derive failed");
        file.write(format!("pub struct {}Entity {}\n", table_entity_name, "{").as_bytes())
            .expect("write content failed");

        let no_null_fields = self.get_no_null_fields();
        for record in records {
            //  println!("current record: {:?}", record);
            let data_type = sql_type::get_type(&record.data_type);
            // println!("data_type:{}", data_type);

            let mut is_nullable = record.is_nullable;
            if self.no_null_field && no_null_fields.contains(&data_type) {
                is_nullable = "NO".to_string();
            }

            let mut row = format!("\tpub {}: {},\n", record.field.to_lowercase(), data_type);
            if is_nullable.eq("YES") {
                println!(
                    "current field:{} is null able,type:{}",
                    record.field, data_type
                );
                row = format!(
                    "\tpub {}: Option<{}>,\n",
                    record.field.to_lowercase(),
                    data_type
                );
            }

            file.write(row.as_bytes()).expect("gen struct field failed");
        }

        // struct end
        file.write(format!("{}\n\n", "}").as_bytes())
            .expect("write content failed");

        // impl table_name for struct
        if self.enable_table_name_fn {
            let tab_fn_tpl = self.get_tab_fn_tpl(table);
            file.write(tab_fn_tpl.as_bytes())
                .expect("gen table_name fn failed");
        }
    }

    fn check_import_duration(&self, records: &Vec<ColumnEntry>) -> bool {
        for record in records {
            let data_type = sql_type::get_type(&record.data_type);
            if data_type == "Duration" {
                return true;
            }
        }

        false
    }

    fn get_tab_fn_tpl(&self, table: &str) -> String {
        let table_entity_name = camel_case(table);
        let header = format!(
            "// impl table_name method for {}Entity\n",
            table_entity_name
        );

        let tab_fn_tpl = format!(
                "impl {}Entity {}\n\tpub fn table_name(&self) -> String {}\n\t\t{}_TABLE.to_string()\n\t{}\n{}",
                table_entity_name,"{","{",
                table.to_uppercase(),"}","}",
            );

        let rows = vec![header, tab_fn_tpl];
        rows.join("").to_string()
    }
}
