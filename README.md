# PicoLisp FFI with Rust

This repo provides a simple example of how to use [PicoLisp](https://software-lab.de/down.html) with [Rust](https://www.rust-lang.org/tools/install) using PicoLisp's FFI `(native)` functionality.

# Requirements

  * Rustc and Cargo `v1.47.0+`
  * PicoLisp 64-bit `v17.12+` or `pil21`

# Getting started

Once you've setup _PicoLisp_ and _Rust_, simply type `make` to build and test the shared library.

# Output

Before I explain what's going on, here's what the output should look like:

```
Received struct: PilStruct {
    byte1: 32,
    byte2: 33,
    character1: 67,
    character2: 68,
    int: -1,
    long: 1,
    string: 0x4847464544434241,
    array: [
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
    ],
}
Result code: 0
Extracted struct:
(de Extracted (42 43)
   ("A" "B")
   65535
   9223372036854775807
   "pilrust"
   (80 105 99 111 76 105 115 112) )
```

# Explain

The code can be found in [extract.l](extract.l) and [src/lib.rs](src/lib.rs). The _Rust_ code is designed as a **shared library** and can be called by PicoLisp's **(native)** function to pass data to/from between both languages.

## PicoLisp code explanation

First, the code allocates 32 bytes of memory, which will be used to store data in a [struct](https://software-lab.de/doc/refS.html#struct).

It then creates a struct named `P` with the following specification:

  * 2 arbitrary bytes
  * 2-bytes containing valid UTF-8 characters
  * 1x 32-bit (4 bytes) signed integer
  * 1x 64-bit (8 bytes) signed long
  * 1x 8-byte null-terminated string
  * 1x 8-byte arbitrary bytes array

Then the following [native](https://software-lab.de/doc/refN.html#native) call is made and its result is stored in the `Res` variable:

```picolisp
(native "target/debug/libpilrust.so" "extract" 'I P)
```

This calls the `extract` function from the _Rust_ library, with the `P` struct as its only parameter. It expects a 32-bit signed integer `I` as the return value (it will be either `0` or `-1`).

Next, the code will extract the `P` structure using the specification described above:

```
(struct P '((B . 2) (C . 2) I N S (B . 8)))
```

Finally, the code will free the previously allocated memory and print the result of the `P` structure.

Some tests run at the end to ensure the data received from _Rust_ is what we expected.

### Note

  * The values sent to the _Rust_ library will be printed by _Rust_ as `Received struct:`.
  * The values received from the _Rust_ library will be printed by _PicoLisp_ as `Extracted struct:`.

## Rust code explanation

The _Rust_ code defines the struct for the received data; it is named `PilStruct` and contains the exact same specification as the `P` struct in the _PicoLisp code explanation_.

The `extract()` function creates a new struct in the variable `newstruct` which contains some new values, different from what was received by _PicoLisp_.

Since FFI is considered [unsafe in _Rust_](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html), the code which actually does the FFI (dereferencing the pointer and writing to it) is contained in an `unsafe` block:

```
unsafe { ... playing with fire ... }
```

Luckily, the _Rust_ function checks if the struct is a **null pointer** before trying to work with it, and returns `-1` if it is. However it doesn't check if it's correctly aligned (it is), so look-out for that!

The code then dereferences the pointer and prints what it received (the entire struct) from _PicoLisp_ as `Received struct:` (mentioned earlier).

Finally, it writes the `newstruct` struct to the pointer and returns `0`. _PicoLisp_ can then read the return code and the new struct data.

# Thoughts

There isn't much to this code, but I thought it would be fun to create a working FFI library that's _not_ written in _C_ and which works perfectly with _PicoLisp_.

Enjoy!

# Contributing

  * For bugs, issues, or feature requests, please [create an issue](https://github.com/aw/picolisp-rust/issues/new).

# License

[0BSD License](LICENSE)

Copyright (c) 2020 Alexander Williams, On-Prem <license@on-premises.com>
