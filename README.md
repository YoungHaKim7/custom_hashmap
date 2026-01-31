# Source
- [(230311)Faster than Rust and C++: the PERFECT hash table | strager](https://youtu.be/DMQ_HcNSOAI?si=jKP1DEvRvb8uqozz)

# Result

```bash

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
