// Started:      24/01/24
// Last updated: 24/03/20



#![feature(let_chains)]

#![allow(unused)]
#![warn(unused_must_use)]

#![allow(clippy::new_without_default)]
#![warn(clippy::todo, clippy::unwrap_used)]



pub mod settings {
	use crate::prelude::*;
	
	pub fn lives_for_stage(stage_num: usize) -> usize {
		stage_num + 1
	}
	pub fn items_per_round_for_stage(stage_num: usize) -> usize {
		stage_num + 1
	}
	pub fn max_items_for_stage(stage_num: usize) -> usize {
		(stage_num + 1) * 2
	}
	pub fn buckshot_reload_data(stage_num: usize) -> (usize, usize, f32, f32) {
		(stage_num * 2, (stage_num * 4).min(8), 0.33, 0.66)
	}
	pub fn credit_per_shot_for_stage(stage_num: usize) -> usize {
		stage_num
	}
	pub fn credits_per_win_for_stage(stage_num: usize) -> usize {
		stage_num * 10
	}
	
	pub const ITEM_CHANCE_COMMON: f32   = 1.0;
	pub const ITEM_CHANCE_UNCOMMON: f32 = 0.6;
	pub const ITEM_CHANCE_RARE: f32     = 0.3;
	pub const ITEM_RARITIES: &[(Item, f32)] = &[
		(Item::Cigarettes     , ITEM_CHANCE_UNCOMMON),
		(Item::MagnifyingGlass, ITEM_CHANCE_UNCOMMON),
		(Item::Beer           , ITEM_CHANCE_COMMON  ),
		(Item::BarrelExtension, ITEM_CHANCE_UNCOMMON),
		(Item::Handcuffs      , ITEM_CHANCE_COMMON  ),
		(Item::UnknownTicket  , ITEM_CHANCE_RARE    ),
		(Item::LiveShell      , ITEM_CHANCE_COMMON  ),
		(Item::BlankShell     , ITEM_CHANCE_UNCOMMON),
	];
	
}



pub mod data;
pub mod utils;



pub mod prelude {
	pub use crate::{data::*, utils, settings};
	pub use rand::Rng;
	pub use anyhow::*;
	pub use smart_read::prelude::*;
}

use crate::prelude::*;
use rand::prelude::SliceRandom;



fn main() {
	
	let player_count = prompt!("How many players? (must be 2 or more) "; 2..);
	let mut players = vec![Player::new(); player_count];
	get_names_and_passwords(&mut players);
	utils::clear();
	
	let mut game_data = GameData {
		players,
		curr_player: 0,
		buckshot: vec!(),
		has_barrel_extension: false,
	};
	
	play_stage(&mut game_data, 1);
	play_stage(&mut game_data, 2);
	play_stage(&mut game_data, 3);
	
	game_data.players.sort_by(|a, b| a.credits.cmp(&b.credits));
	#[allow(clippy::unwrap_used)] // because players will never be empty
	let winner = game_data.players.last().unwrap();
	
	println!("The game is over.");
	utils::wait_and_clear();
	println!("Player {} has won with {} {}", winner.name, winner.credits, utils::pluralize(winner.credits as f32, "credit", "credits"));
	
	println!();
	println!("Final credits:");
	for player in game_data.players.iter().rev() {
		println!("{}: {}", player.name, player.credits);
	}
	
}



pub fn get_names_and_passwords(players: &mut [Player]) {
	for i in 0..players.len() {
		utils::clear();
		println!("Enter the data for player {}", i + 1);
		
		'name_input: loop {
			let new_name = prompt!("Player name: "; NonEmptyInput);
			for player in players.iter().take(i) {
				if player.name == new_name {
					println!("Name is already in use.");
					continue 'name_input;
				}
			}
			players[i].name = new_name;
			break 'name_input;
		}
		
		let player = &mut players[i];
		'password_input: loop {
			let password_1 = prompt!("Password : (empty means no pw)) ");
			let password_2 = prompt!("Re-enter password to confirm: ");
			if password_1 != password_2 {println!("Passwords do not match."); continue 'password_input;}
			player.password = if password_1.is_empty() {None} else {Some(password_1)};
			break 'password_input;
		}
		
	}
}





pub fn play_stage(game_data: &mut GameData, stage_num: usize) {
	
	println!("Stage {stage_num}.");
	
	let lives = settings::lives_for_stage(stage_num);
	println!("Every player has {lives} {}", utils::pluralize(lives as f32, "life", "lives"));
	for player in &mut game_data.players {
		player.lives = lives;
	}
	
	utils::wait_and_clear();
	
	let mut round_num = 0;
	loop {
		
		// start round
		if game_data.curr_player == 0 {
			round_num += 1;
			
			println!("Round {round_num}");
			utils::wait_and_clear();
			
			// give items
			let items_per_round = settings::items_per_round_for_stage(stage_num);
			let max_items = settings::max_items_for_stage(stage_num);
			for player in &mut game_data.players {
				if player.lives == 0 {continue;}
				let new_items_count = items_per_round.min(max_items - player.items.len());
				let mut new_items = Vec::with_capacity(new_items_count);
				for _ in 0..new_items_count {
					let new_item = Item::random();
					new_items.push(new_item);
				}
				println!("Player {} gets: {}", player.name, utils::stringify_list(&new_items));
				player.items.append(&mut new_items);
				utils::wait_and_clear();
			}
			
		}
		
		play_turn(game_data, stage_num);
		if game_data.count_alive_players() < 2 {break;}
		
		'inc_curr_player: loop {
			game_data.curr_player = (game_data.curr_player + 1) % game_data.players.len();
			if game_data.get_player().lives > 0 {break 'inc_curr_player;}
		}
	}
	
	println!("Stage {stage_num} has ended.");
	println!("All items are removed");
	for player in &mut game_data.players {
		player.items.clear();
	}
	
	let credits_for_win = settings::credits_per_win_for_stage(stage_num);
	for player in &mut game_data.players {
		if player.lives == 0 {continue;}
		player.credits += credits_for_win;
		println!();
		println!("Player {} gets {} {}", player.name, credits_for_win, utils::pluralize(credits_for_win as f32, "credit", "credits"));
		utils::wait_and_clear();
	}
	
}



pub fn play_turn(game_data: &mut GameData, stage_num: usize) {
	
	reload_buckshot_if_needed(&mut game_data.buckshot, stage_num);
	
	let player = game_data.get_player_mut();
	println!("Starting {}'s turn.", player.name);
	
	// handcuffs
	match player.handcuffed_level {
		HandcuffedLevel::Uncuffed => {}
		HandcuffedLevel::AlmostUncuffed => {
			player.handcuffed_level = HandcuffedLevel::Uncuffed;
			println!();
			println!("You are now uncuffed.");
		}
		HandcuffedLevel::NewlyHandcuffed => {
			player.handcuffed_level = HandcuffedLevel::AlmostUncuffed;
			println!();
			println!("You are handcuffed, skipping turn.");
			println!();
			return;
		}
	}
	utils::wait_and_clear();
	
	// reset password
	if player.is_resetting_password {
		player.is_resetting_password = false;
		println!("Skipping turn, resetting password instead");
		loop {
			let password_1 = prompt!("Password : (empty means no pw)) ");
			let password_2 = prompt!("Re-enter password to confirm: ");
			if password_1 != password_2 {println!("Passwords do not match."); continue;}
			player.password = if password_1.is_empty() {None} else {Some(password_1)};
			break;
		}
		return;
	}
	
	// password check
	utils::clear();
	println!("Player {}'s turn.", player.name);
	if let Some(password) = &player.password {
		loop {
			let input = prompt!("Enter your password:");
			if &input == password {break;}
			let retry = prompt!("Password is incorrect. Do you want to try again? "; YesNoInput);
			if retry {continue;}
			let reset = prompt!("Do you want to skip your turn and reset? (no = retry) "; YesNoInput);
			if reset {
				player.is_resetting_password = true;
				return;
			}
			utils::clear();
		}
	}
	
	// main turn
	let mut can_use_unknown_ticket = true;
	'outer: loop {
		let mut can_trade = true;
		'inner: loop {
			utils::clear();
			println!("Player {}'s turn.", game_data.get_player().name);
			print_stats(game_data);
			println!();
			let options: &[&str] = if can_trade {&["shoot", "use item", "trade"]} else {&["shoot", "use item"]};
			match read!(options) {
				"shoot" => {
					let shot_ends_turn = shoot(game_data, stage_num);
					if shot_ends_turn {break;}
				},
				"use item" => {
					let popped_last_shell = use_item(game_data, stage_num);
					if popped_last_shell {break 'inner;}
				}
				"trade" => trade(game_data, &mut can_trade),
				_ => unreachable!(),
			}
		}
		
		// unknown ticket
		let live_player_count = game_data.count_alive_players();
		let player: &mut Player = game_data.get_player_mut();
		if
			can_use_unknown_ticket
			&& live_player_count > 1
			&& player.lives > 0
			&& let Some((unknown_ticket_index, _item)) = player.items.iter().enumerate().find(|(_index, item)| **item == Item::UnknownTicket)
		{
			let use_ticket = prompt!("Use unknown ticket? "; YesNoInput);
			if use_ticket {
				can_use_unknown_ticket = false;
				player.items.remove(unknown_ticket_index);
				continue 'outer;
			}
		}
		
		break 'outer;
	}
	
	println!("Your turn has ended.");
	utils::wait_and_clear();
	
}



pub fn reload_buckshot_if_needed(buckshot: &mut Vec<bool>, stage_num: usize) {
	if !buckshot.is_empty() {return;}
	let mut rng = rand::thread_rng();
	
	let (min_bullets, max_bullets, min_percent, max_percent) = settings::buckshot_reload_data(stage_num);
	let bullet_count = rng.gen_range(min_bullets ..= max_bullets);
	let live_percent = rng.gen_range(min_percent ..= max_percent);
	let live_count = (bullet_count as f32 * live_percent).round() as usize;
	let blank_count = bullet_count - live_count;
	*buckshot = Vec::with_capacity(bullet_count);
	for _ in 0..live_count {buckshot.push(true);}
	for _ in 0..blank_count {buckshot.push(false);}
	buckshot.shuffle(&mut rng);
	
	println!("The buckshot is loaded with {live_count} {} and {blank_count} {}", utils::pluralize(live_count as f32, "live", "lives"), utils::pluralize(blank_count as f32, "blank", "blanks"));
	utils::wait_and_clear();
	
}



pub type ShotEndsTurn = bool;

pub fn shoot(game_data: &mut GameData, stage_num: usize) -> ShotEndsTurn {
	utils::clear();
	let player_names =
		game_data.players.iter()
		.filter_map(|p| utils::some_if(&p.name, p.lives > 0))
		.collect::<Vec<_>>();
	let to_shoot = prompt!("Who do you want to shoot?"; player_names);
	let to_shoot_index = game_data.index_of_player(to_shoot);
	let confirm = prompt!(format!("Are you sure you want to shoot {to_shoot}? "); YesNoInput);
	if !confirm {return false;}
	println!();
	utils::clear();
	println!("You point the buckshot at {to_shoot}.");
	utils::wait_and_clear();
	let is_live = game_data.buckshot.pop().unwrap_or_else(|| panic!("The buckshot is empty."));
	let credits = settings::credit_per_shot_for_stage(stage_num);
	let mut shot_ends_turn;
	if is_live {
		println!("The buckshot shoots. You are given {credits} {}.", utils::pluralize(credits as f32, "credit", "credits"));
		let damage = if game_data.has_barrel_extension {2} else {1};
		if damage >= game_data.players[to_shoot_index].lives {
			utils::wait_and_clear();
			println!("{to_shoot} has lost all lives.");
			game_data.players[to_shoot_index].lives = 0;
		} else {
			game_data.players[to_shoot_index].lives -= damage;
		}
		shot_ends_turn = true;
	} else {
		println!("The buckshot clicks. You are given {credits} {}.", utils::pluralize(credits as f32, "credit", "credits"));
		shot_ends_turn = to_shoot_index != game_data.curr_player;
	}
	game_data.get_player_mut().credits += credits;
	game_data.has_barrel_extension = false;
	utils::wait_and_clear();
	if game_data.has_barrel_extension {
		game_data.has_barrel_extension = false;
		println!("The barrel extension is removed.");
		utils::wait_and_clear();
	}
	if game_data.buckshot.is_empty() {shot_ends_turn = true;}
	reload_buckshot_if_needed(&mut game_data.buckshot, stage_num);
	shot_ends_turn
}



pub type PoppedLastShell = bool;

pub fn use_item(game_data: &mut GameData, stage_num: usize) -> PoppedLastShell {
	if game_data.get_player().items.is_empty() {
		println!();
		println!("You do not have any items to use.");
		utils::wait_and_clear();
		return false;
	}
	utils::clear();
	println!("Which item do you want to use?");
	let (to_use_index, to_use) = read!(EnumerateInput (&*game_data.get_player().items));
	let mut popped_last_shell = false;
	match to_use {
		
		Item::Cigarettes => {
			let player = game_data.get_player_mut();
			let mut prompt = String::from("Are you sure you want to use this item? ");
			let max_lives = settings::lives_for_stage(stage_num);
			if player.lives == max_lives {prompt += "You are already at max lives. ";}
			let confirm = prompt!(prompt; YesNoInput);
			if !confirm {return false;}
			player.lives = (player.lives + 1).min(max_lives);
			println!("You used cigarettes.");
			utils::wait_and_clear();
		}
		
		Item::MagnifyingGlass => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			if *game_data.buckshot.last().unwrap_or_else(|| panic!("The buckshot is empty.")) {
				println!("The next shell is live.");
			} else {
				println!("The next shell is blank.");
			}
			utils::wait_and_clear();
		}
		
		Item::Beer => {
			let mut prompt = String::from("Are you sure you want to use this item? ");
			if game_data.buckshot.len() == 1 {prompt += "This is the last round, popping it will end your turn. ";}
			let confirm = prompt!(prompt; YesNoInput);
			if !confirm {return false;}
			if game_data.buckshot.pop().unwrap_or_else(|| panic!("The buckshot is empty.")) {
				println!("You popped a live shell.");
			} else {
				println!("You popped a blank shell.");
			}
			if game_data.buckshot.is_empty() {
				popped_last_shell = true;
			}
			utils::wait_and_clear();
			reload_buckshot_if_needed(&mut game_data.buckshot, stage_num);
		}
		
		Item::BarrelExtension => {
			if game_data.has_barrel_extension {println!("Barrel already has extension"); return false;}
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			game_data.has_barrel_extension = true;
			println!("You used a barrel extension.");
			utils::wait_and_clear();
		}
		
		Item::Handcuffs => {
			let curr_player_name = &game_data.get_player().name;
			let player_names =
				game_data.players.iter()
				.filter_map(|p| utils::some_if(&p.name, p.lives > 0 && &p.name != curr_player_name))
				.collect::<Vec<_>>();
			let to_handcuff = prompt!("Who do you want to handcuff? "; player_names);
			let to_handcuff_index = game_data.index_of_player(to_handcuff);
			let confirm = prompt!(format!("Are you sure you want to handcuff {to_handcuff}? "); YesNoInput);
			if !confirm {return false;}
			let to_handcuff_player = &mut game_data.players[to_handcuff_index];
			if to_handcuff_player.handcuffed_level != HandcuffedLevel::Uncuffed {
				println!("This player is already cuffed");
				return false;
			}
			to_handcuff_player.handcuffed_level = HandcuffedLevel::NewlyHandcuffed;
			println!("You used handcuffs.");
			utils::wait_and_clear();
		}
		
		Item::UnknownTicket => {
			println!("You cannot use this item right now.");
			utils::wait_and_clear();
		}
		
		Item::LiveShell => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			let index = rand::thread_rng().gen_range(0..=game_data.buckshot.len());
			game_data.buckshot.insert(index, true);
			println!("You used added the shell.");
			utils::wait_and_clear();
		}
		
		Item::BlankShell => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			let index = rand::thread_rng().gen_range(0..=game_data.buckshot.len());
			game_data.buckshot.insert(index, false);
			println!("You used added the shell.");
			utils::wait_and_clear();
		}
		
	}
	game_data.get_player_mut().items.remove(to_use_index);
	popped_last_shell
}



pub fn trade(game_data: &mut GameData, can_trade: &mut bool) {
	
	let curr_player_name = &game_data.get_player().name;
	let player_names =
		game_data.players.iter()
		.filter_map(|p| utils::some_if(&p.name, p.lives > 0 && &p.name != curr_player_name))
		.collect::<Vec<_>>();
	let other_player_name = prompt!("Who do you want to trade with? "; player_names);
	let other_player = game_data.index_of_player(other_player_name);
	let mut curr_player_items = vec!();
	let mut other_player_items = vec!();
	
	loop {
		
		utils::clear();
		println!("Your items:");
		for item in &game_data.get_player().items {
			println!("{item}");
		}
		println!();
		println!("Other's items:");
		for item in &game_data.players[other_player].items {
			println!("{item}");
		}
		println!();
		println!("You're trading:");
		for item in &curr_player_items {
			println!("{item}");
		}
		println!();
		println!("Other is trading:");
		for item in &other_player_items {
			println!("{item}");
		}
		println!();
		println!("Commands:");
		println!("Finalize");
		println!("Cancel");
		println!("[add / remove] [my / other] [item name]  (example: \"remove my live shell\")");
		let input = read!().trim().to_lowercase();
		
		// finalize
		if input == "f" || input == "finalize" {
			let trade_was_accepted = prompt_accept_trade(game_data, game_data.curr_player, &curr_player_items, &other_player_items);
			if !trade_was_accepted {return;}
			let trade_was_accepted = prompt_accept_trade(game_data, other_player, &other_player_items, &curr_player_items);
			if !trade_was_accepted {return;}
			for item in curr_player_items {
				game_data.players[other_player].items.push(item);
			}
			for item in other_player_items {
				game_data.get_player_mut().items.push(item);
			}
			println!("The trade was successful.");
			*can_trade = false;
			utils::wait_and_clear();
			return;
		}
		
		// cancel
		if input == "c" || input == "cancel" {
			let confirm = prompt!("Are you sure you want to cancel"; YesNoInput);
			if !confirm {continue;}
			println!("Trade has been canceled.");
			return;
		}
		
		// add / remove
		let input_parts = input.split(' ').collect::<Vec<_>>();
		if input_parts.len() < 3 {
			println!("Could not understand input, did not match \"finalize\", \"cancel\", or 3+ tokens");
			utils::wait();
			continue;
		}
		let is_add = match input_parts[0] {
			"add" | "a" => true,
			"remove" | "r" => false,
			_ => {
				println!("Could not understand input, first token did not match \"finalize\", \"cancel\", \"add\", or \"remove\"");
				utils::wait();
				continue;
			}
		};
		let is_mine = match input_parts[1] {
			"my" | "m" => true,
			"other" | "o" => false,
			_ => {
				println!("Could not understand input, second token did not match \"my\" or \"other\"");
				utils::wait();
				continue;
			}
		};
		let Some(item_to_move) = Item::from_strs(&input_parts[2..]) else {
			println!("Could not understand input, unknown item");
			utils::wait();
			continue;
		};
		let (from, to) = match (is_add, is_mine) {
			(true, true) => (&mut game_data.get_player_mut().items, &mut curr_player_items),
			(true, false) => (&mut game_data.players[other_player].items, &mut other_player_items),
			(false, true) => (&mut curr_player_items, &mut game_data.get_player_mut().items),
			(false, false) => (&mut other_player_items, &mut game_data.players[other_player].items),
		};
		let mut removed = false;
		for i in 0..from.len() {
			if from[i] != item_to_move {continue;}
			from.remove(i);
			removed = true;
			break;
		}
		if !removed {
			println!("Could not find item {item_to_move} in list.");
			utils::wait();
			continue;
		}
		to.push(item_to_move);
		
	}
	
}



pub type DidTrade = bool;

pub fn finalize_trade(game_data: &mut GameData, other_player: usize, curr_player_trading_items: &[Item], other_player_trading_items: &[Item]) -> DidTrade {
	
	let trade_was_accepted = prompt_accept_trade(game_data, game_data.curr_player, curr_player_trading_items, other_player_trading_items);
	if !trade_was_accepted {return false;}
	let trade_was_accepted = prompt_accept_trade(game_data, other_player, other_player_trading_items, curr_player_trading_items);
	if !trade_was_accepted {return false;}
	
	let curr_player_items = &mut game_data.get_player_mut().items;
	for &item in curr_player_trading_items {
		for i in 0..curr_player_items.len() {
			if curr_player_items[i] != item {continue;}
			curr_player_items.remove(i);
			break;
		}
	}
	
	let other_player_items = &mut game_data.players[other_player].items;
	for &item in other_player_trading_items {
		for i in 0..other_player_items.len() {
			if other_player_items[i] != item {continue;}
			other_player_items.remove(i);
			break;
		}
	}
	
	true
}



pub type TradeWasAccepted = bool;

pub fn prompt_accept_trade(game_data: &GameData, curr_player: usize, curr_player_trading_items: &[Item], other_player_trading_items: &[Item]) -> TradeWasAccepted {
	
	utils::clear();
	println!("Addressing {}:", game_data.players[curr_player].name);
	println!();
	println!("You're trading:");
	for item in curr_player_trading_items {
		println!("{item}");
	}
	println!();
	println!("Other is trading:");
	for item in other_player_trading_items {
		println!("{item}");
	}
	println!();
	let confirm = prompt!("Do you agree to this trade? "; YesNoInput);
	if !confirm {
		println!();
		println!("Trade has been canceled.");
		utils::wait_and_clear();
		return false;
	}
	loop {
		let input = prompt!("Enter your password to continue: (if you didn't set a password, enter nothing) ");
		let Some(password) = &game_data.players[curr_player].password else {break;};
		if password == &input {break;}
		let retry = prompt!("Password is incorrect. Try again? "; YesNoInput);
		if retry {continue;}
		let cancel = prompt!("Do you want to cancel the trade? (no = retry) "; YesNoInput);
		if cancel {
			println!("Trade has been canceled.");
			return false;
		}
	}
	
	println!("You have agreed to the trade");
	utils::wait_and_clear();
	true
}



pub fn print_stats(game_data: &GameData) {
	for player in &game_data.players {
		println!();
		println!("Player {}:", player.name);
		println!("\tLives: {}", player.lives);
		println!("\tCredits: {}", player.credits);
		println!("\tIs handcuffed: {}", if player.handcuffed_level == HandcuffedLevel::Uncuffed {"false"} else {"true"});
		println!("\tItems:");
		for item in &player.items {
			println!("\t\t{item}");
		}
	}
}
