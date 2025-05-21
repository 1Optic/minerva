#[ctor::ctor]
fn common_setup() {
    println!("Common Cluster Setup");
}

#[ctor::ctor]
fn common_teardown() {
    println!("Common Cluster Teardown");
}
