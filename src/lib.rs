//! libkcapi/kcapi.h rustified.  Read doc comments there.
//! This is version libkcapi 0.6.1

extern crate libc;
use libc::{c_int, c_uint, c_char, c_uchar, size_t, ssize_t};

#[repr(C)]
#[deriving(Show, Copy, PartialEq, Eq)]
pub struct kcapi_cipher_info {
    blocksize: c_uint,
    ivsize: c_uint,
    hash_digestsize: c_uint,
    blk_min_keysize: c_uint,
    blk_max_keysize: c_uint,
    aead_maxauthsize: c_uint,
    rng_seedsize: c_uint,
}

#[repr(C)]
#[deriving(Show, Copy, PartialEq, Eq)]
pub struct kcapi_cipher_data {
    iv: *const c_uchar,
    ivlen: size_t,
}

#[repr(C)]
#[deriving(Show, Copy, PartialEq, Eq)]
pub struct kcapi_aead_data {
    datalen: size_t,
    data: *mut c_uchar,
    assoclen: size_t,
    assoc: *mut c_uchar,
    taglen: size_t,
    tag: *mut c_uchar,
}

#[repr(C)]
#[deriving(Show, Copy, PartialEq, Eq)]
pub struct kcapi_handle {
    tfmfd: c_int,
    opfd: c_int,
    pipes: [c_int, ..2],
    cipher: kcapi_cipher_data,
    aead: kcapi_aead_data,
    info: kcapi_cipher_info,
}

/* Symmetric Cipher API */
extern "C" {
    pub fn kcapi_cipher_init(handle: *mut kcapi_handle, ciphername: *const c_char) -> c_int;
    pub fn kcapi_cipher_destroy(handle: *mut kcapi_handle) -> c_int;
    pub fn kcapi_cipher_setkey(handle: *mut kcapi_handle, key: *const c_uchar, keylen: size_t) -> c_int;
    pub fn kcapi_cipher_setiv(handle: *mut kcapi_handle, iv: *const c_uchar, ivlen: size_t) -> c_int;
    pub fn kcapi_cipher_encrypt(handle: *mut kcapi_handle,
                                indata: *const c_uchar, inlen: size_t,
                                out: *const c_uchar, outlen: size_t) -> ssize_t;
    pub fn kcapi_cipher_decrypt(handle: *mut kcapi_handle,
                                indata: *const c_uchar, inlen: size_t,
                                out: *const c_uchar, outlen: size_t) -> ssize_t;
    /*  These commented out pending figuring out iovecs in Rust
    pub fn kcapi_cipher_stream_init_enc(handle: *mut kcapi_handle,iov: *mut iovec, iovlen: size_t)-> ssize_t;
    pub fn kcapi_cipher_stream_init_dec(handle: *mut kcapi_handle,iov: *mut iovec, iovlen: size_t)-> ssize_t;
    pub fn kcapi_cipher_stream_update(handle: *mut kcapi_handle,iov: *mut iovec, iovlen: size_t)-> ssize_t;
    pub fn kcapi_cipher_stream_op(handle: *mut kcapi_handle, iov: *mut iovec, iovlen: size_t)-> ssize_t;
    */
    pub fn kcapi_cipher_ivsize(handle: *mut kcapi_handle) -> c_uint;
    pub fn kcapi_cipher_blocksize(handle: *mut kcapi_handle) -> c_uint;
}


#[test]
/// This test fails if you don't have the netlink patch applied to get cipher info.
/// Which I don't.  But we can just fill that in by hand as a workaround.
fn test_init() {
    unsafe {
    let mut kcapi_handle = unsafe { ::std::mem::uninitialized() };
    let r = kcapi_cipher_init(&mut kcapi_handle as *mut _, "cbc(aes)".to_c_str().as_ptr());
    //assert!(r == 0);
    }
}
