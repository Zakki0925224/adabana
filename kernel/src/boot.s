.section ".text.boot"
.global _boot

_boot:
    mrs x1, mpidr_el1
    and x1, x1, #0x3
    cbz x1, 2f

1:  wfe
    b 1b
2:
    ldr x1, =_boot
    mov sp, x1
    bl kernel_main
    b 1b
