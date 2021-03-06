# rangiferโs diary: pt. ii

I started off today by picking up some papers ๐, for the purpose of wrapping presents ๐ and so on (even if Christmas ๐ isnโt for another 11.5 months or soโฆ), and one of the characters who I like to do this with is my I/L ๐ง๐ฉ๏ธ [magelet](https://oddjobs.codeberg.page/odd-jobs.html#luk-mage), **cervine**. Now, technically this is a diary for **rangifer**, not cervine (nor any of my other characters), butโฆ whatever. Iโve been farming up colourful papers [at Red Kentauruses ๐ฅ๐](https://maplelegends.com/lib/map?id=240020000) for a while now; I like to sit at the bottom of the map and gather them alternately on the left and right side, only to maul them repeatedly โ and I do mean _very_ repeatedly โ with Ice Strike, until they are subdued. [Red Kentauruses](https://maplelegends.com/lib/monster?id=8140102) are weak to ice ๐ก๏ธ, and have relatively low (495) MDEF for their level (88), both of which are very good for me as an I/L magelet; MDEF can otherwise chew away a large portion of my damage (lower damage per line means that MDEF values chew up a proportionally larger amount of that damage), and extra damage due to elemental weakness is a multiplier (1.5) that is applied _before_ defense is taken into account, thus making it especially powerful for me as a magelet, for the same reason ๐.

I wanted to try doing the same at [Taipei 101 ๐น๐ผ](https://maplelegends.com/lib/map?id=742000000), as my magelet is level 93, meaning that anything there that is at least level 73 (= 93 โ 20) is fair game. Mannequins of some species are the usual suspect here, and the lowest-level species of mannequin are [Kid Mannequins](https://maplelegends.com/lib/monster?id=9410032), at level 75. Perfect! The wrapping papers drop quite a bit more frequently [at Kid Mannequins](https://maplelegends.com/lib/map?id=742010100) ๐ฎโฆ these mannequins may not be weak to ice, but they have a bit less MDEF (430 < 495) and, more importantly, _a whole lot less HP_ (8.5k < 37.0k). Plus, they tend to spawn a little more readily and accessibly (due to the simple map layout, and the teleporter ๐ช from bottom to top). I expected the amount of EXP that I got (per unit time, i.e. EPM or EPH) to be at least a little bit less, considering how favourable Red Kentauruses are for me, but a quick-and-dirty EPM test showed that I was apparently wrong โ these creepy dolls give enough EXP (in the ballpark of 2.1M EPM) to push my EPM up to at least what I get at Red Kentaurusesโฆ maybe even a bit more!

## Itโs rangifer time

Once my buddy and fellow odd-jobber **Boymoder** logged on, it was KPQ time! We even did a few KPQs with **Mings**, a [permabeginner](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner) of the **Flow ๐** guild (allied with **Oddjobs**) who also played **NoobIet** (`Noobiet`), the [dagger warrior](https://oddjobs.codeberg.page/odd-jobs.html#dagger-warrior) ๐ก๏ธ๐ก๏ธ, from the previous diary entry! After a few KPQs, Mings had to leave to finish up some questing, but Boymoder and I carried on until we hit the fabled _level 30_! Finally, we could advance to second job ๐. Speaking of second job, there are some interesting mechanics at work within both rangiferโs and Boymoderโs second jobs ๐คโฆ

## Fucking [STRlords](https://oddjobs.codeberg.page/odd-jobs.html#str-assassin), how do they work?

As mentioned in the previous entry, Boymoder is a STRlord. While it may seem at first that claws ๐งค are effectively useless for STRlords (unlike for LUK-based assassins), this is not exactly true, and tends to become less true as the STRlord levels up (particularly, culminating in skills like [Shadow Meso ๐ธ](https://maplelegends.com/lib/skill?id=4111004), [Avenger โด๏ธ](https://maplelegends.com/lib/skill?id=4111005), and even [Venomous Star ๐](https://maplelegends.com/lib/skill?id=4120005)). Since we are just advancing to second job, the question presents itself: what skill build makes the most sense for a second job STRlord? To help answer this question, I made use of my [damage calculator ๐งฎ](https://oddjobs.codeberg.page/dmg-calc/) to get some rough estimates of how useful each second job assassin skill is.

### The baseline

The model used here is roughly level 43 (give or take). Level 43 is an arbitrary level that was chosen because it is high enough to have a sizable quantity of second job SP, and is just high enough to make use of the [Maple Kandayo](https://maplelegends.com/lib/equip?id=01472032), and is a good level for LPQ! The weapon being used is assumed to be a maple ๐ claw of some kind ([Maple Claw](https://maplelegends.com/lib/equip?id=01472030), Maple Kandayo, or [Maple Skanda](https://maplelegends.com/lib/equip?id=01472055)). Basic attacking (bound to the `Ctrl` key by default) is used as the baseline here, and [Drain ๐ง](https://maplelegends.com/lib/skill?id=4101005) replaces it when Drain is at least level 1.

The โ**CV**โ in the tables below stands for the [coefficient of variation](https://en.wikipedia.org/wiki/Coefficient_of_variation).

- 220 STR
- 35 DEX (10 from equipment)
- 35 LUK (31 from equipment)
- 65 WATK (33 from weapon + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 2 from cape + 10 from gloves)
- 10% mastery (the lowest possible mastery)
- 100% dmg multi (basic attack)
- 0% crit chance
- 0% crit multi
- speed 3 (all maple claws are speed 3)
- 100 WDEF (the [Bloctopus](https://maplelegends.com/lib/monster?id=3230302) has exactly this much WDEF)

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |       154.9 |   234.7 |
|   **CV** |      15.76% |  12.80% |

### Only maxed [Drain](https://maplelegends.com/lib/skill?id=4101005)

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |       248.1 |   375.9 |
|   **CV** |      15.74% |  12.78% |

### Only maxed [Claw Mastery](https://maplelegends.com/lib/skill?id=4100000)

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |       173.3 |   262.6 |
|   **CV** |       7.94% |   6.45% |

### Only maxed [Critical Throw](https://maplelegends.com/lib/skill?id=4100001)

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |       232.6 |   352.4 |
|   **CV** |      37.30% |  30.30% |

### Only with [Claw Booster](https://maplelegends.com/lib/skill?id=4101003)

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |       154.9 |   258.1 |
|   **CV** |      15.76% |  12.20% |

### All maxed

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |       364.5 |   607.5 |
|   **CV** |      25.20% |  19.52% |

### Only maxed [Lucky Seven](https://maplelegends.com/lib/skill?id=4001344)

This is not a serious entry, but is included here just in case anyone thought that using Lucky Seven was a good idea for STRlords. Unfortunately, the damage formula is just no good for STRlords (STR and DEX are not taken into account at all).

|          | **Per-hit** | **DPS** |
| -------: | ----------: | ------: |
| **Mean** |        95.6 |   144.9 |
|   **CV** |      40.40% |  32.82% |

### Skeletal skill build

So, it seems from the calculations above that a skill build should probably look something like:

1. Max [Haste](https://maplelegends.com/lib/skill?id=4101004)
2. +3 [Endure](https://maplelegends.com/lib/skill?id=4100002) \[3\]
3. Max Drain
4. +3 Claw Mastery \[3\]
5. +1 Critical Throw \[1\]
6. +2 Claw Mastery \[5\]
7. +6 Claw Booster \[6\]
8. Max Critical Throw
9. Max Claw Mastery
10. ???
11. Profit

โฆor something like that.

## Fucking [pugilists](https://oddjobs.codeberg.page/odd-jobs.html#pugilist), how do they work?

The core mechanic of pugilists โ indeed, their _raison dโรชtre_ โ is the fact that pirates (first job and onwards) can attack ๐ซwithout any weapons๐ซ equipped. Naturally, this suggests the question: but how tho? The answer, of course, is punching ๐๐พ. And kicking ๐ฆต๐พ. You know, [unarmed](https://en.wikipedia.org/wiki/Subak) [martial arts](https://en.wikipedia.org/wiki/Taekkyon) [and shit](https://en.wikipedia.org/wiki/Taekwondo). But how _much_ tho?

    maxDmgUnarmed(PSM, STR, DEX, WATK) := (STR * PSM + DEX) * WATK / 100

    minDmgUnarmed(PSM, STR, DEX, WATK) := (STR * PSM * 0.1 * 0.9 + DEX) * WATK / 100

Note that there is no โmasteryโ here; [Knuckler Mastery](https://maplelegends.com/lib/skill?id=5100001) really does only apply when a (proper) knuckler is equipped. Hooray for forever-unstable damage~! โ`PSM`โ stands for โprimary stat multiplierโ, and for unarmed attacks, is 3 for first job pirates, and 4.2 for second (and onwards) job pirates. Yes, thatโs right. It, bizarrely, depends on your job ๐คจ.

First job rangifer:

![First job rangiferโs stats](rangifer-stats-1st-job.png "First job rangiferโs stats")

Second job rangifer:

![Second job rangiferโs stats](rangifer-stats-2nd-job.png "Second job rangiferโs stats")

The only difference between these two images is job advancement. There are no changes in equipment, AP allocation, nor SP allocation between them. Besides the obvious and large difference in damage range, some other quirks are present here:

- Second (and onwards) job brawlers differ from first job pirates in how they calculate their total WACC (`DEX * 0.6 + LUK * 0.3` for pirates, `DEX * 0.9 + LUK * 0.3` for brawlers). So my total WACC gets a considerable boost.
- Second (and onwards) job brawlers also differ from first job pirates in how they calculate their total AVOID, but MapleLegends has made some changes to how this works, and so I honestly have no idea whatโs going on here.

## Second job; prospects

![rangifer hits second job](rangifer-hits-2nd-job.png "rangifer hits second job")

Now that rangifer is a brawler, and level 31, KPQ time has (sadly ๐ข) come to an end. But she has some great things to look forward to:

- The nifty DPS boost from hitting second job ๐ช๐พ.
- Being my first character with somewhat high MAXHP (I do not wash any of my characters), thanks to [Improve MaxHP](https://maplelegends.com/lib/skill?id=5100000).
- Other sweet-ass second job skills, like [Oak Barrel](https://maplelegends.com/lib/skill?id=5101007), [Corkscrew Blow](https://maplelegends.com/lib/skill?id=5101004), and [Backspin Blow](https://maplelegends.com/lib/skill?id=5101002) (brawlers are heaps of fun to play in combat!).
- LPQ! ๐๐๐๐๐

I prefer to quest rather than grind (unless I can party quest, of course, in which case I do that), so the journey from level 31 to level 35 will take place mostly in Nautilus Harbour ๐ข and Ariant ๐๏ธ. I probably wonโt end up finishing the entire first saga of the Ariant questline, because I prefer to start LPQing more-or-less as soon as I hit level 35. So the next entry will probably just be questing, and sometime very soon, you can expect a whole lot of LPQing from rangifer ๐ฆ!
