use {eyre::Result, indoc::indoc};

pub fn version() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    println!("{} v{}", NAME, VERSION);
    Ok(())
}

pub fn help() -> Result<()> {
    let h = indoc! {
        r#"
        coco
        
            an interactive cli for creating conventional commits

        USAGE:
            coco [FLAGS]

        FLAGS:
            -h, --help       Prints help information
            -v, --version    Prints version information
        "#
    };

    println!("{}", h);

    Ok(())
}
