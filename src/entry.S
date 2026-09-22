.section .text.entry
.globl _entry 

_entry:
  csrr t0, mhartid
  bnez t0, .Lpark

  la sp, boot_stack_top
  call rust_main

.Lpark:
  wfi
  j .Lpark

.section .bss.stack, "aw", @nobits
.balign 16
boot_stack:
  .space 4096
boot_stack_top:


