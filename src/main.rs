#![no_std]   // 不使用标准库（嵌入式环境没有操作系统）
#![no_main]  // 不使用标准 main 函数（由 Embassy 异步运行时接管入口）

// ========================
// 引入所需的库与宏
// ========================

use defmt::*;                     // 用于高效的嵌入式日志输出（支持 RTT）
use embassy_executor::Spawner;    // 任务调度器（用于在 Embassy 中启动异步任务）
use embassy_stm32::gpio::{Level, Output, Speed};  // STM32 GPIO 模块：电平、输出模式、速度
use embassy_time::Timer;          // 异步延时定时器（基于 Embassy 的时间调度）
use {defmt_rtt as _, panic_probe as _};  
// defmt_rtt: 通过 RTT 接口输出调试日志（高效、非阻塞）
// panic_probe: 在 panic 时提供简洁报告（避免系统死锁或崩溃）

// ========================
// 程序主入口（异步）
// ========================
// Embassy 使用 async/await 异步模型，所以入口函数必须是 async fn。
// #[embassy_executor::main] 宏会自动生成启动代码，
// 初始化底层硬件、创建异步执行器并运行 main()。
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 初始化所有 STM32 外设（时钟、GPIO、定时器等）
    // `Default::default()` 使用默认配置（通常是 HSI 或 HSE 时钟源）
    let p = embassy_stm32::init(Default::default());

    // 输出一条调试日志信息（通过 RTT）
    info!("Hello World!");

    // ------------------------
    // 配置 LED 引脚
    // ------------------------
    // 创建一个 GPIO 输出对象：
    // 参数：
    //   p.PC13 → 板载 LED（Blue Pill 上为 PC13）
    //   Level::High → 初始电平为高（熄灭，因为 PC13 通常低电平点亮）
    //   Speed::Low → GPIO 速度为低速，足够驱动 LED
    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    // ------------------------
    // 主循环：异步闪烁 LED
    // ------------------------
    loop {
        info!("high");    // 打印日志：LED 拉高
        led.set_high();   // 设置引脚为高电平（LED 熄灭）
        Timer::after_millis(200).await; // 异步延时 600 毫秒（不会阻塞 CPU）

        info!("low");     // 打印日志：LED 拉低
        led.set_low();    // 设置引脚为低电平（LED 点亮）
        Timer::after_millis(1800).await; // 再延时 300 毫秒
    }
}
