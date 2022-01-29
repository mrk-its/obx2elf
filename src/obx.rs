use faerie::{Artifact, ArtifactBuilder, Decl, Link, Reloc, SectionKind};
use log;
use std::{collections::HashMap, fs::File, io::Read, str::FromStr};
use target_lexicon::triple;

#[derive(Default)]
pub struct Obx {
    public_symbols: Vec<PublicSymbol>,
    reloc_blocks: Vec<RelBlock>,
    relocations: Vec<Relocations>,
}

impl Obx {
    pub fn to_elf(&self, name: &str, align: Option<u64>) -> Artifact {
        let mut obj = ArtifactBuilder::new(triple!("mos-unknown-none-unknown-elf"))
            .name(name.to_owned())
            .finish();

        assert!(
            self.reloc_blocks.len() == 1,
            "currently only single reloc block in obx file is supported"
        );

        self.reloc_blocks[0].convert(&self.relocations, &self.public_symbols, &mut obj, align);
        obj
    }

    pub fn parse(in_file: &mut File) -> Obx {
        let mut buf = Vec::new();
        in_file.read_to_end(&mut buf).unwrap();

        let mut obx = Obx::default();
        let mut index = 0;
        while index < buf.len() {
            let header = read_u16(&buf, &mut index);
            match header {
                0xffef => {
                    let blk_type = buf[index] as char;
                    index += 1;
                    let item_size = if blk_type != '>' { 2 } else { 3 };
                    let blk_data_length = read_u16(&buf, &mut index) as usize * item_size;
                    let data = Vec::from(&buf[index..index + blk_data_length]);
                    index += blk_data_length;

                    let rels = Relocations {
                        blk_type,
                        item_size,
                        data,
                    };
                    obx.relocations.push(rels);
                }
                0xffee => {
                    todo!("external symbols");
                }
                0xffed => {
                    let n_symbols = read_u16(&buf, &mut index) as usize;
                    for _ in 0..n_symbols {
                        let symbol_type = buf[index] as char;
                        let label_type = buf[index + 1] as char;
                        index += 2;
                        let label_length = read_u16(&buf, &mut index) as usize;
                        let label_name =
                            std::str::from_utf8(&buf[index..index + label_length]).unwrap();
                        index += label_length;
                        let address = read_u16(&buf, &mut index);
                        if label_type == 'P' {
                            let _proc_cpu_reg = buf[index];
                            index += 1;
                            let proc_type = buf[index] as char;
                            index += 1;
                            let num_params = read_u16(&buf, &mut index);
                            match proc_type {
                                'R' => {
                                    let _param_types = &buf[index..index + num_params as usize];
                                    index += num_params as usize;
                                }
                                'V' => todo!(),
                                _ => unreachable!(),
                            }
                        }
                        let symbol = PublicSymbol {
                            label: label_name.to_string().to_lowercase().replace('.', "_"),
                            address,
                            symbol_type,
                            label_type,
                        };
                        log::info!(
                            "{} {:04x} {} {}",
                            symbol.label,
                            symbol.address,
                            symbol.symbol_type,
                            symbol.label_type
                        );
                        obx.public_symbols.push(symbol);
                    }
                }
                _ => {
                    assert!(index > 2 || header == 0xffff);
                    let start = if header == 0xffff {
                        read_u16(&buf, &mut index)
                    } else {
                        header
                    };
                    let end = read_u16(&buf, &mut index);
                    log::debug!("{:04x} - {:04x}", start, end);
                    if start == 0 {
                        // reloc block
                        let magic = read_u16(&buf, &mut index);
                        assert!(magic == 0x524d);
                        index += 1; // skip unused
                        let cfg: u8 = read(&buf, &mut index);
                        log::debug!("cfg: {}", cfg);

                        let stack_ptr = read_u16(&buf, &mut index);
                        let stack_addr = read_u16(&buf, &mut index);
                        let proc_vars_addr = read_u16(&buf, &mut index);

                        log::debug!("stack_ptr: {:04x}", stack_ptr);
                        log::debug!("stack_addr: {:04x}", stack_addr);
                        log::debug!("proc_vars_addr: {:04x}", proc_vars_addr);

                        let block = RelBlock {
                            data: Vec::from(&buf[index..index + (end - start + 1) as usize]),
                            stack_ptr,
                            stack_addr,
                            proc_vars_addr,
                            cfg,
                        };
                        obx.reloc_blocks.push(block);

                        index += (end - start + 1) as usize;
                    }
                }
            }
        }

        obx
    }
}

use crate::consts::{R_MOS_ADDR16, R_MOS_ADDR16_HI, R_MOS_ADDR16_LO};

#[allow(dead_code)]
#[derive(Debug)]
pub struct RelBlock {
    data: Vec<u8>,
    cfg: u8,
    stack_ptr: u16,
    stack_addr: u16,
    proc_vars_addr: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PublicSymbol {
    label: String,
    address: u16,
    symbol_type: char,
    label_type: char,
}

impl RelBlock {
    fn convert(
        &self,
        relocations: &[Relocations],
        public_symbols: &[PublicSymbol],
        obj: &mut Artifact,
        align: Option<u64>,
    ) {
        let start_symbol_name = ".text.sprites";
        obj.declare(
            start_symbol_name,
            Decl::section(SectionKind::Text).with_align(align),
        )
        .unwrap();
        // obj.define(start_symbol_name, self.data.clone()).unwrap();
        let symbols = public_symbols
            .iter()
            .map(|s| (s.label.clone(), s.address as u64))
            .collect();
        obj.define_with_symbols(start_symbol_name, self.data.clone(), symbols)
            .unwrap();
        let mut declarations = HashMap::<u16, String>::new();
        declarations.insert(0, format!("__start"));
        declarations.insert(self.data.len() as u16, format!("__end"));

        let mut addresses = declarations.keys().collect::<Vec<_>>();
        addresses.sort();

        for rel in relocations {
            for i in rel.offsets() {
                let link = Link {
                    from: start_symbol_name,
                    to: start_symbol_name,
                    at: i.0 as u64,
                };
                let (reloc, addend) = match rel.blk_type {
                    'W' => {
                        let mut idx = i.0 as usize;
                        (R_MOS_ADDR16, read_u16(&self.data, &mut idx) as i32)
                    }
                    '<' => (R_MOS_ADDR16_LO, self.data[i.0 as usize] as i32),
                    '>' => (
                        R_MOS_ADDR16_HI,
                        self.data[i.0 as usize] as i32 * 256 + i.1 as i32,
                    ),
                    _ => todo!(),
                };
                obj.link_with(link, Reloc::Raw { reloc, addend }).unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct Relocations {
    blk_type: char,
    item_size: usize,
    data: Vec<u8>,
}

impl Relocations {
    fn offsets(&self) -> Vec<(u16, u8)> {
        let mut ret = Vec::new();
        let mut index = 0;
        while index < self.data.len() {
            let offs = read_u16(&self.data, &mut index);
            let extra = if self.item_size == 3 {
                self.data[index]
            } else {
                0
            };
            index += self.item_size - 2; // 2 is added by above read_u16
            ret.push((offs, extra));
        }
        ret
    }
}

fn read<T>(buf: &[u8], index: &mut usize) -> T
where
    T: Copy,
{
    let ret = unsafe { *(&buf[*index] as *const u8 as *const T) };
    *index += std::mem::size_of::<T>();
    ret
}

fn read_u16(buf: &[u8], index: &mut usize) -> u16 {
    read(buf, index)
}
