<div style="border: dashed 4px firebrick; text-align: center">

### Deprecation Notice


This middleware has been renamed to [__static__](https://github.com/iron/static).


[__Static-file__](#) will no longer be maintained, but is left here for the sake of lingering dependencies.<br>
You should update your `Cargo.toml` to use [__static__](https://github.com/iron/static) at its new location: <https://github.com/iron/static>.

</div>
<!-- This comment is necessary for the markdown above to render correctly -->

static-file [![Build Status](https://secure.travis-ci.org/iron/static-file.png?branch=master)](https://travis-ci.org/iron/static-file)
====

> Static file-serving handler for the [Iron](https://github.com/iron/iron) web framework.

## Example

This example uses the [mounting handler][mounting-handler] to serve files from several directories.

```rust
let mut mount = Mount::new();

// Serve the shared JS/CSS at /
mount.mount("/", Static::new(Path::new("target/doc/")));
// Serve the static file docs at /doc/
mount.mount("/doc/", Static::new(Path::new("target/doc/static_file/")));
// Serve the source code at /src/
mount.mount("/src/", Static::new(Path::new("target/doc/src/static_file/src/lib.rs.html")));

Iron::new(mount).listen(Ipv4Addr(127, 0, 0, 1), 3000);
```

See [`examples/doc_server.rs`](examples/doc_server.rs) for a complete example that you can compile.

## Overview

static-file is a part of Iron's [core bundle](https://github.com/iron/core).

- Serve static files from a given path.

It works well in combination with the [mounting handler][mounting-handler].

## Installation

If you're using a `Cargo.toml` to manage dependencies, just add the `static_file` package to the toml:

```toml
[dependencies.static_file]

git = "https://github.com/iron/static-file.git"
```

Otherwise, `cargo build`, and the rlib will be in your `target` directory.

## [Documentation](http://ironframework.io/doc/static_file)

Along with the [online documentation](http://ironframework.io/doc/static_file),
you can build a local copy with `cargo doc`.

## Get Help

One of us ([@reem](https://github.com/reem/), [@zzmp](https://github.com/zzmp/),
[@theptrk](https://github.com/theptrk/), [@mcreinhard](https://github.com/mcreinhard))
is usually on `#iron` on the mozilla irc. Come say hi and ask any questions you might have.
We are also usually on `#rust` and `#rust-webdev`.

[mounting-handler]: https://github.com/iron/mount
