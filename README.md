## tenhou-parser-rs

A simple parser for Tenhou logs written in Rust. Respect the output format of the python implement [Riichi-Mahjong-Statistics-Seminar/tenhou-paifu-to-json](https://github.com/Riichi-Mahjong-Statistics-Seminar/tenhou-paifu-to-json/tree/main) except fixing several bugs. For details see [this issue](https://github.com/Riichi-Mahjong-Statistics-Seminar/tenhou-paifu-to-json/issues/6).

Depending on IO speed, it can parse 200~1400 logs per second, at least 10 times faster than the python version.

## Usage

```
Usage: tenhou-parser-rs <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input file, directory or glob pattern
  [OUTPUT]  Output file or directory

Options:
  -h, --help  Print help
```


