use {cc_core::t, eyre::Result, indoc::formatdoc};

pub fn version() -> Result<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const NAME: &str = env!("CARGO_PKG_NAME");
    println!("{} v{}", NAME, VERSION);
    Ok(())
}

pub fn help() -> Result<()> {
    let h = formatdoc! {
        r#"
        coco
        
            {}

        USAGE:
            coco [FLAGS]

        FLAGS:
            -h, --help       {}
            -v, --version    {}
        "#,
        t!("an interactive cli for creating conventional commits"),
        t!("Prints help information"),
        t!("Prints version information")
    };

    println!("{}", h);

    Ok(())
}
