# rangifer’s diary: pt. xliv

## R>1 mageoid for Crimson Balrog runs

Now that this series has already covered “all” [permabeginners](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner) and permabeginner-a-likes (see the previous diary entry), as well as “all” odd warrior jobs (see pt. xli and pt. xlii of this diary), it’s time to tackle the next one on the list (this list is in ascending order of ID, as expressed internally by MapleStory itself):

- beginners (0)
- warriors (100)
- magicians (200)
- archers (300)
- rogues (400)
- pirates (500)

That’s right, it’s mage time. But not just any mages; only goofy-ass odd-jobbed mageoid freaks allowed here!:

- [STR mages](https://oddjobs.codeberg.page/odd-jobs.html#str-mage)
- [DEX mages](https://oddjobs.codeberg.page/odd-jobs.html#dex-mage)
- [Permamagicians](https://oddjobs.codeberg.page/odd-jobs.html#permamagician)
- [Gishes](https://oddjobs.codeberg.page/odd-jobs.html#gish)
- [Gishlets](https://oddjobs.codeberg.page/odd-jobs.html#luk-gish)
- [Magelets](https://oddjobs.codeberg.page/odd-jobs.html#luk-mage)

We can actually eliminate the first two items on the above list; we included STR mages as “generic STR whackers” in the previous entry, and DEX mages as “generic DEX whackers”. The reason for this is that they are effectively indistinguishable from their permabeginner counterparts for the purpose of this analysis. While this _does_ simplify things, the remaining four items leave things still quite complicated indeed.

Our most straightforward archetype is that of the permamagician. Permamagicians never take second job advancement, so we don’t have any complications due to the three distinct mage paths (F/P, I/L, cleric/priest/bishop) offered from second job onwards. Additionally, we will be assuming that our model permamagician is a “normal” (read: entirely or almost entirely INT) one, and is thus not hybridised with other odd mage jobs like e.g. STR mage. Like most mages around this level (roughly level 100), the choice weapon of our permamagician is an Elemental Wand of some kind, because we assume them to be pure INT. Exactly which level 70 Elemental Wand is chosen, doesn’t matter terribly, because the permamagician has no access to elemental spells, so we will just use [the first one](https://maplelegends.com/lib/equip?id=01372035). Unfortunately, this means that we have to deal with TMA (total magic attack) gear for the first time, which functions quite a bit differently than gearing for physical attacks. And furthermore, in a server like MapleLegends, [mages](https://maplelegends.com/lib/skill?id=2321008) and [washing](https://maplelegends.com/lib/cash?id=5050000) gear are in such high demand, that a set of physical-attacking gear and TMA gear that are roughly the same level of quality, can differ _wildly_ in market value. So, choosing TMA gear that is “equivalent” to the kind of gear that we have supplied our physical-attackers with, seems to be a troubling task. To make things easier, any time that we want purely TMA gear, I will just be using a lightly modified version of the actual equipment that I use on my own I/L [magelet](https://oddjobs.codeberg.page/odd-jobs.html#luk-mage), **cervine**.

Speaking of magelets, our magelet models will be equipped very similarly to our permamagician model, with two caveats. One caveat is that the exact element of the magelet’s weapon matters, as we assume that none of our magelets are permamagicians, so they will all have access to elemental spells. The other caveat is that, because we assume all models to be “roughly level 100”, this brings Elemental _Staves_ into consideration. Elemental Staves are only equippable by those on the higher end of “roughly level 100”, because they require the mage to be at least level 103. So, in order to get numbers somewhere in between using an Elemental Wand and an Elemental Staff, we will be giving the magelet a TMA advantage on their gear equal to half of the difference between a wand and a staff. Besides the gearing considerations, we will have three different magelet models: one for each of the third job mage classes (F/P, I/L, priest). Actually, the [Heal](https://maplelegends.com/lib/skill?id=2301002) skill somewhat complicates things here; Heal is the main weapon of the cleric/priest magelet, because its damage (although, not its healing) scales on LUK. In addition, Heal changes its per-target damage based on how many targets there are in total. This makes Heal a super weird case here, but we will, for completeness, include a single model that does use Heal; we will use the best possible case, where the monster is the only other target of Heal besides the magelet themselves (and also, obviously the monster will be undead, and therefore susceptible to Heal).

This brings us to the gishlets and the gishes. These two odd jobs are extraordinarily complex; they prove to be more difficult to plan, build, and equip than perhaps any other job (odd or not) in the entire game, and their two modes of attacking (melee and magic) make our analysis even hairier than it already is. So we are actually going to leave gish(let)s their entire own entry in this series.

With all of that said, let’s take a look at our models:

### The model permamagician

- 613 INT (7 from [wand](https://maplelegends.com/lib/equip?id=01372035), 6 from [shield](https://maplelegends.com/lib/equip?id=01092003), 25 from hat/helmet, 3 from [glasses](https://bbb.hidden-street.net/eq/eye-accessory/broken-glasses), 5 from earrings, 10 from cape, 16 from [overall](https://maplelegends.com/lib/equip?id=01051098), 7 from gloves, 5 from [pendant](https://maplelegends.com/lib/equip?id=01122014), 3 from [shoes](https://maplelegends.com/lib/equip?id=01072078), 5 from rings)
- 146 raw MATK (97 from [wand](https://maplelegends.com/lib/equip?id=01372035), 9 from [shield](https://maplelegends.com/lib/equip?id=01092003), 10 from earrings, 6 from cape, 4 from gloves, 20 from [Wizard Elixir](https://maplelegends.com/lib/use?id=2002018))

And, as you’d expect, our permamagician will only be attacking with [Magic Claw](https://maplelegends.com/lib/skill?id=2001005).

### The model magelet

- 112 INT (7 from [wand](https://maplelegends.com/lib/equip?id=01372035)/[staff](https://maplelegends.com/lib/equip?id=01382045), 6 from [shield](https://maplelegends.com/lib/equip?id=01092003), 25 from hat/helmet, 3 from [glasses](https://bbb.hidden-street.net/eq/eye-accessory/broken-glasses), 5 from earrings, 10 from cape, 16 from [overall](https://maplelegends.com/lib/equip?id=01051098), 7 from gloves, 5 from [pendant](https://maplelegends.com/lib/equip?id=01122014), 3 from [shoes](https://maplelegends.com/lib/equip?id=01072078), 5 from rings)
- 535 LUK (30 of which is from gear)
- 163 raw MATK (114 from [wand](https://maplelegends.com/lib/equip?id=01372035)/[staff](https://maplelegends.com/lib/equip?id=01382045), 9 from [shield](https://maplelegends.com/lib/equip?id=01092003), 10 from earrings, 6 from cape, 4 from gloves, 20 from [Meditation](https://maplelegends.com/lib/skill?id=2101001)/[Wizard Elixir](https://maplelegends.com/lib/use?id=2002018))

For our F/P and I/L magelets, we assume that they are using an Elemental Wand/Staff that is appropriate for the spell being cast. We assume maxed [Element Amplification](https://maplelegends.com/lib/skill?id=2110001), and at least level 11 [Spell Booster](https://maplelegends.com/lib/skill?id=2111005). Because the F/P has three main attacking skills in third job ([Poison Mist](https://maplelegends.com/lib/skill?id=2111003), [Explosion](https://maplelegends.com/lib/skill?id=2111002), and [Element Composition](https://maplelegends.com/lib/skill?id=2111006)), whereas the I/L really only has two ([Ice Strike](https://maplelegends.com/lib/skill?id=2211002) and [Element Composition](https://maplelegends.com/lib/skill?id=2211006)), the F/P is in a bit of a tough position here. Thankfully, although [Fire Arrow](https://maplelegends.com/lib/skill?id=2101004) is 30 points of basic attack behind Element Composition, Fire Arrow does at least have a smaller attack period; we will include both spells (assuming that either one is maxed), to be fair to our F/P magelet.

For our priest magelet, we will have the aforementioned [Heal](https://maplelegends.com/lib/skill?id=2301002) case, as well as a case that uses [Magic Claw](https://maplelegends.com/lib/skill?id=2001005) (which will, at the same time, represent a magelet permamagician!), and two for [SR](https://maplelegends.com/lib/skill?id=2311004) (elementally neutral & weak).

For all elemental spells, we will be splitting into two cases, one for the monster being elementally neutral towards the spell, and one for it being elementally weak.

### Comparing single-target DPS

As usual, we assume that all player characters and monsters have the same level. And, as in the “R>1 pog ranged for…” series, we will use 600 WDEF & 600 MDEF to reasonably represent a low- or mid-level boss monster.

| model                                      |    DPS |
| :----------------------------------------- | -----: |
| Magelet (F/P Ele Comp; weak)               | 4520.7 |
| Magelet (I/L Ele Comp; weak)               | 4192.2 |
| Magelet (Fire Arrow; weak)                 | 3976.9 |
| Permamagician                              | 3307.3 |
| Magelet (F/P Ele Comp; neutral)            | 2877.8 |
| Magelet (Heal; 1 non-self target)          | 2853.7 |
| Magelet (I/L Ele Comp; neutral)            | 2658.8 |
| Magelet (Fire Arrow; neutral)              | 2498.2 |
| Magelet (SR; weak)                         | 1206.1 |
| Magelet (SR; neutral)                      |  699.2 |
| Magelet (priest/permamagician; Magic Claw) |  234.4 |

So the first thing that we notice here is that the elemental magelets can outperform our permamagician in cases where the monster is elementally weak to their element(s). This perhaps comes as a surprise, considering that the permamagician is pure INT, whereas our magelets are INTless by definition. But in more neutral settings, our permamagician comes out on top.

With our priestlet, we can see extremely clearly the effects of the 600 MDEF; Magic Claw takes the biggest hit, because MDEF is (unlike WDEF) effectively applied to individual _lines_ of damage, and Magic Claw has two of these! So MDEF counts against them _twice_, leaving the priestlet/permamagicianlet at a measly 234.4 DPS when using Magic Claw… And SR doesn’t fare very well either, because the priestlet has no pre-MDEF damage multipliers (besides elemental weakness, in that particular case), unlike the elemental magelets, who have both Element Amplification and an Elemental Wand/Staff.

We also, of course, want to compare these figures to the figures calculated in the previous series, and in previous installations of this series. I’ve added some class-based (class as in: beginner, warrior, mage, archer, rogue, pirate) emojis to the “model” column to add some readability to this now quite lengthy table (N.B. some or none of these emojis may show up if you are viewing this on the MapleLegends forums, for some reason(‽)):

| model                                         |     DPS |
| :-------------------------------------------- | ------: |
| ⚔️ dagger warrior                             | 14828.4 |
| 🥷 STRmit (SM)                                 | 13807.5 |
| ⚔️ DEX WK (fire weak)                         | 11856.8 |
| ⚔️ DEXsader                                   | 10729.0 |
| ⚔️ DEX WK (lightning weak)                    |  9977.1 |
| ⚔️ wand warrior                               |  9446.7 |
| ⚔️ DEX WK (ice weak)                          |  8097.5 |
| 🥷 Permarogue                                  |  7658.0 |
| ⚔️ DEX WK (fire neutral)                      |  7470.9 |
| 🏹 Wood(wo)man (bow)                          |  6725.3 |
| 🏹 Wood(wo)man (xbow)                         |  6690.6 |
| 🏴‍☠️ Permapirate                                |  6669.1 |
| 🏴‍☠️ Swashbuckler (yes Octo)                    |  6598.6 |
| ⚔️ DEX WK (lightning neutral)                 |  6217.8 |
| ⚔️ permawarrior                               |  5934.0 |
| 🏴‍☠️ Swashbuckler (no Octo)                     |  5614.4 |
| 🏹 Permarcher                                 |  5614.0 |
| ⚔️ DEXgon knight                              |  5054.7 |
| 🧙 Magelet (F/P Ele Comp; weak)               |  4520.7 |
| ⚔️ LUK WK (fire weak)                         |  4353.5 |
| 🧙 Magelet (I/L Ele Comp; weak)               |  4192.2 |
| 🧙 Magelet (Fire Arrow; weak)                 |  3976.9 |
| ⚔️ LUKsader                                   |  3868.9 |
| ⚔️ LUK WK (lightning weak)                    |  3545.8 |
| 🧙 Permamagician                              |  3307.3 |
| 🧙 Magelet (F/P Ele Comp; neutral)            |  2877.8 |
| 🧙 Magelet (Heal; 1 non-self target)          |  2853.7 |
| ⚔️ LUK WK (ice weak)                          |  2738.0 |
| 🧙 Magelet (I/L Ele Comp; neutral)            |  2658.8 |
| 🧙 Magelet (Fire Arrow; neutral)              |  2498.2 |
| ⚔️ LUK WK (fire neutral)                      |  2468.8 |
| 🔰 Generic STR whacker                        |  2227.8 |
| ⚔️ LUK WK (lightning neutral)                 |  1930.3 |
| ⚔️ LUK DK                                     |  1350.2 |
| 🔰 Wandginner                                 |  1308.7 |
| 🧙 Magelet (SR; weak)                         |  1206.1 |
| 🔰 Generic DEX whacker                        |   966.0 |
| 🔰 Generic claw-wielding non-rogue            |   841.5 |
| 🧙 Magelet (SR; neutral)                      |   699.2 |
| 🧙 Magelet (priest/permamagician; Magic Claw) |   234.4 |

Humourously enough, we now have two entries that can be beaten out by our [“generic claw-wielding non-rogue”](https://oddjobs.codeberg.page/odd-jobs.html#besinner)… But on the bright side for our odd mages considered in this entry, some of them compare quite favourably to the [permabeginners](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner) and permabeginner-a-likes, as well as to [LUK warriors](https://oddjobs.codeberg.page/odd-jobs.html#luk-warrior)! As usual, many of these comparisons are quite awkward; for example, priestlets and I/L magelets are built more for multi-target DPS, not single-target. And we don’t even consider any of the poison abilities of our F/P magelet!

**_IMPORTANT REMINDERS BEFORE ANYONE GOES AROUND TOUTING THESE NUMERIC FIGURES:_** Keep in mind (and I cannot stress this enough) that this is a purely one-dimensional — and somewhat shoddy — analysis using dummy models, _and_ that jobs cannot be reduced to raw single-target DPS numbers. The odd jobs that are listed above differ quite a bit in their playstyles and range of abilities. Furthermore, this only considers characters that are roughly level 100.
