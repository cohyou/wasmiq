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

/// args_get(argv: Pointer<Pointer<u8>>, argv_buf: Pointer<u8>) -> Result<(), errno>
fn args_get(argv: *mut *mut u8, argv_buf: *mut u8) -> Result<(), Errno> { unimplemented!() }
/// args_sizes_get() -> Result<(size, size), errno>
fn args_sizes_get() -> Result<(Size, Size), Errno> { unimplemented!() }
/// environ_get(environ: Pointer<Pointer<u8>>, environ_buf: Pointer<u8>) -> Result<(), errno>
fn environ_get(environ: *mut *mut u8, environ_buf: *mut u8) -> Result<(), Errno> { unimplemented!() }
/// environ_sizes_get() -> Result<(size, size), errno>
fn environ_sizes_get() -> Result<(Size, Size), Errno> { unimplemented!() }
/// clock_res_get(id: clockid) -> Result<timestamp, errno>
fn clock_res_get(id: Clockid) -> Result<Timestamp, Errno> { unimplemented!() }
/// clock_time_get(id: clockid, precision: timestamp) -> Result<timestamp, errno>
fn clock_time_get(id: Clockid, precision: Timestamp) -> Result<Timestamp, Errno> { unimplemented!() }
/// fd_advise(fd: fd, offset: filesize, len: filesize, advice: advice) -> Result<(), errno>
fn fd_advise(fd: Fd, offset: Filesize, len: Filesize, advice: Advice) -> Result<(), Errno> { unimplemented!() }
/// fd_allocate(fd: fd, offset: filesize, len: filesize) -> Result<(), errno>
fn fd_allocate(fd: Fd, offset: Filesize, len: Filesize) -> Result<(), Errno> { unimplemented!() }
/// fd_close(fd: fd) -> Result<(), errno>
fn fd_close(fd: Fd) -> Result<(), Errno> { unimplemented!() }
/// fd_datasync(fd: fd) -> Result<(), errno>
fn fd_datasync(fd: Fd) -> Result<(), Errno> { unimplemented!() }
/// fd_fdstat_get(fd: fd) -> Result<fdstat, errno>
fn fd_fdstat_get(fd: Fd) -> Result<Fdstat, Errno> { unimplemented!() }
/// fd_fdstat_set_flags(fd: fd, flags: Fdflags) -> Result<(), errno>
fn fd_fdstat_set_flags(fd: Fd, flags: Fdflags) -> Result<(), Errno> { unimplemented!() }
/// fd_fdstat_set_rights(fd: fd, fs_rights_base: rights, fs_rights_inheriting: rights) -> Result<(), errno>
fn fd_fdstat_set_rights(fd: Fd, fs_rights_base: Rights, fs_rights_inheriting: Rights) -> Result<(), Errno> { unimplemented!() }
/// fd_filestat_get(fd: fd) -> Result<filestat, errno>
fn fd_filestat_get(fd: Fd) -> Result<Filestat, Errno> { unimplemented!() }
/// fd_filestat_set_size(fd: fd, size: filesize) -> Result<(), errno>
fn fd_filestat_set_size(fd: Fd, Size: Filesize) -> Result<(), Errno> { unimplemented!() }
/// fd_filestat_set_times(fd: fd, atim: timestamp, mtim: timestamp, fst_flags: fstflags) -> Result<(), errno>
fn fd_filestat_set_times(fd: Fd, atim: Timestamp, mtim: Timestamp, fst_flags: Fstflags) -> Result<(), Errno> { unimplemented!() }
/// fd_pread(fd: fd, iovs: iovec_array, offset: filesize) -> Result<size, errno>
fn fd_pread(fd: Fd, iovs: IovecArray, offset: Filesize) -> Result<Size, Errno> { unimplemented!() }
/// fd_prestat_get(fd: fd) -> Result<prestat, errno>
fn fd_prestat_get(fd: Fd) -> Result<Prestat, Errno> { unimplemented!() }
/// fd_prestat_dir_name(fd: fd, path: Pointer<u8>, path_len: size) -> Result<(), errno>
fn fd_prestat_dir_name(fd: Fd, path: *mut u8, path_len: Size) -> Result<(), Errno> { unimplemented!() }
/// fd_pwrite(fd: fd, iovs: ciovec_array, offset: filesize) -> Result<size, errno>
fn fd_pwrite(fd: Fd, iovs: CiovecArray, offset: Filesize) -> Result<Size, Errno> { unimplemented!() }
/// fd_read(fd: fd, iovs: iovec_array) -> Result<size, errno>
fn fd_read(fd: Fd, iovs: IovecArray) -> Result<Size, Errno> { unimplemented!() }
/// fd_readdir(fd: fd, buf: Pointer<u8>, buf_len: size, cookie: dircookie) -> Result<size, errno>
fn fd_readdir(fd: Fd, buf: *mut u8, buf_len: Size, cookie: Dircookie) -> Result<Size, Errno> { unimplemented!() }
/// fd_renumber(fd: fd, to: Fd) -> Result<(), errno>
fn fd_renumber(fd: Fd, to: Fd) -> Result<(), Errno> { unimplemented!() }
/// fd_seek(fd: fd, offset: filedelta, whence: whence) -> Result<filesize, errno>
fn fd_seek(fd: Fd, offset: Filedelta, whence: Whence) -> Result<Filesize, Errno> { unimplemented!() }
/// fd_sync(fd: fd) -> Result<(), errno> 
fn fd_sync(fd: Fd) -> Result<(), Errno>  { unimplemented!() }
/// fd_tell(fd: fd) -> Result<filesize, errno>
fn fd_tell(fd: Fd) -> Result<Filesize, Errno> { unimplemented!() }
/// fd_write(fd: fd, iovs: ciovec_array) -> Result<size, errno>
fn fd_write(fd: Fd, iovs: CiovecArray) -> Result<Size, Errno> { unimplemented!() }
/// path_create_directory(fd: fd, path: string) -> Result<(), errno>
fn path_create_directory(fd: Fd, path: String) -> Result<(), Errno> { unimplemented!() }
/// path_filestat_get(fd: fd, flags: lookupflags, path: string) -> Result<filestat, errno> 
fn path_filestat_get(fd: Fd, flags: Lookupflags, path: String) -> Result<Filestat, Errno> { unimplemented!() }
/// path_filestat_set_times(fd: fd, flags: lookupflags, path: string, atim: timestamp, mtim: timestamp, fst_flags: fstflags) -> Result<(), errno>
fn path_filestat_set_times(fd: Fd, flags: Lookupflags, path: String, atim: Timestamp, mtim: Timestamp, fst_flags: Fstflags) -> Result<(), Errno> { unimplemented!() }
/// path_link(old_fd: fd, old_flags: lookupflags, old_path: string, new_fd: fd, new_path: string) -> Result<(), errno>
fn path_link(old_fd: Fd, old_flags: Lookupflags, old_path: String, new_fd: Fd, new_path: String) -> Result<(), Errno> { unimplemented!() }
/// path_open(fd: fd, dirflags: lookupflags, path: string, oflags: oflags, fs_rights_base: rights, fs_rights_inheriting: rights, fdflags: fdflags) -> Result<fd, errno>
fn path_open(fd: Fd, dirflags: Lookupflags, path: String, oflags: Oflags, fs_rights_base: Rights, fs_rights_inheriting: Rights, fdflags: Fdflags) -> Result<Fd, Errno> { unimplemented!() }
/// path_readlink(fd: fd, path: string, buf: Pointer<u8>, buf_len: size) -> Result<size, errno>
fn path_readlink(fd: Fd, path: String, buf: *mut u8, buf_len: Size) -> Result<Size, Errno> { unimplemented!() }
/// path_remove_directory(fd: fd, path: string) -> Result<(), errno>
fn path_remove_directory(fd: Fd, path: String) -> Result<(), Errno> { unimplemented!() }
/// path_rename(fd: fd, old_path: string, new_fd: fd, new_path: string) -> Result<(), errno>
fn path_rename(fd: Fd, old_path: String, new_fd: Fd, new_path: String) -> Result<(), Errno> { unimplemented!() }
/// path_symlink(old_path: string, fd: fd, new_path: string) -> Result<(), errno>
fn path_symlink(old_path: String, fd: Fd, new_path: String) -> Result<(), Errno> { unimplemented!() }
/// path_unlink_file(fd: fd, path: string) -> Result<(), errno>
fn path_unlink_file(fd: Fd, path: String) -> Result<(), Errno> { unimplemented!() }
/// poll_oneoff(in: ConstPointer<subscription>, out: Pointer<event>, nsubscriptions: size) -> Result<size, errno>
fn poll_oneoff(in_: *const Subscription, out: *mut Event, nsubscriptions: Size) -> Result<Size, Errno> { unimplemented!() }
/// proc_exit(rval: exitcode)
fn proc_exit(rval: Exitcode) { unimplemented!() }
/// proc_raise(sig: signal) -> Result<(), errno>
fn proc_raise(sig: Signal) -> Result<(), Errno> { unimplemented!() }
/// sched_yield() -> Result<(), errno> 
fn sched_yield() -> Result<(), Errno> { unimplemented!() }
/// random_get(buf: Pointer<u8>, buf_len: size) -> Result<(), errno>
fn random_get(buf: *mut u8, buf_len: Size) -> Result<(), Errno> { unimplemented!() }
/// sock_recv(fd: fd, ri_data: iovec_array, ri_flags: riflags) -> Result<(size, roflags), errno>
fn sock_recv(fd: Fd, ri_data: IovecArray, ri_flags: Riflags) -> Result<(Size, Roflags), Errno> { unimplemented!() }
/// sock_send(fd: fd, si_data: ciovec_array, si_flags: siflags) -> Result<size, errno>
fn sock_send(fd: Fd, si_data: CiovecArray, si_flags: Siflags) -> Result<Size, Errno> { unimplemented!() }
/// sock_shutdown(fd: fd, how: sdflags) -> Result<(), errno>
fn sock_shutdown(fd: Fd, how: Sdflags) -> Result<(), Errno> { unimplemented!() }