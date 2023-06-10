# gen-table
    gen-table is a tool that teaches mysql table structures to generate rust struct code,
    which is easy for developers to use and automatically manage table structure generation.

# install
    cargo install gen-table
# help
```
gen-table -h
Hello, welcome to gen-table
gen-table for mysql table structures convert rust code

Usage: gen-table [OPTIONS] --dsn <dsn> --table <table>

Options:
  -d, --dsn <dsn>          mysql dsn,eg:mysql://root:root1234@localhost/test
  -o, --out_dir <out_dir>  gen code output dir [default: src/model]
  -t, --table <table>      tables eg:orders,users
  -e, --enable_tab_name    whether to generate table_name method for struct
  -n, --no_null            whether to allow a field of null type
  -h, --help               Print help
  -V, --version            Print version
```

# how to use
```shell
gen-table -d=mysql://root:root1234@localhost/test -t=news,news_topics
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
