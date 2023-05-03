use enigo::{Enigo, MouseControllable, MouseButton};
use std::{thread::sleep, time::{Duration, Instant}, env, process::exit, iter::Skip};


fn click_mouse(enigo: &mut Enigo, args: &mut Skip<env::Args>) {
    let button = match args.next().unwrap().as_str() {
        "Left" => MouseButton::Left,
        "Right" => MouseButton::Right,
        "Middle" => MouseButton::Middle,
        "Back" => MouseButton::Back,
        "Forward" => MouseButton::Forward,
        &_ => MouseButton::Left,
    };

    enigo.mouse_down(button);
    sleep(Duration::from_secs(args.next().unwrap().parse::<u64>().unwrap()));
    enigo.mouse_up(button);

}

#[allow(unused_assignments)]
fn move_mouse(enigo: &mut Enigo, args: &mut Skip<env::Args>) {
    /* 
        Used solution from
        - https://stackoverflow.com/questions/5902751/simulate-mouse-cursor-move-in-c-sharp-between-two-coordinates
        to get the mouse to move linearly.
        
        My version is probably clunky compared to the C# but it works
    */
    let mut end_x: f64 = args.next().unwrap().parse::<i32>().unwrap() as f64;
    let mut end_y: f64 = args.next().unwrap().parse::<i32>().unwrap() as f64;
    let relative: bool = args.next().unwrap().as_str() == "true";
    let duration: Duration = Duration::from_millis(args.next().unwrap().parse::<u64>().unwrap());
    let _current_position: (i32, i32) = enigo.mouse_location();
    let start_x: f64 = _current_position.0 as f64;
    let start_y: f64 = _current_position.1 as f64;

    if relative {
        end_x += start_x;
        end_y += start_y;
    }

    if duration.is_zero() {
        return enigo.mouse_move_to(end_x as i32, end_y as i32);
    }

    let delta_x: f64 = end_x - start_x;
    let delta_y: f64 = end_y - start_y;
    let stopwatch: Instant = Instant::now();
    let mut time_fraction: f64 = 0.0 as f64;

    loop {
        time_fraction = f64::min(1.0, stopwatch.elapsed().as_millis() as f64 / duration.as_millis() as f64);

        enigo.mouse_move_to(
            (start_x + time_fraction * delta_x) as i32,
            (start_y + time_fraction * delta_y) as i32
        );

        if !(time_fraction < 1.0) {
            break;
        }
    }

}

fn scroll_mouse(enigo: &mut Enigo, args: &mut Skip<env::Args>) {
    let amount = args.next().unwrap().parse::<i32>().unwrap();
    
    match args.next().unwrap().as_str() {
        "Up" => enigo.mouse_scroll_y(-amount),
        "Down" => enigo.mouse_scroll_y(amount),
        "Left" => enigo.mouse_scroll_x(-amount),
        "Right" => enigo.mouse_scroll_x(-amount),
        &_ => ()
    };
}

fn main() {
    let mut enigo: Enigo = Enigo::new();
    let mut args: std::iter::Skip<env::Args> = env::args().skip(1);
    
    match args.next().unwrap().as_str() {
        "click" => click_mouse(&mut enigo, &mut args),
        "move" => move_mouse(&mut enigo, &mut args),
        "scroll" => scroll_mouse(&mut enigo, &mut args),
        &_ => exit(1),
    }
}