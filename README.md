# base_emoji

[![crates.io](http://meritbadge.herokuapp.com/base_emoji)](https://crates.io/crates/base_emoji)
[![Build Status](https://travis-ci.org/badboy/base-emoji-rs.svg?branch=master)](https://travis-ci.org/badboy/base-emoji-rs)

Convert everything to Emojis.

🐫🔦🔥🎲🐬🍀🍟🔋🐬🎲🐬🍀🎁🍟💧👂🔥🚪🔋🍟🔦🔋🚗👣🔦🌍👂🍤🐗

Reimplementation of [base_emoji](https://github.com/pfrazee/base-emoji) (JavaScript).

## Documentation

[Documentation available online](https://docs.rs/base_emoji).

## Example

```rust
let input = [0xde, 0xad, 0xbe, 0xef];
let output = "❄️🐼🚓👅";

assert_eq!(base_emoji::to_string(&input), output);
```

## Encoding (same as original implementation)

Citing [the README](https://github.com/pfrazee/base-emoji/blob/04b6c1e24ae5071804285cb358162628ea4a9bc8/README.md):

> The emojis used are in `emojis.json`. There are 843 emojis there, but the
> converter reads sequences of 8 bits at a time, and so only maps the value to
> the first 256 of them. To stay consistent with other renderings, make sure you
> don't change the order of your emojis.json.

## License

MIT. See [LICENSE](LICENSE).
