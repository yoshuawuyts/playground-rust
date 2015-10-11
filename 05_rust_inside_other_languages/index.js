const ffi = require('ffi')

const lib = ffi.Library('target/release/libembed', {
  process: [ 'void', [] ]
})

lib.process()
console.log('done')
