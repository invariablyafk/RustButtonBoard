use gilrs::{Gilrs, Event};
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::path::Path;
use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;

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

fn main() -> Result<()> {

    println!("Loading vocabulary..");

    let mut file = File::open("words.json").expect("Unable to open words.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to open words.json"); //.ok();


    // Parse the string of data into serde_json::Value.
    let words_map: Vec<Value> = serde_json::from_str(&contents)?;

    // Create hash Map to hold the word<->file mapping.
    let mut vocabulary: HashMap<String, String> = HashMap::new();

    for word in words_map {
        println!("Added word: {} --> {}", word["button"], word["file"]);
        vocabulary.insert(String::from(word["button"].as_str().unwrap()), String::from(word["file"].as_str().unwrap()));
    }

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
                        gilrs::Button::South         => get_filename("South", id, &mut vocabulary),
                        gilrs::Button::East          => get_filename("East", id, &mut vocabulary),
                        gilrs::Button::North         => get_filename("North", id, &mut vocabulary),
                        gilrs::Button::West          => get_filename("West", id, &mut vocabulary),
                        gilrs::Button::C             => get_filename("C", id, &mut vocabulary),
                        gilrs::Button::Z             => get_filename("Z", id, &mut vocabulary),
                        gilrs::Button::LeftTrigger   => get_filename("LeftTrigger", id, &mut vocabulary),
                        gilrs::Button::LeftTrigger2  => get_filename("LeftTrigger2", id, &mut vocabulary),
                        gilrs::Button::RightTrigger  => get_filename("RightTrigger", id, &mut vocabulary),
                        gilrs::Button::RightTrigger2 => get_filename("RightTrigger2", id, &mut vocabulary),
                        gilrs::Button::Select        => get_filename("Select", id, &mut vocabulary),
                        gilrs::Button::Start         => get_filename("Start", id, &mut vocabulary),
                        gilrs::Button::Mode          => get_filename("Mode", id, &mut vocabulary),
                        gilrs::Button::LeftThumb     => get_filename("LeftThumb", id, &mut vocabulary),
                        gilrs::Button::RightThumb    => get_filename("RightThumb", id, &mut vocabulary),
                        gilrs::Button::DPadUp        => get_filename("DPadUp", id, &mut vocabulary),
                        gilrs::Button::DPadDown      => get_filename("DPadDown", id, &mut vocabulary),
                        gilrs::Button::DPadLeft      => get_filename("DPadLeft", id, &mut vocabulary),
                        gilrs::Button::DPadRight     => get_filename("DPadRight", id, &mut vocabulary),
                        gilrs::Button::Unknown       => get_filename("Unknown", id, &mut vocabulary),
                    };

                    if file_name != "" {
                        let file_path = Path::new(&file_name);
                        if file_path.exists() {
                            let file = std::fs::File::open(&file_path).unwrap();
                            let source = Decoder::new(file).unwrap();
                            sink.append(source);
                        } else {
                            println!("Joystick{}-{:?} file not found: {}", id, button_code, file_name);
                        }
                    } else {
                        println!("No Mapping for Joystick{}-{:?}", id, button_code);
                    }

                },
                _ => {}
            }

        }

        sink.sleep_until_end();
    }

}