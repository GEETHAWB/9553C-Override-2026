mod robot;
mod screen;
mod ticker;

use std::time::Duration;

use crate::robot::*;
use crate::ticker::*;
use vexide::prelude::*;

impl Compete for Robot {
    async fn autonomous(&mut self) {
        // auto routine
    }

    async fn driver(&mut self) {
        let mut ticker = Ticker::new(Duration::from_millis(15));
        loop {
            self.screen.print_at([400, 4], "DRVR"); // cool corner label

            let state = self.controller.state().unwrap_or_default();

            // Supposed arcade drive setup
            let forward = state.left_stick.y();
            let turn = state.right_stick.x();
            let left_voltage = (forward + turn) * Motor::V5_MAX_VOLTAGE;
            let right_voltage = (forward - turn) * Motor::V5_MAX_VOLTAGE;

            // Set the voltage of the drive trains
            if let Err(e) = self
                .left_train
                .iter_mut()
                .try_for_each(|motor| motor.set_voltage(left_voltage))
            {
                self.port_error(e);
            }
            if let Err(e) = self
                .right_train
                .iter_mut()
                .try_for_each(|motor| motor.set_voltage(right_voltage))
            {
                self.port_error(e);
            }

            ticker.tick().await;
        }
    }
}

#[vexide::main]
async fn main(per: Peripherals) {
    let robot = Robot::init(per);
    robot.compete().await;
}
