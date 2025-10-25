#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::pwm::{simple_pwm::SimplePwm, Channel};
use embassy_stm32::time::mhz;
use embassy_stm32::Peripherals;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("SG90 Servo Demo (STM32F103C8T6)");

    let p = embassy_stm32::init(Default::default());

    // // TIM1_CH1 (PA8)
    // let mut pwm = SimplePwm::new(
    //     p.TIM1,
    //     Some(p.PA8),
    //     None,
    //     None,
    //     None,
    //     mhz(50),           // SG90 需要 50Hz PWM (周期 20ms)
    //     Default::default(),
    // );

    // pwm.enable(Channel::Ch1);

    // loop {
    //     // 舵机转到 0 度
    //     set_servo_angle(&mut pwm, Channel::Ch1, 0.0);
    //     Timer::after(Duration::from_secs(1)).await;

    //     // 转到 90 度
    //     set_servo_angle(&mut pwm, Channel::Ch1, 90.0);
    //     Timer::after(Duration::from_secs(1)).await;

    //     // 转到 180 度
    //     set_servo_angle(&mut pwm, Channel::Ch1, 180.0);
    //     Timer::after(Duration::from_secs(1)).await;
    // }
}

// /// 将角度映射到占空比
// fn set_servo_angle(pwm: &mut SimplePwm<'_>, ch: Channel, angle: f32) {
//     let min_pulse_ms = 0.5; // 0 度 -> 0.5ms
//     let max_pulse_ms = 2.5; // 180 度 -> 2.5ms
//     let period_ms = 20.0;   // PWM 周期 20ms

//     let pulse_ms = min_pulse_ms + (angle / 180.0) * (max_pulse_ms - min_pulse_ms);
//     let duty = pulse_ms / period_ms;

//     let max_duty = pwm.get_max_duty();
//     let duty_val = (duty * max_duty as f32) as u16;

//     pwm.set_duty(ch, duty_val);
//     info!("Set angle {:.1}° -> duty {}", angle, duty_val);
// }
