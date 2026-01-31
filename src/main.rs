use custom_hashmap::AssemblyHash;

fn main() {
    let s = "hello";
    let hash = s.assembly_hash();
    println!("Hash of '{}': {}", s, hash);

    let short = "ab"; // Too short (< 4 bytes)
    let hash_err = short.assembly_hash();
    println!("Hash of '{}' (error case): {}", short, hash_err);
}
