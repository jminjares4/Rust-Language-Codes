use std::fmt::Result;
use std::io::Result as IoResult;

fn funciton1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}