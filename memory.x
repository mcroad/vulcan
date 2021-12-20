MEMORY
{
    FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = 2048K
    FLASH_ISR (rx)  : ORIGIN = 0x08000000, LENGTH = 128K    /* sector 0, 128K */
    FLASH_FS (r)    : ORIGIN = 0x08020000, LENGTH = 128K    /* sector 1, 128K */
    FLASH_TEXT (rx) : ORIGIN = 0x08040000, LENGTH = 1792K   /* sectors 6*128 + 8*128 */
    DTCM (xrw)      : ORIGIN = 0x20000000, LENGTH = 128K    /* Used for storage cache */
    RAM (xrw)       : ORIGIN = 0x24000000, LENGTH = 512K    /* AXI SRAM */
}

/* The location of the stack can be overridden using the
   `_stack_start` symbol.  Place the stack at the end of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);