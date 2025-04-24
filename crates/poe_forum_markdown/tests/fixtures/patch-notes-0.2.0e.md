## 0.2.0e Patch Notes

This patch contains many of the changes discussed in our recent news
posts, including changes to Act 3, player balance changes and new Runes
for attributes, as well as other improvements and bug fixes.

### Changes to Act 3 Areas

- Removed the dead end in The Drowned City as well as the part of
the level that leads up to it. This reduces the overall size of
the area quite a lot, but more importantly prevents the
frustrating situation where you go the wrong way and have to
backtrack.
- Utzaal had the exact same problem as the Drowned City and we are
doing exactly the same fix, but in addition has had a second
dead end removed where the entrance to the Treasure Vault is
located in the Present version of this area.
- The Apex of Filth has had the layout updated to resemble more of
a linear flow, like Aggorat, making it smaller and faster to
navigate.
- The Azak Bog is now more rectangular, and slightly smaller.
- The Infested Barrens now has a bottleneck requiring you to go
past the entrance to The Matlan Waterways near the start meaning
that there are less things you need to try to find in this area.
- The Jungle Ruins exit to the Infested Barrens has been made more
clear, and moved the checkpoint further so that you are more
likely to see it. This area has also been made slightly smaller.
- Some variations of the Chimeral Wetlands have had the Temple of
Chaos and Jiquani's Machinarium entrances moved closer
together to reduce backtracking.
- Added an extra Soul Core to both Jiquani's Machinarium and
Jiquani's Sanctum to reduce the average amount of time it
takes to find them.
- The checkpoint at the end of Jiquani's Machinarium has been
moved to be beside the final Stone Altar, so you can quickly
return to the Altar upon finding the Soul Core.
- Some sections of water in The Matlan Waterways have been merged
together to reduce the total amount of necessary levers to pull.
- Increased the instance timeout time for areas with side-areas
off them. This should reduce how often you return from a side
area and find the main area has been reset.
- Added a few more checkpoints to a handful of areas that
didn't have sufficient checkpoints.
- The density of areas has been adjusted to account for level
generation changes in this patch.

### Player Balance Changes

**Bleed/Chaos Innoculation Changes**

- Bleeding no longer only considers damage dealt to the
target's life. Damage dealt to energy shield (or mana) can
now cause bleeding.
- Chaos Inoculation now additionally makes you immune to bleeding.
- Most Passive Skill Tree clusters that granted additional Stun
Threshold based on your maximum Energy Shield now also grant
additional Ailment threshold based on your maximum Energy
Shield.
- Added new Jewel and Time-lost Jewel modifiers for additional
Ailment threshold based on your maximum Energy Shield.
- The Jewel Modifier for additional Stun Threshold based on your
maximum Energy Shield now rolls 5-15% (previously 5-10%).
Existing items can be improved to the new ranges by using a
Divine Orb.
- The Time-lost Jewel Modifier for additional Stun Threshold based
on your maximum Energy Shield now rolls 1-2% (previously 1%).
Existing items can be improved to the new ranges by using a
Divine Orb.

**Sorceress Specific Changes**

- Arc now releases damaging pulses when Shocked enemies are Hit,
but can no longer Shock. It does not consume the Shock. The Arc
now deals 9 to 51 at Gem level 1 (previously 9 to 50), scaling
up to 82 to 462 at Gem level 20 (previously 72 to 410). The
Pulse deals 5 to 31 Lighting Damage at Gem level 1, scaling up
to 49 to 277 at Gem level 20, and has a radius of 1.5 metres.
- Eye of Winter now gains bonus elemental damage when passing
through other elemental orbs like Solar Orb, Orb of Storms or
Frost Bomb.
- Incinerate's Fire Exposure Duration is now 8 seconds
(previously 2 seconds).
- Mana Tempest can now be used with all spells, and has been
changed to cause empowered spells to split to additional targets
instead of chaining or forking.
- Frostbolt's Explosion radius is now 2.4 metres (previously
1.6).
- The Freeze Buildup from Ice Nova and Ice Nova when cast near a
Frostbolt are now matching. Ice Nova now has 100-157% more
Freeze Buildup at Gem levels 1-20 (previously 50% only when cast
on Frostbolt).
- Frost Bomb now has an Exposure duration of 8 seconds (previously
5).
- Mana Remnants now has a 25% chance to spawn a Remnant on killing
an enemy affected by an Elemental Ailment (instead of only
Shock). Now Spawns a Remnant on Critically Hitting a target
affected by an Elemental Ailment, no more than once every 2
second (instead of only Shock).
- Spreading ignites, such as from Wildfire Support or the
Cracklecreep Unique Ring, now occurs after a 1 second delay
(previously 2 seconds).
- Wildfire Support now causes ignite to spread to enemies within a
1.5 metre radius (previously 2 metre).

**Huntress Specific Changes**

- Parried enemies can no longer evade your attacks.
- Rapid Assault's Explosion now inflicts Bleeding on Hit, and
has a radius of 2.4 metres (previously 2 metres). The Explosion
now deals 142-312% of Attack Damage at Gem levels 5-20
(previously 85-187%).
- Spearfield now has a Spear duration of 10 seconds (previously
6). The Explosion now deals 17-49% of Attack Damage at Gem
levels 5-20 (previously 17-37%).
- Herald of Blood no longer destroys the corpses of Rare and
Unique monsters when they explode, allowing you to use Ritual
Sacrifice on the Rare monsters.

**Warrior Specific Changes**

- The Temper Weapon Skill, granted by the Smith of Kitava's
Against the Anvil Ascendancy Passive Skill, now causes each
strike of the anvil Empowers your next 3 Melee Attacks while
Channelling (previously 1). Now has a maximum of 12 Empowered
Attacks (previously 4), and now Channels 25% faster. Combust now
deals 80-290% of Attack Damage at Gem levels 1-20 (previously
175-714%).
- Shield Wall can now be detonated by Warcries as well as Slams.
- Armour break inflicted on players now lasts 4 seconds
(previously 12). Armour break inflicted on non-players still
lasts 12 seconds.
- Fully broken armour inflicted on players no longer causes the
player to take increased physical damage. Fully broken armour on
non-players remains unchanged at 20% increased physical damage
taken from hits.

**Ranger Specific Changes**

- Stormcaller Arrow's Bolt now has an impact radius of
1.6-2.4 metres at Gem levels 3-20 (previously 1.1-1.9 metres).
If any target is hit by Stormcaller Arrow's Bolt is
Shocked, it now also Shocks Enemies within a 1.7-3 metre radius
at Gem levels 3-20.
- Lightning Rod now deals 26-94% of Attack Damage at Gem levels
1-20 (previously 20-72%). Now has a Maximum Arrow duration of 20
seconds (previously 12).
- Lightning Arrow's beam targeting radius is now 3.2 metres
(previously 2.4).
- Tornado Shot now has a maximum Tornado duration of 15 seconds at
all Gem levels (previously 7-7.9 at Gem levels 11-20).

**Other Player Balance**

- Rally support is no longer restricted to Strikes or Slams, and
can now support any Melee Attack you use yourself.
- Glory (used by Hammer of the Gods and Spear of Solaris) is now
no longer consumed if you are interrupted while using the skill.

**New Support Gem**

- We have added a support gem called Inhibitor that prevents
charges being consumed but increases the damage of the supported
skill by 4% for each type of charge you have. Inhibitor is
offered at uncut support tier 2+.

### Delirium Changes

- Delirium encounters now last approximately 2.5 times longer.
- The front of the Delirium Fog can now continue expanding while
doing other mechanics, though the back of the fog will still
pause. This prevents scenarios where you pause the fog while on
the edge of it and accidentally end the Delirium.
- Strongboxes now pause Delirium fog when they are opened, in
addition to the pause they already had on completion.

### Monster Speed Changes

- Many human monsters including the Cultists in Freythorn, the
Faridun and the Tribal Humans in Act Three have behaviour where
they can interrupt their melee attacks if the player moves too
far out of range during the attack, especially for attacks that
have multiple hits like a swipe left into swipe right. These
interrupt events have been primarily removed especially on
things that were attacking very fast as it caused the monsters
to be relentlessly able to pursue and attack you giving you no
time to engage or use skills between their attacks.

**Act 1**

- The Haste Aura Monster modifier no longer appears on monsters
that are already fast.
- Werewolf Prowlers and Tendril Prowlers now will enter a walking
stance (as opposed to running) after performing a melee action,
they will only begin running again if you get a certain distance
away from them. This behaviour has been applied to many faster
monsters.
- Hungering Stalkers now have 12% less Life and Damage, they were
already relatively weak but we have lowered it a bit further to
account for their high movement speed and attack speed. They are
intended to engage and attack quickly, but be weak and die fast.
- Reduced the number of Bloom Serpents found in The Red Vale.
- Halved the number of Venomous Crabs and Venomous Crab Matriarchs
in the Hunting Grounds.
- The Cultists in Freythorn no longer have interrupt events on
their attacks as described above.
- The Cultists in Freythorn wielding Axes and Maces in Freythorn
now walk after performing a Melee Action, only running again
once you exit a certain distance.
- Slightly decreased the number of Cultists in Freythorn.
- Blood Cretins on death Blood Pools have had their duration
decreased from 6 seconds to 4 seconds, and fixed the area of
effect to match the visual more closely.
- Reduced the overall density of more challenging monsters in
Ogham Manor.

**Act 2**

- Boulder Ants have been replaced by Risen Maraketh in the Valley
of the Titans. The density of Monsters in this area has also
been adjusted.
- The Faridun have all been modified to remove the interrupt
events on their attacks as described above.

**Act 3**

- Diretusk Boar and Antlion Charger's are now more likely to
push you to the side instead of pushing you along with them when
they charge you.
- The Lost City monster pack composition has been adjusted to
result in less ranged monsters.
- Massively adjusted the Azak Bog, firstly by the aforementioned
changes to interrupt events, but also changed the monster
composition of the area to have less Ranged and Elite monsters.
- Fixed an issue where the Slitherspitter's poison spray in
Venom Crypts was dealing Chaos Damage instead of Physical Damage
unintentionally.

### Boss Changes

- Lowered the amount and size of Chaos Rains (the purple ones) in
the Viper Napuatzi fight, and cleaned up the visual left
afterwards faster to make the following drop locations more
obvious.
- Uxmal, the Beastlord can no longer recharge Energy Shield while
they are in the air, uses his Flame Breath less often, and has
had the number of times they can change locations through the
fight reduced.
- The arena of Xyclucian has had its ground foliage removed in
order to make his effects more visible.

### Player Minion Changes

- We have changed the way that minion revive timers work. When
your first minion dies, it sets the revive timer to 7.5 seconds
as before, but each successive minion that dies increases it by
less and less (still capped to a max of 7.5 seconds). This
should heavily mitigate the situation where most of your minions
are dead, but the revive timer keeps resetting to 7.5 seconds
over and over.
- Disenchanting a Bind Spectre or Tame Beast gem will unbind them,
allowing you to use them again.
- Tamed beasts can now fit through gaps of the same size that the
player can.

### Crafting Changes

- We have now finished adding all of the mods to runes for caster
weapons. Desert, Glacial, Storm, Iron, Body, Mind, Rebirth,
Inspiration, Stone and Vision runes all now work on Wands and
Staves, with their own set of modifiers.
- Renly's abandoned shop in Ogham Village now also has a
Blank rune which Renly can forge into any elemental rune of your
choosing in case you didn't find any up to that point.
- Added 12 Artificers Orb's to fixed locations throughout the
campaign, allowing you to craft with them more often.
- Added 3 new types of runes for attributes allowing you to fix
early game requirement issues: Adept, Robust and Resolve.

### Finding Rare Monsters in Endgame

- During endgame a common issue is missing a Rare Monster in the
corner of the map that you didn't happen to explore. In
order to mitigate this issue, we have changed rares to show up
on the minimap at all times.

### Other Improvements

- Added a Rune to a fixed location in The Titan Grotto.
- Increased the number of Armourer's Scraps and
Blacksmith's Whetstones that can be found in the Mawdun
Quarry.
- Dread Servant's undead tornados no longer benefit from
additional projectile modifiers, and now deal damage far less
frequently.
- Antlion Charger's charge is now more likely to push you to
the sides as opposed to dragging you along with it. They also
now have less Antlion's and more fodder when found in Map
areas.
- Rathbreaker and Caedron, the Hyena Lord have had their damage
reduced.
- Added lighting to Rogue Exiles to reduce the likelihood of
losing track of them.
- Further improvements to the pushiness of various monsters (read:
less pushing).

### Bug Fixes

- Fixed a bug where Blood Boils from the Ritualist ascendancy
didn't propagate if the monster exploded on death, such as
when using Herald of Blood.
- Fixed a bug where Divine Orbs were affecting Fractured
Modifiers.
- Fixed a bug where Unique Jewels that affect Passives in Radius
were not functioning correctly with small Attribute Passive
Skills, or small class-specific Passive Skills.
- Fixed a bug where partial Armour Break had an infinite duration,
if you or the target never had Fully Broken Armour.
- Fixed a bug where the Combust from the Temper Weapon Skill,
granted by the Against the Anvil Smith of Kitava Ascendancy
Passive Skill, was failing to deal damage.
- Fixed a bug where taming a beast in the Trial of the Sekhemas
could prevent you from progressing the encounters (for real this
time).
- Fixed a bug where Manifested Weapons could fail to hit a target
if they were too close to them.
- Fixed a bug where Whirlwind Lance just skedaddled when supported
by Fork.
- Fixed a performance issue with Whirlwind Lance.
- Fixed a bug where it was possible to apply Thorns damage to
yourself, such as through using the Crown of the Pale King and
Fireflower Unique Items.
- Fixed a bug where Electrocute was not counting towards "per
Elemental Ailment on the Enemy".
- Fixed a bug where Bloodhound's Mark could have an incorrect
Heavy Stun buildup.
- Fixed a bug where Unsteady Tempo was able to support Channelling
Skills, though it had no effect when it did.
- Fixed a bug where strongboxes could sometimes fail to open.
- Fixed an issue where cancelling out of the open animation for
the Ixchel's Torment Unique Strongbox would cause it to not
open.
- Fixed a bug where Essence of the Infinite was dealing damage far
too early.
- Fixed an issue where the Sandspit Map was failing to add
additional packs of monsters.
- Fixed a bug where a forcefield could appear in the middle of the
Boss arena in The Copper Citadel Map.
- Fixed a bug where you could become stuck in unwalkable terrain
when using the Alpine Ridge or Castaway Map Boss Checkpoints.
- Fixed a bug where Hideout Maps were unable to be completed if
Delwyn was present.
- Fixed a bug that caused desync to occur when trying to walk
around the default Map Device in a Hideout.
- Fixed a bug where Parry could become unbound from your skill bar
after changing areas when using a controller.
- Fixed a bug where skills were disabled if you were revived while
spectating party members when using a controller.
- Fixed a bug where you could be prevented from levelling up your
Companions when using a controller.
- Fixed a bug where the Split keyword popup stated Projectiles
that Split fired at a number of targets within 4 metres, when
the actual radius is 6 metres.
- Fixed a client crash that could occur with Ancestral Warrior
Totem in party play.
- Fixed a client crash related to applying the Impaled debuff.
- Fixed three other client crashes.
- Fixed two instance crashes.

This patch may take roughly 15 minutes to become available to download
on PlayStation after it has been deployed.