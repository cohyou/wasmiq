use crate::wasi::{
    Errno,
    Size,
    Fd,
    Timestamp,
    Clockid,
    Filesize,
    Advice,
    Fdstat,
    Fdflags,
    Rights,
    Filestat,
    Fstflags,
    IovecArray,
    Prestat,
    CiovecArray,
    Dircookie,
    Filedelta,
    Whence,
    Lookupflags,
    Oflags,
    Exitcode,
    Signal,
    Sdflags,
    Event,
    Riflags,
    Roflags,
    Siflags,
    Subscription,
};

/// Read command-line argument data. 
/// The size of the array should match that returned by args_sizes_get. 
/// Each argument is expected to be \0 terminated.
fn args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> Result<(), Errno> { unimplemented!() }

/// Return command-line argument data sizes.
///
/// Returns the number of arguments and the size of the argument string data, or an error.
fn args_sizes_get() -> Result<(Size, Size), Errno> { unimplemented!() }

/// Read environment variable data.
/// The sizes of the buffers should match that returned by environ_sizes_get.
/// Key/value pairs are expected to be joined with =s, and terminated with \0s.
fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> Result<(), Errno> { unimplemented!() }

/// Return environment variable data sizes.
/// 
/// Returns the number of environment variable arguments and the size of the environment variable data.
fn environ_sizes_get() -> Result<(Size, Size), Errno> { unimplemented!() }

/// Return the resolution of a clock. 
/// Implementations are required to provide a non-zero value for supported clocks. 
/// For unsupported clocks, return errno::inval. 
/// Note: This is similar to clock_getres in POSIX.
/// 
/// The resolution of the clock, or an error if one happened.
fn clock_res_get(id: Clockid) -> Result<Timestamp, Errno> { unimplemented!() }

/// Return the time value of a clock. Note: This is similar to clock_gettime in POSIX.
/// 
/// The time value of the clock.
fn clock_time_get(id: Clockid, precision: Timestamp) -> Result<Timestamp, Errno> { unimplemented!() }

/// Provide file advisory information on a file descriptor. 
/// Note: This is similar to posix_fadvise in POSIX.
fn fd_advise(fd: Fd, offset: Filesize, len: Filesize, advice: Advice) -> Result<(), Errno> { unimplemented!() }

/// Force the allocation of space in a file. 
/// Note: This is similar to posix_fallocate in POSIX.
fn fd_allocate(fd: Fd, offset: Filesize, len: Filesize) -> Result<(), Errno> { unimplemented!() }

/// Close a file descriptor. 
/// Note: This is similar to close in POSIX.
fn fd_close(fd: Fd) -> Result<(), Errno> { unimplemented!() }

/// Synchronize the data of a file to disk. 
/// Note: This is similar to fdatasync in POSIX.
fn fd_datasync(fd: Fd) -> Result<(), Errno> { unimplemented!() }

/// Get the attributes of a file descriptor. 
/// Note: This returns similar flags to fsync(fd, F_GETFL) in POSIX, as well as additional fields.
/// 
/// The buffer where the file descriptor's attributes are stored.
fn fd_fdstat_get(fd: Fd) -> Result<Fdstat, Errno> { unimplemented!() }

/// Adjust the flags associated with a file descriptor. 
/// Note: This is similar to fcntl(fd, F_SETFL, flags) in POSIX.
fn fd_fdstat_set_flags(fd: Fd, flags: Fdflags) -> Result<(), Errno> { unimplemented!() }

/// Adjust the rights associated with a file descriptor. 
/// This can only be used to remove rights, and returns errno::notcapable if called in a way that would attempt to add rights
fn fd_fdstat_set_rights(fd: Fd, fs_rights_base: Rights, fs_rights_inheriting: Rights) -> Result<(), Errno> { unimplemented!() }

/// Return the attributes of an open file.
/// 
/// The buffer where the file's attributes are stored.
fn fd_filestat_get(fd: Fd) -> Result<Filestat, Errno> { unimplemented!() }

/// Adjust the size of an open file. 
/// If this increases the file's size, the extra bytes are filled with zeros. 
/// Note: This is similar to ftruncate in POSIX.
fn fd_filestat_set_size(fd: Fd, Size: Filesize) -> Result<(), Errno> { unimplemented!() }

/// Adjust the timestamps of an open file or directory. 
/// Note: This is similar to futimens in POSIX.
fn fd_filestat_set_times(fd: Fd, atim: Timestamp, mtim: Timestamp, fst_flags: Fstflags) -> Result<(), Errno> { unimplemented!() }

/// Read from a file descriptor, without using and updating the file descriptor's offset. 
/// Note: This is similar to preadv in POSIX.
/// 
/// The number of bytes read.
fn fd_pread(fd: Fd, iovs: IovecArray, offset: Filesize) -> Result<Size, Errno> { unimplemented!() }

/// Return a description of the given preopened file descriptor.
/// 
/// The buffer where the description is stored.
fn fd_prestat_get(fd: Fd) -> Result<Prestat, Errno> { unimplemented!() }

/// Return a description of the given preopened file descriptor.
fn fd_prestat_dir_name(fd: Fd, path: *mut u8, path_len: Size) -> Result<(), Errno> { unimplemented!() }

/// Write to a file descriptor, without using and updating the file descriptor's offset. 
/// Note: This is similar to pwritev in POSIX.
/// 
/// The number of bytes written.
fn fd_pwrite(fd: Fd, iovs: CiovecArray, offset: Filesize) -> Result<Size, Errno> { unimplemented!() }

/// Read from a file descriptor. 
/// Note: This is similar to readv in POSIX.
/// 
/// The number of bytes read.
fn fd_read(fd: Fd, iovs: IovecArray) -> Result<Size, Errno> { unimplemented!() }

/// Read directory entries from a directory. 
/// When successful, the contents of the output buffer consist of a sequence of directory entries. 
/// Each directory entry consists of a dirent object, followed by dirent::d_namlen bytes holding the name of the directory entry. 
/// This function fills the output buffer as much as possible, potentially truncating the last directory entry. 
/// This allows the caller to grow its read buffer size in case it's too small to fit a single large directory entry, or skip the oversized directory entry.
/// 
/// The number of bytes stored in the read buffer. If less than the size of the read buffer, the end of the directory has been reached.
fn fd_readdir(fd: Fd, buf: *mut u8, buf_len: Size, cookie: Dircookie) -> Result<Size, Errno> { unimplemented!() }

/// Atomically replace a file descriptor by renumbering another file descriptor. 
/// Due to the strong focus on thread safety, this environment does not provide a mechanism to duplicate or renumber a file descriptor to an arbitrary number, like dup2(). 
/// This would be prone to race conditions, as an actual file descriptor with the same number could be allocated by a different thread at the same time. 
/// This function provides a way to atomically renumber file descriptors, which would disappear if dup2() were to be removed entirely.
fn fd_renumber(fd: Fd, to: Fd) -> Result<(), Errno> { unimplemented!() }

/// Move the offset of a file descriptor. 
/// Note: This is similar to lseek in POSIX.
/// 
/// The new offset of the file descriptor, relative to the start of the file.
fn fd_seek(fd: Fd, offset: Filedelta, whence: Whence) -> Result<Filesize, Errno> { unimplemented!() }

/// Synchronize the data and metadata of a file to disk. 
/// Note: This is similar to fsync in POSIX.
fn fd_sync(fd: Fd) -> Result<(), Errno>  { unimplemented!() }

/// Return the current offset of a file descriptor. 
/// Note: This is similar to lseek(fd, 0, SEEK_CUR) in POSIX.
/// 
/// The current offset of the file descriptor, relative to the start of the file.
fn fd_tell(fd: Fd) -> Result<Filesize, Errno> { unimplemented!() }

/// Write to a file descriptor. 
/// Note: This is similar to writev in POSIX.
fn fd_write(fd: Fd, iovs: CiovecArray) -> Result<Size, Errno> { unimplemented!() }

/// Create a directory. 
/// Note: This is similar to mkdirat in POSIX.
fn path_create_directory(fd: Fd, path: String) -> Result<(), Errno> { unimplemented!() }

/// Return the attributes of a file or directory. 
/// Note: This is similar to stat in POSIX.
/// 
/// The buffer where the file's attributes are stored.
fn path_filestat_get(fd: Fd, flags: Lookupflags, path: String) -> Result<Filestat, Errno> { unimplemented!() }

/// Adjust the timestamps of a file or directory. 
/// Note: This is similar to utimensat in POSIX.
fn path_filestat_set_times(fd: Fd, flags: Lookupflags, path: String, atim: Timestamp, mtim: Timestamp, fst_flags: Fstflags) -> Result<(), Errno> { unimplemented!() }

/// Create a hard link. 
/// Note: This is similar to linkat in POSIX.
fn path_link(old_fd: Fd, old_flags: Lookupflags, old_path: String, new_fd: Fd, new_path: String) -> Result<(), Errno> { unimplemented!() }

/// Open a file or directory. 
/// The returned file descriptor is not guaranteed to be the lowest-numbered file descriptor not currently open; 
/// it is randomized to prevent applications from depending on making assumptions about indexes, since this is error-prone in multi-threaded contexts. 
/// The returned file descriptor is guaranteed to be less than 2**31. 
/// Note: This is similar to openat in POSIX.
/// 
/// The file descriptor of the file that has been opened.
fn path_open(fd: Fd, dirflags: Lookupflags, path: String, oflags: Oflags, fs_rights_base: Rights, fs_rights_inheriting: Rights, fdflags: Fdflags) -> Result<Fd, Errno> { unimplemented!() }

/// Read the contents of a symbolic link. 
/// Note: This is similar to readlinkat in POSIX.
/// 
/// The number of bytes placed in the buffer.
fn path_readlink(fd: Fd, path: String, buf: *mut u8, buf_len: Size) -> Result<Size, Errno> { unimplemented!() }

/// Remove a directory. 
/// Return errno::notempty if the directory is not empty. 
/// Note: This is similar to unlinkat(fd, path, AT_REMOVEDIR) in POSIX.
fn path_remove_directory(fd: Fd, path: String) -> Result<(), Errno> { unimplemented!() }

/// Rename a file or directory. 
/// Note: This is similar to renameat in POSIX.
fn path_rename(fd: Fd, old_path: String, new_fd: Fd, new_path: String) -> Result<(), Errno> { unimplemented!() }

/// Create a symbolic link. 
/// Note: This is similar to symlinkat in POSIX.
fn path_symlink(old_path: String, fd: Fd, new_path: String) -> Result<(), Errno> { unimplemented!() }

/// Unlink a file. 
/// Return errno::isdir if the path refers to a directory. 
/// Note: This is similar to unlinkat(fd, path, 0) in POSIX.
fn path_unlink_file(fd: Fd, path: String) -> Result<(), Errno> { unimplemented!() }

/// Concurrently poll for the occurrence of a set of events.
/// 
/// The number of events stored.
fn poll_oneoff(in_: *const Subscription, out: *mut Event, nsubscriptions: Size) -> Result<Size, Errno> { unimplemented!() }

/// Terminate the process normally. 
/// An exit code of 0 indicates successful termination of the program. 
/// The meanings of other values is dependent on the environment.
fn proc_exit(rval: Exitcode) { unimplemented!() }

/// Send a signal to the process of the calling thread. 
/// Note: This is similar to raise in POSIX.
fn proc_raise(sig: Signal) -> Result<(), Errno> { unimplemented!() }

/// Temporarily yield execution of the calling thread. 
/// Note: This is similar to sched_yield in POSIX.
fn sched_yield() -> Result<(), Errno> { unimplemented!() }

/// Write high-quality random data into a buffer. 
/// This function blocks when the implementation is unable to immediately provide sufficient high-quality random data. 
/// This function may execute slowly, so when large mounts of random data are required, it's advisable to use this function to seed a pseudo-random number generator, rather than to provide the random data directly.
fn random_get(buf: *mut u8, buf_len: Size) -> Result<(), Errno> { unimplemented!() }

/// Receive a message from a socket. 
/// Note: This is similar to recv in POSIX, though it also supports reading the data into multiple buffers in the manner of readv.
/// 
/// Number of bytes stored in ri_data and message flags.
fn sock_recv(fd: Fd, ri_data: IovecArray, ri_flags: Riflags) -> Result<(Size, Roflags), Errno> { unimplemented!() }

/// Send a message on a socket. 
/// Note: This is similar to send in POSIX, though it also supports writing the data from multiple buffers in the manner of writev.
/// 
/// Number of bytes transmitted.
fn sock_send(fd: Fd, si_data: CiovecArray, si_flags: Siflags) -> Result<Size, Errno> { unimplemented!() }

/// Shut down socket send and receive channels. 
/// Note: This is similar to shutdown in POSIX.
fn sock_shutdown(fd: Fd, how: Sdflags) -> Result<(), Errno> { unimplemented!() }