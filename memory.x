MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x8005000, LENGTH = 76K
  RAM : ORIGIN = 0x20000000, LENGTH = 24K
}

_stext = ORIGIN(FLASH) + 0x808;

SECTIONS {
  .start_block ORIGIN(FLASH) + 0x800 : {
    KEEP(*(.boot_sig))
  } > FLASH
}
INSERT AFTER .vector_table;
