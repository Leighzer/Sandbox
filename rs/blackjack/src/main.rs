use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs::File;
use std::io::stdin;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

// TODO dealer blackjack insurance.. maybe
// TODO Split - probably add support for players having multiple hands

// Cards
// 1(or 11) 2, 3, 4, 5, 6, 7, 8, 9, 10, J(10), Q(10), K(10)

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

            match player_action_buffer.to_lowercase().trim() {
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

        player_profile.balance += play_hand(&mut deck, &player_profile, player_bet);

        save_player_profile_to_disk(&player_profile);

        if player_profile.balance <= 0 {
            println!("You are broke. You have been kicked out of the casino.");
            is_game_running = false;
        }
    }
}

fn play_hand(deck: &mut Vec<u8>, player_profile: &PlayerProfile, initial_player_bet: i32) -> i32 {
    let mut player_bet = initial_player_bet;
    let mut dealer_hand: Vec<u8> = Vec::<u8>::new();
    let mut player_hand: Vec<u8> = Vec::<u8>::new();

    let mut previous_player_actions: Vec<PlayerAction> = vec![];

    let mut player_action_buffer = String::new();

    let mut game_outcome: Option<GameOutcome> = None;

    deal_from_deck(deck, &mut player_hand);
    deal_from_deck(deck, &mut dealer_hand);

    deal_from_deck(deck, &mut player_hand);
    deal_from_deck(deck, &mut dealer_hand);

    let mut available_player_actions: Vec<PlayerAction> = get_player_actions(
        player_profile.balance,
        initial_player_bet,
        &player_hand,
        &previous_player_actions,
    );

    let is_player_blackjack = get_hand_sum(&player_hand) == 21;
    let is_dealer_blackjack = get_hand_sum(&dealer_hand) == 21;
    if is_player_blackjack && is_dealer_blackjack {
        println!("You and the dealer hit blackjack!");
        game_outcome = Some(GameOutcome::Draw);
    } else if is_player_blackjack {
        println!("You hit blackjack!");
        game_outcome = Some(GameOutcome::PlayerWinBlackjack);
    } else if is_dealer_blackjack {
        println!("The dealer hit blackjack!");
        game_outcome = Some(GameOutcome::PlayerLost);
    }

    // show blackjack hands
    if game_outcome.is_some() {
        print_hands(&dealer_hand, &player_hand, false);
    }

    if game_outcome.is_none() {
        print_hands(&dealer_hand, &player_hand, true);

        // player action loop until they are done with hand
        let mut is_player_hand_done = false;
        while !is_player_hand_done {
            print_player_actions(&available_player_actions);

            let mut has_player_action = false;
            while !has_player_action {
                stdin()
                    .read_line(&mut player_action_buffer)
                    .expect("Error: failed to read player input from stdin.");
                match player_action_buffer.to_lowercase().trim() {
                    "h" => {
                        if !available_player_actions.contains(&PlayerAction::Hit) {
                            println!("You cannot hit at this time. Please enter a valid option.");
                            print_player_actions(&available_player_actions);
                        } else {
                            previous_player_actions.push(PlayerAction::Hit);
                            has_player_action = true;

                            println!("You decided to hit!");
                            deal_from_deck(deck, &mut player_hand);
                            let player_hand_sum: u8 = get_hand_sum(&player_hand);

                            if player_hand_sum > 21 {
                                println!("Sorry you have busted!");
                                is_player_hand_done = true;
                                game_outcome = Some(GameOutcome::PlayerLost);
                            }
                        }
                    }
                    "s" => {
                        if !available_player_actions.contains(&PlayerAction::Stay) {
                            println!("You cannot stay at this time. Please enter a valid option.");
                            print_player_actions(&available_player_actions);
                        } else {
                            previous_player_actions.push(PlayerAction::Stay);
                            has_player_action = true;

                            println!("You decided to stay!");
                            is_player_hand_done = true;
                        }
                    }
                    "d" => {
                        if !available_player_actions.contains(&PlayerAction::DoubleDown) {
                            println!(
                                "You cannot double down at this time. Please enter a valid option."
                            );
                            print_player_actions(&available_player_actions);
                        } else {
                            previous_player_actions.push(PlayerAction::DoubleDown);
                            has_player_action = true;

                            player_bet *= 2;
                            println!(
                                "You decided to double down! Your bet is now {}!",
                                player_bet
                            );
                            deal_from_deck(deck, &mut player_hand);
                            let player_hand_sum: u8 = get_hand_sum(&player_hand);
                            if player_hand_sum > 21 {
                                println!("Sorry you have busted!");
                                game_outcome = Some(GameOutcome::PlayerLost);
                            }

                            is_player_hand_done = true;
                        }
                    }
                    "p" => {
                        if !available_player_actions.contains(&PlayerAction::Split) {
                            println!("You cannot split at this time. Please enter a valid option.");
                            print_player_actions(&available_player_actions);
                        } else {
                            // TODO do split stuff here
                        }
                    }
                    _ => {
                        println!("Please enter a valid option.");
                        print_player_actions(&available_player_actions);
                    }
                }
                player_action_buffer = String::new();
                available_player_actions = get_player_actions(player_profile.balance, player_bet, &player_hand, &previous_player_actions);
            }

            print_hands(&dealer_hand, &player_hand, true);
        }
    }

    if game_outcome.is_none() {
        println!("Dealer hand starts!");

        let mut is_dealer_hand_done = false;
        while !is_dealer_hand_done {
            let dealer_hand_sum: u8 = get_hand_sum(&dealer_hand);

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
                    game_outcome = Some(GameOutcome::PlayerWin);
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
        let player_hand_sum: u8 = get_hand_sum(&player_hand);

        let dealer_hand_sum = get_hand_sum(&dealer_hand);

        println!("Dealer has {}", dealer_hand_sum);
        println!("Player has {}", player_hand_sum);

        match player_hand_sum.cmp(&dealer_hand_sum) {
            Ordering::Equal => {
                game_outcome = Some(GameOutcome::Draw);
            }
            Ordering::Greater => {
                game_outcome = Some(GameOutcome::PlayerWin);
            }
            Ordering::Less => {
                // if player_hand > dealer_hand
                game_outcome = Some(GameOutcome::PlayerLost);
            }
        }
    }

    // end of hand logic - return bet to update balance
    match game_outcome {
        Some(GameOutcome::PlayerWin) => {
            println!("You won {}!", player_bet);

            player_bet
        }
        Some(GameOutcome::PlayerWinBlackjack) => {
            let player_bet_blackjack = (player_bet as f32 * 1.5) as i32;
            println!("You won {}!", player_bet_blackjack);

            player_bet_blackjack
        }
        Some(GameOutcome::PlayerLost) => {
            println!("You lost {}!", player_bet);

            -player_bet
        }
        Some(GameOutcome::Draw) => {
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

fn all_elements_equal<T: PartialEq>(vec: &[T]) -> bool {
    vec.first()
        .map(|first| vec.iter().all(|x| x == first))
        .unwrap_or(true)
}

fn get_player_actions(
    player_balance: i32,
    player_bet: i32,
    player_hand: &Vec<u8>,
    previous_player_actions: &[PlayerAction],
) -> Vec<PlayerAction> {
    let mut player_actions = vec![PlayerAction::Hit, PlayerAction::Stay];

    if player_bet * 2 <= player_balance
        && !previous_player_actions.contains(&PlayerAction::DoubleDown)
    {
        player_actions.push(PlayerAction::DoubleDown);
    }

    if player_hand.len() == 2 && all_elements_equal(player_hand) {
        //player_actions.push(PlayerAction::Split);
    }

    player_actions
}

fn print_player_actions(player_actions: &[PlayerAction]) {
    let player_actions_string_output = player_actions
        .iter()
        .map(|action| match action {
            PlayerAction::Hit => "(h)it",
            PlayerAction::Stay => "(s)tay",
            PlayerAction::DoubleDown => "(d)ouble down",
            PlayerAction::Split => "s(p)lit",
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", player_actions_string_output)
}

#[derive(Debug, PartialEq)]
enum PlayerAction {
    Hit,
    Stay,
    DoubleDown,
    Split,
}

enum GameOutcome {
    PlayerWin,
    PlayerWinBlackjack,
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

    #[allow(clippy::let_and_return)]
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

fn get_hand_sum(hand: &[u8]) -> u8 {
    let min_sum: u8 = hand.iter().sum();

    let number_of_aces = hand.iter().filter(|&&x| x == 1_u8).count() as u8;

    let max_ace_10_padding = (21_u8.saturating_sub(min_sum)) / 10_u8; // max amount of 10s we can add without going over 21

    // compare what aces we have to the ideal amount of padding to be added
    // make sure we add the best amount we can considering how much aces we have
    let ace_adjustment = std::cmp::min(number_of_aces, max_ace_10_padding);

    #[allow(clippy::let_and_return)]
    let hand_value = min_sum + (ace_adjustment * 10);

    hand_value
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

#[test]
fn test_get_hand_sum() {
    assert_eq!(get_hand_sum(&vec![10, 10]), 20);
    assert_eq!(get_hand_sum(&vec![10, 10, 10]), 30);
    assert_eq!(get_hand_sum(&vec![5, 6]), 11);
    assert_eq!(get_hand_sum(&vec![6, 10]), 16);
    assert_eq!(get_hand_sum(&vec![1, 10]), 21);
    assert_eq!(get_hand_sum(&vec![4, 5]), 9);
    assert_eq!(get_hand_sum(&vec![4, 5, 1]), 20);
    assert_eq!(get_hand_sum(&vec![4, 5, 1, 1]), 21);
    assert_eq!(get_hand_sum(&vec![4, 5, 1, 1, 1]), 12);
    assert_eq!(get_hand_sum(&vec![10, 10, 1]), 21);
    assert_eq!(get_hand_sum(&vec![10, 8]), 18);
    assert_eq!(get_hand_sum(&vec![10, 8, 1]), 19);
    assert_eq!(get_hand_sum(&vec![10, 8, 1, 1]), 20);
    assert_eq!(get_hand_sum(&vec![]), 0);
    assert_eq!(get_hand_sum(&vec![1]), 11);
    assert_eq!(get_hand_sum(&vec![1, 1]), 12);
    assert_eq!(get_hand_sum(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1]), 19);
    assert_eq!(get_hand_sum(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 20);
    assert_eq!(get_hand_sum(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 21);
    assert_eq!(get_hand_sum(&vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 12);
    assert_eq!(get_hand_sum(&vec![4]), 4);
}
