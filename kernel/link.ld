ENTRY(_boot);

SECTIONS
{
    . = 0x80000;

    .text :
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    PROVIDE(_data = .);
    .data :
    {
        *(.data .data.*)
    }

    .bss (NOLOAD) :
    {
        . = ALIGN(16);
        *(.bss .bss.*)
        *(COMMON)
    }

   /DISCARD/ :
   {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
   }
}

__bss_start = ADDR(.bss);
__bss_end = ADDR(.bss) + SIZEOF(.bss);
