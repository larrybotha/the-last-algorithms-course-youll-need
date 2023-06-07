import {inspect} from 'node:util'

const log = console.log.bind(console)

// at this point we have a series of 0s
const xs = new ArrayBuffer(6)
log('initialise array buffer')
log(xs)

/**
  * We create a _view_ into the array buffer,
  * in this case, we are declarging that we are
* working with 8-bit unsigned integers
  */
const u8array = new Uint8Array(xs);
log('\ninitialise u8 int view into array')
log(u8array)

// with ta view into the array buffer, we can manipulate
// the values in that array:
u8array[0] = 45
log('\nassign index 0 to 45')
log(`buffer: ${inspect(xs)}`) // index 0 is changes
log(`view: ${u8array}`) // index 0 is changes

u8array[2] = 45
log('\nassign index 2 to 45')
log(`buffer: ${inspect(xs)}`) // index 0 is changes
log(`view: ${u8array}`) // index 0 is changes

const u16array = new Uint16Array(xs)
log('\ninitialise u8 int view into array')
log(u16array)

/**
  * Here we use a larger number than 45 - twice the size in
  * terms of memory, and we can see that the array buffer now
  * has the bytes at positions 4 and 5 both occupied
  */
u16array[2] = 0x4545
log('\nassign index 2 to 0x4545 on u16 view')
log(`buffer: ${inspect(xs)}`) // index 0 is changes
log(`view: ${u16array}`) // index 0 is changes
