The team is currently aware of a bug with WASD in the 1.2.3 Patch and will be deploying a
hotfix as soon as possible.# Bug Fixes
## Skills
- Fixed an issue where player was unable to properly use Flame Rush while channeling Focus
with Mana Guide equipped.
- Fixed Crest of Unity not forcing Elemental Nova to have all elemental tags.
- Fixed a bug with Volatile Reversal where allocating Immutable Past did not prevent
triggering effects that require being able to jump backwards in time, such as Warped
Time.
- Fixed a display bug with Volatile Reversal where allocating ‘Immutable Past’ did not
remove the buff UI icon displaying how long you have remaining on your ability to
re-activate Volatile Reversal to jump a second time.
- Fixed an issue where Shield Throw would create multiple shields if Void Knight mastery
was selected.
- Fixed a bug with Rebuke where Burst of Speed was not granting Haste, and was instead
granting 25% increased movement speed.
- Fixed a bug where, under certain conditions, Fallen From Grace and Order of Lagon could
fail to convert Smite’s base damage to void or lightning respectively.
- Fixed a bug with the Forge Guard passive tree where many sources of Haste didn’t have
their effects converted to block chance by Forgemaster’s Might.
- Fixed a bug where Heartseeker would target invulnerable enemies.
- Fixed a bug where Heartseeker would prioritize targeting certain enemy types over
others. For example, Exiled Mages could not be targeted unless there were no other
enemies in range.
- In the Bladedancer passive tree, Argent Veil has been clarified to state that it only
triggers when damage taken by enemies puts you below 70% health (rather than any means
of dropping below 70% health).
- The alt text for Shadow Cascade now has additional information clarifying that Shadow
Cascade has 10% base critical strike chance, instead of the typical 5%.
- Fixed a bug where the visual trails for Umbral Blades would travel less distance than
the blades in the ground.
- Fixed a bug where a portion of Dread Shade visuals would be inside of the minion they
were attached to.
- Fixed a bug where Thorn Shield stayed in spawn place after scene transitions.
- The Blizzard’s Wrath, Thunderous Storm, and Grand Cyclone nodes now display subskill
tooltips for Tempest Strike’s spells, allowing their descriptions and tags to be viewed.
They do not currently offer DPS estimates.
- Fixed a bug where changing summoned minions in town could cause both types of minions to
be auto re-summoned when exiting the town.
- Fixed an issue where players could get stuck after using movement abilities in the
Tundra near the inactive bridge.
## Imprinted Items
Made a number of fixes and changes to Imprinted items:- Fixed an issue where altering the original imprinted item could affect the imprint
itself.
- Fixed a bug where imprinted Unique, Set, and Legendary items were unable to result in
generating different Uniques or Sets than the imprinted one.
- Fixed imprinted item remaining Weavers Will and Affix tiers not correctly corresponding
to dropped item average Weaver’s Will for imprinted Legendary Weaver’s Will items.
- Fixed an issue where replacing an imprinted item on the Weaver Tree sometimes didn’t
remove the other item visually.
- Added a chance for imprinted normal/magic/rare/exalted items to result in drops of
Unique and Set items of the same item type.
- Reworked Forging Potential roll for similar items. Low and medium Forging Potential
imprinted items result in drops with higher FP on average, and even low Forging
Potential imprinted items have a chance to result in items with normal Forging
Potential.
- Guests’ Imprint nodes now work as long as the Host has the same imprint node. The
guests’ imprinted items are used for their drops instead of the Host’s. This does not
include Woven Echo imprint nodes.
## Dungeon, Monolith and Quest
- Fixed a bug where using Portal charms would result in one more mod than intended for
dungeon tiers where a mod was added when you started the dungeon.
- Fixed a bug where the each cast of the Imperial Soulmage’s Hungering Soul ability would
permanently increase the damage of any Flaming Soul cast by the Imperial Pyromancer.
- Fixed an issue where dying on the Monolith Hub with items around you could visually
duplicate the items on the floor.
- Fixed a bug that prevented the quest pulse directing you to the Temple of Eterra from
appearing after defeating the Giant Weather Statue boss in the Courtyard.
- Fixed an issue where the Eternity Cache would not play its reveal visual effect.
- Fixed an issue where you could see Memory Amber remaining from previous zones.
- Fixed an issue where players could interact with a echo portal and a Cemetery
entrance/exit in the same frame, causing transition errors and preventing movement.
## Items
- Fixed a bug where the Loom Walker idol (increased stun chance and added stun avoidance)
was acting as a multiplier to other sources of increased stun chance, rather than adding
to them.
- Fixed a bug where Mantle of the Pale Ox was giving an amount of increased health for the
player that was different from what was stated. For existing items, the listed values
for “20-32% increased health for you and your minions” will change to accurately reflect
the amount of increased health it was granting to the player.
- Fixed a bug where Grasp of the Blood Mage’s mana spent reflected to you as damage could
set you to 1 health after taking you out of Reaper Form. It can still take you out of
Reaper Form, but your health will be restored to full as usual.
## Input
- Fixed an issue where players could use Warpath without moving when using WASD.
- Fixed an issue on controller where UI fields would be skipped while navigating Unique
item names in the Bazaar.
## UI/Visual
- Fixed an issue where CTRL+F hotkey didn’t work for selecting the Weaver Tree search
field.
- Fixed an issue where the Skill panel’s blinking indicator for when a “+X to Skill” affix
is removed did not function.
- Fixed an issue where gold could be lost when attempting to swap to an incompatible
Blessing.
- Fixed a bug which could cause the Blessing Reward Replacement Panel to display the wrong
Blessing to be replaced.
- Fixed a bug where Champion icons could remain on the map after they died.
- Fixed a bug where Argolos the Blessed had a pink object rotating around him when in his
final phase.
- Fixed a visual issue where loot labels would stay stuck on the screen.
- Fixed layout issues for ability tooltips when shown for the first time.
- Fixed a visual bug where having one rune in the Forge displayed as having zero available
runes instead.
- Fixed incorrect stat color on Advent of the Erased – "Less Damage Over Time Taken While
You Have Haste”.
- Fixed the format of the “Potion Health Gain Converted to Ward” stat on the Character
Stats panel.
## Other
- Fixed the Tangled Lasers from the Possessed Witch mod to counting as a hit, so it could
stun and crit. It now correctly counts as a DoT.
Tangled Lasers now deal 52% less damage.
- Fixed a bug where Admiral Harton’s staff could continue spinning after death.
- Fixed Warpath MTX from sometimes spawning doubled visual effects.
- Screenshots submitted through the in-game bug reporting tool will now correctly capture
tooltips on screen.
- Fixed issue where Champion’s Gate would no longer change to its music when returning
from an Arena run.
- Fixed various random selection systems being biased towards specific outcomes.
- Fixed a hitch when talking to an NPC for the first time in each scene.
# Improvements
## Input
- The Elemental Nova ability now targets enemies correctly when playing with the Arcane
Projection node on a controller.
- Added an alternative interaction mode for WASD that disables ground movement with the
left mouse button while allowing rebinding for ability, to be used for both regular
interactions and ability casting.
- Improved controller navigation on the Mastery respec panel.
- Added prompt on controller for the “View Stall” button in the Bazaar Panel.
## Gameplay
- Defeating a Timeline boss in Empowered timelines now takes you to Echo of a World
instead of the Monolith Hub area.
- Added a new UI reminder below the chat screen that shows up when ground item tooltips
are hidden to make it clearer when this setting is enabled.
- Updated the pickup radius of Tomes of Experience and Tomes of Favor in line with the
overall pickup radius increase done for 1.2.
- Adjusted positions of several objects that were obstructing statue interaction in the
Courtyard and updated minimap for the scene for better navigation.
## Combat
- The Storm Swells created by Champions with the Whirlpool mod now move and turn more
slowly.
- Fateweaver Atropos’ Necrospin now hits less frequently (4 times per second, reduced from
8).
- Reduced the jitter in the Draal Queen’s Corrosive Bile projectile online.
- Reduced the jitter in the movement of the projectiles created by the crystals in the
Majasa encounter.
## UI, Visuals and Movement
- Adjusted Rogue movement animations and shield joint to reduce clipping between large
shields and Rogue armor sets.
- Fixed clipping issues affecting the Sentinel’s T28 Body Armor when combined with certain
boot models.
- Improved visuals for Wolves that have been converted to deal cold damage.
- Improved pet movement on sloped surfaces.
- Reduced visual noise created by Phoenix Flamethrower ability from Phoenix’s Shrine buff.
- Improved Character Stats panel text scaling and wrapping.
Made several Stash search improvements:- Increased vertical stash tab text limit to 30 characters to match the individual tab
search
- Stash searching now supports macros, expressions, and regex
- Updated some in-zone item UIs (e.g. Nemesis, Woven Offering), to
automatically set a relevant search macro or expression based on the
restrictions of the item slot when opening the stash
- Regex
Wrap your search in / to search using case-insensitive regex
e.g. /1[4-9] attunement/ to find items with 14-19 attunement
- Macros
Macros are short hands that match supported properties of items e.g.
LP0 (is unique with no legendary potential)
T6+ (has tier 6 or higher affix)
prefixes2 (has 2 prefixes)
Item Potential
LP (is non-WW unique)
WW (is WW unique or legendary)
WT (is enchantable idol)
FP (is forgeable equipment)
Item Types
Set (grants set bonus)
RealSet (is set item)
ReforgedSet (is reforged set item)
Experimentable (is boots/gloves/belt)
Equipment Requirements
lvl (required level)
CoF (Circle of Fortune tagged)
MG (Merchant’s Guild tagged)
trade (can be traded)
Affix Tier
T (at least 1 affix tier matches)
Affix Counts
Prefixes
Suffixes
Affixes
Sealed
Experimental
Personal
- Expressions
Macros can be combined into expressions using | or
& e.g.
LP3+|WW20+ to find high potential uniques
prefixes1&T7 to find T7 exalts with an open prefix
## Audio
- The Ring of Shields sub-skill, Shrapnel, now plays audio when triggered.
- Cinematic audio now plays through your selected audio device.
- Added new emerging and death sound effects to the crystals that can be destroyed during
the encounter against Majasa.
## Other Changes
- Added “Bow Mastery” before Marksman passive bonuses to improve text clarity.
- The appearance inventory now shows the correct equipped items when changing scenes.
- Reduced cases where issues with Parties could lead to players either disconnecting or
needing to wait for a long time to get into the game.
# Known Issues
Please be aware that our team is still working to correct more issues and improve many more
areas of Last Epoch. To see a short, non-exhaustive list of what we are still working on,
please check here on ourForum Known Bugs Listsor
ask our Community Managers inDiscord!