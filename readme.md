No More Secrets (now in rust)
===============

This is a very silly port of the original [nms](https://github.com/bartobri/no-more-secrets/) into rust because I was bored.

### Running 

You can build it with `cargo build` or run directly with `cargo run`:

```
Options:
    -b, --background COLOR
                        sets background color
    -c, --color COLOR   sets foreground color
    -s, --mask-blank    if enabled then spaces are encrypted too
    -h, --help          print this help menu
    -a, --autodecrypt   if set enables autodecrypt, you don't have to press a
                        key for start the decryption process
```

In the spirit of the original `nms` you can pipe text into it or launch in interactive mode when you have to type text first.


### TODO

* Support ansi-colored piped input would be nice
