//timer imports
extern crate timer;
extern crate chrono;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::path::PathBuf;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;

//filepicker import
use native_dialog::{FileDialog, MessageDialog, MessageType};


/*TODO
    -find a way to make program work at different intervals via timer
    -maybe find a way to choose an animation file via filepicker
    -print out each frame separated by =====
*/

// For my purposes I think a repeater or a single call within a for loop will work best to playback the animation
// Look at evil_hangman make_word_database() for how a read buffer works
fn timer_pract(){
    println!("repeat or single callback?");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
    let answer = line.trim();

    let timer = timer::Timer::new();
    if answer.eq("1"){
        let (tx, rx) = channel();

        let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
            // This closure is executed on the scheduler thread,
            // so we want to move it away asap.

            let _ignored = tx.send(()); // Avoid unwrapping here.
        });

        rx.recv().unwrap();
        println!("This code has been executed after 3 seconds");
    }else{
        
        // Number of times the callback has been called.
        let count = Arc::new(Mutex::new(0));

        // Start repeating. Each callback increases `count`.
        let guard = {
            let count = count.clone();
            timer.schedule_repeating(chrono::Duration::milliseconds(100), move || {
                *count.lock().unwrap() += 1;

                // Put what you want to repeat beneath here
                println!("repeat");
            })
        };

        // Sleep one second. The callback should be called ~200 times.
        thread::sleep(std::time::Duration::new(1, 0));
        let count_result = *count.lock().unwrap();

        // Now drop the guard. This should stop the timer.
        drop(guard);
        thread::sleep(std::time::Duration::new(0, 100));
    }
    
}

fn file_picker_pract(){
    //File Chooser Dialouge
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Text File", &["txt"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    //Confirmation box
    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        println!("{:#?}", path);
    }
}

fn pick_anim() {
    let path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Text File", &["txt"])
        .show_open_single_file()
        .unwrap();

    let path = match path {
        Some(path) => path,
        None => return,
    };

    //Confirmation box
    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();
    if yes{
        let mut speed = 1;
        print!("How fast would you like to play the animation? (frame rate = answer * 100 miliseconds): ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok().expect("failed to read line");
        let answer = line.trim();

        if answer.eq("0") || answer.is_empty(){
            play_anim(path, speed);
        }else{
            speed = answer.parse::<i64>().expect("Could not parse first Row input");
            play_anim(path, speed);
        }
    }
}

fn play_anim(file:PathBuf, speed:i64){
    let timer = timer::Timer::new();
    let reader = BufReader::new(File::open(file).expect("Can't read dictionary"));
    
    let mut scene = Vec::new();
    for line in reader.lines(){
        let row = line.unwrap();
        
        if row.eq("====="){

            let (tx, rx) = channel();
            let _guard = timer.schedule_with_delay(chrono::Duration::milliseconds(100*speed), move || {
                // This closure is executed on the scheduler thread,
                // so we want to move it away asap.
    
                let _ignored = tx.send(()); // Avoid unwrapping here.
            });
    
            rx.recv().unwrap();
            for frame in &scene{
                println!("{}", frame);
            }
            scene.clear();

        }else{
            scene.push(row);
        }
        

        
    }
}

fn main() {
    pick_anim();
}
