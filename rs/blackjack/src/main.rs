use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::time::Duration;

// command line interface - for now we'll call the exe and play blackjack maybe with a certain balance ...(maybe save this to a file and pickup later?)...

// start the game loop DONE
// shuffle a deck of cards DONE
// deal starting cards DONE
// LATER check for blackjack - short circuit if dealer or player has blackjack + calc payouts
// get player input - hit or stay DONE
// no more player actions? evaluate hands - who wins DONE
// update balance DONE
// go back to the start of the loop DONE

// MISC TODO keep a running deck between hands instead of refreshing the deck each time...
// TODO aces having 'multiple' values of 1 OR 11 !!!
// TODO read player balance from disk - maybe enter who is playing, and look for the file - save balance when they leave the table...
// TODO split getting bet step from continuing to play step
// TODO DOUBLE DOWN
// TODO Split

// Cards
// Ace 1 OR 10 !!!
// 2, 3, 4, 5, 6, 7, 8, 9, 10, J(10), Q(10), K(10)

fn main() {
    let mut is_game_running: bool = true;

    let mut player_balance = 500; // TODO load from filesystem???
    let mut player_action_buffer = String::new();

    while is_game_running {
        println!("You now have {} chips.", player_balance);
        println!(
            "How much would you like to bet? Enter anything else if you would like to leave the table."
        );

        let mut player_bet = 0;
        let mut has_player_bet = false;

        while !has_player_bet {
            let _ = stdin().read_line(&mut player_action_buffer);

            match player_action_buffer.trim().parse::<i32>() {
                Ok(integer) => {
                    if integer > player_balance {
                        println!(
                            "You can only bet up to your balance {}. Please enter your bet again.",
                            player_balance
                        );
                    } else {
                        player_bet = integer;
                        has_player_bet = true;
                    }
                }
                Err(_) => {
                    println!("Thanks for playing.");
                    std::process::exit(0);
                }
            }
            player_action_buffer = String::new();
        }

        player_balance += play_hand(player_bet);

        if player_balance <= 0 {
            println!("You are broke. You have been kicked out of the casino.");
            is_game_running = false;
        }
    }
}

fn play_hand(player_bet: i32) -> i32 {
    let mut deck: Vec<u8> = Vec::<u8>::new();

    for _ in 0..4 {
        for j in 1..=13 {
            if j > 10 {
                deck.push(10);
            } else {
                deck.push(j);
            }
        }
    }

    // shuffle the deck
    deck.shuffle(&mut thread_rng());

    let mut dealer_hand: Vec<u8> = Vec::<u8>::new();
    let mut player_hand: Vec<u8> = Vec::<u8>::new();

    let mut player_action_buffer = String::new();

    let mut game_outcome: Option<GameOutcomes> = None;

    deal_from_deck(&mut deck, &mut player_hand);
    deal_from_deck(&mut deck, &mut dealer_hand);

    deal_from_deck(&mut deck, &mut player_hand);
    deal_from_deck(&mut deck, &mut dealer_hand);

    // TODO check for blackjack
    // check for blackjack here for dealer
    // check for player blackjacks...

    print_hands(&dealer_hand, &player_hand, true);

    // player action loop until they are done with hand
    let mut is_player_hand_done = false;
    while !is_player_hand_done {
        println!("(h)it or (s)tay?");

        // do this a better way
        // in a while loop - complain about bad user input until we get good input
        // TODO double down???
        let mut has_player_action = false;
        while !has_player_action {
            let _ = stdin().read_line(&mut player_action_buffer);
            match player_action_buffer.trim().to_lowercase().as_str() {
                "h" => {
                    has_player_action = true;

                    println!("You decided to hit!");
                    deal_from_deck(&mut deck, &mut player_hand);
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
                deal_from_deck(&mut deck, &mut dealer_hand);

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
    let card = deck.remove(deck.len() - 1);

    hand.push(card);
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
