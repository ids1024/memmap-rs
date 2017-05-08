extern crate syscall;

use std::io;
use std::os::unix::io::AsRawFd;
use std::fs::File;

use ::Protection;
use ::MmapOptions;

pub struct MmapInner {
    ptr: usize,
    len: usize,
    annon: bool,
}

impl MmapInner {
    pub fn open(file: &File, _prot: Protection, offset: usize, len: usize) -> io::Result<MmapInner> {
        // TODO: handle Protection
        // TODO: error handling
        let ptr = unsafe { syscall::fmap(file.as_raw_fd(), offset, len) }.unwrap();
        Ok(MmapInner{ptr, len, annon: false})
    }

    pub fn anonymous(len: usize, _prot: Protection, _options: MmapOptions) -> io::Result<MmapInner> {
        // TODO: handle Protection
        // TODO: handle MmapOptions
        // TODO: error handling
        let ptr = unsafe { syscall::physalloc(len) }.unwrap();
        Ok(MmapInner{ptr, len, annon: true})
    }

    pub fn flush(&self, _offset: usize, _len: usize) -> io::Result<()> {
        // FIXME
        Ok(())
    }

    pub fn flush_async(&self, _offset: usize, _len: usize) -> io::Result<()> {
        // FIXME
        Ok(())
    }

    pub fn set_protection(&mut self, _prot: Protection) -> io::Result<()> {
        // FIXME
        Ok(())
    }

    pub fn ptr(&self) -> *const u8 {
        self.ptr as *const u8
    }

    pub fn mut_ptr(&mut self) -> *mut u8 {
        self.ptr as *mut u8
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for MmapInner {
    fn drop(&mut self) {
        // TODO: Error handling
        if self.annon {
            unsafe { syscall::physfree(self.ptr, self.len) }.unwrap();
        } else {
            unsafe { syscall::funmap(self.ptr) }.unwrap();
        }
    }
}

unsafe impl Sync for MmapInner { }
unsafe impl Send for MmapInner { }
