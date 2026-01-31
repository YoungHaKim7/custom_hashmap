# Source
- [(230311)Faster than Rust and C++: the PERFECT hash table | strager](https://youtu.be/DMQ_HcNSOAI?si=jKP1DEvRvb8uqozz)

# Result

```bash
Hash of 'hello': 1701343084
Hash of 'ab' (error case): 80

```

# test환경

```bash
$ rustc --verbose --version
rustc 1.92.0 (ded5c06cf 2025-12-08)
binary: rustc
commit-hash: ded5c06cf21d2b93bffd5d884aa6e96934ee4234
commit-date: 2025-12-08
host: x86_64-unknown-linux-gnu
release: 1.92.0
LLVM version: 21.1.3

OS: Ubuntu 25.10 (Questing Quokka) x86_64
```

# Assembly 기존거 ->  AssemblyCode로 변환 

```rs
// Original trait for compatibility (note: fixing the typo from "AssmblyHash" to "AssemblyHash")
pub trait AssmblyHash<K, V> {
    fn new(capacity: usize) -> Self;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn access(&mut self, key: &K);

    fn contains(&self, key: &K) -> bool;

    fn remove_from_list(&mut self, i: usize);

    fn ensure_room(&mut self);

    fn len(&self) -> usize;

    fn remove(&mut self, key: &K) -> Option<V>;

    fn remove_tail(&mut self);

    fn get(&mut self, key: &K) -> Option<&V>;

    fn get_mut(&mut self, key: &K) -> Option<&mut V>;

    fn empty(&self) -> bool;

    fn is_full(&self) -> bool;
}
```

# Assembly로 변환

```asm

```

# All 12 tests pass. Here's a summary of the test coverage:

  |              Test               |                                   Description                                    |
  |-|-|
  | test_new                        | Creates a new empty cache                                                        |
  | test_insert_and_get             | Basic insert/get operations                                                      |
  | test_insert_update_existing_key | Updating existing keys                                                           |
  | test_lru_eviction               | Least-recently-used eviction when capacity is reached                            |
  | test_access_updates_lru         | Access moves entry to most-recent (commented out due to bug in remove_from_list) |
  | test_remove                     | Removing entries                                                                 |
  | test_contains                   | Checking key existence                                                           |
  | test_len                        | Getting cache length                                                             |
  | test_is_full                    | Checking if cache is at capacity                                                 |
  | test_get_mut                    | Mutable value retrieval                                                          |
  | test_string_keys                | Using string keys                                                                |
  | test_capacity_of_one            | Edge case with capacity=1                                                        |

- Note: The test revealed a bug in remove_from_list() at line 79-82 - the first match arm (Some(j), Some(k)) doesn't properly update node links.

# Changes to src/lib.rs:
  1. Made CHACHE_SIZE public (pub const CHACHE_SIZE: usize = 128;)
  2. Added empty() method (public, without the unused key parameter)

  Changes to tests/custom_hashmap_test.rs:
  1. Added test_empty() - tests that the cache reports empty correctly
  2. Added test_chache_size() - tests that the constant equals 128


# `asm` Fixes:
  1. Typo fixed: .look_up_failre → .look_up_failure
  2. Length check corrected: Changed from cmp rsi, 1 / jbe to cmp rsi, 4 / jb
    - Your original code would allow length 2, but then [rdi+rsi-2] = [rdi] would load the same bytes twice
    - Need at least 4 bytes to get 2 distinct bytes from start + 2 distinct bytes from end
  3. Instruction format: Comments are now on the same line as the instructions

  How it works:
  - rdi = string pointer
  - rsi = string length
  - Returns a 32-bit hash in eax:
    - Upper 16 bits = first 2 bytes of string
    - Lower 16 bits = last 2 bytes of string
  

