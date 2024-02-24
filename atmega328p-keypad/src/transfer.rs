/* Functions */
// Control Transfer function (if I can both read and write in the same function)
// Interrupt
// Bulk

/* Structs */
// Control report struct

// The idea is to have a big struct that has a device handle, a device and everything that a
// transfer needs device-wise. So that from lib I can initialize the big device info struct, and
// then call the correct functions

// TRANSFERS: they all need: dhandle, call appropriate function, desriptor to hold the correct
// values to pass as parameters to the transfer function call
// these functions should only call the underlining rusb function, nothing more,
// anything else they might need has to be implemented in a different function, maybe a
// utility module
//

// BIG DEVICE STRUCT info
//
