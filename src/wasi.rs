mod wasi_snapshot_preview1;
mod module;

pub use module::allocate_funcs as snapshot_preview1;

/// File or memory access pattern advisory information.
enum Advice {
    /// The application has no advice to give on its behavior with respect to the specified data.
    Normal,
    /// The application expects to access the specified data sequentially from lower offsets to higher offsets.
    Sequential,
    /// The application expects to access the specified data in a random order.
    Random,
    /// The application expects to access the specified data in the near future.
    Willneed,
    /// The application expects that it will not access the specified data in the near future.
    Dontneed,
    /// The application expects to access the specified data once and then not reuse it thereafter.
    Noreuse,
}

/// A region of memory for scatter/gather writes.
struct Ciovec {
    /// The address of the buffer to be written.
    buf: *const u8,
    /// The length of the buffer to be written.
    buf_len: Size,
}

type CiovecArray = Vec<Ciovec>;

/// Identifiers for clocks.
enum Clockid {
    /// The clock measuring real time. Time value zero corresponds with 1970-01-01T00:00:00Z.
    Realtime,
    /// The store-wide monotonic clock, which is defined as a clock measuring real time, whose value cannot be adjusted and which cannot have negative clock jumps. The epoch of this clock is undefined. The absolute time value of this clock therefore has no meaning.
    Monotonic,
    /// The CPU-time clock associated with the current process.
    ProcessCputimeId,
    /// The CPU-time clock associated with the current thread.
    ThreadCputimeId,
}

/// Identifier for a device containing a file system. Can be used in combination with inode to uniquely identify a file or directory in the filesystem.
type Device = u64;

/// A reference to the offset of a directory entry.
/// The value 0 signifies the start of the directory.
type Dircookie = u64;

/// A directory entry.
struct Dirent {
    d_next: Dircookie,
    d_ino: Inode,
    d_namlen: Dirnamlen,
    d_type: Filetype,
}

/// The type for the dirent::d_namlen field of dirent struct.
type Dirnamlen = u32;

/// Error codes returned by functions. Not all of these error codes are returned by the functions provided by this API; some are used in higher-level library layers, and others are provided merely for alignment with POSIX.
enum Errno {
    /// No error occurred. System call completed successfully.
    Success,
    /// Argument list too long.
    Toobig,
    /// Permission denied.
    Acces,
    /// Address in use.
    Addrinuse,
    /// Address not available.
    Addrnotavail,
    /// Address family not supported.
    Afnosupport,
    /// Resource unavailable, or operation would block.
    Again,
    /// Connection already in progress.
    Already,
    /// Bad file descriptor.
    Badf,
    /// Bad message.
    Badmsg,
    /// Device or resource busy.
    Busy,
    /// Operation canceled.
    Canceled,
    /// No child processes.
    Child,
    /// Connection aborted.
    Connaborted,
    /// Connection refused.
    Connrefused,
    /// Connection reset.
    Connreset,
    /// Resource deadlock would occur.
    Deadlk,
    /// Destination address required.
    Destaddrreq,
    /// Mathematics argument out of domain of function.
    Dom,
    /// Reserved.
    Dquot,
    /// File exists.
    Exist,
    /// Bad address.
    Fault,
    /// File too large.
    Fbig,
    /// Host is unreachable.
    Hostunreach,
    /// Identifier removed.
    Idrm,
    /// Illegal byte sequence.
    Ilseq,
    /// Operation in progress.
    Inprogress,
    /// Interrupted function.
    Intr,
    /// Invalid argument.
    Inval,
    /// I/O error.
    Io,
    /// Socket is connected.
    Isconn,
    /// Is a directory.
    Isdir,
    /// Too many levels of symbolic links. 
    Loop,
    /// File descriptor value too large.
    Mfile,
    /// Too many links.
    Mlink,
    /// Message too large.
    Msgsize,
    /// Reserved.
    Multihop,
    /// Filename too long.
    Nametoolong,
    /// Network is down.
    Netdown,
    /// Connection aborted by network.
    Netreset,
    /// Network unreachable.
    Netunreach,
    /// Too many files open in system.
    Nfile,
    /// No buffer space available.
    Nobufs,
    /// No such device.
    Nodev,
    /// No such file or directory.
    Noent,
    /// Executable file format error.
    Noexec,
    /// No locks available.
    Nolck,
    /// Reserved.
    Nolink,
    /// Not enough space.
    Nomem,
    /// No message of the desired type.
    Nomsg,
    /// Protocol not available.
    Noprotoopt,
    /// No space left on device.
    Nospc,
    /// Function not supported.
    Nosys,
    /// The socket is not connected.
    Notconn,
    /// Not a directory or a symbolic link to a directory.
    Notdir,
    /// Directory not empty.
    Notempty,
    /// State not recoverable.
    Notrecoverable,
    /// Not a socket.
    Notsock,
    /// Not supported, or operation not supported on socket.
    Notsup,
    /// Inappropriate I/O control operation.
    Notty,
    /// No such device or address.
    Nxio,
    /// Value too large to be stored in data type.
    Overflow,
    /// Previous owner died.
    Ownerdead,
    /// Operation not permitted.
    Perm,
    /// Broken pipe.
    Pipe,
    /// Protocol error.
    Proto,
    /// Protocol not supported.
    ProtonoSupport,
    /// Protocol wrong type for socket.
    Prototype,
    /// Result too large.
    Range,
    /// Read-only file system.
    Rofs,
    /// Invalid seek.
    Spipe,
    /// No such process.
    Srch,
    /// Reserved.
    Stale,
    /// Connection timed out.
    Timedout,
    /// Text file busy.
    Txtbsy,
    /// Cross-device link.
    Xdev,
    /// Extension: Capabilities insufficient.
    Notcapable,
}

/// An event that occurred.
struct Event {
    /// User-provided value that got attached to subscription::userdata.
    user_data: Userdata,
    /// If non-zero, an error that occurred while processing the subscription request.
    error: Errno,
    /// The type of event that occured
    tp: Eventtype,
    /// The contents of the event, if it is an eventtype::fd_read or eventtype::fd_write. eventtype::clock events ignore this field.
    fd_readwrite: EventFdReadwrite
}

/// The contents of an event when type is eventtype::fd_read or eventtype::fd_write.
struct EventFdReadwrite {
    /// The number of bytes available for reading or writing.
    nbytes: Filesize,
    /// The state of the file descriptor.
    flags: Eventrwflags,
}

/// The state of the file descriptor subscribed to with eventtype::fd_read or eventtype::fd_write.
struct Eventrwflags {
    /// The peer of this socket has closed or disconnected.
    fd_readwrite_hangup: bool,
}

/// Type of a subscription to an event or its occurrence.
enum Eventtype {
    /// The time value of clock subscription_clock::id has reached timestamp subscription_clock::timeout.
    Clock,
    /// File descriptor subscription_fd_readwrite::file_descriptor has data available for reading. This event always triggers for regular files.
    FdRead,
    /// File descriptor subscription_fd_readwrite::file_descriptor has capacity available for writing. This event always triggers for regular files.
    FdWrite,
}

/// Exit code generated by a process when exiting.
type Exitcode = u32;

/// A file descriptor handle.
type Fd = Handle;

type Handle = u32;

/// File descriptor flags.
struct Fdflags {
    /// Append mode: Data written to the file is always appended to the file's end.
    append: bool,
    /// Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.
    dsync: bool,
    /// Non-blocking mode.
    nonblock: bool,
    /// Synchronized read I/O operations.
    rsync: bool,
    /// Write according to synchronized I/O file integrity completion. In addition to synchronizing the data stored in the file, the implementation may also synchronously update the file's metadata.
    sync: bool,
}

/// File descriptor attributes.
struct Fdstat {
    /// File type.
    fs_filetype: Filetype,
    /// File descriptor flags.
    fs_flags: Fdflags,
    /// Rights that apply to this file descriptor.
    fs_rights_base: Rights,
    /// Maximum set of rights that may be installed on new file descriptors that are created through this file descriptor, e.g., through path_open.
    fs_rights_inheriting: Rights,
}

/// Relative offset within a file.
type Filedelta = i64;

/// Non-negative file size or length of a region within a file.
type Filesize = u64;

/// File attributes.
struct Filestat {
    /// Device ID of device containing the file.
    dev: Device,
    /// File serial number.
    ino: Inode,
    /// File type.
    filetype: Filetype,
    /// Number of hard links to the file.
    nlink: Linkcount,
    /// For regular files, the file size in bytes. For symbolic links, the length in bytes of the pathname contained in the symbolic link.
    size: Filesize,
    /// Last data access timestamp.
    atim: Timestamp,
    /// Last data modification timestamp.
    mtim: Timestamp,
    /// Last file status change timestamp.
    ctim: Timestamp,
}

/// The type of a file descriptor or file.
enum Filetype {
    /// The type of the file descriptor or file is unknown or is different from any of the other types specified.
    Unknown,
    /// The file descriptor or file refers to a block device inode.
    BlockDevice,
    /// The file descriptor or file refers to a character device inode.
    CharacterDevice,
    /// The file descriptor or file refers to a directory inode.
    Directory,
    /// The file descriptor or file refers to a regular file inode.
    RegularFile,
    /// The file descriptor or file refers to a datagram socket.
    SocketDgram,
    /// The file descriptor or file refers to a byte-stream socket.
    SocketStream,
    /// The file refers to a symbolic link inode.
    SymbolicLink,
}

/// Which file time attributes to adjust.
struct Fstflags {
    /// Adjust the last data access timestamp to the value stored in filestat::atim.
    atim: bool,
    /// Adjust the last data access timestamp to the time of clock clockid::realtime.
    atim_now: bool,
    /// Adjust the last data modification timestamp to the value stored in filestat::mtim.
    mtim: bool,
    /// Adjust the last data modification timestamp to the time of clock clockid::realtime.
    mtim_now: bool,
}

/// File serial number that is unique within its file system.
type Inode = u64;

/// A region of memory for scatter/gather reads.
struct Iovec {
    /// The address of the buffer to be filled.
    buf: *mut u8,
    /// The length of the buffer to be filled.
    buf_len: Size,
}

type IovecArray = Vec<Iovec>;

/// Number of hard links to an inode.
type Linkcount = u64;

/// Flags determining the method of how paths are resolved.
struct Lookupflags {
    /// As long as the resolved path corresponds to a symbolic link, it is expanded.
    symlink_follow: bool,
}

/// Open flags used by path_open.
struct Oflags {
    /// Create file if it does not exist.
    creat: bool,
    /// Fail if not a directory.
    directory: bool,
    /// Fail if file already exists.
    excl: bool,
    /// Truncate file to size 0.
    trunc: bool,
}

/// Identifiers for preopened capabilities.
enum Preopentype {
    /// A pre-opened directory.
    Dir,
}

/// Information about a pre-opened capability.
struct Prestat {
    /// 
    dir: PrestatDir,
}

/// The contents of a $prestat when type is preopentype::dir.
struct PrestatDir {
    /// The length of the directory name for use with fd_prestat_dir_name.
    pr_name_len: Size,
}

/// Flags provided to sock_recv.
struct Riflags {
    /// Returns the message without removing it from the socket's receive queue.
    recv_peek: bool,
    /// On byte-stream sockets, block until the full amount of data can be returned.
    recv_waitall: bool,
}

/// File descriptor rights, determining which actions may be performed.
struct Rights {
    /// The right to invoke fd_datasync. If path_open is set, includes the right to invoke path_open with fdflags::dsync.
    fd_datasync: bool,
    /// The right to invoke fd_read and sock_recv. If rights::fd_seek is set, includes the right to invoke fd_pread.
    fd_read: bool,
    /// The right to invoke fd_seek. This flag implies rights::fd_tell.
    fd_seek: bool,
    /// The right to invoke fd_fdstat_set_flags.
    fd_fdstat_set_flags: bool,
    /// The right to invoke fd_sync. If path_open is set, includes the right to invoke path_open with fdflags::rsync and fdflags::dsync.
    fd_sync: bool,
    /// The right to invoke fd_seek in such a way that the file offset remains unaltered (i.e., whence::cur with offset zero), or to invoke fd_tell.
    fd_tell: bool,
    /// The right to invoke fd_write and sock_send. If rights::fd_seek is set, includes the right to invoke fd_pwrite.
    fd_write: bool,
    /// The right to invoke fd_advise.
    fd_advise: bool,
    /// The right to invoke fd_allocate.
    fd_allocate: bool,
    /// The right to invoke path_create_directory.
    path_create_direcotry: bool,
    /// If path_open is set, the right to invoke path_open with oflags::creat.
    path_create_file: bool,
    /// The right to invoke path_link with the file descriptor as the source directory.
    path_link_source: bool,
    /// The right to invoke path_link with the file descriptor as the target directory.
    path_link_target: bool,
    /// The right to invoke path_open.
    path_open: bool,
    /// The right to invoke fd_readdir.
    fd_readdir: bool,
    /// The right to invoke path_readlink.
    path_readlink: bool,
    /// The right to invoke path_rename with the file descriptor as the source directory.
    path_rename_source: bool,
    /// The right to invoke path_rename with the file descriptor as the target directory.
    path_rename_target: bool,
    /// The right to invoke path_filestat_get.
    path_filestat_get: bool,
    /// The right to change a file's size (there is no path_filestat_set_size). If path_open is set, includes the right to invoke path_open with oflags::trunc.
    path_filestat_set_size: bool,
    /// The right to invoke path_filestat_set_times.
    path_filestat_set_times: bool,
    /// The right to invoke fd_filestat_set_size.
    fd_filestat_set_size: bool,
    /// The right to invoke fd_filestat_set_times.
    fd_filestat_set_times: bool,
    /// The right to invoke path_symlink.
    path_symlink: bool,
    /// The right to invoke path_remove_directory.
    path_remove_directory: bool,
    /// The right to invoke path_unlink_file.
    path_unlink_file: bool,
    /// If rights::fd_read is set, includes the right to invoke poll_oneoff to subscribe to eventtype::fd_read. If rights::fd_write is set, includes the right to invoke poll_oneoff to subscribe to eventtype::fd_write.
    poll_fd_readwrite: bool,
    /// The right to invoke sock_shutdown.
    sock_shutdown: bool,
}

/// Flags returned by sock_recv.
struct Roflags {
    /// Returned by sock_recv: Message data has been truncated.
    recv_data_truncated: bool,
}

/// Which channels on a socket to shut down.
struct Sdflags {
    /// Disables further receive operations.
    rd: bool,
    /// Disables further send operations.
    wr: bool,
}

/// Flags provided to sock_send. As there are currently no flags defined, it must be set to zero.
type Siflags = u16;

/// Signal condition.
enum Signal {
    /// No signal. Note that POSIX has special semantics for kill(pid, 0), so this value is reserved.
    None,
    /// Hangup. Action: Terminates the process.
    Hup,
    /// Terminate interrupt signal. Action: Terminates the process.
    Int,
    /// Terminal quit signal. Action: Terminates the process.
    Quit,
    /// Illegal instruction. Action: Terminates the process.
    Ill,
    /// Trace/breakpoint trap. Action: Terminates the process.
    Trap,
    /// Process abort signal. Action: Terminates the process. 
    Abrt,
    /// Access to an undefined portion of a memory object. Action: Terminates the process.
    Bus,
    /// Erroneous arithmetic operation. Action: Terminates the process.
    Fpe,
    /// Kill. Action: Terminates the process.
    Kill,
    /// User-defined signal 1. Action: Terminates the process.
    Usr1,
    /// Invalid memory reference. Action: Terminates the process.
    Segv,
    /// User-defined signal 2. Action: Terminates the process.
    Usr2,
    /// Write on a pipe with no one to read it. Action: Ignored.
    Pipe,
    /// Alarm clock. Action: Terminates the process.
    Alrm,
    /// Termination signal. Action: Terminates the process.
    Term,
    /// Child process terminated, stopped, or continued. Action: Ignored.
    Chld,
    /// Continue executing, if stopped. Action: Continues executing, if stopped.
    Cont,
    /// Stop executing. Action: Stops executing.
    Stop,
    /// Terminal stop signal. Action: Stops executing.
    Tstp,
    /// Background process attempting read. Action: Stops executing.
    Ttin,
    /// Background process attempting write. Action: Stops executing.
    Ttou,
    /// High bandwidth data is available at a socket. Action: Ignored.
    Urg,
    /// CPU time limit exceeded. Action: Terminates the process.
    Xcpu,
    /// File size limit exceeded. Action: Terminates the process.
    Xfsz,
    /// Virtual timer expired. Action: Terminates the process.
    Vtalrm,
    /// Profiling timer expired. Action: Terminates the process.
    Prof,
    /// Window changed. Action: Ignored.
    Winch,
    /// I/O possible. Action: Terminates the process.
    Poll,
    /// Power failure. Action: Terminates the process.
    Pwr,
    /// Bad system call. Action: Terminates the process.
    Sys,
}

type Size = u32;

/// Flags determining how to interpret the timestamp provided in subscription_clock::timeout.
struct Subclockflags {
    /// If set, treat the timestamp provided in subscription_clock::timeout as an absolute timestamp of clock subscription_clock::id. If clear, treat the timestamp provided in subscription_clock::timeout relative to the current time value of clock subscription_clock::id.
    subscription_clock_abstime: bool,
}

/// Subscription to an event.
struct Subscription {
    /// User-provided value that is attached to the subscription in the implementation and returned through event::userdata.
    userdata: Userdata,
    /// The type of the event to which to subscribe, and its contents
    u: SubscriptionU,
}

/// The contents of a subscription when type is eventtype::clock.
struct SubscriptionClock {
    /// The clock against which to compare the timestamp.
    id: Clockid,
    /// The absolute or relative timestamp.
    timeout: Timestamp,
    /// The amount of time that the implementation may wait additionally to coalesce with other events.
    precision: Timestamp,
    /// Flags specifying whether the timeout is absolute or relative
    flags: Subclockflags,
}

/// The contents of a subscription when type is type is eventtype::fd_read or eventtype::fd_write.
struct SubscriptionFdReadwrite {
    /// The file descriptor on which to wait for it to become ready for reading or writing.
    file_descriptor: Fd,
}

/// The contents of a subscription.
enum SubscriptionU {
    /// 
    Clock,
    ///
    FdRead,
    ///
    FdWrite,
}

/// Timestamp in nanoseconds.
type Timestamp = u64;

/// User-provided value that may be attached to objects that is retained when extracted from the implementation.
type Userdata = u64;

/// The position relative to which to set the offset of the file descriptor.
enum Whence {
    /// Seek relative to start-of-file.
    Set,
    /// Seek relative to current position.
    Cur,
    /// Seek relative to end-of-file.
    End,
}