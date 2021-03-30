use crate::{
    ModuleInst,
    Name,
    ExternVal,
    Error,
};

pub fn instance_export(moduleinst: ModuleInst, name: Name) -> Result<ExternVal, Error> {
    for exportinst in moduleinst.exports {
        if exportinst.name == name {
            return Ok(exportinst.value);
        }
    }
    Err(Error::Invalid)
}