# rv8803

[![Crates.io Version](https://img.shields.io/crates/v/rv8803)](https://crates.io/crates/rv8803)
[![Released API docs](https://img.shields.io/docsrs/rv8803)](https://docs.rs/rv8803/)

## Quickstart

> [!TIP]
> At the end you will see a link to the latest docs which you can view locally.

Build this by running:

```shell script
./build.sh
```

> [!NOTE]  
> You will need to configure your target MCU accordingly.

This example uses the `stm32wle5cc` metapac configuration for the RAKwireless RAK3172 module.

```rust
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hsi = true;
        config.rcc.hse = None;
        config.rcc.pll = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV1, // PLLM
            mul: PllMul::MUL21,      // PLLN
            divp: Some(PllPDiv::DIV2),
            divq: Some(PllQDiv::DIV2), // PLLQ
            divr: Some(PllRDiv::DIV7), // PLL1_R
        });
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV1;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
    }
    let p = embassy_stm32::init(config);
    info!("Configure STM32 for Skynet(TM) AI bootup. Muwahahahahhaha!");

    let i2c2_periph = p.I2C2;
    let led2 = p.PA0;
    let led2 = &mut gpio::Output::new(led2, gpio::Level::Low, gpio::Speed::Medium);

    // RAK19007 board J12
    //  - pin 1: VCC 3V3
    //  - pin 2: GND
    //  - pin 3: SCL
    //  - pin 4: SDA
    //
    // RAK3172 Package (STM32wle5cc)
    // https://docs.embassy.dev/embassy-stm32/git/stm32wle5cc/i2c/trait.SclPin.html
    // https://docs.embassy.dev/embassy-stm32/git/stm32wle5cc/i2c/trait.SdaPin.html
    let mut i2c = I2c::new_blocking(
        i2c2_periph,
        p.PA12, // scl
        p.PA11, // sda
        Hertz(400_000),
        Default::default(),
    );

    let mut rv8803: Driver<i2c::I2c<'_, mode::Blocking>, SevenBitAddress> = Driver::new(i2c);

    let mut data = Some(rv8803::ClockData::new());
    // let mut data: Option<ClockData> = None;

    if let Some(mut d) = data {
        let now = DateTimeBuilder::new()
            .year(CurrentYear::new(2024))
            .month(Month::October)
            .date(7)
            .weekday(Weekday::Monday)
            .hours(0)
            .minutes(0)
            .seconds(0)
            .build();
        d.set(&now);

        // Uncomment below to update the time on the RTC clock chip;
        // You can then comment this out to continue fetching the latest time.
        // let _ = rv8803.update(rv8803::ClockData::new(), &Some(d)).unwrap();
    }

    info!("Starting loop()...");
    loop {
        pulse_heartbeat(
            |_| {
                // Connect to the RV8803 RTC and fetch the latest time.
                if let Some(d) = data {
                    match rv8803.now(d) {
                        Ok(mut data) => {
                            let data = rv8803::prelude::LoggableClockData::new(data);
                            info!("{}", data)
                        }
                        Err(err) => match err {
                            rv8803::prelude::DriverError::I2c(err) => {
                                error!("Unknown I2C error: {:?}", err)
                            }
                            _ => error!("Cannot fetch latest from RTC: Unknown error!"),
                        },
                    }
                } else {
                    error!("Clock data not provided!");
                }
            },
            (),
            led2,
        )
        .await;
    }
}

pub async fn pulse_heartbeat<F>(f: F, callback: (), led: &mut gpio::Output<'_>)
where
    F: FnOnce(()),
{
    // trace!("Heartbeat pulse...");

    led.set_high();
    Timer::after_millis(800).await;

    f(callback);

    led.set_low();
    Timer::after_millis(200).await;
}
```

You can also use a Raspberry Pi Pico (RP2040) to get started:

```rust

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut p = embassy_rp::init(Default::default());
    info!("Configure RP2040 for Skynet(TM) AI bootup. Muwahahahahhaha!");

    let mut led = Output::new(p.PIN_25, Level::Low);

    // I2C Setup
    info!("Starting I2C Setup");
    let sda = p.PIN_14; // Pico pin 16
    let scl = p.PIN_15; // Pico pin 17
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = 400_000;
    let mut i2c = i2c::I2c::new_async(p.I2C1, scl, sda, Irqs, i2c_config);
    info!("I2C Initialized");

    // RV8803, default address 0x32h
    let mut rv8803: Driver<i2c::I2c<'_, I2C1, i2c::Async>, SevenBitAddress> = Driver::new(i2c);

    let mut data = Some(rv8803::ClockData::new());
    // let mut data: Option<ClockData> = None;

    if let Some(mut d) = data {
        let now = DateTimeBuilder::new()
            .year(CurrentYear::new(2024))
            .month(Month::October)
            .date(7)
            .weekday(Weekday::Monday)
            .hours(0)
            .minutes(0)
            .seconds(0)
            .build();
        d.set(&now);

        // Uncomment below to update the time on the RTC clock chip;
        // You can then comment this out to continue fetching the latest time.
        // let _ = rv8803.update(rv8803::ClockData::new(), &Some(d)).unwrap();
    }

    // ...
}
```

Refer to the [docs](https://docs.rs/rv8803/latest/rv8803/) for details.


## Minimum supported Rust version (MSRV)

This project is tested against rust `stable`.


## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2024, [Michael de Silva](mailto:michael@cyberdynea.io)
