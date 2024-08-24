use region::{protect, Protection};

unsafe fn jmp(addr: *const u8) {
    let fn_ptr: fn() = std::mem::transmute(addr);
    fn_ptr();
}

fn main() -> Result<(), region::Error> {
    let instructions =
        std::fs::read("examples/helloworld.bin").expect("Should have been able to read the file");
    println!("{:?}", instructions);
    let entry_point = instructions.as_ptr();

    unsafe {
        protect(
            entry_point,
            instructions.len(),
            Protection::READ_WRITE_EXECUTE,
        )?;
    }

    
    unsafe {
        jmp(entry_point);
    }

    Ok(())
}
