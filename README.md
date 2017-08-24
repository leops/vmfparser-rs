vmfparser-rs
=============

[Another parser](https://github.com/leops/vmfparser) for the VMF format, this
time in Rust.

VMF is not a very complex format, so most of the API should be self-explanatory.
Perhaps the most interesting feature is the ability to "brin your own key": the
parser is agnostic over what type is used for the keys (block and property
names), as long as it implements `From<&str>`. This means you can use a good old
`String`, or a [string_cache](https://github.com/servo/string-cache)::Atom using
a static table pre-filled with some VMF-related keywords.
