MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH =  128K /* BANK_1 */ /* Subtract the size of the CONFIG section from FLASH, if you're using storage */
    CONFIG: ORIGIN = ORIGIN(FLASH) + LENGTH(FLASH), LENGTH = 0K /* Change this to a multiple of your chip's erase size, if you're using storage */
    RAM   : ORIGIN = 0x20000000, LENGTH =   32K
}

__config_start = ORIGIN(CONFIG) - ORIGIN(FLASH);
__config_end = __config_start + LENGTH(CONFIG);
