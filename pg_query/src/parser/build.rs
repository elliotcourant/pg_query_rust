extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("pg_query.c")
        .include("include")
        .compile("pg_query.a");
}