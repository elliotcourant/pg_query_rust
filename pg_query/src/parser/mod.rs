extern crate libc;

#[link(name = "pg_query")]
extern {
    fn pg_query_init();
}

pub fn init() {
    unsafe { pg_query_init() };
}