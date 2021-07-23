#![warn(unused_imports)]

use gilrs::{Gilrs, Event};
use rodio::{Decoder, OutputStream, Sink};
use std::collections::HashMap;
use std::path::Path;
use serde_json::{Result, Value};



// words = {
//     "48" : {"word" : "Zulu",    "sound": None},
//     "49" : {"word" : 'Look',    "sound": None},
//     "50" : {"word" : 'Allen',   "sound": None},
//     "51" : {"word" : 'Goodbye' ,    "sound": None}, 
//     "52" : {"word" : 'Tal' ,    "sound": None}, 

//     "53" : {"word" : 'Happy',    "sound": None},     
//     "54" : {"word" : 'Love',     "sound": None}, 
//     "55" : {"word" : 'Good',   "sound": None},
//     "56" : {"word" : 'Hurt',      "sound": None},
//     "57" : {"word" : 'Come',      "sound": None},
//     "58" : {"word" : 'Want',      "sound": None},
//     "59" : {"word" : 'Go',        "sound": None},
    
//     "60" : {"word" : 'Bed',       "sound": None},
//     "61" : {"word" : 'Yes',       "sound": None},
//     "62" : {"word" : 'Potty',     "sound": None},
//     "63" : {"word" : 'No',        "sound": None},
//     "64" : {"word" : "Food",      "sound": None},   
               
//     "65" : {"word" : 'Water',    "sound": None},
//     "66" : {"word" : 'Later',    "sound": None},
//     "67" : {"word" : 'Outside',  "sound": None},
//     "68" : {"word" : 'Now',      "sound": None},
//     "69" : {"word" : 'Home',     "sound": None},
//     "70" : {"word" : 'Play',     "sound": None},
//     "71" : {"word" : 'Walk',     "sound": None},
    
//     "72" : {"word" : 'Treat',    "sound": None}

// }


fn get_filename(button_name: &str, joystick_id: gilrs::GamepadId, vocabulary: &mut HashMap<String, &str>) -> String {
    let mut key = String::from("Joystick");
    key.push_str( joystick_id.to_string().as_str() );
    key.push_str("-");
    key.push_str(button_name);
    println!("{}", key);
    
    if vocabulary.contains_key(key.as_str()){ 
        return vocabulary.get(&key).unwrap().to_string();
    } else {
        return String::from("");
    };
}

fn main() -> Result<()> {

    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"[
        {
            "button": "Joystick1-RightTrigger",
            "file": "../dog_words/Alex/Zulu-200.wav"
        },
        {
            "button": "Joystick0-RightTrigger",
            "file": "../dog_words/Alex/Allen-200.wav"
        },
        {
            "button": "Joystick0-LeftTrigger2",
            "file": "../dog_words/Alex/Allen-200.wav"
        }]"#;

    // // Parse the string of data into serde_json::Value.
    let wordsMap: Vec<Value> = serde_json::from_str(data)?;

    // // Access parts of the data by indexing with square brackets.
    

    let mut vocabulary: HashMap<String, &str> = HashMap::new();

    for word in wordsMap {
        // vocabulary.insert(word["button"].to_string(),word["file"].to_string());
        println!("{} --> {}", word["button"], word["file"]);
        vocabulary.insert(word["button"].to_string(),"../dog_words/Alex/Zulu-200.wav");
    }

    vocabulary.insert(String::from("Joystick1-RightTrigger"),"../dog_words/Alex/Zulu-200.wav");

    
    
    


    // Gamepad Setup?
    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
  
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        // Examine new events
        while let Some(Event { id, event, .. }) = gilrs.next_event() {

            match event {
                gilrs::EventType::ButtonPressed(button_code, _) => {
                    
                    let file_name = match button_code {
                        gilrs::Button::RightTrigger => get_filename("RightTrigger", id, &mut vocabulary),
                        _ => String::from(""),
                    };

                    if file_name != "" {
                        let file_path = Path::new(&file_name);
                        if file_path.exists() {
                            let file = std::fs::File::open(&file_path).unwrap();
                            let source = Decoder::new(file).unwrap();
                            sink.append(source);
                        }
                    } else {
                        println!("No Mapping for Joystick {}: {:?}", id, button_code);
                    }

                },
                _ => {}
            }

        }

        sink.sleep_until_end();
    }

}