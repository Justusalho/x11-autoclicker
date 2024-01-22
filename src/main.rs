use std::{time::{Duration, Instant}, thread::sleep, process::exit};
use enigo::{Enigo, MouseControllable, MouseButton};
use rustop::opts;

fn main() {

    // ---
    // Parse the command line arguments
    // ---

    let (args, _rest) = opts! {
        synopsis "A simple autoclicker program for the command line. (x11)";
        opt mouse_x:Option<i32>,
            desc:"Cursor x position. (default: Current position)",
            short:'x';
        opt mouse_y:Option<i32>,
            desc:"Cursor y position. (default: Current position)",
            short:'y';
        opt click_rate:f64=5.,
            desc:"Clicks/second. Must be 0-200. (Set a value between 0 and 1 to have gaps >1s)",
            short:'r';
        opt button:String=String::from("left"), desc:"Mouse button to click.";
        opt click_limit:Option<u64>, desc:"A limit on the number of total clicks.";
        opt time_limit:Option<String>, desc:"A time limit on the clicker.";
        opt wait_period:Option<String>, desc:"Time to wait before starting the clicker.";
        opt prevent_movement:bool=false,
            desc:"Always move the cursor back to given coordinates before clicking. (Needs either a limit or the flag --force)";
        opt force:bool, desc:"Force run even with potentially bad options.";
        opt verbose:bool=false, desc:"Print the location of each click.";
    }
    .parse_or_exit();

    
    // ---
    // Set up the clicker according to the parameters
    // ---

    let mut enigo = Enigo::new();
    
    // Mouse position
    let x = args.mouse_x.unwrap_or(enigo.mouse_location().0);
    let y = args.mouse_y.unwrap_or(enigo.mouse_location().1);
    
    // Click rate
    if args.click_rate < 0. || args.click_rate > 200. {
        println!("Error: Value of option click-rate must be 0-200");
        exit(1);
    }
    let click_cooldown = Duration::from_secs_f64(1. / args.click_rate as f64);

    // Button
    let button = match args.button.as_str() {
        "left"    | "l" => MouseButton::Left,
        "right"   | "r" => MouseButton::Right,
        "middle"  | "m" => MouseButton::Middle,
        "forward" | "f" => MouseButton::Forward,
        "back"    | "b" => MouseButton::Back,
        _ => {
            println!("Error: Supported mouse buttons are: left, right, middle, forward, back!");
            exit(1);
        }
    };

    // Click and time limits
    let limit_clicks = args.click_limit.is_some();
    let mut click_counter = args.click_limit.unwrap_or(0);

    let limit_time = args.time_limit.is_some();
    let mut timer = parse_duration::parse(
        &args.time_limit.unwrap_or(String::from("0"))
    ).unwrap_or_else(|_| {
        println!("Error: Could not parse time from given string!");
        exit(1);
    });

    // Wait period
    let wait = parse_duration::parse(
        &args.wait_period.unwrap_or(String::from("0"))
    ).unwrap_or_else(|_| {
        println!("Error: Could not parse time from given string!");
        exit(1);
    });

    // Prevent movement
    if args.prevent_movement && !args.force {
        if !( limit_clicks || limit_time ) {
            println!("Error: Option prevent-movement is not allowed without a time or click limit. (Use flag -f to force these options)");
            exit(1);
        }
    }
    
    sleep(wait);


    // ---
    // Start the clicker
    // ---

    enigo.mouse_move_to(x, y);
    
    let mut prev_time = Instant::now();
    loop {

        // Decrease the click counter if applicable
        if limit_clicks {
            if click_counter < 1 {
                break;
            } else {
                click_counter -= 1;
            }
        }

        // ... and the same for timer
        if limit_time {
            let current_time = Instant::now();
            if let Some(remaining_time) = timer.checked_sub(current_time-prev_time) {
                timer = remaining_time;
            } else {
                break;
            }
            prev_time = current_time;
        }
        
        // Move the mouse to (x,y)
        if args.prevent_movement {
            enigo.mouse_move_to(x, y);
        }

        // Print mouse location
        if args.verbose {
            println!(
                "Clicking at ({};{})",
                enigo.mouse_location().0,
                enigo.mouse_location().1
            );
        }

        // Click the specified mouse button
        enigo.mouse_click(button);

        sleep(click_cooldown);
    }

}
