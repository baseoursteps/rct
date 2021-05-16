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
        .or(env::args().nth(0))
        .ok_or(elErr::None)?;

    let myself = File::open(&arg);

    let map = unsafe { memmap2::Mmap::map(&myself?)? };

    if map.len() < mem::size_of::<Elf64_Ehdr>() {
        Err(elErr::Misc("File is smalled than elf header"))?
    }

    let hdr = unsafe {
        (map[..mem::size_of::<Elf64_Ehdr>()].as_ptr() as *const Elf64_Ehdr)
            .as_ref()
            .ok_or(elErr::None)?
    };

    if hdr.e_ident[..4] != ELFMAG[..4] {
        Err(elErr::Misc("File is not an elf"))?
    }

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
        slice::from_raw_parts(bytes[begin + 1..end].as_ptr() as *const u8, count)
            .split(|val| *val == 0)
            .filter(|s| !s.is_empty())
            .map(|s| std::str::from_utf8_unchecked(s))
    };

    //profit
    println!("{:#?}", hdr);

    // find the ones
    for (i, s) in sects.iter().enumerate() {
        if s.sh_type == SHT_STRTAB {
            println!("Section #{} {:#?} has the following strings:", i, s);

            let strs = read_strs(&map, s);

            strs.for_each(|val| println!("{}", val));
        }
    }

    Ok(())
}
