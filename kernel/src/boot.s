.section ".text.boot"

.global _boot

_boot:
    // Read CPU ID, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, master_cpu
1:  wfe
    b       1b

master_cpu:
    mov    x20, x0 // Save DTB address
    // If in EL2, switch to EL1
    mrs     x0, CurrentEL
    lsr     x0, x0, #2
    cmp     x0, #2
    b.ne    set_stack

    // Switch to EL1
    mov     x0, #0x5          // EL1h mode, all interrupts disabled
    msr     spsr_el2, x0
    adr     x0, set_stack
    msr     elr_el2, x0
    eret

set_stack:
    // Set up stack pointer
    ldr     x1, =stack_top
    mov     sp, x1

    // Clear BSS section
    ldr     x2, =__bss_start
    ldr     x3, =__bss_end
    mov     x4, #0
bss_clear:
    cmp     x2, x3
    b.ge    start_kernel
    str     x4, [x2], #8
    b       bss_clear

start_kernel:
    mov x0, x20 // Restore DTB address
    // Jump to Rust code, should not return
    bl      kernel_main

    // Halt CPU if Rust code returns unexpectedly
1:  wfe
    b       1b

.section .bss
.balign 16
stack_bottom:
    .skip 8192  // 8KB stack
stack_top:
