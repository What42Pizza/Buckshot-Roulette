use crate::prelude::*;
use std::fmt::{Display, Formatter};



pub struct GameData {
	pub players: Vec<Player>,
	pub curr_player: usize,
	pub buckshot: Vec<bool>,
	pub has_barrel_extension: bool,
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
	pub fn index_of_player(&self, player_name: &str) -> usize {
		self.players.iter().enumerate().find(|(_index, player)| &*player.name == player_name).unwrap().0
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
	MagnifyingGlass,
	Beer,
	BarrelExtension,
	Handcuffs,
	UnknownTicket,
	LiveShell,
	BlankShell,
}

impl Item {
	pub fn random() -> Self {
		let total_rarities = settings::ITEM_RARITIES.iter().fold(0., |total, (_, rarity)| total + rarity);
		let chosen = rand::thread_rng().gen_range(0. .. total_rarities);
		let mut total = 0.;
		for (item, rarity) in settings::ITEM_RARITIES {
			total += rarity;
			if total >= chosen {
				return *item;
			}
		}
		unreachable!()
	}
	pub fn from_strs(input: &[&str]) -> Option<Self> {
		Some(match input {
			&["cigarettes"] => Self::Cigarettes,
			&["magnifying", "glass"] => Self::MagnifyingGlass,
			&["beer"] => Self::Beer,
			&["barrel", "extension"] => Self::BarrelExtension,
			&["handcuffs"] => Self::Handcuffs,
			&["unknown", "ticket"] => Self::UnknownTicket,
			&["live", "shell"] => Self::LiveShell,
			&["blank", "shell"] => Self::BlankShell,
			_ => return None,
		})
	}
}

impl Display for Item {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{}", match self {
			Self::Cigarettes => "Cigarettes",
			Self::MagnifyingGlass => "Magnifying Glass",
			Self::Beer => "Beer",
			Self::BarrelExtension => "Barrel Extension",
			Self::Handcuffs => "Handcuffs",
			Self::UnknownTicket => "Unknown Ticket",
			Self::LiveShell => "Live Shell",
			Self::BlankShell => "Blank Shell",
		})
	}
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HandcuffedLevel {
	Uncuffed,
	AlmostUncuffed,
	NewlyHandcuffed,
}
