use zephyr_sys::raw::{k_objects, k_sem, k_timeout_t};

use super::NegErr;
use crate::kobj::*;
use crate::time::Timeout;

// Declare the Zephyr struct to be a kernel object
unsafe impl KObj for k_msgq {
    const OTYPE: k_objects = zephyr_sys::raw::k_objects_K_OBJ_MSGQ;
}

pub use zephyr_sys::raw::k_msgq as KMsgq;

crate::make_static_wrapper!(k_msgq, zephyr_sys::raw::k_msgq);

/// Raw syscall API
pub trait MessageQueueSyscalls {
    unsafe fn k_msgq_init(msgq: &k_msgq, buffer: *mut libc::c_char, msg_size: usize, max_msgs: u32);
    unsafe fn k_msgq_alloc_init(msgq: &k_msgq , msg_size: usize, max_msgs: u32);
    fn k_msgq_cleanup(msgq: &k_msgq) -> libc::c_int; 
    fn k_msgq_put(msgq: &k_msgq, data: * mut libc::c_void, timeout: k_timeout_t) -> libc::c_int;
    fn k_msgq_get(msgq: &k_msgq, data: * mut libc::c_void, timeout: k_timeout_t) -> libc::c_int;    
    fn k_msgq_peek(msgq: &k_msgq, data: * mut libc::c_void) -> libc::c_int;
    fn k_msgq_peek_at(msgq: &k_msgq, data: * mut libc::c_void, idx: libc::c_uint) -> libc::c_int;
    fn k_msgq_purge(msgq: &k_msgq);
    fn k_msgq_num_free_get(msgq: &k_msgq) -> libc::c_uint;
    // TODO: k_msgq_get_attrs
    fn k_msgq_num_used_get(msgq: &k_msgq) -> libc::c_uint;
}

macro_rules! trait_impl {
    ($context:ident, $context_struct:path) => {
        impl MessageQueueSyscalls for $context_struct {
            unsafe fn k_msgq_init(msgq: &k_msgq, 
                                    buffer: *mut libc::c_char, msg_size: usize, max_msgs: u32) {
                zephyr_sys::syscalls::$context::k_msgq_init(
                    msgq as *const _ as *mut _,
                    buffer, msg_size, max_msgs);
            }

            unsafe fn k_msgq_alloc_init(msgq: &k_msgq, 
                                        msg_size: usize, max_msgs: u32) {
                zephyr_sys::syscalls::$context::k_msgq_alloc_init(
                    msgq as *const _ as *mut _,
                    msg_size, max_msgs);
            }

            fn k_msgq_cleanup(msgq: &k_msgq) -> libc::c_int {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_cleanup(
                        msgq as *const _ as *mut _)
                }
            }

            fn k_msgq_put(msgq: &k_msgq, data: * mut libc::c_void, timeout: k_timeout_t) 
                -> libc::c_int {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_put(
                        msgq as *const _ as *mut _,
                        data, timeout)
                }
            }   

            fn k_msgq_get(msgq: &k_msgq, data: * mut libc::c_void, timeout: k_timeout_t) 
                -> libc::c_int {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_get(
                        msgq as *const _ as *mut _,
                        data, timeout)
                }
            }
            fn k_msgq_peek(msgq: &k_msgq, data: * mut libc::c_void) -> libc::c_int {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_peek(
                        msgq as *const _ as *mut _,
                        data)
                }
            }

            fn k_msgq_peek_at(msgq: &k_msgq, data: * mut libc::c_void, idx: libc::c_uint) 
                -> libc::c_int {
                    unsafe {
                        zephyr_sys::syscalls::$context::k_msgq_peek_at(
                            msgq as *const _ as *mut _,
                            data, idx)
                    }
                }
            fn k_msgq_purge(msgq: &k_msgq) {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_purge(
                        msgq as *const _ as *mut _)
                }
            }
            fn k_msgq_num_free_get(msgq: &k_msgq) -> libc::c_uint {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_num_free_get(
                        msgq as *const _ as *mut _)
                }
            }
            // TODO: k_msgq_get_attrs
            fn k_msgq_num_used_get(msgq: &k_msgq) -> libc::c_uint {
                unsafe {
                    zephyr_sys::syscalls::$context::k_msgq_num_used_get(
                        msgq as *const _ as *mut _)
                }
            }
        
        }
    };
}

trait_impl!(kernel, crate::context::Kernel);
trait_impl!(user, crate::context::User);
trait_impl!(any, crate::context::Any);

use super::ZephyrResult;

pub struct MessageQueue<'m, T: Sized>
{
    msgq: &'m KMsgq,
    buffer: &'m mut[T]
}

impl<'m, T: Sized> MessageQueue<'m, T> {
    fn new(msgq: &k_msgq, buffer: &'m mut[T], max_msgs: u32) -> MessageQueue<'m,T> 
    {
        k_msgq_init(msgq, buffer, std::mem::sizeof<T>, max_msgs);
        
        MessageQueue {
            msgq: msgq,
            buffer: buffer
        }
    }
    
    fn alloc_init(msgq: &k_msgq, max_msgs: u32);
    // fn cleanup(msgq: &k_msgq) -> libc::c_int; 
    fn put(&mut self, data: &T, timeout: k_timeout_t) -> Result<(),usize>;
    fn get(&mut self, timeout: k_timeout_t) -> Result<T,usize>;
    fn peek(&self) -> Result<&T,usize>;
    fn peek_at(&self, usize) -> Result<&T,usize>
    fn purge(&mut self);
    fn num_free_get(&self) -> usize;
    // TODO: k_msgq_get_attrs
    fn num_used_get(&self) -> usize;
}

impl Semaphore for k_sem {
    unsafe fn init<C: SemaphoreSyscalls>(&self, initial_count: u32, limit: u32) {
        C::k_sem_init(&self, initial_count, limit)
    }

    fn take<C: SemaphoreSyscalls>(&self) {
        C::k_sem_take(self, zephyr_sys::raw::K_FOREVER.into())
            .neg_err()
            .expect("sem take");
    }

    fn take_timeout<C: SemaphoreSyscalls>(&self, timeout: Timeout) -> bool {
        match C::k_sem_take(self, timeout.0).neg_err() {
            Ok(_) => Ok(true),
            Err(zephyr_sys::raw::EBUSY) => Ok(false),
            Err(zephyr_sys::raw::EAGAIN) => Ok(false),
            Err(e) => Err(e),
        }
        .expect("sem take")
    }

    fn try_take<C: SemaphoreSyscalls>(&self) -> bool {
        match C::k_sem_take(self, (zephyr_sys::raw::K_NO_WAIT).into()).neg_err() {
            Ok(_) => Ok(true),
            Err(zephyr_sys::raw::EBUSY) => Ok(false),
            Err(e) => Err(e),
        }
        .expect("sem take")
    }

    fn give<C: SemaphoreSyscalls>(&self) {
        C::k_sem_give(self)
    }

    fn reset<C: SemaphoreSyscalls>(&self) {
        C::k_sem_reset(self)
    }

    fn count<C: SemaphoreSyscalls>(&self) -> u32 {
        // .into() will fail to compile on platforms where uint != u32
        // can do a conversion if that case ever occurs
        C::k_sem_count_get(self).into()
    }
}
