use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::stdin;
use std::io::BufReader;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

// TODO aces having 'multiple' values of 1 OR 11 !!!
// TODO split getting bet step from continuing to play step
// TODO DOUBLE DOWN
// TODO Split

// Cards
// Ace 1 OR 10 !!!
// 2, 3, 4, 5, 6, 7, 8, 9, 10, J(10), Q(10), K(10)

const PLAYER_STARTING_BALANCE: i32 = 500;

fn main() {
    let mut is_game_running: bool = true;

    create_player_profile_if_not_exists();

    let mut player_profile: PlayerProfile = load_player_profile_from_disk();

    if player_profile.balance <= 0 {
        println!(
            "We see you are out of chips. Here, have {} chips on the house.",
            PLAYER_STARTING_BALANCE
        );
        player_profile.balance = PLAYER_STARTING_BALANCE;
        save_player_profile_to_disk(&player_profile);
    }

    let mut player_action_buffer = String::new();

    let mut deck: Vec<u8> = Vec::<u8>::new();

    while is_game_running {
        println!("You now have {} chips.", player_profile.balance);
        println!("How much would you like to bet? (e)xit if you would like to leave the table.");

        let mut player_bet = 0;
        let mut has_player_bet = false;

        while !has_player_bet {
            stdin()
                .read_line(&mut player_action_buffer)
                .expect("Error: failed to read input from stdin.");

            player_action_buffer = player_action_buffer.trim().to_string();
            match player_action_buffer.as_str() {
                "e" => {
                    println!("Thanks for playing.");
                    std::process::exit(0);
                }
                val => match val.parse::<i32>() {
                    Ok(integer) => {
                        if integer > player_profile.balance {
                            println!(
                                    "You can only bet up to your balance {}. Please enter your bet again.",
                                    player_profile.balance
                                );
                        } else if integer <= 0 {
                            println!("You must bet at least 1 chip to play.");
                        } else {
                            player_bet = integer;
                            has_player_bet = true;
                        }
                    }
                    Err(_) => {
                        println!("Invalid input. Please enter your bet or (e)xit the table.");
                    }
                },
            }
            player_action_buffer = String::new();
        }

        player_profile.balance += play_hand(&mut deck, player_bet);

        save_player_profile_to_disk(&player_profile);

        if player_profile.balance <= 0 {
            println!("You are broke. You have been kicked out of the casino.");
            is_game_running = false;
        }
    }
}

fn play_hand(deck: &mut Vec<u8>, player_bet: i32) -> i32 {
    let mut dealer_hand: Vec<u8> = Vec::<u8>::new();
    let mut player_hand: Vec<u8> = Vec::<u8>::new();

    let mut player_action_buffer = String::new();

    let mut game_outcome: Option<GameOutcomes> = None;

    deal_from_deck(deck, &mut player_hand);
    deal_from_deck(deck, &mut dealer_hand);

    deal_from_deck(deck, &mut player_hand);
    deal_from_deck(deck, &mut dealer_hand);

    // TODO check for blackjack
    // check for blackjack here for dealer
    // check for player blackjacks...

    print_hands(&dealer_hand, &player_hand, true);

    // player action loop until they are done with hand
    let mut is_player_hand_done = false;
    while !is_player_hand_done {
        println!("(h)it or (s)tay?");

        let mut has_player_action = false;
        while !has_player_action {
            stdin()
                .read_line(&mut player_action_buffer)
                .expect("Error: failed to read player input from stdin.");
            match player_action_buffer.trim().to_lowercase().as_str() {
                "h" => {
                    has_player_action = true;

                    println!("You decided to hit!");
                    deal_from_deck(deck, &mut player_hand);
                    let player_hand_sum: u8 = player_hand.iter().sum();

                    if player_hand_sum > 21 {
                        println!("Sorry you have busted!");
                        is_player_hand_done = true;
                        game_outcome = Some(GameOutcomes::PlayerLost);
                    }
                }
                "s" => {
                    has_player_action = true;

                    println!("You decided to stay!");
                    is_player_hand_done = true;
                }
                _ => {
                    println!("Please enter a valid option. (h)it or (s)tay.");
                }
            }
            player_action_buffer = String::new();
        }

        print_hands(&dealer_hand, &player_hand, true);
        player_action_buffer = String::new();
    }

    if game_outcome.is_none() {
        println!("Dealer hand starts!");

        let mut is_dealer_hand_done = false;
        while !is_dealer_hand_done {
            let dealer_hand_sum: u8 = dealer_hand.iter().sum();

            if dealer_hand_sum < 17 {
                // dealer hit
                println!("Dealer hits!");
                deal_from_deck(deck, &mut dealer_hand);

                let mut dealer_hand_sum = 0;
                for i in &dealer_hand {
                    dealer_hand_sum += i;
                }

                if dealer_hand_sum > 21 {
                    println!("Dealer has busted!");
                    is_dealer_hand_done = true;
                    game_outcome = Some(GameOutcomes::PlayerWin);
                }
            } else {
                // dealer stay
                println!("Dealer stays!");
                is_dealer_hand_done = true;
            }
            print_hands(&dealer_hand, &player_hand, false);
            std::thread::sleep(Duration::from_millis(1000));
        }
    }

    // if there is not already a winner from earlier
    // compare hands
    if game_outcome.is_none() {
        let player_hand_sum: u8 = player_hand.iter().sum();

        let dealer_hand_sum = dealer_hand.iter().sum();

        println!("Dealer has {}", dealer_hand_sum);
        println!("Player has {}", player_hand_sum);

        match player_hand_sum.cmp(&dealer_hand_sum) {
            Ordering::Equal => {
                game_outcome = Some(GameOutcomes::Draw);
            }
            Ordering::Greater => {
                game_outcome = Some(GameOutcomes::PlayerWin);
            }
            Ordering::Less => {
                // if player_hand > dealer_hand
                game_outcome = Some(GameOutcomes::PlayerLost);
            }
        }
    }

    // end of hand logic - return bet to update balance
    match game_outcome {
        Some(GameOutcomes::PlayerWin) => {
            println!("You won {}!", player_bet);

            player_bet
        }
        Some(GameOutcomes::PlayerLost) => {
            println!("You lost {}!", player_bet);

            -player_bet
        }
        Some(GameOutcomes::Draw) => {
            println!("Push! no change!");

            0
        }
        None => {
            panic!("Invalid game_outcome None");
        }
    }
}

fn deal_from_deck(deck: &mut Vec<u8>, hand: &mut Vec<u8>) {
    if deck.is_empty() {
        shuffle_new_deck(deck);
    }

    let card = deck.remove(deck.len() - 1);

    hand.push(card);
}

fn shuffle_new_deck(deck: &mut Vec<u8>) {
    for _ in 0..4 {
        for j in 1..=13 {
            if j > 10 {
                deck.push(10);
            } else {
                deck.push(j);
            }
        }
    }

    deck.shuffle(&mut rand::thread_rng());
}

// TODO - perhaps we have multiple player hands
fn print_hands(dealer_hand: &Vec<u8>, player_hand: &Vec<u8>, hide_first_dealer_card: bool) {
    print_hand("Dealer", dealer_hand, hide_first_dealer_card);
    print_hand("Player", player_hand, false);
}

fn print_hand(player_name: &str, hand: &Vec<u8>, hide_first_card: bool) {
    let mut hand_string = "[".to_string();
    if hide_first_card {
        for i in 0..hand.len() {
            if i == 0 {
                hand_string.push('*');
            } else {
                hand_string.push_str(&(hand[i].to_string()));
            }
            if i < hand.len() - 1 {
                hand_string.push(' ');
            }
        }
    } else {
        for i in 0..hand.len() {
            hand_string.push_str(&(hand[i].to_string()));
            if i < hand.len() - 1 {
                hand_string.push(' ');
            }
        }
    }
    hand_string.push(']');

    println!("{}: {}", player_name, hand_string);
}

enum GameOutcomes {
    PlayerWin,
    Draw,
    PlayerLost,
}

fn get_player_profile_path_buf() -> PathBuf {
    let exe_path =
        std::env::current_exe().expect("Error: Failed to get the current executable path.");
    let exe_dir = exe_path
        .parent()
        .expect("Error: Failed to get directory of the current executable.");
    let file_name = "player_profile.json";
    let full_path = exe_dir.join(file_name);

    full_path
}

fn create_player_profile_if_not_exists() {
    let full_path = get_player_profile_path_buf();

    if !full_path.exists() {
        println!(
            "We see you are a new player! We are starting your account with {} chips.",
            PLAYER_STARTING_BALANCE
        );
        save_player_profile_to_disk(&PlayerProfile { balance: 500 })
    }
}

fn load_player_profile_from_disk() -> PlayerProfile {
    let full_path = get_player_profile_path_buf();

    // Open the file in read-only mode.
    let file = File::open(full_path).expect("Error: Player profile file not found.");

    let reader = BufReader::new(file);

    // Deserialize the JSON data into `MyStruct`.
    let player_data: PlayerProfile =
        serde_json::from_reader(reader).expect("Error: Failed to parse player profile data.");

    player_data
}

fn save_player_profile_to_disk(player_profile: &PlayerProfile) {
    let full_path = get_player_profile_path_buf();
    let file = File::create(&full_path).unwrap_or_else(|_| {
        panic!(
            "Error: Failed to create player profile file at {}",
            full_path.display()
        )
    });
    serde_json::to_writer(file, player_profile).expect("Error: Failed to save player profile.");
}

// fn read_line_from_stdin() -> std::io::Result<String> {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)?;
//     Ok(input.trim().to_string())
// }

#[derive(Debug, Serialize, Deserialize)]
struct PlayerProfile {
    pub balance: i32,
}
