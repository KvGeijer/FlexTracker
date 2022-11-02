## The bad flex tracker ##

This can be used to track your hours with the cli locally using Rust. So far it (kinda) sucks, but later on it might suck slightly less. Amazing right?

### Installing ###

You can of course just use
> cargo run -- -h

to run the program from its root folder. But the recommended way is to compile it and add the binaries to your path.
> cargo build
> export PATH="<path-to-here>/target/debug
> flex_cli -h

This way you can now run the program anywhere and your logs should be saved in a reasonable data folder depending on your OS.

To further get help you can use -h for the subcommands as well. Or just ask me...
