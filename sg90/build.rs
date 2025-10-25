fn main() {
    // println!("cargo:rustc-link-arg-bins=--nmagic");
    // 这一行被注释掉了。`--nmagic` 通常用于嵌入式开发中链接裸机程序（no OS）时，
    // 禁用页对齐（page alignment），从而让链接器在放置段时更灵活。
    // 如果启用它，可以减少二进制文件大小并避免某些内存浪费。

    // 向 Cargo 构建系统输出指令，告诉它在编译“可执行二进制文件（bins）”时，
    // 传递额外的链接参数 `-nostartfiles` 给 rustc。
    // `-nostartfiles` 告诉链接器不要使用标准的启动文件（如 crt0.o）。
    // 这是裸机（bare-metal）程序常见做法，因为没有操作系统来提供启动例程。
    println!("cargo:rustc-link-arg-bins=-nostartfiles");

    // 添加自定义链接脚本 `link.x`。
    // 这个脚本定义了程序在内存中的布局（代码段、数据段、堆栈、外设地址等）。
    // 嵌入式系统中，链接脚本非常关键，用于告诉链接器程序如何映射到具体硬件内存。
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    // 添加 `defmt.x` 链接脚本。
    // 这是 `defmt`（一种高效的嵌入式日志格式库）使用的附加链接脚本。
    // 它在构建时确保 `defmt` 所需的符号和内存布局正确被链接进最终固件。
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
