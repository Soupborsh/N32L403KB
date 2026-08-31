MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  # Uncomment this for build as T90 .atk app
  # TODO: automate this using feature flag.
  # FLASH : ORIGIN = 0x8005000, LENGTH = 76K
  FLASH : ORIGIN = 0x8000000, LENGTH = 128K
  RAM : ORIGIN = 0x20000000, LENGTH = 24K
}

# _stext = ORIGIN(FLASH) + 0x808;

# SECTIONS {
#   .start_block ORIGIN(FLASH) + 0x800 : {
#     KEEP(*(.boot_sig))
#   } > FLASH
# }
# INSERT AFTER .vector_table;
