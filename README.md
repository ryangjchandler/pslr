# `pslr`

A small binary wrapper around the [`publicsuffix`](https://docs.rs/publicsuffix) crate that outputs suffixes and registrable domains based on data provided by [publicsuffix.org](https://publicsuffix.org).

## Installation

### Cargo

```sh
cargo install pslr
```

### Manual installation

Find [the latest release](/releases) and download the associated binary for your platform, placing the extracted binary somewhere in your `$PATH`.

## Usage

### Get the suffix of a name

```
$ pslr --suffix www.example.com
com
```

### Get the registrable domain of a name

```
$ pslr --domain www.example.com
example.com
```

### Use a custom suffix list

```
$ pslr --list ./path/to/list.dat --suffix www.example.com
```

## Credits

* [Ryan Chandler](https://github.com/ryangjchandler)
* [rushmorem](https://github.com/rushmorem) for the [publicsuffix](https://github.com/rushmorem/publicsuffix) crate
