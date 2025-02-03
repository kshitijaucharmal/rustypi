ENTRY(_start)

SECTIONS
{
    /* Load the kernel at 0x80000, where the RPi3B expects it */
    . = 0x80000;

    /* Kernel text/code section */
    .text :
    {
        *(.text._start)   /* Ensure entry point is the first thing in .text */
        *(.text*)         /* Other code */
    }

    /* Read-only data section */
    .rodata :
    {
        *(.rodata*)
    }

    /* Initialized data section */
    .data :
    {
        *(.data*)
    }

    /* Uninitialized data (BSS) */
    .bss :
    {
        *(COMMON)
        *(.bss*)
    }

    /* Ensure 16-byte alignment for stack */
    . = ALIGN(16);

    /* Stack grows downward from high memory */
    _stack_top = 0x8000000; /* 128MB mark */
    _stack_bottom = _stack_top - 0x8000; /* 32KB stack */

    PROVIDE(_stack_start = _stack_bottom);
}
