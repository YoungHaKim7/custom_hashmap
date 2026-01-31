/// Assembly hash function using inline assembly
/// Takes first 2 bytes and last 2 bytes of the input
/// Returns: combined u32 hash value or 80 on error (string too short)
#[cfg(target_os = "linux")]
#[inline(always)]
pub unsafe fn look_up_identifier(data: *const u8, len: usize) -> u32 {
    // SAFETY: The caller must ensure `data` is valid for reading `len` bytes
    unsafe {
        let result: u32;
        std::arch::asm!(
            // Check if length >= 4
            "cmp rsi, 4",
            "jb 2f",                      // Jump to failure if below

            // Success case: compute hash
            "movzx eax, WORD PTR [rdi]",       // Load first 2 bytes
            "movzx edx, WORD PTR [rdi + rsi - 2]", // Load last 2 bytes
            "shl eax, 16",                      // Shift to upper half
            "or eax, edx",                      // Combine
            "jmp 3f",                           // Jump to end

            // Failure case: return error code 80
            "2:",
            "mov eax, 80",

            // End
            "3:",
            in("rdi") data,
            in("rsi") len,
            lateout("eax") result,
            // clobbered registers
            clobber_abi("system")
        );
        result
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
pub unsafe fn look_up_identifier(data: *const u8, len: usize) -> u32 {
    // SAFETY: The caller must ensure `data` is valid for reading `len` bytes
    unsafe {
        let result: u32;
        std::arch::asm!(
            // Check if length >= 4
            "cmp {len}, #4",
            "b.lo 2f",                      // Jump to failure if below (unsigned less)

            // Success case: compute hash
            "ldrh w0, [{data}]",           // Load first 2 bytes
            "sub x1, {len}, #2",
            "ldrh w2, [{data}, x1]",       // Load last 2 bytes
            "lsl w0, w0, #16",             // Shift to upper half
            "orr w0, w0, w2",              // Combine
            "b 3f",                        // Jump to end

            // Failure case: return error code 80
            "2:",
            "mov w0, #80",

            // End
            "3:",
            data = in(reg) data,
            len = in(reg) len,
            lateout("w0") result,
            // clobbered registers
            clobber_abi("system")
        );
        result
    }
}

pub trait AssemblyHash {
    fn assembly_hash(&self) -> u32;
}

// Implement for String
impl AssemblyHash for String {
    fn assembly_hash(&self) -> u32 {
        self.as_str().assembly_hash()
    }
}

// Implement for &str
impl AssemblyHash for &str {
    fn assembly_hash(&self) -> u32 {
        unsafe { look_up_identifier(self.as_ptr(), self.len()) }
    }
}

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
