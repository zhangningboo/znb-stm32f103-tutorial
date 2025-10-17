//! Blinks several LEDs stored in an array
//! --------------------------------------
//! 本示例展示如何在 STM32F103（或兼容芯片）上通过 HAL 驱动多个 LED 闪烁。
//! 主要内容：
//! - 使用 `syst`（系统定时器 SysTick）生成周期中断（定时 1 Hz）
//! - 通过 GPIO 控制多颗 LED
//! - 使用数组统一管理多个引脚
//!
//! 适合演示嵌入式 Rust 的基础 GPIO 控制与延时机制。

#![deny(unsafe_code)]  // 禁止使用 unsafe 代码，保证内存安全
#![no_std]             // 不链接标准库（嵌入式系统没有操作系统）
#![no_main]            // 不使用默认 main 函数入口（由 Cortex-M RT 提供入口）

use panic_halt as _;   // panic 时程序直接停止运行（适合裸机环境）

use nb::block;         // 提供阻塞式等待宏（将非阻塞函数转换为阻塞调用）

use cortex_m_rt::entry;          // 定义嵌入式程序入口 #[entry]
use stm32f1xx_hal::{pac, prelude::*, timer::Timer}; // HAL 外设与通用 trait 导入

#[entry]
fn main() -> ! {
    // ------------------------
    // 获取内核与外设访问句柄
    // ------------------------
    let cp = cortex_m::Peripherals::take().unwrap(); // Cortex-M 内核外设（如 SYST）
    let dp = pac::Peripherals::take().unwrap();      // STM32 设备外设（GPIO、RCC、TIM等）

    // ------------------------
    // 初始化时钟控制器
    // ------------------------
    let mut rcc = dp.RCC.constrain(); // 约束（constrain）RCC，进入安全配置模式

    // ------------------------
    // 分割 GPIOA / GPIOC
    // ------------------------
    // 通过 split() 方法，拆分出单独的引脚控制结构
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    // ------------------------
    // 配置 SysTick 定时器
    // ------------------------
    // Timer::syst()：使用系统定时器 SYST 创建一个计时器实例
    // counter_hz()：以 Hz 为单位配置频率
    let mut timer = Timer::syst(cp.SYST, &rcc.clocks).counter_hz();

    // 让定时器以 1 Hz 频率运行 → 每秒触发一次
    timer.start(1.Hz()).unwrap();

    // ------------------------
    // 初始化 LED 引脚数组
    // ------------------------
    // 将两个不同端口的引脚都配置为“推挽输出”模式
    // erase() 将不同类型的引脚擦除为通用类型，方便放入数组
    let mut leds = [
        gpioc.pc13.into_push_pull_output(&mut gpioc.crh).erase(), // PC13 常为板载 LED
        gpioa.pa1.into_push_pull_output(&mut gpioa.crl).erase(),  // 另一个可用引脚
    ];

    // ------------------------
    // 主循环：闪烁所有 LED
    // ------------------------
    loop {
        // 等待一次定时器事件（阻塞直到 1 秒过去）
        block!(timer.wait()).unwrap();

        // 所有 LED 置高电平（点亮）
        for led in leds.iter_mut() {
            led.set_high();
        }

        // 再等待 1 秒
        block!(timer.wait()).unwrap();

        // 所有 LED 置低电平（熄灭）
        for led in leds.iter_mut() {
            led.set_low();
        }
    }
}
