### 烧录
```shell
$ cargo build --release --target thumbv7m-none-eabi
$ cargo flash --chip stm32f103C8 --release --target thumbv7m-none-eabi
```