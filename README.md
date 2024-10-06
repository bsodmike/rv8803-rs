# rv8803

[![Crates.io Version](https://img.shields.io/crates/v/rv8803)](https://crates.io/crates/rv8803)
[![Released API docs](https://img.shields.io/docsrs/rv8803)](https://docs.rs/rv8803/)

## API usage

Here's an example with `embassy_stm32`; you will need to configure the I2C peripheral according to the exact chip used. This example uses the `stm32wl55cc-cm4` metapac info.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.ls = LsConfig::default_lse();
        config.rcc.hse = Some(Hse {
            freq: Hertz(32_000_000),
            mode: HseMode::Bypass,
            prescaler: HsePrescaler::DIV1,
        });
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL6,
            divp: None,
            divq: Some(PllQDiv::DIV2), // PLL1_Q clock (32 / 2 * 6 / 2), used for RNG
            divr: Some(PllRDiv::DIV2), // sysclk 48Mhz clock (32 / 2 * 6 / 2)
        });
    }
    let p = embassy_stm32::init_primary(config, &SHARED_DATA);
    info!("Hello World!");
    let i2c1_periph = p.I2C1;

    // External RTC with the rv8803
    let i2c = I2c::new(
        i2c1_periph,
        p.PA9,  // scl
        p.PA10, // sda
        Irqs,
        p.DMA1_CH2, //tx_dma
        p.DMA1_CH1,
        Hertz(100_000),
        Default::default(),
    );

    let device_address: u8 = 0x32;
    let bus = shared_bus::BusManagerCortexM::new(i2c);

    let mut rtc = rv8803::rtc::RTClock::<
        embassy_stm32::i2c::I2c<'_, embassy_stm32::mode::Async>,
        embassy_stm32::i2c::Error,
        cortex_m::interrupt::Mutex<core::cell::RefCell<I2c<'_, embassy_stm32::mode::Async>>>,
    >::new(&bus, &device_address);

    info!("Starting loop()...");
    loop {
        Timer::after_secs(1).await;

        let mut buf = [0u8, 8];
        if let Ok(_succeeded) = rtc.update_time(&mut buf) {
            defmt::debug!("Updated time: {}", buf);
        }
    }
}

```

Refer to the [docs](https://docs.rs/rv8803/latest/rv8803/) for details.

### WARNING!

The [latest release](https://crates.io/crates/rv8803) simply stabilises a new public API, however, this still needs testing on actual hardware and is pending.

Should you wish to contribute towards this effort, kindly do so by opening issues/PRs.  Thanks!

### Building

This runs a `release` build, runs tests and generates docs.

```shell script
./build.sh
```

## Minimum supported Rust version

This project is tested against rust `nightly`.

## License

Refer to LICENSE.