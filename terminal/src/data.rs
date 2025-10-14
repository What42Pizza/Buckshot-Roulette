use crate::prelude::*;
use std::fmt::{Display, Formatter};



pub struct GameData {
	pub players: Vec<Player>,
	pub curr_player: usize,
	pub buckshot: Vec<bool>,
	pub has_barrel_extension: bool,
	pub is_inverted: bool,
}

impl GameData {
	pub fn get_player(&self) -> &Player {
		&self.players[self.curr_player]
	}
	pub fn get_player_mut(&mut self) -> &mut Player {
		&mut self.players[self.curr_player]
	}
	pub fn count_alive_players(&self) -> usize {
		self.players.iter().filter(|p| p.lives > 0).count()
	}
	pub fn count_known_shells(&self) -> (usize, usize) {
		let known_lives = self.buckshot.iter().rev().enumerate().filter(|(i, shell)| **shell ^ (self.is_inverted && *i == 0)).count();
		let known_blanks = self.buckshot.len() - known_lives;
		(known_lives, known_blanks)
	}
	pub fn index_of_player(&self, player_name: &str) -> Option<usize> {
		self.players.iter().enumerate().find(|(_index, player)| &*player.name == player_name).map(|(index, _player)| index)
	}
}



#[derive(Debug, Clone)]
pub struct Player {
	
	pub name: String,
	pub password: Option<String>,
	pub is_resetting_password: bool,
	
	pub lives: usize,
	pub credits: usize,
	pub items: Vec<Item>,
	pub handcuffed_level: HandcuffedLevel,
	
}

impl Player {
	pub fn new() -> Self {
		Self {
			
			name: String::new(),
			password: None,
			is_resetting_password: false,
			
			lives: 0,
			credits: 0,
			items: vec!(),
			handcuffed_level: HandcuffedLevel::Uncuffed,
			
		}
	}
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Item {
	Cigarettes,
	ExpiredMedicine,
	MagnifyingGlass,
	Beer,
	BarrelExtension,
	Magazine,
	Handcuffs,
	UnknownTicket,
	LiveShell,
	BlankShell,
	GoldShell,
	Inverter,
}

pub const ITEMS_LIST: &[Item] = &[
	Item::Cigarettes,
	Item::ExpiredMedicine,
	Item::MagnifyingGlass,
	Item::Beer,
	Item::BarrelExtension,
	Item::Magazine,
	Item::Handcuffs,
	Item::UnknownTicket,
	Item::LiveShell,
	Item::BlankShell,
	Item::GoldShell,
	Item::Inverter,
];

impl Item {
	pub fn random() -> Self {
		let total_rarities =
			ITEMS_LIST.iter().cloned()
			.map(settings::get_item_rarity)
			.fold(0., |total, rarity| total + rarity);
		let chosen = rand::thread_rng().gen_range(0. .. total_rarities);
		let mut total = 0.;
		for item in ITEMS_LIST {
			total += settings::get_item_rarity(*item);
			if total >= chosen {
				return *item;
			}
		}
		unreachable!()
	}
	pub fn to_input_option(&self, i: usize) -> InputOption<&Self> {
		let names: &[String] = match self {
			Self::Cigarettes => &[self.to_string(), "c".to_string(), "cig".to_string()],
			Self::ExpiredMedicine => &[self.to_string(), "e".to_string(), "exp".to_string()],
			Self::MagnifyingGlass => &[self.to_string(), "m".to_string(), "magn".to_string()],
			Self::Beer => &[self.to_string(), "ber".to_string()],
			Self::BarrelExtension => &[self.to_string(), "b".to_string(), "bar".to_string()],
			Self::Magazine => &[self.to_string(), "maga".to_string()],
			Self::Handcuffs => &[self.to_string(), "h".to_string(), "hand".to_string()],
			Self::UnknownTicket => &[self.to_string(), "u".to_string(), "unk".to_string()],
			Self::LiveShell => &[self.to_string(), "l".to_string(), "liv".to_string()],
			Self::BlankShell => &[self.to_string(), "bla".to_string()],
			Self::GoldShell => &[self.to_string(), "g".to_string(), "gol".to_string()],
			Self::Inverter => &[self.to_string(), "i".to_string(), "inv".to_string()],
		};
		InputOption::new(i.to_string(), names, self)
	}
}

impl Display for Item {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{}", match self {
			Self::Cigarettes => "Cigarettes",
			Self::ExpiredMedicine => "Expired Medicine",
			Self::MagnifyingGlass => "Magnifying Glass",
			Self::Beer => "Beer",
			Self::BarrelExtension => "Barrel Extension",
			Self::Magazine => "Magazine",
			Self::Handcuffs => "Handcuffs",
			Self::UnknownTicket => "Unknown Ticket",
			Self::LiveShell => "Live Shell",
			Self::BlankShell => "Blank Shell",
			Self::GoldShell => "Gold Shell",
			Self::Inverter => "Inverter",
		})
	}
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HandcuffedLevel {
	Uncuffed,
	AlmostUncuffed,
	NewlyHandcuffed,
}
