start-dialog = 
	Welcome to MonstroCity, { $username }!
	
	This is a pre-alpha version of a cyberpunk RPG/MUD.
	
	You should start by creating your character with `/character`. You can also use `/settings` to change your language, or `/help` for more information.

character-dialog =
	*[{ $nickname }]({ $mention_url })*, _level { $player_level } { $player_class }._
	
	Currently in { $player_location }.
	
	.button-attributes = 🔢 Attributes
	.button-inventory  = 🎒 Inventory
	.button-remove     = 🗑 Remove character
	.button-rename     = ✏ Rename character

character-creation-class-selection-dialog =
	Choose your class. Each class has unique set of skills, with their advantages and weaknesses.
	
	Click on a class for an explanation of that class.
	
	.button-decker   = 💻 Decker
	.button-samurai  = ⚔ Street Samurai
	.button-ninja    = 🗡 Ninja
	.button-engineer = ⚙ Engineer

character-creation-class-decker =
	*Deckers* use hacking and extensive cybernetics to their advantage. They can hack microprosessor, to unlock doors and control turrets, for example, or hack the cybernetic eyes of an enemy to temporarily blind them.
	
	Their "deck" are their primary weapon, but they usually carry smartguns as a backup. Their high-level cybernetics offer their smartguns expectional accuracy, but they usually lack the physical strength of a street warrior.
	
	Primary atttribute: Int
	
	.button-choose = ✔ Choose Decker
	.button-back   = 🔙 Class selection
	
character-creation-class-samurai =
	*Street samurai* wield sharp swords, and single-handled guns. They rely on their physical fitness, and cybernetics that boost their speed and strength.
	
	Primary attributes: Str, Agi
	
	.button-choose = ✔ Choose Street Samurai
	.button-back   = 🔙 Class selection

character-creation-class-ninja =
	*Ninjas* lurk in the shadows, stalking their prey until it is time to strike. They use daggers, handguns, and other silent weapons. They rely on thermo-optic camoflage, enhanced senses, and quickness to take down their enemies.
	
	Primary attribute: Agi, Int
	
	.button-choose = ✔ Choose Ninja
	.button-back   = 🔙 Class selection
	
character-creation-class-engineer =
	*Engineers* employ wide variety of mechanical constructs to aid them in combat. These include turrets, drones, and large battleframes. They also tend to like heavy weapons, suchs as grenade launchers and machine guns.
	
	Primary attributes: Str, Int
	
	.button-choose = ✔ Choose Engineer
	.button-back   = 🔙 Class selection	
