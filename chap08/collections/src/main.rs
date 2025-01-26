
mod vector;
mod string;
mod hashmap;

fn main() {
    section("VECTOR", vector::main);
    section("STRING", string::main);
    section("HASHMAP", hashmap::main);
}

fn section<F>(tag: &str, f: F) 
where F: FnOnce() -> () {
    println!("=================== Section {tag} Start ======================");
    f();
    println!("=================== Section {tag} End   ======================");
}
