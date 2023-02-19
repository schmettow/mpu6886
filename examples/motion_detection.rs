use mpu6886::{*, device::MOT_DETECT_STATUS};
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;
use embedded_hal::blocking::delay::DelayMs;

fn main() -> Result<(), mpu6886Error<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1")
        .map_err(mpu6886Error::I2c)?;

    let mut delay = Delay;
    let mut mpu = mpu6886::new(i2c);
    
    mpu.init(&mut delay)?;
    mpu.setup_motion_detection()?;

    let mut count: u8 = 0;

    loop {
        if mpu.get_motion_detected()? {
            println!("YEAH BUDDY. Motion by axes: {:b}", mpu.read_byte(MOT_DETECT_STATUS::ADDR)?);
            count += 1;
        }

        delay.delay_ms(10u8);

        if count > 5 {
            mpu.reset_device(&mut delay)?;
            break;
        }
    }

    Ok(())
}
