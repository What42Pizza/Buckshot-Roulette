// Started:      24/01/24
// Last updated: 24/04/04



#![feature(let_chains)]

#![allow(clippy::new_without_default)]
#![warn(clippy::todo, clippy::unwrap_used, clippy::expect_used)]



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
	pub fn get_item_rarity(item: Item) -> f32 {
		match item {
			Item::Cigarettes      => ITEM_CHANCE_UNCOMMON,
			Item::ExpiredMedicine => ITEM_CHANCE_COMMON  ,
			Item::MagnifyingGlass => ITEM_CHANCE_UNCOMMON,
			Item::Beer            => ITEM_CHANCE_COMMON  ,
			Item::BarrelExtension => ITEM_CHANCE_UNCOMMON,
			Item::Magazine        => ITEM_CHANCE_UNCOMMON,
			Item::Handcuffs       => ITEM_CHANCE_RARE    ,
			Item::UnknownTicket   => ITEM_CHANCE_RARE    ,
			Item::LiveShell       => ITEM_CHANCE_COMMON  ,
			Item::BlankShell      => ITEM_CHANCE_UNCOMMON,
			Item::GoldShell       => ITEM_CHANCE_RARE    ,
			Item::Inverter        => ITEM_CHANCE_UNCOMMON,
		}
	}
	
}



pub mod data;
pub mod utils;



pub mod prelude {
	pub use crate::{data::*, utils, settings};
	pub use rand::Rng;
	pub use anyhow::*;
	pub use smart_read::prelude::*;
}

use std::{fs::File, sync::Mutex};

use crate::prelude::*;
use rand::prelude::SliceRandom;



fn main() {
	
	let player_count = prompt!("How many players? (must be 2 or more) "; 2..);
	log!("Player count: {player_count}");
	let mut players = vec![Player::new(); player_count];
	get_names_and_passwords(&mut players);
	utils::clear();
	
	let add_house = prompt!("Add House? "; YesNoInput);
	log!("Add House: {add_house}");
	if add_house {
		let mut house = Player::new();
		house.name = String::from("House");
		players.insert(0, house);
	}
	utils::clear();
	
	let mut game_data = GameData {
		players,
		curr_player: 0,
		buckshot: vec!(),
		has_barrel_extension: false,
		inverter_used: false,
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
			let new_name = prompt!("Player name: "; SimpleValidate (|input| {
				let input = input.trim();
				if input.is_empty() {return Err(String::from("Invalid input, cannot be empty."));}
				if &*input.to_lowercase() == "house" {return Err(String::from("Invalid input, cannot name self \"House\"."))}
				std::result::Result::Ok(())
			}));
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
		
		log!("Name: {}, Password: {:?}", &player.name, &player.password);
	}
}





pub fn play_stage(game_data: &mut GameData, stage_num: usize) {
	
	println_log!("Stage {stage_num}.");
	
	let lives = settings::lives_for_stage(stage_num);
	println!("Every player has {lives} {}", utils::pluralize(lives as f32, "life", "lives"));
	for player in &mut game_data.players {
		player.lives = lives;
	}
	
	utils::wait_and_clear();
	
	let mut round_num = 1;
	println_log!("Round {round_num}");
	utils::wait_and_clear();
	give_items(game_data, stage_num);
	reload_buckshot_if_needed(game_data, stage_num);
	loop {
		
		if &*game_data.get_player().name == "House" {
			play_as_house(game_data, stage_num);
		} else {
			play_turn(game_data, stage_num);
		}
		if game_data.count_alive_players() < 2 {break;}
		
		'inc_curr_player: loop {
			game_data.curr_player += 1;
			if game_data.curr_player == game_data.players.len() {
				game_data.curr_player = 0;
				round_num += 1;
				println!("Round {round_num}");
				utils::wait_and_clear();
				give_items(game_data, stage_num);
			}
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



pub fn give_items(game_data: &mut GameData, stage_num: usize) {
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
		println_log!("Player {} gets: {}", player.name, utils::stringify_list(&new_items));
		player.items.append(&mut new_items);
		utils::wait_and_clear();
	}
}



pub fn play_as_house(game_data: &mut GameData, stage_num: usize) {
	'turn: loop {
		let house = &mut game_data.players[0];
		
		println_log!("Starting House's turn.");
		
		
		// handcuffs
		match house.handcuffed_level {
			HandcuffedLevel::Uncuffed => {}
			HandcuffedLevel::AlmostUncuffed => {
				house.handcuffed_level = HandcuffedLevel::Uncuffed;
				println!();
				println_log!("House is now uncuffed.");
			}
			HandcuffedLevel::NewlyHandcuffed => {
				house.handcuffed_level = HandcuffedLevel::AlmostUncuffed;
				println!();
				println_log!("House is handcuffed, skipping turn.");
				println!();
				return;
			}
		}
		utils::wait_and_clear();
		
		
		// use cigarettes, magazine, handcuffs
		let mut lives_count = game_data.buckshot.iter().filter(|x| **x).count();
		let mut blanks_count = game_data.buckshot.len() - lives_count;
		for i in (0..house.items.len()).rev() {
			let house = &mut game_data.players[0];
			match house.items[i] {
				
				Item::Cigarettes if house.lives < settings::lives_for_stage(stage_num) => {
					house.lives += 1;
					println_log!("House uses Cigarettes.");
				}
				
				Item::ExpiredMedicine if house.lives > 1 && house.lives + 2 <= settings::lives_for_stage(stage_num) => {
					println_log!("House uses Expired Medicine.");
					utils::wait();
					let gives_lives = rand::thread_rng().gen::<f32>() < 0.4;
					if gives_lives {
						println_log!("+2 lives.");
						house.lives += 2;
					} else {
						println_log!("-1 life.");
						house.lives -= 1;
					}
				}
				
				Item::Handcuffs => {
					let mut players_to_handcuff = 
						game_data.players[1..].iter_mut()
						.filter(|player| player.lives > 0 && player.handcuffed_level == HandcuffedLevel::Uncuffed)
						.collect::<Vec<_>>();
					if players_to_handcuff.is_empty() {continue;}
					let player_to_handcuff_index = rand::thread_rng().gen_range(0..players_to_handcuff.len());
					players_to_handcuff[player_to_handcuff_index].handcuffed_level = HandcuffedLevel::NewlyHandcuffed;
					println_log!("House handcuffs {}.", players_to_handcuff[player_to_handcuff_index].name);
				}
				
				Item::Magazine => {
					game_data.buckshot.clear();
					println_log!("House uses Magazine");
					reload_buckshot_if_needed(game_data, stage_num);
					game_data.players[0].items.remove(i);
					continue;
				}
				
				Item::Beer if blanks_count > lives_count && (blanks_count + lives_count > 2) => {
					#[allow(clippy::expect_used)]
					let popped_is_live = game_data.buckshot.pop().expect("The buckshot is empty.");
					println_log!("House uses Beer, pops a {}.", if popped_is_live {"live"} else {"blank"});
				}
				
				_ => continue,
			}
			game_data.players[0].items.remove(i);
			utils::wait_and_clear();
		}
		let house = &mut game_data.players[0];
		
		
		// use shells
		for i in (0..house.items.len()).rev() {
			match house.items[i] {
				Item::LiveShell => {
					let index = rand::thread_rng().gen_range(0..=game_data.buckshot.len());
					game_data.buckshot.insert(index, true);
					lives_count += 1;
					println_log!("House uses a Live Shell.");
				}
				Item::BlankShell => {
					let index = rand::thread_rng().gen_range(0..=game_data.buckshot.len());
					game_data.buckshot.insert(index, false);
					blanks_count += 1;
					println_log!("House uses a Blank Shell.");
				}
				_ => continue,
			}
			house.items.remove(i);
			utils::wait_and_clear();
		}
		
		
		// use gold shells
		let mut gold_shell_count = 0;
		for i in (0..house.items.len()).rev() {
			if house.items[i] != Item::GoldShell {continue;}
			game_data.buckshot.push(true);
			println_log!("House uses a Gold Shell.");
			lives_count += 1;
			gold_shell_count += 1;
			house.items.remove(i);
			utils::wait_and_clear();
		}
		
		
		'shoot: loop {
			let house = &mut game_data.players[0];
			
			
			// decide if live & use magnifying glass
			let mut assumed_live =
				if gold_shell_count > 0 {
					true
				} else if lives_count == 0 {
					false
				} else if blanks_count == 0 {
					true
				} else if let Some((magnifying_glass_index, _)) = house.items.iter().enumerate().find(|(_index, item)| **item == Item::MagnifyingGlass) {
					println_log!("House uses Magnifying Glass.");
					utils::wait_and_clear();
					house.items.remove(magnifying_glass_index);
					#[allow(clippy::expect_used)]
					*game_data.buckshot.last().expect("The buckshot is empty.")
				} else {
					let live_percent = lives_count as f32 / game_data.buckshot.len() as f32;
					rand::thread_rng().gen::<f32>() < live_percent
				};
			
			
			// use inverter
			if !assumed_live && let Some((i, _)) = house.items.iter().enumerate().find(|(_, item)| **item == Item::Inverter) {
				println_log!("House uses Inverter.");
				utils::wait_and_clear();
				#[allow(clippy::expect_used)]
				let last = game_data.buckshot.last_mut().expect("The buckshot is empty.");
				*last = !*last;
				assumed_live = !assumed_live;
				house.items.remove(i);
				game_data.inverter_used = true;
			}
			
			
			// use barrel extension
			if assumed_live && let Some((barrel_extension_index, _)) = house.items.iter().enumerate().find(|(_index, item)| **item == Item::BarrelExtension) {
				house.items.remove(barrel_extension_index);
				game_data.has_barrel_extension = true;
				println_log!("House uses Barrel Extension.");
				utils::wait_and_clear();
			}
			
			
			if assumed_live {
				
				// decide who to shoot
				let mut highest_lives = 0;
				let players_to_shoot =
					game_data.players[1..].iter_mut()
					.filter(|player| {
						highest_lives = highest_lives.max(player.lives);
						player.lives > 0
					})
					.collect::<Vec<_>>();
				let mut players_to_shoot =
					players_to_shoot.into_iter()
					.filter(|player| player.lives == highest_lives)
					.collect::<Vec<_>>();
				let player_to_shoot = players_to_shoot.remove(rand::thread_rng().gen_range(0..players_to_shoot.len()));
				
				// shoot
				println_log!("House points the buckshot at {}.", player_to_shoot.name);
				utils::wait_and_clear();
				#[allow(clippy::expect_used)]
				let is_live = game_data.buckshot.pop().expect("The buckshot is empty.");
				if is_live {
					println_log!("The buckshot shoots.");
					let damage = if game_data.has_barrel_extension {2} else {1};
					if damage >= player_to_shoot.lives {
						utils::wait_and_clear();
						println_log!("{} has lost all lives.", player_to_shoot.name);
						player_to_shoot.lives = 0;
						player_to_shoot.items.clear();
					} else {
						player_to_shoot.lives -= damage;
					}
				} else {
					println_log!("The buckshot clicks.");
				}
				utils::wait_and_clear();
				reload_buckshot_if_needed(game_data, stage_num);
				game_data.players[0].credits += settings::credit_per_shot_for_stage(stage_num);
				game_data.has_barrel_extension = false;
				
				break 'shoot;
			} else {
				
				// shoot self
				println_log!("House points the buckshot at itself.");
				utils::wait_and_clear();
				#[allow(clippy::expect_used)]
				let is_live = game_data.buckshot.pop().expect("The buckshot is empty.");
				if is_live {
					println_log!("The buckshot shoots.");
					let damage = if game_data.has_barrel_extension {2} else {1};
					if damage >= house.lives {
						utils::wait_and_clear();
						println_log!("House has lost all lives.");
						house.lives = 0;
						house.items.clear();
						return;
					} else {
						house.lives -= damage;
					}
					utils::wait_and_clear();
					break 'shoot;
				} else {
					println_log!("The buckshot clicks.");
				}
				utils::wait_and_clear();
				game_data.players[0].credits += settings::credit_per_shot_for_stage(stage_num);
				game_data.has_barrel_extension = false;
				if game_data.buckshot.is_empty() {
					reload_buckshot_if_needed(game_data, stage_num);
					break 'shoot;
				}
				continue 'shoot;
				
			}
			
			
		}
		
		
		// use unknown ticket
		let house = &mut game_data.players[0];
		if let Some((unknown_ticket_index, _)) = house.items.iter().enumerate().find(|(_index, item)| **item == Item::UnknownTicket) {
			println_log!("House uses unknown ticket");
			house.items.remove(unknown_ticket_index);
			continue 'turn;
		} else {
			break 'turn;
		}
		
		
	}
}



pub fn play_turn(game_data: &mut GameData, stage_num: usize) {
	
	let player = game_data.get_player_mut();
	println_log!("Starting {}'s turn.", player.name);
	
	// handcuffs
	match player.handcuffed_level {
		HandcuffedLevel::Uncuffed => {}
		HandcuffedLevel::AlmostUncuffed => {
			player.handcuffed_level = HandcuffedLevel::Uncuffed;
			println!();
			println_log!("You are now uncuffed.");
		}
		HandcuffedLevel::NewlyHandcuffed => {
			player.handcuffed_level = HandcuffedLevel::AlmostUncuffed;
			println!();
			println_log!("You are handcuffed, skipping turn.");
			println!();
			return;
		}
	}
	utils::wait_and_clear();
	
	// reset password
	if player.is_resetting_password {
		player.is_resetting_password = false;
		println_log!("Resetting password.");
		loop {
			let password_1 = prompt!("Password : (empty means no pw)) ");
			let password_2 = prompt!("Re-enter password to confirm: ");
			if password_1 != password_2 {println!("Passwords do not match."); continue;}
			player.password = if password_1.is_empty() {None} else {Some(password_1)};
			utils::clear();
			break;
		}
		log!("New password: {:?}", player.password);
	}
	
	// password check
	utils::clear();
	println_log!("Player {}'s turn.", player.name);
	if let Some(password) = &player.password {
		loop {
			let input = prompt!("Enter your password: ");
			if &input == password {break;}
			let retry = prompt!("Password is incorrect. Do you want to try again? "; YesNoInput);
			if retry {continue;}
			let reset = prompt!("Do you want to skip your turn and reset? (no = retry) "; YesNoInput);
			utils::clear();
			if reset {
				log!("");
				player.is_resetting_password = true;
				return;
			}
		}
	}
	
	// main turn
	let mut can_use_unknown_ticket = true;
	'outer: loop {
		let mut can_trade = true;
		'inner: loop {
			utils::clear();
			println_log!("Player {}'s turn.", game_data.get_player().name);
			println!();
			print_stats(game_data);
			println!();
			let options: &[&str] = if can_trade {&["shoot", "use item", "trade"]} else {&["shoot", "use item"]};
			let chosen_option = read!(options);
			log!("Doing: {chosen_option}");
			match chosen_option {
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
				log!("Used unknown ticket");
				can_use_unknown_ticket = false;
				player.items.remove(unknown_ticket_index);
				continue 'outer;
			}
		}
		
		break 'outer;
	}
	
	println_log!("Your turn has ended.");
	utils::wait_and_clear();
	
}



pub fn reload_buckshot_if_needed(game_data: &mut GameData, stage_num: usize) {
	let buckshot = &mut game_data.buckshot;
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
	game_data.inverter_used = false;
	
	println_log!("The buckshot is loaded with {live_count} {} and {blank_count} {}", utils::pluralize(live_count as f32, "live", "lives"), utils::pluralize(blank_count as f32, "blank", "blanks"));
	log!("Buckshot contents: {buckshot:?}");
	utils::wait_and_clear();
	
}



pub type ShotEndsTurn = bool;

pub fn shoot(game_data: &mut GameData, stage_num: usize) -> ShotEndsTurn {
	utils::clear();
	let player_names =
		game_data.players.iter().enumerate()
		.filter_map(|(i, player)| {
			if player.lives == 0 {return None;}
			Some(OptionWithData {display_name: player.name.to_string(), data: i})
		})
		.collect::<Vec<_>>();
	let OptionWithData {display_name: to_shoot, data: to_shoot_index} = prompt!("Who do you want to shoot?"; player_names);
	let confirm = prompt!(format!("Are you sure you want to shoot {to_shoot}? "); YesNoInput);
	if !confirm {return false;}
	println!();
	utils::clear();
	println_log!("You point the buckshot at {to_shoot}.");
	utils::wait_and_clear();
	#[allow(clippy::expect_used)]
	let is_live = game_data.buckshot.pop().expect("The buckshot is empty.");
	let credits = settings::credit_per_shot_for_stage(stage_num);
	let mut shot_ends_turn;
	if is_live {
		println_log!("The buckshot shoots. You are given {credits} {}.", utils::pluralize(credits as f32, "credit", "credits"));
		let damage = if game_data.has_barrel_extension {2} else {1};
		if damage >= game_data.players[to_shoot_index].lives {
			utils::wait_and_clear();
			println_log!("{to_shoot} has lost all lives.");
			let to_shoot_player = &mut game_data.players[to_shoot_index];
			to_shoot_player.lives = 0;
			to_shoot_player.items.clear();
		} else {
			game_data.players[to_shoot_index].lives -= damage;
		}
		shot_ends_turn = true;
	} else {
		println_log!("The buckshot clicks. You are given {credits} {}.", utils::pluralize(credits as f32, "credit", "credits"));
		shot_ends_turn = to_shoot_index != game_data.curr_player;
	}
	game_data.get_player_mut().credits += credits;
	game_data.has_barrel_extension = false;
	utils::wait_and_clear();
	if game_data.has_barrel_extension {
		game_data.has_barrel_extension = false;
		println_log!("The barrel extension is removed.");
		utils::wait_and_clear();
	}
	if game_data.buckshot.is_empty() {shot_ends_turn = true;}
	reload_buckshot_if_needed(game_data, stage_num);
	shot_ends_turn
}



pub type ItemEndedTurn = bool;

pub fn use_item(game_data: &mut GameData, stage_num: usize) -> ItemEndedTurn {
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
			println_log!("You used cigarettes.");
			utils::wait_and_clear();
		}
		
		Item::ExpiredMedicine => {
			let player = game_data.get_player_mut();
			let mut prompt = String::from("Are you sure you want to use this item? ");
			let max_lives = settings::lives_for_stage(stage_num);
			if player.lives == max_lives {prompt += "You are already at max lives. ";}
			let confirm = prompt!(prompt; YesNoInput);
			if !confirm {return false;}
			println_log!("You used Expired Medicine.");
			utils::wait();
			let gives_lives = rand::thread_rng().gen::<f32>() < 0.4;
			if gives_lives {
				let new_lives = 2.min(max_lives - player.lives);
				println_log!("+{new_lives} {}.", utils::pluralize(new_lives as f32, "life", "lives"));
				player.lives += new_lives;
			} else {
				println_log!("-1 life.");
				player.lives -= 1;
				if player.lives == 0 {
					println_log!("You have run out of lives.");
					utils::wait_and_clear();
					return true;
				}
			}
			utils::wait_and_clear();
		}
		
		Item::MagnifyingGlass => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			#[allow(clippy::expect_used)]
			if *game_data.buckshot.last().expect("The buckshot is empty.") {
				println_log!("The next shell is live.");
			} else {
				println_log!("The next shell is blank.");
			}
			utils::wait_and_clear();
		}
		
		Item::Beer => {
			let mut prompt = String::from("Are you sure you want to use this item? ");
			if game_data.buckshot.len() == 1 {prompt += "This is the last round, popping it will end your turn. ";}
			let confirm = prompt!(prompt; YesNoInput);
			if !confirm {return false;}
			#[allow(clippy::expect_used)]
			if game_data.buckshot.pop().expect("The buckshot is empty.") {
				println_log!("You popped a live shell.");
			} else {
				println_log!("You popped a blank shell.");
			}
			if game_data.buckshot.is_empty() {
				popped_last_shell = true;
			}
			utils::wait_and_clear();
			reload_buckshot_if_needed(game_data, stage_num);
		}
		
		Item::BarrelExtension => {
			if game_data.has_barrel_extension {println_log!("Barrel already has extension"); return false;}
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			game_data.has_barrel_extension = true;
			println_log!("You used a barrel extension.");
			utils::wait_and_clear();
		}
		
		Item::Magazine => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			game_data.buckshot.clear();
			log!("Used magazine");
			reload_buckshot_if_needed(game_data, stage_num);
		}
		
		Item::Handcuffs => {
			let curr_player_name = &game_data.get_player().name;
			let player_names =
				game_data.players.iter().enumerate()
				.filter_map(|(i, player)| {
					if player.lives == 0 {return None;}
					if &player.name == curr_player_name {return None;}
					if player.handcuffed_level != HandcuffedLevel::Uncuffed {return None;}
					Some(OptionWithData {display_name: player.name.to_string(), data: i})
				})
				.collect::<Vec<_>>();
			if player_names.is_empty() {
				println!("There are no players that can be handcuffed.");
				utils::wait_and_clear();
				return false;
			}
			let OptionWithData {display_name: to_handcuff, data: to_handcuff_index} = prompt!("Who do you want to handcuff? "; player_names);
			let confirm = prompt!(format!("Are you sure you want to handcuff {to_handcuff}? "); YesNoInput);
			if !confirm {return false;}
			let to_handcuff_player = &mut game_data.players[to_handcuff_index];
			log!("Handcuffing player {}", &to_handcuff_player.name);
			if to_handcuff_player.handcuffed_level != HandcuffedLevel::Uncuffed {
				println_log!("This player is already cuffed");
				return false;
			}
			to_handcuff_player.handcuffed_level = HandcuffedLevel::NewlyHandcuffed;
			println_log!("You used handcuffs.");
			utils::wait_and_clear();
		}
		
		Item::UnknownTicket => {
			log!("Tried using unknown ticket");
			println!("You cannot use this item right now.");
			utils::wait_and_clear();
		}
		
		Item::LiveShell => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			let index = rand::thread_rng().gen_range(0..=game_data.buckshot.len());
			game_data.buckshot.insert(index, true);
			println!("You used added the shell.");
			log!("Added live shell. New buckshot contents: {:?}", &game_data.buckshot);
			utils::wait_and_clear();
		}
		
		Item::BlankShell => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			let index = rand::thread_rng().gen_range(0..=game_data.buckshot.len());
			game_data.buckshot.insert(index, false);
			println!("You used added the shell.");
			log!("Added blank shell. New buckshot contents: {:?}", &game_data.buckshot);
			utils::wait_and_clear();
		}
		
		Item::GoldShell => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			game_data.buckshot.push(true);
			println!("You used added the shell.");
			log!("Added gold shell");
			utils::wait_and_clear();
		}
		
		Item::Inverter => {
			let confirm = prompt!("Are you sure you want to this item? "; YesNoInput);
			if !confirm {return false;}
			#[allow(clippy::expect_used)]
			let last = game_data.buckshot.last_mut().expect("The buckshot is empty.");
			*last = !*last;
			game_data.inverter_used = true;
			println!("It has been done.");
			log!("Used inverter");
			utils::wait_and_clear();
		}
		
	}
	game_data.get_player_mut().items.remove(to_use_index);
	popped_last_shell
}



pub fn trade(game_data: &mut GameData, can_trade: &mut bool) {
	
	let curr_player_name = &game_data.get_player().name;
	let player_names =
		game_data.players.iter().enumerate()
		.filter_map(|(i, player)| {
			if player.lives == 0 {return None;}
			if &player.name == curr_player_name {return None;}
			Some(OptionWithData {display_name: player.name.to_string(), data: i})
		})
		.collect::<Vec<_>>();
	let OptionWithData {display_name: other_player_name, data: other_player} = prompt!("Who do you want to trade with? "; player_names);
	log!("Started trade with {other_player_name}");
	let mut curr_trading_items = vec![false; game_data.get_player().items.len()];
	let mut other_trading_items = vec![false; game_data.players[other_player].items.len()];
	
	loop {
		
		utils::clear();
		println!("Your items:");
		for (i, (item, is_trading)) in game_data.get_player().items.iter().zip(&curr_trading_items).enumerate() {
			if *is_trading {
				println!("{}: [{item}]", i + 1);
			} else {
				println!("{}:  {item} ", i + 1);
			}
		}
		println!();
		println!();
		println!();
		println!("Other's items:");
		for (i, (item, is_trading)) in game_data.players[other_player].items.iter().zip(&other_trading_items).enumerate() {
			if *is_trading {
				println!("{}: [{item}]", i + 1);
			} else {
				println!("{}:  {item} ", i + 1);
			}
		}
		println!();
		println!();
		println!();
		println!("Commands:");
		println!("Finalize");
		println!("Cancel");
		println!("Toggle [my / other] #[item index]  (example: \"toggle my #2\")");
		let input = read!(NonWhitespaceInput).trim().to_lowercase();
		
		// finalize
		if input == "f" || input == "finalize" {
			let trade_was_accepted = prompt_accept_trade(game_data, game_data.curr_player, other_player, &curr_trading_items, &other_trading_items);
			if !trade_was_accepted {return;}
			let trade_was_accepted = prompt_accept_trade(game_data, other_player, game_data.curr_player, &other_trading_items, &curr_trading_items);
			if !trade_was_accepted {return;}
			log!("Finalizing trade...");
			for (i, is_trading) in curr_trading_items.iter().enumerate().rev() {
				if !is_trading {continue;}
				let item_to_move = game_data.get_player_mut().items.remove(i);
				log!("{item_to_move} from curr to other");
				game_data.players[other_player].items.push(item_to_move);
			}
			for (i, is_trading) in other_trading_items.iter().enumerate().rev() {
				if !is_trading {continue;}
				let item_to_move = game_data.players[other_player].items.remove(i);
				log!("{item_to_move} from other to curr");
				game_data.get_player_mut().items.push(item_to_move);
			}
			println_log!("The trade was successful.");
			*can_trade = false;
			utils::wait_and_clear();
			return;
		}
		
		// cancel
		if input == "c" || input == "cancel" {
			let confirm = prompt!("Are you sure you want to cancel? "; YesNoInput);
			if !confirm {continue;}
			println_log!("Trade has been canceled.");
			return;
		}
		
		// toggle
		let input_parts = input.split(' ').collect::<Vec<_>>();
		if input_parts[0] == "t" || input_parts[0] == "toggle" {
			if input_parts.len() != 3 {
				println!("Could not understand input, must be exactly 3 tokens.");
				utils::wait();
				continue;
			}
			let trading_items = match input_parts[1] {
				"my" | "m" => &mut curr_trading_items,
				"other" | "o" => &mut other_trading_items,
				_ => {
					println!("Could not understand input, second token did not match \"my\" or \"other\".");
					utils::wait();
					continue;
				}
			};
			let mut index_token = input_parts[2];
			if index_token.starts_with("#") {
				index_token = &index_token[1..];
			}
			let index = match index_token.parse::<usize>() {
				std::result::Result::Ok(v) => v,
				std::result::Result::Err(err) => {
					println!("Could parse input third token: {err}");
					utils::wait_and_clear();
					continue;
				}
			};
			let index = index - 1;
			if index >= trading_items.len() {
				println!("Cannot toggle index, it is too high.");
				utils::wait_and_clear();
				continue;
			}
			trading_items[index] = !trading_items[index];
			continue;
		}
		
		println!("Could not understand input.");
		utils::wait_and_clear();
		
	}
	
}



pub type TradeWasAccepted = bool;

pub fn prompt_accept_trade(game_data: &GameData, curr_player: usize, other_player: usize, curr_player_trading_items: &[bool], other_player_trading_items: &[bool]) -> TradeWasAccepted {
	utils::clear();
	
	if &*game_data.players[curr_player].name == "House" {
		let curr_items_value = curr_player_trading_items.iter().enumerate().fold(0., |acc, (i, is_trading)| {
			if !*is_trading {return 0.;}
			let item = game_data.players[0].items[i];
			acc + 1. / settings::get_item_rarity(item)
		});
		let other_items_value = other_player_trading_items.iter().enumerate().fold(0., |acc, (i, is_trading)| {
			if !*is_trading {return 0.;}
			let item = game_data.players[other_player].items[i];
			acc + 1. / settings::get_item_rarity(item)
		});
		if other_items_value > curr_items_value * 0.95 {
			println!("House accepts.");
			utils::wait_and_clear();
			return true;
		} else {
			println!("House does not accept.");
			utils::wait_and_clear();
			return false;
		}
	}
	
	println!("Addressing {}:", game_data.players[curr_player].name);
	println!();
	println!("You're trading:");
	for (item, is_trading) in game_data.players[curr_player].items.iter().zip(curr_player_trading_items.iter()) {
		if !*is_trading {continue;}
		println!("{item}");
	}
	println!();
	println!("Other is trading:");
	for (item, is_trading) in game_data.players[other_player].items.iter().zip(other_player_trading_items.iter()) {
		if !*is_trading {continue;}
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
	
	println_log!("You have agreed to the trade");
	utils::wait_and_clear();
	true
}



pub fn print_stats(game_data: &GameData) {
	if game_data.inverter_used {
		println!("Buckshot contents are hidden.");
	} else {
		let lives = game_data.buckshot.iter().filter(|x| **x).count();
		let blanks = game_data.buckshot.len() - lives;
		match (lives > 0, blanks > 0) {
			(true, true) => println!("The buckshot contains {} {} and {} {}", lives, utils::pluralize(lives as f32, "live", "lives"), blanks, utils::pluralize(blanks as f32, "blank", "blanks")),
			(true, false) => println!("The buckshot contains {} {}", lives, utils::pluralize(lives as f32, "live", "lives")),
			(false, true) => println!("The buckshot contains {} {}", blanks, utils::pluralize(blanks as f32, "blank", "blanks")),
			(false, false) => panic!("The buckshot is empty."),
		}
	}
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



lazy_static::lazy_static! {
    static ref LOG_WRITER: Mutex<File> = {
        let file = File::create("log.txt").expect("Failed to create log file.");
        Mutex::new(file)
    };
}

#[macro_export]
macro_rules! log {
	(raw: $string:expr) => {{
		use std::io::Write;
		
		let mut log_writer = LOG_WRITER.lock().expect("Could not lock LOG_WRITER.");
		log_writer.write_all($string.as_bytes()).expect("Could not write to log.");
		log_writer.write_all(&[b'\n']).expect("Could not write to log.");
		
	}};
	($($arg:tt)*) => {
		log!(raw: format!($($arg)*))
	};
}

#[macro_export]
macro_rules! println_log {
	($($arg:tt)*) => {{
		let formatted = format!($($arg)*);
		println!("{formatted}");
		log!(raw: formatted);
	}};
}
