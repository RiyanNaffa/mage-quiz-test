mod audio_player;
mod story;
mod quiz;

use story::*;
use quiz::*;

use std::io;


fn main() -> Result<(), &'static str> {
    audio_player::play();
    // quiz_1();
    opening();
    intro();
    let (mut name, mut age): (String, String) = (String::new(), String::new());
    println!("Enter your name below:");
    let name_input = match io::stdin().read_line(&mut name) {
        Ok(_) => name.to_owned(),
        Err(_) => {
            println!("Cannot read your name. System will refer to you as 'Player1'.");
            "Player1".to_owned()
        },
    };
    println!("Enter your age below:");
    let age_input = match io::stdin().read_line(&mut age){
        Ok(_) => {
            let temp = age.trim().to_owned();
            match temp.parse::<i32>(){
                Ok(i) => i,
                Err(_) => {
                    println!("Cannot read your age. System will take 15 as your age.");
                    15
                }
            }
        },
        Err(_) => {
            println!("Cannot read your age. System will take 15 as your age.");
            15
        }
    };
    // ===== CONSTRUCT A NEW PLAYER =====
    let mut player = Player::new(&name_input, age_input);

    // Game Over by Under Age ;_;
    match player.age_confirmation_message() {
        true => return Err("Underage player."),
        _ => (),
    }
    
    // START!
    start_message();

    // ===== CHAPTER 1 =====
    chapter_1();
    match quiz_1(&mut player){
        // GAME OVER BY LOSING
        true => return Err("Losing a stage."),
        _ => (),
    }
    
    // ===== CHAPTER 2 =====
    chapter_2();
    match quiz_2(&mut player){
        // GAME OVER BY LOSING
        true => return Err("Losing a stage."),
        _ => (),
    }
    
    // ===== CHAPTER 3 =====
    chapter_3();
    match quiz_3(&mut player){
        // GAME OVER BY LOSING
        true => return Err("Losing a stage."),
        _ => (),
    }
    
    // ===== GAME OVER BY WINNING ======
    // (kind of exhaustive, until the end)
    game_over_message(player.get_name(), player.get_progress());
    player.progress_message();
    player.the_end_progress();
    player.display_trophies();
    Ok(())
}
