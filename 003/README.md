# rangifer’s diary: pt. iv

When rangifer first logged on today, **Boymoder** was already LPQing 🛑🐀🐙👀🐋! Her party was full, so I was grinding out wrapping papers 🎁📃 on my I/L 🧊🌩️ [magelet](https://oddjobs.codeberg.page/odd-jobs.html#luk-mage), **cervine**, while I waited. Then our ally (and otherwise guildmate) **Mings**, the level 35 [beginner](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner), logged on and expressed interest in LPQing as well. Then, our F/P 🔥🧪 [gishlet](https://oddjobs.codeberg.page/odd-jobs.html#luk-gish) guildmate **TestChars** (otherwise known as **Outside** or **braidgame**) also expressed interest! Unfortunately, Mings had to leave before we could get the party going, but the other three of us were able to cobble together a nifty LPQ party ✨!

With our original LPQ party, we had a somewhat challenging (although still reasonably swift) fight with Alishar 🐋 at the end… and we did very well! I’m still getting the hang of controlling my character now that I have so many attacking skills 😫 (as brawlers/marauders/buccaneers do), and I just hope that no one notices how much I’m stuggling to press the keyboard in the right ways 😆! That being said, playing a brawler is heaps of fun, and who needs weapons when you’ve got ten thousand ways to beat your enemies up 👊🏾🦵🏾👊🏾🦵🏾…? Here we are posing for the camera 📸:

![Say cheese!](say-cheese.png "Say cheese!")

## Strong 💪 throw, deadly star 🌠

In other news, Boymoder has gotten herself a very nice [Shinobi Bracer](https://maplelegends.com/lib/equip?id=01472054) for when she hits level 55~! Unfortunately, this is only _after_ having extremely poor luck scrolling a [Maple 🍁 Skanda](https://maplelegends.com/lib/equip?id=01472055) 😬… _failing six_ out of the seven [60% scrolls](https://maplelegends.com/lib/use?id=2044701) on a weapon that is already very pricey to begin with is absolutely brutal ☹️… this new Shinobi Bracer replaces the Skanda, being better than the Skanda in every way 🎉.

Since I went over some of the math 🧮 behind second job [STRlords](https://oddjobs.codeberg.page/odd-jobs.html#str-assassin) in part ii of this diary, I’ll take the time here to go over some similar aspects, but of third job rather than second. STRlords get two new notable attacking skills in third job: [Shadow Meso 🌑💸](https://maplelegends.com/lib/skill?id=4111004) and [Avenger ✴️](https://maplelegends.com/lib/skill?id=4111005). They are also aided by getting access to [Shadow Partner 👥](https://maplelegends.com/lib/skill?id=4111002) as well! Let’s consider what the DPS of these skills looks like for STRlords, assuming that they are maxed.

### Shadow Meso

Our model monster is the same level as the player character, and has 400 WDEF. Shadow Meso says in its description: “Nullifies the monsters’ ‘weapon def. up’ and ‘magic guard up.’”; [Hidden Street’s version](https://bbb.hidden-street.net/character/class/hermit) instead says “Ignores the enemies’ ‘weapon def. up’ and ‘magic guard up.’” (the discrepancy may just be a difference in GMS version). While this is a bit of an odd thing for this skill to be doing (it would seem to be stepping on the toes of [Armor Crash](https://maplelegends.com/lib/skill?id=1111007) and [Magic Crash](https://maplelegends.com/lib/skill?id=1211009)…), there’s no reason to really suspect that Shadow Meso _ignores WDEF entirely_, so we will assume that it interacts normally with WDEF. Shadow Meso isn’t a very popular skill, but if anyone wants to actually do some damage tests, that would be awesome.

Shadow Meso’s damage is pretty straightforward: it deals damage equal to ten times the number of mesos thrown. The minimum and maximum number of mesos thrown is shown in the skill description, and we assume that the number of mesos thrown by a given use of Shadow Meso is [uniformly distributed](https://en.wikipedia.org/wiki/Discrete_uniform_distribution). Also, it has its own notion of critical hits: it does not interact with [Critical Throw](https://maplelegends.com/lib/skill?id=4100001) at all, instead listing its own critical probability in the description, and listing its own critical hit damage (“Damage + 50%”, i.e. 1.5 times the damage of a non-crit).

At max level, the minimum number of mesos thrown is 340, and the maximum is 800. So the minimum and maximum damages of a non-crit, before WDEF, are 340 ⋅ 10 = 3400, and 800 ⋅ 10 = 8000, respectively. To then take WDEF into account, we see that the minimum and maximum damages after WDEF reduction are, respectively:

    minDmgAfterWdef(minDmgPreWdef, WDEF, monsterLevel, playerLevel) := minDmgPreWdef * (1 - 0.01 * max{monsterLevel - playerLevel, 0}) - WDEF * 0.6

    maxDmgAfterWdef(maxDmgPreWdef, WDEF, monsterLevel, playerLevel) := maxDmgPreWdef * (1 - 0.01 * max{monsterLevel - playerLevel, 0}) - WDEF * 0.5

Here, “`minDmgPreWdef`” is the minimum damage dealt before WDEF is taken into account, likewise for “`maxDmgPreWdef`”, and “`max`” stands for “[maximum](https://en.wikipedia.org/wiki/Maxima_and_minima) \[value of a [set][set]\]”. Because we already made the assumption that the monster and the player character are the same level, `(1 - 0.01 * max{monsterLevel - playerLevel, 0})` is just `1`, so this simplifies to:

    minDmgAfterWdef′(minDmgPreWdef, WDEF) := minDmgPreWdef - WDEF * 0.6

    maxDmgAfterWdef′(maxDmgPreWdef, WDEF) := maxDmgPreWdef - WDEF * 0.5

Since we assume that the monster has 400 WDEF, we can calculate that `minDmgAfterWdef′(3400, 400) = 3400 - 400 * 0.6 = 3160`, and that `maxDmgAfterWdef′(8000, 400) = 8000 - 400 * 0.5 = 7800`. So our minimum and maximum damages for a non-crit are 3160 and 7800, respectively, once we take WDEF into account. Critical damage is calculated after (W/M)DEF, so to get the minimum and maximum damages of a critical hit, we simply multiply these two numbers by 1.5, and get 3160 ⋅ 1.5 = 4740, and 7800 ⋅ 1.5 = 11700, respectively.

Again, we take the damage — given that the damage is either a critical, _or_ given that it is not a critical — to be uniformly distributed, so we can easily get the [expected](https://en.wikipedia.org/wiki/Expected_value) damage of a given hit (again, knowing whether or not it’s a critical hit) by taking the [arithmetic mean](https://en.wikipedia.org/wiki/Arithmetic_mean) of the minimum and maximum. For a non-crit, this is (3160 + 7800) ÷ 2 = 5480 damage. For a crit, this is (4740 + 11700) ÷ 2 = 8220 damage. We know from Shadow Meso’s skill description that at max level, it has a 10% = 0.1 probability of critting, and thus a 1 − 0.1 = 0.9 probability of not critting. Because expectation is [linear](https://en.wikipedia.org/wiki/Linear_map), we can get the expected damage of a given Shadow Meso — but this time, _without_ knowing whether it will be a crit or not — by simply calculating [the weighted sum of the two](https://en.wikipedia.org/wiki/Bernoulli_distribution#Mean) expectations:

- Damage given that it is a non-crit (5480).
- Damage given that it is a crit (8220).

(5480 ⋅ 0.9) + (8220 ⋅ 0.1) = 5754 damage.

Because we are assuming that the STRlord in question has their skills maxed, their Shadow Partner throws a secondary Shadow Meso every time that the STRlord throws their own Shadow Meso, and the secondary (shadow) one deals 50% of the damage of the normal one. I somewhat suspect (but cannot prove; anyone who knows more about Shadow Partner than I do, please do chime in) that Shadow Partner’s damage multiplier is applied _before_ WDEF is taken into account, so we have `minDmgAfterWdef′(3400 * 0.5, 400) = 3400 * 0.5 - 400 * 0.6 = 1460`, and `maxDmgAfterWdef′(8000 * 0.5, 400) = 8000 * 0.5 - 400 * 0.5 = 3800`. Min and max crit damages are then 1460 ⋅ 1.5 = 2190, and 3800 ⋅ 1.5 = 5700, respectively. For the expectation of non-crits, we have (1460 + 3800) ÷ 2 = 2630 damage; for crits, (2190 + 5700) ÷ 2 = 3945 damage. (2630 ⋅ 0.9) + (3945 ⋅ 0.1) = 2761.5 damage per secondary (Shadow-Partner-induced) Shadow Meso. Note that 2761.5 ⋅ 2 = 5523 ≠ 5754, so simply multiplying the Shadow-Meso-sans-Shadow-Partner expectation by 1.5 would have given us a subtly incorrect result! Working with some WDEF makes the calculations here a good deal more arduous, but I’ve chosen 400 WDEF for realism, and because working through the WDEF calculations is… maybe illustrative(‽)

So our expected damage for a single use of Shadow Meso with Shadow Partner is 5754 + 2761.5 = 8515.5 damage. Then our expected DPS (damage per second) is (again, due to the linearity of expectation) just that value times the attack [frequency](https://en.wikipedia.org/wiki/Frequency). According to [**LazyBui**’s “Attack Speed Reference”](http://www.southperry.net/showthread.php?t=3217), the attack [period](https://en.wikipedia.org/wiki/Frequency#Period_versus_frequency) for a claw with speed 2 (almost all claws, and certainly any that are both level ≥35 and equippable by a STRlord, have speed ≤4; speed ≤4 means a speed of exactly 2 whenever [Claw Booster](https://maplelegends.com/lib/skill?id=4101003) is active) throwing Shadow Mesos is 600 ms = 0.6 s. The attack frequency is then 1 ÷ 0.6 s = 1.666… [Hz](https://en.wikipedia.org/wiki/Hertz). 8515.5 damage ⋅ 1.666… Hz = 14192.5 DPS.

14192.5 DPS in a realistic scenario is nothing to sneeze at 😮! Ordinary (LUK-based) hermits/nightlords tend to neglect Shadow Meso entirely, since its damage only really scales with skill level, and does not scale at all with any of the character’s stats (WATK, LUK, DEX, STR, etc.). However, in the hands of a STRlord, who only uses claws in one of their two modes of combat (their other being melee ⚔️), Shadow Meso can be an extremely potent addition to their arsenal that puts their single-target DPS above that of almost any [other odd job](https://oddjobs.codeberg.page/odd-jobs.html) 🤯!

### Avenger

Doing similar calculations for Avenger is somewhat simplified by the fact that _Avenger ignores WDEF whenever the player character’s level is greater than or equal to the level of their target_. Because we already assumed that these two levels are the same, we can totally ignore WDEF for the purpose of Avenger damage calculations (phewf! 😌). Avenger is a much more ordinary skill in comparison to Shadow Meso, so it uses the usual physical damage formulae:

    minDmgClaw(LUK, STR, DEX, WATK, MASTERY) := (LUK * 3.6 * 0.9 * MASTERY + STR + DEX) * WATK / 100

    maxDmgClaw(LUK, STR, DEX, WATK) := (LUK * 3.6 + STR + DEX) * WATK / 100

Of course, now that we’re dealing with actual stats (LUK, STR, DEX, WATK), we have to decide what level this hypothetical STRlord is and what their gear is like. Here’s the model I chose (based on a roughly level 102 hermit, because that’s roughly the minimum level to have Shadow Partner, Shadow Meso, and Avenger maxed):

- 525 STR (25 of which is from gear)
- 55 DEX (30 of which is from gear)
- 60 LUK (56 of which is from gear)
- 102 WATK (42 from [claw](https://maplelegends.com/lib/equip?id=01472055) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves + 27 from [Ilbis](https://maplelegends.com/lib/use?id=2070006))

So we have `minDmgClaw(60, 525, 55, 102, 60%) = (60 * 3.6 * 0.9 * 60% + 525 + 55) * 102 / 100 = 710.5728`, and `maxDmgClaw(60, 525, 55, 102) = (60 * 3.6 + 525 + 55) * 102 / 100 = 811.92`. Of course, Avenger does more than 100% damage: at max level, 180%. Because we also have maxed Critical Throw, crits will deal 180% + 100% = 280% damage. The skill description for Critical Throw reads “critical damage 200%” at max level, but should read “critical damage +100%”; the “200%” figure assumes a basic attack (100% + 100% = 200%), but we are not basic attacking… we are using Avenger. Also, the Avenger star thrown by Shadow Partner will do only 50% damage. Because Avenger is ignoring WDEF entirely in this case, the calculations are much simpler:

- Expected raw damage: (710.5728 + 811.92) ÷ 2 = 761.2464 damage.
- Non-Shadow-Partner star, non-crit: 761.2464 ⋅ 180% = 1370.24352 damage.
- Non-Shadow-Partner star, crit: 761.2464 ⋅ 280% = 2131.48992 damage.
- Non-Shadow-Partner star: 1370.24352 ⋅ 0.5 + 2131.48992 ⋅ 0.5 = 1750.86672 damage.
- Non-Shadow-Partner star + Shadow Partner star: 1750.86672 ⋅ 1.5 = 2626.3 damage.

Then we obtain the DPS in the same way as before. Unfortunately, Avenger is ever-so-slightly slower than Shadow Meso, with an attack period of 630 ms = 0.63 s (rather than 600 ms). So the attack frequency is then 1 ÷ 0.63 s = 1.5873 [Hz](https://en.wikipedia.org/wiki/Hertz). 2626.3 damage ⋅ 1.5873 Hz ≈ 4169 DPS. This is a factor of 14192.5 ÷ 4169 ≈ 3.4 less than Shadow Meso, but of course, this is only single-target DPS, and Avenger targets up to 6 monsters at once! So both skills are quite useful when training ✨.

## Strong arm 💪, deadly punch 👊?

So how does a third-job pugilist compare?? Well, that may be for a future diary entry…

![F3](rangifer-f3.png "F3")

[set]: https://en.wikipedia.org/wiki/Set_(mathematics)
