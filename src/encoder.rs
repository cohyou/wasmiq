use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;

use super::*;
use instr::*;
// use context::*;

type Byte = u8;

pub fn module_encode(module: &Module) -> std::io::Result<()> {
    let mut file = File::create("wasm/_.wasm")?;
    file.write_all(&module2wasm(&module))?;
    Ok(())
}

fn module2wasm(module: &Module) -> Vec<Byte> {
    [
        b"\0asm".to_vec(),
        vec![0x01, 0x00, 0x00, 0x00],
        typesection2wasm(&module.types),
        importsection2wasm(&module.imports),
        funcsection2wasm(&module.funcs),
        tablesection2wasm(&module.tables),
        memorysection2wasm(&module.mems),
        globalsection2wasm(&module.globals),
        exportsection2wasm(&module.exports),
        startsection2wasm(&module.start),
        elementsection2wasm(&module.elem),
        codesection2wasm(&module.funcs),
        datasection2wasm(&module.data),
    ]
    .concat()
}

fn typesection2wasm(types: &Vec<FuncType>) -> Vec<Byte> {
    section2wasm(1, vector2wasm(types.iter().map(functype2wasm).collect()))
}

fn importsection2wasm(imps: &Vec<Import>) -> Vec<Byte> {
    section2wasm(2, vector2wasm(imps.iter().map(import2wasm).collect())) 
}

fn funcsection2wasm(funcs: &Vec<Func>) -> Vec<Byte> {
    let typeindices = funcs.iter().map(|f| &f.tp).map(typeidx2wasm).collect::<Vec<Vec<Byte>>>().concat();
    section2wasm(3, bytevector2wasm(typeindices))
}

fn tablesection2wasm(tables: &Vec<Table>) -> Vec<Byte> {
    section2wasm(4, vector2wasm(tables.iter().map(table2wasm).collect())) 
}

fn memorysection2wasm(mems: &Vec<Mem>) -> Vec<Byte> {
    section2wasm(5, vector2wasm(mems.iter().map(mem2wasm).collect()))
}

fn globalsection2wasm(globals: &Vec<Global>) -> Vec<Byte> {
    section2wasm(6, vector2wasm(globals.iter().map(global2wasm).collect()))
}

fn exportsection2wasm(exps: &Vec<Export>) -> Vec<Byte> {
    section2wasm(7, vector2wasm(exps.iter().map(export2wasm).collect())) 
}

fn startsection2wasm(stt: &Option<Start>) -> Vec<Byte> {
    if let Some(start) = stt {
        section2wasm(8, funcidx2wasm(&start.0)) 
    } else {
        vec![]
    }
}

fn elementsection2wasm(elems: &Vec<Elem>) -> Vec<Byte> {
    section2wasm(9, vector2wasm(elems.iter().map(elem2wasm).collect())) 
}

fn codesection2wasm(funcs: &Vec<Func>) -> Vec<Byte> {
    section2wasm(10, vector2wasm(funcs.iter().map(code2wasm).collect())) 
}

fn datasection2wasm(data: &Vec<Data>) -> Vec<Byte> {
    section2wasm(11, vector2wasm(data.iter().map(data2wasm).collect())) 
}

fn import2wasm(imp: &Import) -> Vec<Byte> {
    [
        name2wasm(&imp.module),
        name2wasm(&imp.name),
        importdesc2wasm(&imp.desc),
    ]
    .concat()
}

fn importdesc2wasm(desc: &ImportDesc) -> Vec<Byte> {
    match desc {
        ImportDesc::Func(idx) =>  [vec![0x00], typeidx2wasm(idx)].concat(),
        ImportDesc::Table(tt) =>  [vec![0x01], tabletype2wasm(tt)].concat(),
        ImportDesc::Mem(mt) =>    [vec![0x02], memtype2wasm(mt)].concat(),
        ImportDesc::Global(gt) => [vec![0x03], globaltype2wasm(gt)].concat(),
    }
}

fn table2wasm(t: &Table) -> Vec<Byte> { tabletype2wasm(&t.0) }

fn mem2wasm(mem: &Mem) -> Vec<Byte> { memtype2wasm(&mem.0) }

fn global2wasm(global: &Global) -> Vec<Byte> {
    [
        globaltype2wasm(&global.tp),
        expr2wasm(&global.init),
    ]
    .concat()
}

fn export2wasm(exp: &Export) -> Vec<Byte> {
    [
        name2wasm(&exp.name),        
        exportdesc2wasm(&exp.desc),
    ]
    .concat()
}

fn exportdesc2wasm(desc: &ExportDesc) -> Vec<Byte> {
    match desc {
        ExportDesc::Func(idx) =>   [vec![0x00], typeidx2wasm(idx)].concat(),
        ExportDesc::Table(idx) =>  [vec![0x01], tableidx2wasm(idx)].concat(),
        ExportDesc::Mem(idx) =>    [vec![0x02], memidx2wasm(idx)].concat(),
        ExportDesc::Global(idx) => [vec![0x03], globalidx2wasm(idx)].concat(),
    }
}

fn elem2wasm(elem: &Elem) -> Vec<Byte> {
    [
        tableidx2wasm(&elem.table),
        expr2wasm(&elem.offset),
        bytevector2wasm(elem.init.iter().map(funcidx2wasm)
            .collect::<Vec<Vec<Byte>>>().concat()),
    ]
    .concat()
}

fn code2wasm(func: &Func) -> Vec<Byte> {
    let f = func2wasm(func);
    [
        unsigned32_to_wasm(f.len().try_into().unwrap()),
        f, 
    ]
    .concat()
}

fn func2wasm(func: &Func) -> Vec<Byte> {
    [
        func.locals.iter().map(local2wasm)
            .collect::<Vec<Vec<Byte>>>().concat(),
        expr2wasm(&func.body),
    ]
    .concat()
}

fn local2wasm(local: &ValType) -> Vec<Byte> {
    [
        0x01,  // TODO: to be compressed
        valtype2wasm(local),
    ]
    .to_vec()
}

fn data2wasm(data: &Data) -> Vec<Byte> {
    [
        memidx2wasm(&data.data),
        expr2wasm(&data.offset),
        datastring2wasm(&data.init),
    ]
    .concat()
}

fn datastring2wasm(_ds: &Vec<Byte>) -> Vec<Byte> {
    // string2wasm(ds)
    unimplemented!()
}

fn section2wasm(id: Byte, cont: Vec<Byte>) -> Vec<Byte> {
    [
        vec![id],
        unsigned32_to_wasm(cont.len().try_into().unwrap()),
        cont,
    ]
    .concat()
}

fn typeidx2wasm(idx: &TypeIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }
fn funcidx2wasm(idx: &FuncIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }
fn tableidx2wasm(idx: &TableIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }
fn memidx2wasm(idx: &MemIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }
fn globalidx2wasm(idx: &GlobalIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }
fn localidx2wasm(idx: &LocalIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }
fn labelidx2wasm(idx: &LabelIdx) -> Vec<Byte> { unsigned32_to_wasm(*idx) }

fn expr2wasm(expr: &Expr) -> Vec<Byte> {
    instrs2wasm(&expr.0)
}

fn instrs2wasm(instrs: &Vec<Instr>) -> Vec<Byte> {
    [
        instrs.iter().map(instr2wasm).collect::<Vec<Vec<Byte>>>().concat(),
        vec![0x0B],
    ]
    .concat()
}

fn instr2wasm(instr: &Instr) -> Vec<Byte> {
    match instr {
        Instr::Unreachable => vec![0x00],
        Instr::Nop => vec![0x01],
        Instr::Block(rt, instrs) => [
            vec![0x02], blocktype2wasm(rt), instrs2wasm(instrs), vec![0x0B]
        ].concat(),
        Instr::Loop(rt, instrs) => [
            vec![0x03], blocktype2wasm(rt), instrs2wasm(instrs), vec![0x0B]
        ].concat(),
        Instr::If(rt, instrs1, instrs2) => {
            let true_term = [vec![0x04], blocktype2wasm(rt), instrs2wasm(instrs1)].concat();
            if let Some(instr2) = instrs2 {
                [true_term, vec![0x05], instrs2wasm(instr2), vec![0x0B]].concat()
            } else {
                [true_term, vec![0x0B]].concat()
            }
        },
        Instr::Br(labelidx) => [vec![0x0C], labelidx2wasm(labelidx)].concat(),
        Instr::BrIf(labelidx) => [vec![0x0D], labelidx2wasm(labelidx)].concat(),
        Instr::BrTable(indices, idx) => [
            vec![0x0E], 
            vector2wasm(indices.iter().map(labelidx2wasm).collect()), 
            labelidx2wasm(idx)
        ].concat(),
        Instr::Return => vec![0x0F],
        Instr::Call(funcidx) => [vec![0x10], funcidx2wasm(funcidx)].concat(),
        Instr::CallIndirect(typeidx) => [vec![0x11], typeidx2wasm(typeidx), vec![0x00]].concat(),

        Instr::Drop(_) => vec![0x1A],
        Instr::Select(_) => vec![0x1B],

        Instr::LocalGet(x) => [vec![0x20], localidx2wasm(x)].concat(),
        Instr::LocalSet(x) => [vec![0x21], localidx2wasm(x)].concat(),
        Instr::LocalTee(x) => [vec![0x22], localidx2wasm(x)].concat(),
        Instr::GlobalGet(x) => [vec![0x23], globalidx2wasm(x)].concat(),
        Instr::GlobalSet(x) => [vec![0x24], globalidx2wasm(x)].concat(),

        Instr::Load(ValType::I32, memarg) => [vec![0x28], memarg2wasm(memarg)].concat(),
        Instr::Load(ValType::I64, memarg) => [vec![0x29], memarg2wasm(memarg)].concat(),
        Instr::Load(ValType::F32, memarg) => [vec![0x2A], memarg2wasm(memarg)].concat(),
        Instr::Load(ValType::F64, memarg) => [vec![0x2B], memarg2wasm(memarg)].concat(),
        Instr::ILoad8(ValSize::V32, ValSign::S, memarg) => [vec![0x2C], memarg2wasm(memarg)].concat(),
        Instr::ILoad8(ValSize::V32, ValSign::U, memarg) => [vec![0x2D], memarg2wasm(memarg)].concat(),
        Instr::ILoad16(ValSize::V32, ValSign::S, memarg) => [vec![0x2E], memarg2wasm(memarg)].concat(),
        Instr::ILoad16(ValSize::V32, ValSign::U, memarg) => [vec![0x2F], memarg2wasm(memarg)].concat(),

        Instr::ILoad8(ValSize::V64, ValSign::S, memarg) => [vec![0x30], memarg2wasm(memarg)].concat(),
        Instr::ILoad8(ValSize::V64, ValSign::U, memarg) => [vec![0x31], memarg2wasm(memarg)].concat(),
        Instr::ILoad16(ValSize::V64, ValSign::S, memarg) => [vec![0x32], memarg2wasm(memarg)].concat(),
        Instr::ILoad16(ValSize::V64, ValSign::U, memarg) => [vec![0x33], memarg2wasm(memarg)].concat(),
        Instr::I64Load32(ValSign::S, memarg) => [vec![0x34], memarg2wasm(memarg)].concat(),
        Instr::I64Load32(ValSign::U, memarg) => [vec![0x35], memarg2wasm(memarg)].concat(),

        Instr::Store(ValType::I32, memarg) => [vec![0x36], memarg2wasm(memarg)].concat(),
        Instr::Store(ValType::I64, memarg) => [vec![0x37], memarg2wasm(memarg)].concat(),
        Instr::Store(ValType::F32, memarg) => [vec![0x38], memarg2wasm(memarg)].concat(),
        Instr::Store(ValType::F64, memarg) => [vec![0x39], memarg2wasm(memarg)].concat(),

        Instr::IStore8(ValSize::V32, memarg) => [vec![0x3A], memarg2wasm(memarg)].concat(),
        Instr::IStore16(ValSize::V32, memarg) => [vec![0x3B], memarg2wasm(memarg)].concat(),
        Instr::IStore8(ValSize::V64, memarg) => [vec![0x3C], memarg2wasm(memarg)].concat(),
        Instr::IStore16(ValSize::V64, memarg) => [vec![0x3D], memarg2wasm(memarg)].concat(),
        Instr::I64Store32(memarg) => [vec![0x3E], memarg2wasm(memarg)].concat(),
        Instr::MemorySize => vec![0x3F, 0x00],
        Instr::MemoryGrow => vec![0x40, 0x00],

        Instr::I32Const(n) => [vec![0x41], unsigned32_to_wasm(*n)].concat(),
        Instr::I64Const(n) => [vec![0x42], unsigned64_to_wasm(*n)].concat(),
        Instr::F32Const(n) => [vec![0x43], n.to_bits().to_le_bytes().to_vec()].concat(),
        Instr::F64Const(n) => [vec![0x44], n.to_bits().to_le_bytes().to_vec()].concat(),

        Instr::ITestOp(vs, ITestOp::Eqz) => {
            match vs {
                ValSize::V32 => vec![0x45],
                ValSize::V64 => vec![0x50],
            }
        },
        Instr::IRelOp(vs, irelop) => {
            match vs {
                ValSize::V32 => {
                    match irelop {
                        IRelOp::Eq => vec![0x46],
                        IRelOp::Ne => vec![0x47],
                        IRelOp::Lt(ValSign::S) => vec![0x48],
                        IRelOp::Lt(ValSign::U) => vec![0x49],
                        IRelOp::Gt(ValSign::S) => vec![0x4A],
                        IRelOp::Gt(ValSign::U) => vec![0x4B],
                        IRelOp::Le(ValSign::S) => vec![0x4C],
                        IRelOp::Le(ValSign::U) => vec![0x4D],
                        IRelOp::Ge(ValSign::S) => vec![0x4E],
                        IRelOp::Ge(ValSign::U) => vec![0x4F],
                    }
                },
                ValSize::V64 => {
                    match irelop {
                        IRelOp::Eq => vec![0x51],
                        IRelOp::Ne => vec![0x52],
                        IRelOp::Lt(ValSign::S) => vec![0x53],
                        IRelOp::Lt(ValSign::U) => vec![0x54],
                        IRelOp::Gt(ValSign::S) => vec![0x55],
                        IRelOp::Gt(ValSign::U) => vec![0x56],
                        IRelOp::Le(ValSign::S) => vec![0x57],
                        IRelOp::Le(ValSign::U) => vec![0x58],
                        IRelOp::Ge(ValSign::S) => vec![0x59],
                        IRelOp::Ge(ValSign::U) => vec![0x5A],
                    }
                },
            }
        },
        Instr::FRelOp(vs, frelop) => {
            match vs {
                ValSize::V32 => {
                    match frelop {
                        FRelOp::Eq => vec![0x5B],
                        FRelOp::Ne => vec![0x5C],
                        FRelOp::Lt => vec![0x5D],
                        FRelOp::Gt => vec![0x5E],
                        FRelOp::Le => vec![0x5F],
                        FRelOp::Ge => vec![0x60],
                    }
                },
                ValSize::V64 => {
                    match frelop {
                        FRelOp::Eq => vec![0x61],
                        FRelOp::Ne => vec![0x62],
                        FRelOp::Lt => vec![0x63],
                        FRelOp::Gt => vec![0x64],
                        FRelOp::Le => vec![0x65],
                        FRelOp::Ge => vec![0x66],
                    }
                },
            }
        },
        Instr::IUnOp(vs, iunop) => {
            match vs {
                ValSize::V32 => {
                    match iunop {
                        IUnOp::Clz => vec![0x67],
                        IUnOp::Ctz => vec![0x68],
                        IUnOp::Popcnt => vec![0x69],
                    }
                },
                ValSize::V64 => {
                    match iunop {
                        IUnOp::Clz => vec![0x79],
                        IUnOp::Ctz => vec![0x7A],
                        IUnOp::Popcnt => vec![0x7B],
                    }
                },
            }
        },
        Instr::IBinOp(vs, ibinop) => {
            match vs {
                ValSize::V32 => {
                    match ibinop {
                        IBinOp::Add => vec![0x6A],
                        IBinOp::Sub => vec![0x6B],
                        IBinOp::Mul => vec![0x6C],
                        IBinOp::Div(ValSign::S) => vec![0x6D],
                        IBinOp::Div(ValSign::U) => vec![0x6E],
                        IBinOp::Rem(ValSign::S) => vec![0x6F],
                        IBinOp::Rem(ValSign::U) => vec![0x70],
                        IBinOp::And => vec![0x71],
                        IBinOp::Or => vec![0x72],
                        IBinOp::Xor => vec![0x73],
                        IBinOp::Shl => vec![0x74],
                        IBinOp::Shr(ValSign::S) => vec![0x75],
                        IBinOp::Shr(ValSign::U) => vec![0x76],
                        IBinOp::Rotl => vec![0x77],
                        IBinOp::Rotr => vec![0x78],
                    }
                },
                ValSize::V64 => {
                    match ibinop {
                        IBinOp::Add => vec![0x7C],
                        IBinOp::Sub => vec![0x7D],
                        IBinOp::Mul => vec![0x7E],
                        IBinOp::Div(ValSign::S) => vec![0x7F],
                        IBinOp::Div(ValSign::U) => vec![0x80],
                        IBinOp::Rem(ValSign::S) => vec![0x81],
                        IBinOp::Rem(ValSign::U) => vec![0x82],
                        IBinOp::And => vec![0x83],
                        IBinOp::Or => vec![0x84],
                        IBinOp::Xor => vec![0x85],
                        IBinOp::Shl => vec![0x86],
                        IBinOp::Shr(ValSign::S) => vec![0x87],
                        IBinOp::Shr(ValSign::U) => vec![0x88],
                        IBinOp::Rotl => vec![0x89],
                        IBinOp::Rotr => vec![0x8A],
                    }
                },
            }
        },
        Instr::FUnOp(vs, funop) => {
            match vs {
                ValSize::V32 => {
                    match funop {
                        FUnOp::Abs => vec![0x8B],
                        FUnOp::Neg => vec![0x8C],
                        FUnOp::Ceil => vec![0x8D],
                        FUnOp::Floor => vec![0x8E],
                        FUnOp::Trunc => vec![0x8F],
                        FUnOp::Nearest => vec![0x90],
                        FUnOp::Sqrt => vec![0x91],
                    }
                },
                ValSize::V64 => {
                    match funop {
                        FUnOp::Abs => vec![0x99],
                        FUnOp::Neg => vec![0x9A],
                        FUnOp::Ceil => vec![0x9B],
                        FUnOp::Floor => vec![0x9C],
                        FUnOp::Trunc => vec![0x9D],
                        FUnOp::Nearest => vec![0x9E],
                        FUnOp::Sqrt => vec![0x9F],
                    }
                },
            }
        },
        Instr::FBinOp(vs, fbinop) => {
            match vs {
                ValSize::V32 => {
                    match fbinop {
                        FBinOp::Add => vec![0x92],
                        FBinOp::Sub => vec![0x93],
                        FBinOp::Mul => vec![0x94],
                        FBinOp::Div => vec![0x95],
                        FBinOp::Min => vec![0x96],
                        FBinOp::Max => vec![0x97],
                        FBinOp::Copysign => vec![0x98],
                    }
                },
                ValSize::V64 => {
                    match fbinop {
                        FBinOp::Add => vec![0xA0],
                        FBinOp::Sub => vec![0xA1],
                        FBinOp::Mul => vec![0xA2],
                        FBinOp::Div => vec![0xA3],
                        FBinOp::Min => vec![0xA4],
                        FBinOp::Max => vec![0xA5],
                        FBinOp::Copysign => vec![0xA6],
                    }
                },
            }
        },
        Instr::CvtOp(cvtop) => {
            match cvtop {
                CvtOp::IExtend8S(ValSize::V32) => unimplemented!(),
                CvtOp::IExtend8S(ValSize::V64) => unimplemented!(),
                CvtOp::IExtend16S(ValSize::V32) => unimplemented!(),
                CvtOp::IExtend16S(ValSize::V64) => unimplemented!(),
                CvtOp::I64Extend32S => unimplemented!(),
                CvtOp::I32WrapFromI64 => vec![0xA7],
                CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::S) => vec![0xA8],
                CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::U) => vec![0xA9],
                CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::S) => vec![0xAA],
                CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::U) => vec![0xAB],

                CvtOp::I64ExtendFromI32(ValSign::S) => vec![0xAC],
                CvtOp::I64ExtendFromI32(ValSign::U) => vec![0xAD],

                CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::S) => vec![0xAE],
                CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::U) => vec![0xAF],
                CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::S) => vec![0xB0],
                CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::U) => vec![0xB1],

                CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::S) => vec![0xB2],
                CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::U) => vec![0xB3],
                CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::S) => vec![0xB4],
                CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::U) => vec![0xB5],

                CvtOp::F32DemoteFromF64 => vec![0xB6],
        
                CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::S) => vec![0xB7],
                CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::U) => vec![0xB8],
                CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::S) => vec![0xB9],
                CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::U) => vec![0xBA],
                                
                CvtOp::F64PromoteFromF32 => vec![0xBB],

                CvtOp::IReinterpretFromF(ValSize::V32) => vec![0xBC],
                CvtOp::IReinterpretFromF(ValSize::V64) => vec![0xBD],
                CvtOp::FReinterpretFromI(ValSize::V32) => vec![0xBE],
                CvtOp::FReinterpretFromI(ValSize::V64) => vec![0xBF],

                CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V32, ValSign::U) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V32, ValSign::S) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V64, ValSign::U) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V64, ValSign::S) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V32, ValSign::U) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V32, ValSign::S) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V64, ValSign::U) => unimplemented!(),
                CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V64, ValSign::S) => unimplemented!(),
            }
        }
        // _ => unimplemented!(),
    }
}

fn memarg2wasm(memarg: &MemArg) -> Vec<Byte> {
    [
        unsigned32_to_wasm(memarg.align),
        unsigned32_to_wasm(memarg.offset),
    ].concat()
}

fn globaltype2wasm(gt: &GlobalType) -> Vec<Byte> {
    [
        valtype2wasm(&gt.0),
        match gt.1 {
             Mut::Const => 0x00,
             Mut::Var => 0x01,
        }
    ]
    .to_vec()
}

fn tabletype2wasm(tt: &TableType) -> Vec<Byte> {
    [
        vec![0x70],
        limits2wasm(&tt.0),
    ]
    .concat()
}

fn memtype2wasm(mt: &MemType) -> Vec<Byte> {
    limits2wasm(&mt.0)
}

fn limits2wasm(lim: &Limits) -> Vec<Byte> {
    if let Some(max) = lim.max {
        [
            vec![0x01],
            unsigned32_to_wasm(lim.min.try_into().unwrap()),
            unsigned32_to_wasm(max.try_into().unwrap()),
        ]
        .concat()
    } else {
        [
            vec![0x00],
            unsigned32_to_wasm(lim.min.try_into().unwrap()),
        ]
        .concat()
    }
}

fn functype2wasm(func: &FuncType) -> Vec<Byte> {
    [
        vec![0x60],
        bytevector2wasm(func.0.iter().map(valtype2wasm).collect()),
        bytevector2wasm(func.1.iter().map(valtype2wasm).collect()),
    ]
    .concat()
}

fn blocktype2wasm(blocktype: &BlockType) -> Vec<Byte> {
    match blocktype {
        BlockType::TypeIdx(_typeidx) => {
            unimplemented!()
        },
        BlockType::ValType(None) => {
            vec![]
        },
        BlockType::ValType(Some(valtype)) => {
            vec![valtype2wasm(valtype)]
        },
    }
}

fn valtype2wasm(vt: &ValType) -> Byte {
    match vt {
        ValType::I32 => 0x7F,
        ValType::I64 => 0x7E,
        ValType::F32 => 0x7D,
        ValType::F64 => 0x7C,
    }
}

fn name2wasm(name: &Name) -> Vec<Byte> {
    string2wasm(name)
}

fn bytevector2wasm(seq: Vec<Byte>) -> Vec<Byte> {
    [
        unsigned32_to_wasm(seq.len().try_into().unwrap()),
        seq,
    ]
    .concat()
}

fn vector2wasm(seq: Vec<Vec<Byte>>) -> Vec<Byte> {
    [
        unsigned32_to_wasm(seq.len().try_into().unwrap()),
        seq.concat(),
    ]
    .concat()
}

fn string2wasm(s: &String) -> Vec<Byte> {
    let bytes = s.clone().into_bytes();
    [
        unsigned32_to_wasm(bytes.len().try_into().unwrap()),
        bytes,
    ]
    .concat()
}

fn unsigned32_to_wasm(n: u32) -> Vec<Byte> {
    unsigned32_to_leb128(n)
}

fn unsigned32_to_leb128(n: u32) -> Vec<Byte> {
    let mut encoded = vec![];
    let mut n_u32 = n;
    loop {
        // println!("{:#32b}", n_u32);
        let b = n_u32 & 0x0000007F;
        if b == n_u32 {
            encoded.push(b as Byte);
            return encoded;
        } else {
            encoded.push((b as Byte) + 0x80);
            n_u32 >>= 7;
        }
    }
}

fn unsigned64_to_wasm(n: u64) -> Vec<Byte> {
    unsigned64_to_leb128(n)
}

fn unsigned64_to_leb128(n: u64) -> Vec<Byte> {
    let mut encoded = vec![];
    let mut n_u64 = n;
    loop {
        // println!("{:#32b}", n_u32);
        let b = n_u64 & 0x000000000000007F;
        if b == n_u64 {
            encoded.push(b as Byte);
            return encoded;
        } else {
            encoded.push((b as Byte) + 0x80);
            n_u64 >>= 7;
        }
    }
}

#[test]
fn test_globaltype2wasm() {
// let module = Module::default();
    let gt = GlobalType(ValType::I32, Mut::Const);
    assert_eq!(globaltype2wasm(&gt), vec![0x7F, 0x00]);
    let gt = GlobalType(ValType::I64, Mut::Var);
    assert_eq!(globaltype2wasm(&gt), vec![0x7E, 0x01]);
}

#[test]
fn test_limits2wasm() {
    let lim = Limits { min: 2, max: None };
    assert_eq!(limits2wasm(&lim), vec![0x00, 2]);
    let lim = Limits { min: 5, max: Some(8) };
    assert_eq!(limits2wasm(&lim), vec![0x01, 5, 8]);    
}

#[test]
fn test_functype2wasm() {    
    let ft = (vec![], vec![]);
    assert_eq!(functype2wasm(&ft), vec![0x60, 0, 0]);
    let ft = (vec![ValType::I32], vec![]);
    assert_eq!(functype2wasm(&ft), vec![0x60, 1, 0x7F, 0]);
    let ft = (vec![], vec![ValType::I64]);
    assert_eq!(functype2wasm(&ft), vec![0x60, 0, 1, 0x7E]);
    let ft = (vec![ValType::F32], vec![ValType::F64]);
    assert_eq!(functype2wasm(&ft), vec![0x60, 1, 0x7D, 1, 0x7C]);
}

#[test]
// fn test_blocktype2wasm() {
//     assert_eq!(blocktype2wasm(&vec![]), vec![]);
//     assert_eq!(blocktype2wasm(&vec![ValType::I32]), vec![0x7F]);
// }

#[test]
fn test_name2wasm() {
    assert_eq!(name2wasm(&"a".into()), vec![1, 97]);
    assert_eq!(name2wasm(&"そば".into()), vec![6, 227, 129, 157, 227, 129, 176]);
}

#[test]
fn test_vector2wasm() {
    assert_eq!(vector2wasm(vec![vec![1, 2, 3], vec![10, 20, 30]]), vec![2, 1, 2, 3, 10, 20, 30]);
}

#[test]
fn test_unsigned32_to_leb128() {
    assert_eq!(unsigned32_to_leb128(1), vec![1]);
    assert_eq!(unsigned32_to_leb128(0x7F), vec![0x7F]);
    assert_eq!(unsigned32_to_leb128(0x80), vec![0x80, 0x01]);
    assert_eq!(unsigned32_to_leb128(624485), vec![0xE5, 0x8E, 0x26]);
}