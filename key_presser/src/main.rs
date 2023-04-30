use enigo::{Enigo, MouseControllable, KeyboardControllable, Key, MouseButton};
use std::{thread::sleep, str::Split, time::Duration, env};

fn determine_key(action: &String) -> Key {
    let ret_val: Key = match action.as_str() {
        "Meta" | "Windows" => Key::Meta,
        "Control" => Key::Control,
        "Alt" => Key::Alt,
        "Tab" => Key::Tab,
        "Shift" => Key::Shift,
        "Space" => Key::Space,
        "Esc" => Key::Escape,
        "F1" => Key::F1,
        "F2" => Key::F2,
        "F3" => Key::F3,
        "F4" => Key::F4,
        "F5" => Key::F5,
        "F6" => Key::F6,
        "F7" => Key::F7,
        "F8" => Key::F8,
        "F9" => Key::F9,
        "F10" => Key::F10,
        "F11" => Key::F11,
        "F12" => Key::F12,
        "F13" => Key::F13,
        "F14" => Key::F14,
        "F15" => Key::F15,
        "F16" => Key::F16,
        "F17" => Key::F17,
        "F18" => Key::F18,
        "F19" => Key::F19,
        "F20" => Key::F20,
        "F21" => Key::F21,
        "F22" => Key::F22,
        "F23" => Key::F23,
        "F24" => Key::F24,
        &_ => Key::Layout(action.chars().next().unwrap()),
    };

    ret_val
}

fn determine_click(action: &String) -> Option<MouseButton> {
    let ret_val= match action.as_str() {
        "Left" => MouseButton::Left,
        "Right" => MouseButton::Right,
        "" => MouseButton::Middle,
        &_ => todo!(),
    };

    Some(ret_val)
}

fn main() {
    let mut enigo: Enigo = Enigo::new();
    let mut args: std::iter::Skip<env::Args> = env::args().skip(1);
    let _info: String = args.next().unwrap();
    let mut _info_split: Split<&str> = _info.split("-");
    let action_type: &str = _info_split.next().unwrap();
    let duration: u64 = _info_split.next().unwrap().parse::<u64>().unwrap();

    if action_type == "key" {
        for action in  args {
            enigo.key_down(determine_key(&action));
        }
        
        sleep(Duration::from_secs(duration));
        
        for action in env::args().skip(2) {
            enigo.key_up(determine_key(&action));
        }
    }
    else if action_type == "mouse" {
        let button: MouseButton = determine_click(&args.next().unwrap()).unwrap();
        enigo.mouse_down(button);
        sleep(Duration::from_secs(duration));
        enigo.mouse_up(button);
    }
}