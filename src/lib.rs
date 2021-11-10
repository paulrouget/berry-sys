//! # Berry Rust bindings
//!
//! Berry documentation: <https://github.com/berry-lang/berry>

#![no_std]
#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces)]
#![cfg_attr(
  feature = "cargo-clippy",
  allow(clippy::new_without_default, clippy::new_without_default)
)]
#![cfg_attr(
  feature = "cargo-clippy",
  warn(
    clippy::float_arithmetic,
    clippy::mut_mut,
    clippy::nonminimal_bool,
    clippy::map_unwrap_or,
    clippy::print_stdout,
  )
)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(clippy::all)]
mod sys {
  include!(concat!(env!("OUT_DIR"), "/berry.rs"));
}

use log::*;

use sys::size_t;
use cty;

static s_bytes: &[u8; 76] = include_bytes!("/tmp/test.o");
static mut s_cursor: usize = 0;

#[no_mangle]
unsafe extern "C" fn be_writebuffer(buffer: *const cty::c_char, length: size_t) {
  let slice: &[u8] = core::slice::from_raw_parts(buffer as *const u8, length as _);
  let slice = &*(slice as *const _ as *const [u8]);
  let s = core::str::from_utf8(&slice);
  info!("berry: {} {}", slice.len(), s.unwrap());
}

#[no_mangle]
extern "C" fn be_fread(hfile: *mut cty::c_void, buffer: *mut cty::c_void, length: size_t) -> size_t {
  let cursor = unsafe { s_cursor };
  let max_read = s_bytes.len() - cursor;
  let count = if length as usize > max_read {
    max_read
  } else {
    length as usize
  };
  let mut slice: &mut [u8] = unsafe {
    core::slice::from_raw_parts_mut(buffer as *mut u8, count)
  };
  slice.copy_from_slice(s_bytes.get(cursor..cursor+count).unwrap());
  unsafe { 
    s_cursor += count;
  }
  count as _
}

#[no_mangle]
extern "C" fn be_fopen(filename: *const cty::c_char, modes: *const cty::c_char) -> *mut cty::c_void {
  1 as _
}

#[no_mangle]
extern "C" fn be_fclose(hfile: *mut cty::c_void) -> cty::c_int {
  unsafe {
    s_cursor = 0;
  }
  0
}

#[no_mangle]
extern "C" fn be_fseek(hfile: *mut cty::c_void, offset: cty::c_long) -> cty::c_int {
  // println!("FIXME");
  0
}

#[no_mangle]
extern "C" fn be_readstring(buffer: *mut cty::c_char, size: size_t) -> *mut cty::c_char {
  // println!("FIXME");
  return 0 as _
}

#[no_mangle]
extern "C" fn be_fflush(hfile: *mut cty::c_void) -> cty::c_long {
  // println!("FIXME");
  0
}

#[no_mangle]
extern "C" fn be_fwrite(hfile: *mut cty::c_void, buffer: *const cty::c_void, length: size_t) -> size_t {
  // println!("FIXME");
  0
}

#[no_mangle]
extern "C" fn be_fsize(hfile: *mut cty::c_void) -> size_t {
  // println!("FIXME");
  0
}

#[no_mangle]
extern "C" fn be_ftell(hfile: *mut cty::c_void) -> cty::c_long {
  // println!("be_ftell");
  0
}

#[no_mangle]
extern "C" fn be_fgets(hfile: *mut cty::c_void, buffer: *mut cty::c_void, size: cty::c_int,) -> *mut cty::c_char {
  // println!("be_fgets");
  0 as _
}

// #[no_mangle]
// extern "C" fn be_embedder_abort() {
// }

// #[no_mangle]
// extern "C" fn be_embedder_exit() {
// }

// #[no_mangle]
// extern "C" fn be_embedder_malloc(size: size_t) -> *mut cty::c_void {
//   // FIXME
// }

// #[no_mangle]
// extern "C" fn be_embedder_free(ptr: *mut cty::c_void) {
//   // FIXME
// }

// #[no_mangle]
// extern "C" fn be_embedder_realloc(ptr: *mut cty::c_void) {
//   // FIXME
// }

/// Create VM
pub fn create_vm() {
  unsafe {
    let vm = sys::be_vm_new();
    let name = b"xxx\0";
    let mut res = sys::be_loadmode(vm, name.as_ptr() as _, false);
    if res == sys::berrorcode_BE_OK as _ {
      res = sys::be_pcall(vm, 0);
    }

    // println!("RES: {}", res);

    let res: sys::berrorcode = res as _;

    match res {
      sys::berrorcode_BE_OK => info!("ok"),
      sys::berrorcode_BE_EXCEPTION => {
        error!("Exception");
        sys::be_dumpexcept(vm);
      },
      sys::berrorcode_BE_EXIT => {
        error!("Exit");
        sys::be_toindex(vm, -1);
      }
      sys::berrorcode_BE_IO_ERROR => {
        error!("IO Error");
        let error = sys::be_tostring(vm, -1);
      },
      sys::berrorcode_BE_MALLOC_FAIL => error!("error: memory allocation failed.\n"),
      _ => error!("Unexpected: {}", res),
    }
  }
}
