use gilrs::{Gilrs, Event};
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::path::Path;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use subprocess::{Exec, Popen};
use std::process::exit;
extern crate glob;
use glob::glob;
use std::time::{SystemTime, UNIX_EPOCH};

fn speak_button_name(button_name: &str, joystick_id: gilrs::GamepadId) {
    let mut key = String::from("Joystick ");
    key.push_str( joystick_id.to_string().as_str() );
    key.push_str(", Button: ");
    key.push_str(button_name);
    speak(key.as_str());
}

fn speak(text: &str){
    let exit_status = {
        Exec::cmd("echo").arg(text) | Exec::cmd("festival").arg("--tts")
    }.join();
    println!("Spoke: {}", text);
}

fn main() -> Result<()> {

    // Gamepad Setup?
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    println!("Found the following gamepads:");
    for (_id, gamepad) in gilrs.gamepads() {
        println!("\t{}: {} is {:?}", _id, gamepad.name(), gamepad.power_info());
    }

    loop {
  
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.next_event() {

            match event {
                gilrs::EventType::ButtonPressed(button_code, _) => {
                    
                    let file_name = match button_code {
                        gilrs::Button::South         => speak_button_name("South", id),
                        gilrs::Button::East          => speak_button_name("East", id),
                        gilrs::Button::North         => speak_button_name("North", id),
                        gilrs::Button::West          => speak_button_name("West", id),
                        gilrs::Button::C             => speak_button_name("C", id),
                        gilrs::Button::Z             => speak_button_name("Z", id),
                        gilrs::Button::LeftTrigger   => speak_button_name("Left Trigger", id),
                        gilrs::Button::LeftTrigger2  => speak_button_name("Left Trigger 2", id),
                        gilrs::Button::RightTrigger  => speak_button_name("Right Trigger", id),
                        gilrs::Button::RightTrigger2 => speak_button_name("Right Trigger 2", id),
                        gilrs::Button::Select        => speak_button_name("Select", id),
                        gilrs::Button::Start         => speak_button_name("Start", id),
                        gilrs::Button::Mode          => speak_button_name("Mode", id),
                        gilrs::Button::LeftThumb     => speak_button_name("Left Thumb", id),
                        gilrs::Button::RightThumb    => speak_button_name("Right Thumb", id),
                        gilrs::Button::DPadUp        => speak_button_name("D Pad Up", id),
                        gilrs::Button::DPadDown      => speak_button_name("D Pad Down", id),
                        gilrs::Button::DPadLeft      => speak_button_name("D Pad Left", id),
                        gilrs::Button::DPadRight     => speak_button_name("D Pad Right", id),
                        gilrs::Button::Unknown       => speak_button_name("Unknown", id)
                    };

                },
                _ => {}
            }

        }

        sink.sleep_until_end();
    }

    std::process::exit(0x0100);

}