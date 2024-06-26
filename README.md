# Buckshot Roulette With Friends

This is a work-in-progress low-tech way to play a multiple-player version of [Buckshot Roulette](https://mikeklubnika.itch.io/buckshot-roulette) by Mike Klubnika.

Basically, for a while now, I've wanted something that me and my family could play while on road trips, and Buckshot Roulette seemed like the perfect choice. This is meant to be coordinated by a single person (me), who calls out what happens and performs the actions of each player. As such, this focuses entirely on mechanics, making it ugly but functional.

<br>

## Note: The android app version likely will not be finished, so I recommend using the terminal version instead.

<br>

Because this is made for multiple players instead of player vs ai, the rules are fairly significantly different, so here's the full rules for this version: (feel free to copy any / all of this)

```
Stage 1:

Every player now has 2 lives. The buckshot is loaded with 2-4 shells with a live percentage of 40% - 60%. Whenever the buckshot is emptied, it is refilled with 2-4 shells with a live percentage of 40% - 60%. When a player shoots, they are given a credit. At the start of every round, each player is given 2 items (up to 4 items). The first round starts. Once a single player is left, that player is given 10 credits, and the game moves to stage 2.

Stage 2:

Every player now has 3 lives. The buckshot is loaded with 4-8 shells with an live percentage of 40% - 60%. Whenever the buckshot is emptied, it is refilled with 4-8 shells with a live percentage of 40% - 60%. When a player shoots, they are given two credits. At the start of every round, each player is given 3 item (up to 6 items). The first round starts. Once a single player is left, that player is given 20 credits, and the game moves to stage 3.

Stage 3:

Every player now has 4 lives. The buckshot is loaded with 6-8 shells with an live percentage of 40% - 60%. Whenever the buckshot is emptied, it is refilled with 6-8 shells with a live percentage of 40% - 60%. When a player shoots, they are given three credits. At the start of every round, each player is given 4 item (up to 8 items). The first round starts. Once a single player is left, that player is given 30 credits, and the game ends. Whoever has the most credits wins.


When a player's turn starts, they can use one or more items, trade, or shoot an active player (including themselves). If they shoot another player, their turn ends. If they shoot themselves with a blank, their turn continues (use item(s), shoot, or trade). If they shoot themselves with a live, their turn ends. If they shoot themselves with a blank and the buckshot is emptied, their turn ends.

You can only trade once per turn. At the start of each stage, the list of players is shuffled, the buckshot is emptied and reloaded, all items are removed, and all handcuffs are removed. Players must always end their turns on a shot (live or blank, others or themselves).


Items:
Cigarettes (uncommon): Regain a life (up to how many the stage starts with)
Expired Medicine (common): 40% chance to gain 2 lives, 60% chance to lose a life
Magnifying Glass (uncommon): Check if current round is live
Beer (common): Empty the current round (ends turn if buckshot is emptied)
Barrel Extension (uncommon): Doubles shotgun damage and credits (removed after shot, regardless of live or blank)
Magazine (uncommon): Empties the buckshot and reloads it (does not end turn)
Handcuffs (rare): Skips a player's next turn
Unknown Ticket (rare): Restarts turn (can only be used once per round; only used whenever turn would end; allows trading again)
Live Shell (common): Adds a live to the buckshot (in a random position)
Blank Shell (uncommon): Adds a blank to the buckshot (in a random position)
Gold Shell (rare): Adds a live to the top of the buckshot
Inverter (common): Swaps the current round with its opposite. If the number of lives and blanks is always shown, it must not change after inverter use


Common: 1x multiplier
Uncommon: 0.6x multiplier
Rare: 0.3x multiplier
```

Also, the android version is my first Android app (or any app for that reason). I'm only making this for myself, so I can't guarantee any support
