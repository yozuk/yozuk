# yozuk-skill-punycode

[Punycode](https://en.wikipedia.org/wiki/Punycode) encoder and decoder.

## Examples

### Encoding

This skill detects domain names with non-ASCII characters.

Domain names must end with TLD listed in https://www.iana.org/domains/root/db.

```
🍒.example.com
=> xn--si8h.example.com
```

```
よづく.テスト
=> xn--y8jva1l.xn--zckzah
```

### Decoding

```
xn--3o8h
=> 🐯
```

```
xn--si8h.example.com
=> 🍒.example.com
```