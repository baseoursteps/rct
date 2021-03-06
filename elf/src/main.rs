use core::slice;
use std::{env, fs::File, mem, usize};

use elf::{
    error::{self, Error as elErr},
    types::{Elf64_Ehdr, Elf64_Shdr, ELFMAG, SHT_STRTAB},
};
use memmap2::Mmap;

fn main() -> error::Result<()> {
    let arg = env::args()
        .nth(1)
        .or_else(|| env::args().next())
        .ok_or(elErr::None)?;

    let map = unsafe { memmap2::Mmap::map(&File::open(&arg)?)? };

    if map.len() < mem::size_of::<Elf64_Ehdr>() {
        return Err(elErr::Misc(format!(
            "File {} is smaller than elf header",
            arg
        )));
    }

    let hdr = unsafe {
        (map[..mem::size_of::<Elf64_Ehdr>()].as_ptr() as *const Elf64_Ehdr)
            .as_ref()
            .ok_or(elErr::None)?
    };

    if hdr.e_ident[..4] != ELFMAG[..4] {
        return Err(elErr::Misc(format!("File {} is not an elf", arg)));
    }
    println!("{:#?}", hdr);

    // get section headers
    let sects = unsafe {
        let shoff = hdr.e_shoff as usize;
        let shend = shoff + (hdr.e_shnum * hdr.e_shentsize) as usize;
        slice::from_raw_parts(
            map[shoff..shend].as_ptr() as *const Elf64_Shdr,
            hdr.e_shnum as usize,
        )
    };

    // // section header corresponding to e_shstrndx -- section header names
    // let stringsect = &sects[hdr.e_shstrndx as usize];

    // println!("stringsect {:#?}", stringsect);
    let read_strs = |bytes: &Mmap, header: &Elf64_Shdr| unsafe {
        let begin = header.sh_offset as usize;
        let count = header.sh_size as usize;
        let end = begin + count;
        // first byte is always 0
        slice::from_raw_parts(bytes[begin + 1..end].as_ptr() as *const u8, count)
            .split(|val| *val == 0)
            .filter(|s| !s.is_empty())
            .map(|s| std::str::from_utf8_unchecked(s))
    };

    // find the ones holding symbol strings
    sects
        .iter()
        .enumerate()
        .filter(|(_, s)| s.sh_type == SHT_STRTAB)
        .for_each(|(i, s)| {
            println!("Section {} {:#?} has the following strings:", i, s);
            read_strs(&map, s).for_each(|val| println!("{}", val));
        });

    Ok(())
}
