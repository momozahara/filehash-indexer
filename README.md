# FileHash-Indexer
A tool use to read files hash recursively at a specific part and store into json format

## Installation
### Installer
- [Release](https://github.com/momozahara/filehash-indexer/releases/latest)
### Remote
```
cargo install --git https://github.com/momozahara/filehash-indexer.git
```
### Local
```
cargo install --path .
```

## Usage
```
hash-indexer
--path -p <value>
--version -v <value> #format https://regex101.com/r/JOytBR/1/codegen?language=rust
--pretty # optional
--print # optional
```
```
hash-indexer -p .\src\ -v 1.0.0-alpha1 --pretty --print
```

## Format
```json
{
  "version": "1.0.0-alpha1",
  "assets": [
    {
      "path": ".\\main.rs",
      "hash": "6ceb986a666c337e9da107c6a7309375587b2c03d1bb2e8e9231a4ebb29c4530"
    }
  ]
}
```