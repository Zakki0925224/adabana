# adabana

**adabana** is a hobby operating system for Raspberry Pi 3 Model B, written in Rust.

## Third party

-   [raspbootin](https://github.com/mrvn/raspbootin)

## How to run

```sh
# install required packages
$ sudo apt update && sudo apt install -y gcc-arm-none-eabi

# rustup
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

$ git clone https://github.com/Zakki0925224/adabana.git
$ cd adabana
$ python3 ./taskpy run
```
