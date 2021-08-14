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

fn get_filename(button_name: &str, joystick_id: gilrs::GamepadId, vocabulary: &mut HashMap<String, String>) -> String {
    let mut key = String::from("Joystick");
    key.push_str( joystick_id.to_string().as_str() );
    key.push_str("-");
    key.push_str(button_name);
    
    
    if vocabulary.contains_key(key.as_str()){ 
        println!("Playing: {}", vocabulary.get(&key).unwrap().to_string());
        return vocabulary.get(&key).unwrap().to_string();
    } else {
        return String::from("");
    };
}

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

    println!("Loading current vocabulary from words.json..");

    let mut file = File::open("words.json").expect("Unable to open words.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to open words.json"); //.ok();


    // Parse the string of data into serde_json::Value.
    let words_map: Vec<Value> = serde_json::from_str(&contents)?;

    // Create hash Map to hold the word<->file mapping.
    let mut vocabulary: HashMap<String, String> = HashMap::new();

    for word in words_map {
        println!("Loaded setting for word: {} --> {}", word["button"], word["file"]);
        vocabulary.insert(String::from(word["button"].as_str().unwrap()), String::from(word["file"].as_str().unwrap()));
    }

    // Gamepad Setup?
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    println!("Found the following gamepads:");
    for (_id, gamepad) in gilrs.gamepads() {
        println!("\t{}: {} is {:?}", _id, gamepad.name(), gamepad.power_info());
    }

    // Word Files in Vocab Directory.
    for e in glob("./vocabulary/Alex/*.wav").expect("Failed to find any *.wav files.") {
        println!("Found audio file: {}", e.unwrap().display());
    }

    // std::process::exit(0x0100);

    // let exit_status = (Exec::shell("echo").arg("Hello") | Exec::cmd("/usr/bin/festival").arg("--tts")).join();
    // let exit_status = (Exec::cmd("/usr/bin/find . -type f") | Exec::cmd("/usr/bin/sort")).join();



    // let exit_status = (Exec::cmd("find . -type f") | Exec::cmd("sort")).join()?;

    // let paths = fs::read_dir("./").unwrap();
    // for path in paths {
    //     println!("File: {}", path.unwrap().path().display())
    // }

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

}