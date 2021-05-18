use crate::{
    Module,
    FuncType,
    Func,
    func_alloc,
    FuncInst,
    HostFuncInst,
    Store,
    Val,
    Error,
};

pub fn allocate_funcs() -> Vec<HostFuncInst> {
    vec![
        HostFuncInst { tp: FuncType::default(), hostcode: args_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: args_sizes_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: environ_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: environ_sizes_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: clock_res_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: clock_time_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_advise, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_allocate, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_close, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_datasync, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_fdstat_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_fdstat_set_flags, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_fdstat_set_rights, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_filestat_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_filestat_set_size, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_filestat_set_times, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_pread, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_prestat_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_prestat_dir_name, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_pwrite, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_read, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_readdir, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_renumber, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_seek, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_sync, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_tell, },
        HostFuncInst { tp: FuncType::default(), hostcode: fd_write, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_create_directory, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_filestat_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_filestat_set_times, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_link, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_open, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_readlink, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_remove_directory, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_rename, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_symlink, },
        HostFuncInst { tp: FuncType::default(), hostcode: path_unlink_file, },
        HostFuncInst { tp: FuncType::default(), hostcode: poll_oneoff, },
        HostFuncInst { tp: FuncType::default(), hostcode: proc_exit, },
        HostFuncInst { tp: FuncType::default(), hostcode: proc_raise, },
        HostFuncInst { tp: FuncType::default(), hostcode: sched_yield, },
        HostFuncInst { tp: FuncType::default(), hostcode: random_get, },
        HostFuncInst { tp: FuncType::default(), hostcode: sock_recv, },
        HostFuncInst { tp: FuncType::default(), hostcode: sock_send, },
        HostFuncInst { tp: FuncType::default(), hostcode: sock_shutdown, },   
    ]
}

fn args_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn args_sizes_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn environ_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn environ_sizes_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn clock_res_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn clock_time_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_advise(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_allocate(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_close(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_datasync(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_fdstat_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_fdstat_set_flags(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_fdstat_set_rights(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_filestat_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_filestat_set_size(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_filestat_set_times(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_pread(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_prestat_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_prestat_dir_name(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_pwrite(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_read(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_readdir(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_renumber(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_seek(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_sync(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_tell(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn fd_write(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_create_directory(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_filestat_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_filestat_set_times(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_link(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_open(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_readlink(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_remove_directory(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_rename(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_symlink(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn path_unlink_file(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn poll_oneoff(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn proc_exit(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn proc_raise(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn sched_yield(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn random_get(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn sock_recv(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn sock_send(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }
fn sock_shutdown(store: &mut Store, args: Vec<Val>) -> Result<Vec<Val>, Error> { unimplemented!() }


pub fn get() -> Module {
    let types = types();
    let funcs = funcs();
    Module {
        id: Some("wasi_snapshot_preview1".to_string()),
        types: types,
        funcs: funcs,
        tables: Vec::default(),
        mems: Vec::default(),
        globals: Vec::default(),
        elem: Vec::default(),
        data: Vec::default(),
        start: None,
        imports: Vec::default(),
        exports: vec![],
    }
}

fn types() -> Vec<FuncType> {
    vec![]
}

fn funcs() -> Vec<Func> {
    vec![]
}