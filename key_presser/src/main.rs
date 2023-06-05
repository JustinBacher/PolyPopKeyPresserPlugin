use std::{thread::sleep,  time::Duration, env, net::TcpListener, thread::spawn};

use enigo::{Enigo, KeyboardControllable, Key};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

struct Point {
    x: f64,
    y: f64,
}

#[derive(Deserialize)]
struct Event {
    kind: u8,
    duration: u64,
    actions: Option<Vec<&'a str>>,
    relative: Option<bool>,
    point: Option<Point>,
}

fn determine_key(action: &String) -> Key {
    let ret_val: Key = match action.as_str() {
        "Meta" => Key::Meta,
        "Alt" => Key::Alt,
        "Tab" => Key::Tab,
        "Control" => Key::Control,
        "RControl" => Key::Control,
        "LControl" => Key::Control,
        "Shift" => Key::Shift,
        "RShift" => Key::Shift,
        "LShift" => Key::Shift,
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
        "Backspace" => Key::Backspace,
		"CapsLock" => Key::CapsLock,
		"Delete" => Key::Delete,
		"DownArrow" => Key::DownArrow,
		"UpArrow" => Key::UpArrow,
		"RightArrow" => Key::RightArrow,
		"LeftArrow" => Key::LeftArrow,
		"Home" => Key::Home,
    	"End" => Key::End,
		"Insert" => Key::Insert,
		"PageDown" => Key::PageDown,
		"PageUp" => Key::PageUp,
		"Pause" => Key::Pause,
		"Print" => Key::Print,
		"Escape" => Key::Escape,
        "GamepadA" => Key::GamepadA,
		"GamepadB" => Key::GamepadB,
		"GamepadX" => Key::GamepadX,
		"GamepadY" => Key::GamepadY,
		"GamepadMenu" => Key::GamepadMenu,
		"GamepadView" => Key::GamepadView,
		"GamepadDPadDown" => Key::GamepadDPadDown,
		"GamepadDPadLeft" => Key::GamepadDPadLeft,
		"GamepadDPadRight" => Key::GamepadDPadRight,
		"GamepadDPadUp" => Key::GamepadDPadUp,
		"GamepadLeftShoulder" => Key::GamepadLeftShoulder,
		"GamepadLeftTrigger" => Key::GamepadLeftTrigger,
		"GamepadRightShoulder" => Key::GamepadRightShoulder,
		"GamepadRightTrigger" => Key::GamepadRightTrigger,
		"GamepadLeftThumbstickButton" => Key::GamepadLeftThumbstickButton,
		"GamepadLeftThumbstickDown" => Key::GamepadLeftThumbstickDown,
		"GamepadLeftThumbstickLeft" => Key::GamepadLeftThumbstickLeft,
		"GamepadLeftThumbstickRight" => Key::GamepadLeftThumbstickRight,
		"GamepadLeftThumbstickUp" => Key::GamepadLeftThumbstickUp,
		"GamepadRightThumbstickButton" => Key::GamepadRightThumbstickButton,
		"GamepadRightThumbstickDown" => Key::GamepadRightThumbstickDown,
		"GamepadRightThumbstickLeft" => Key::GamepadRightThumbstickLeft,
		"GamepadRightThumbstickRight" => Key::GamepadRightThumbstickRight,
		"GamepadRightThumbstickUp" => Key::GamepadRightThumbstickUp,
		&_ => Key::Layout(action.chars().next().unwrap()),
    };

    ret_val
}

fn click_mouse(enigo: &mut Enigo, event: Event) {
    let button = match event.actions.unwrap().first() {
        "Left" => MouseButton::Left,
        "Right" => MouseButton::Right,
        "Middle" => MouseButton::Middle,
        "Back" => MouseButton::Back,
        "Forward" => MouseButton::Forward,
        &_ => MouseButton::Left,
    };

    enigo.mouse_down(button);
    sleep(Duration::from_secs(event.duration));
    enigo.mouse_up(button);

}

#[allow(unused_assignments)]
fn move_mouse(enigo: &mut Enigo, event: Event) {
    /* 
        Used solution from
        - https://stackoverflow.com/questions/5902751/simulate-mouse-cursor-move-in-c-sharp-between-two-coordinates
        to get the mouse to move linearly.
        
        My version is probably clunky compared to the C# but it works
    */
    let mut end_x: f64 = event.point.x?;
    let mut end_y: f64 = event.point.y?;
    let _current_position: (i32, i32) = enigo.mouse_location();
    let start_x: f64 = _current_position.0 as f64;
    let start_y: f64 = _current_position.1 as f64;

    if event.relative? {
        end_x += start_x;
        end_y += start_y;
    }

    if event.duration.is_zero() {
        return enigo.mouse_move_to(end_x as i32, end_y as i32);
    }

    let delta_x: f64 = end_x - start_x;
    let delta_y: f64 = end_y - start_y;
    let stopwatch: Instant = Instant::now();
    let mut time_fraction: f64 = 0.0 as f64;

    loop {
        time_fraction = f64::min(1.0, stopwatch.elapsed().as_millis() as f64 / event.duration.as_millis() as f64);

        enigo.mouse_move_to(
            (start_x + time_fraction * delta_x) as i32,
            (start_y + time_fraction * delta_y) as i32
        );

        if !(time_fraction < 1.0) {
            break;
        }
    }
}

fn scroll_mouse(enigo: &mut Enigo, event: &Event) {
    let amount = i32::from(event.duration?);
    
    match event.actions.unwrap().first() {
        "Up" => enigo.mouse_scroll_y(-amount),
        "Down" => enigo.mouse_scroll_y(amount),
        "Left" => enigo.mouse_scroll_x(-amount),
        "Right" => enigo.mouse_scroll_x(-amount),
        &_ => ()
    };
}

fn pushKeys(enigo: &mut Enigo, event: &Event) {
    let mut keys: Vec<Key> = Vec::new();

    for action in event.actions? {
        let key: Key = determine_key(&action);
        keys.push(key);
        enigo.key_down(key);
    }
    
    sleep(Duration::from_secs(event.duration));
    
    for key in &keys {
        enigo.key_up(*key);
    }
}

fn main() {
    env_logger::init();
    let mut enigo: Enigo = Enigo::new();
    let mut args: std::iter::Skip<env::Args> = env::args().skip(1);
    let host: String = args.next().unwrap();
    let server = TcpListener::bind(host).unwrap();
    
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_text() {
                    let event: Event = serde_json::from_str(msg.into_text().unwrap())?;
                    
                    match event.kind {
                        0 => pushKeys(&mut enigo, event),
                        1 => click_mouse(&mut enigo, event),
                        2 => move_mouse(&mut enigo, event),
                        3 => scroll_mouse(&mut enigo, event),
                        &_ => exit(1),
                    }
                    websocket.send(msg).unwrap();
                }
            }
        });
    }
}
