# gen-table
    gen-table is a tool that teaches mysql table structures to generate rust struct code,
    which is easy for developers to use and automatically manage table structure generation.

# install
```shell
    cargo install gen-table
```
or
```shell
cargo install --git https://github.com/daheige/rs-tbox
```

# help
```
gen-table -h
Hello, welcome to gen-table
gen-table for mysql table structures convert to rust code

Usage: gen-table [OPTIONS] --dsn <dsn> --table <table>

Options:
  -d, --dsn <dsn>
          mysql dsn,eg:mysql://root:root1234@localhost/test
  -o, --out_dir <out_dir>
          gen code output dir [default: src/model]
  -t, --table <table>
          tables eg:orders,users
  -e, --enable_tab_name <enable_tab_name>
          whether to generate table_name method for struct [default: true]
  -n, --no_null <no_null_field>
          whether to allow a field of null type [default: false]
  -s, --serde <is_serde>
          whether to use serde serialization and deserialization [default: false]
  -h, --help
          Print help
```

# how to use
```shell
gen-table -d=mysql://root:root1234@localhost/test -t=news,news_topics -o=src/model
Hello, welcome to gen-table
tables:news,news_topics enable_table_name:true no_null_field:false
gen tables:["news", "news_topics"] rust code
gen code for table:news
current field:created_at is null able,type:Duration
current field:updated_at is null able,type:Duration
current field:deleted_at is null able,type:Duration
current field:title is null able,type:String
current field:slug is null able,type:String
current field:content is null able,type:String
current field:status is null able,type:String
gen code for table:news finish
gen code for table:news_topics
gen code for table:news_topics finish
```

The generated rust code:
src/model/mod.rs
```
// code generated by gen-table. DO NOT EDIT!!!
pub mod news;
pub mod news_topics;

```

src/model/news.rs
```rust
// code generated by gen-table. DO NOT EDIT!!!
// gen code for news table.
use std::time::Duration;
// NEWS_TABLE for news table
const NEWS_TABLE: &str = "news";

// NewsEntity for news table
#[derive(Debug, Default)]
pub struct NewsEntity {
	pub id: i64,
	pub created_at: Option<Duration>,
	pub updated_at: Option<Duration>,
	pub deleted_at: Option<Duration>,
	pub title: Option<String>,
	pub slug: Option<String>,
	pub content: Option<String>,
	pub status: Option<String>,
}

// impl table_name method for NewsEntity
impl NewsEntity {
	pub fn table_name(&self) -> String {
		NEWS_TABLE.to_string()
	}
}
```

src/model/news_topics.rs
```rust
// code generated by gen-table. DO NOT EDIT!!!
// gen code for news_topics table.
// NEWS_TOPICS_TABLE for news_topics table
const NEWS_TOPICS_TABLE: &str = "news_topics";

// NewsTopicsEntity for news_topics table
#[derive(Debug, Default)]
pub struct NewsTopicsEntity {
	pub news_id: i64,
	pub topic_id: i64,
}

// impl table_name method for NewsTopicsEntity
impl NewsTopicsEntity {
	pub fn table_name(&self) -> String {
		NEWS_TOPICS_TABLE.to_string()
	}
}
```

# serde support
gen-table -d=mysql://root:root1234@localhost/test -t=news_topics -s=true
```rust
// code generated by gen-table. DO NOT EDIT!!!
// gen code for news_topics table.
use serde::{Deserialize, Serialize};

// NEWS_TOPICS_TABLE for news_topics table
const NEWS_TOPICS_TABLE: &str = "news_topics";

// NewsTopicsEntity for news_topics table
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewsTopicsEntity {
	pub news_id: i64,
	pub topic_id: i64,
}

// impl table_name method for NewsTopicsEntity
impl NewsTopicsEntity {
	pub fn table_name(&self) -> String {
		NEWS_TOPICS_TABLE.to_string()
	}
}
```

When the code is generated, you need to add the following to your Cargo.toml: 
(For the serde version, choose the corresponding version according to your project)

```rust
serde = { version = "1.0.196",features = ["derive"]}
serde_json = "1.0.114"
```
serde_json is only used when doing json serialization, and is generally not used.
I sincerely hope that the above content will be helpful to you,Thank you.
