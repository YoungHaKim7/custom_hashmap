look_up_identifier:
    cmp rsi, 4                ; Need at least 4 bytes for 2 from start + 2 from end
    jb .look_up_failure       ; Jump if below (unsigned) to failure

.hash:
    movzx eax, WORD PTR [rdi]       ; Load first 2 bytes from start
    movzx edx, WORD PTR [rdi+rsi-2] ; Load last 2 bytes from end
    shl eax, 16                      ; Shift first 2 bytes to upper half
    or eax, edx                      ; Combine: upper half = start, lower half = end
    ret

.look_up_failure:
    mov eax, 80              ; Return error code 80
    ret
