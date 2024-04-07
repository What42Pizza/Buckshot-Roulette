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
	pub fn from_strs(input: &[&str]) -> Option<Self> {
		Some(match *input {
			["cigarettes"] => Self::Cigarettes,
			["expired", "medicine"] => Self::ExpiredMedicine,
			["magnifying", "glass"] => Self::MagnifyingGlass,
			["beer"] => Self::Beer,
			["barrel", "extension"] => Self::BarrelExtension,
			["magazine"] => Self::Magazine,
			["handcuffs"] => Self::Handcuffs,
			["unknown", "ticket"] => Self::UnknownTicket,
			["live", "shell"] => Self::LiveShell,
			["blank", "shell"] => Self::BlankShell,
			["gold", "shell"] => Self::GoldShell,
			["inverter"] => Self::Inverter,
			_ => return None,
		})
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
