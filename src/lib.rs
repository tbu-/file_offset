//
//! `file_offset` provides a platform-independent way of atomically reading and
//! writing files at specified offsets.
//!
//! ```
//! use file_offset::FileExt;
//! use std::fs::File;
//! use std::str;
//!
//! let mut buffer = [0; 2048];
//! let f = File::open("src/lib.rs").unwrap();
//! f.read_offset(&mut buffer, 3);
//! print!("{}", str::from_utf8(&buffer).unwrap());
//! ```

use std::fs::File;
use std::io;

mod sys;

/// This trait provides the extension methods for reading and writing files at
/// specified offsets.
///
/// Note the difference between Windows and Unix behavior listed in the
/// "Platform-specific behavior" sections.
pub trait FileExt {
    /// Reads a number of bytes, starting at a given file offset.
    ///
    /// Returns the number of bytes read.
    ///
    /// The offset is relative to the start of the file and thus independent of
    /// the current cursor. Note that similarly to `File::read`, returning with
    /// a short read is not an error. Additionally, read errors that are of
    /// `ErrorKind::Interrupted` are transient and the read call should
    /// usually be retried.
    ///
    /// # Platform-specific behavior
    ///
    /// This function delegates to `std::os::unix::fs::FileExt::read_at` and
    /// thus the `pread64` function on Unix and to
    /// `std::os::windows::fs::FileExt::seek_read` and hence to a `ReadFile`
    /// function call using the `lpOverlapped` parameter on Windows.
    ///
    /// The actions performed by these functions are **not identical**. In
    /// particular, the Windows version of this function moves the file cursor,
    /// whereas the Unix version does not.
    fn read_offset(&self, buf: &mut [u8], offset: u64) -> io::Result<usize>;

    /// Writes a number of bytes, starting at a given file offset.
    ///
    /// Returns the number of bytes written.
    ///
    /// The offset is relative to the start of the file and thus independent of
    /// the current cursor. Note that similarly to `File::write`, returning
    /// with a short write is not an error. Additionally, write errors that are
    /// of `ErrorKind::Interrupted` are transient and the write call should
    /// usually be retried.
    ///
    /// # Platform-specific behavior
    ///
    /// This function delegates to `std::os::unix::fs::FileExt::write_at` and
    /// thus the `pwrite64` function on Unix and to
    /// `std::os::windows::fs::FileExt::seek_write` and hence to a `WriteFile`
    /// function call using the `lpOverlapped` parameter on Windows.
    ///
    /// The actions performed by these functions are **not identical**. In
    /// particular, the Windows version of this function moves the file cursor,
    /// whereas the Unix version does not.
    fn write_offset(&self, buf: &[u8], offset: u64) -> io::Result<usize>;
}

impl FileExt for File {
    #[inline]
    fn read_offset(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        sys::read_offset(self, buf, offset)
    }
    #[inline]
    fn write_offset(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        sys::write_offset(self, buf, offset)
    }
}
