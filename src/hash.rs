#[cfg(target_os = "linux")]
#[inline(always)]
pub unsafe fn look_up_identifier(data: *const u8, len: usize) -> u32 {
    unsafe {
        let result: u32;
        std::arch::asm!(
            "cmp rsi, 4",
            "jb 2f",
            "movzx eax, WORD PTR [rdi]",
            "movzx edx, WORD PTR [rdi + rsi - 2]",
            "shl eax, 16",
            "or eax, edx",
            "jmp 3f",
            "2:",
            "mov eax, 80",
            "3:",
            in("rdi") data,
            in("rsi") len,
            lateout("eax") result,
            clobber_abi("system")
        );
        result
    }
}

#[cfg(target_os = "macos")]
#[inline(always)]
pub unsafe fn look_up_identifier(data: *const u8, len: usize) -> u32 {
    unsafe {
        let result: u32;
        std::arch::asm!(
            "cmp {len}, #4",
            "b.lo 2f",
            "ldrh w0, [{data}]",
            "sub x1, {len}, #2",
            "ldrh w2, [{data}, x1]",
            "lsl w0, w0, #16",
            "orr w0, w0, w2",
            "b 3f",
            "2:",
            "mov w0, #80",
            "3:",
            data = in(reg) data,
            len = in(reg) len,
            lateout("w0") result,
            clobber_abi("system")
        );
        result
    }
}

pub trait AssemblyHash {
    fn assembly_hash(&self) -> u32;
}

impl AssemblyHash for String {
    fn assembly_hash(&self) -> u32 {
        self.as_str().assembly_hash()
    }
}

impl AssemblyHash for &str {
    fn assembly_hash(&self) -> u32 {
        unsafe { look_up_identifier(self.as_ptr(), self.len()) }
    }
}
