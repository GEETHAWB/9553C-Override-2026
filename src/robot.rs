use crate::screen::ScreenModule;
use std::time::Duration;
use vexide::color::Color;
use vexide::{prelude::*, smart::PortError};

pub struct Robot {
    pub left_train: [Motor; 1], // Just to be more explicit, I'm using an array
    pub right_train: [Motor; 1],
    pub controller: Controller,
    pub screen: ScreenModule,
}

impl Robot {
    pub fn init(p: Peripherals) -> Self {
        let mut screen = ScreenModule::new(p.display);
        screen.clear_screen();
        screen.print(":: Screen Module Initialized");

        // Initialize Motors for drive trains
        let left_front = Motor::new(p.port_1, Gearset::Green, Direction::Forward);
        //let left_back = Motor::new(p.port_2, Gearset::Green, Direction::Forward);
        let right_front = Motor::new(p.port_11, Gearset::Green, Direction::Reverse);
        //let right_back = Motor::new(p.port_14, Gearset::Green, Direction::Reverse);
        screen.print(":: Motors Initialized");

        let left_train = [left_front /*left_back*/];
        let right_train = [right_front /*right_back*/];

        // Other components
        let controller = p.primary_controller;
        screen.print(":: Controller Discovered");

        let mut robot = Robot {
            left_train,
            right_train,
            controller,
            screen,
        };

        robot.screen.set_color(Color::GREEN);
        robot.screen.print(":: Robot Initialized Successfully!");
        robot.screen.set_color(Color::WHITE);
        robot
    }

    pub fn port_error(&mut self, err: PortError) {
        self.screen.clear_screen();

        let port_text = match err {
            PortError::Disconnected { port } => {
                format!("Port {port} disconnected!")
            }
            PortError::IncorrectDevice {
                expected,
                actual,
                port,
            } => {
                format!(
                    "Incorrect device on port {port}. \nExpected: {expected:?}\nActual: {actual:?}"
                )
            }
        };

        self.screen.set_color(Color::RED);
        self.screen.print(port_text);
    }

    #[inline]
    pub fn sleep(&self, millis: u64) {
        let _ = sleep(Duration::from_millis(millis));
    }
}
