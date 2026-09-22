OUTPUT_ARCH(riscv)
ENTRY(_entry)

SECTIONS
{
  . = 0x80000000;

  .text : {
    KEEP(*(.text.entry))
    *(.text .text.*)
  }

  .rodata : {
    . = ALIGN(16);
    *(.srodata .srodata.*)
    *(.rodata .rodata.*)
  }

  .data : {
    . = ALIGN(16);
    *(.sdata .sdata.*)
    *(.data .data.*)
  }

  .bss : {
    . = ALIGN(16);
    *(.bss.stack)
    *(.sbss .sbss.*)
    *(.bss .bss.*)
    *(COMMON)
  }
}
