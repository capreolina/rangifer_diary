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

Speaking of magelets, our magelet models will be equipped very similarly to our permamagician model, with two caveats. One caveat is that the exact element of the magelet’s weapon matters, as we assume that none of our magelets are permamagicians, so they will all have access to elemental spells. The other caveat is that, because we assume all models to be “roughly level 100”, this brings Elemental _Staves_ into consideration. Elemental Staves are only equippable by those on the higher end of “roughly level 100”, because they require the mage to be at least level 103. So, in order to get numbers somewhere in between using an Elemental Wand and an Elemental Staff, we will be giving the magelet a TMA advantage on their gear equal to half of the difference between a wand and a staff. Besides the gearing considerations, we will have three different magelet models: one for each of the third job mage classes (F/P, I/L, priest).

From here, it starts to get considerably hairier. Consider the gishlet (a.k.a. LUK gish), who is equally specialised in melee combat and magical combat. Right off the bat, we will assume a “normal” gishlet build, which is DEXless; gishlets are INTless by definition, but not _necessarily_ DEXless. This still leaves the issue of how much AP is going into STR vs. going into LUK, and the issue of what equipment our gishlet is using. To solve these problems straightforwardly and simply, we will first assume that our gishlet model wants to be capable of successfully hitting a 35 AVOID monster 100% of the time, given that the monster is at or below their level. 35 AVOID is chosen because, while this _is_ somewhat lower than the AVOID of some monsters that the gishlet may want to train on (for example, [Latest Hits Compilations](https://maplelegends.com/lib/monster?id=9410030) have 35 AVOID, but [Greatest Oldies](https://maplelegends.com/lib/monster?id=9410031) have 40, so they will “**MISS**” the Greatest Oldies sometimes), it’s a reasonable bar to set for the gishlet while still retaining solid melee damage. Mixed attackers (attackers who deal both physical and magical damage) are generally limited by their MACC rather than their WACC, so we will just look at how much INT+LUK the gishlet needs to achieve this MACC goal, and assume that they will end up with enough WACC along the way (remember that LUK contributes to both MACC and WACC):

> effectiveMacc := (⌊INT ÷ 10⌋ + ⌊LUK ÷ 10⌋) ÷ (AVOID + 1) ⋅ (1 + 0.0415 ⋅ δ)

Above, we have the formula for calculating effective MACC, due to **Technolink**’s post, which was probably posted around late 2007/early 2008, and reposted by **Russt**, then reposted on **AyumiLove**’s blog [here](https://ayumilovemaple.wordpress.com/2009/09/06/maplestory-formula-compilation/). Technolink attributes this formula to **Thikket** and **Nekonecat**. The lowercase delta (“δ”) represents the difference in level between the monster and the player-character, to a minimum of zero. Because we assume the monster to be at or below the gishlet’s level, δ = 0, so we can simplify:

> effectiveMacc := (⌊INT ÷ 10⌋ + ⌊LUK ÷ 10⌋) ÷ (AVOID + 1)

And we want an AVOID of 35, so:

> effectiveMacc := (⌊INT ÷ 10⌋ + ⌊LUK ÷ 10⌋) ÷ 36

And we will treat ⌊INT ÷ 10⌋ + ⌊LUK ÷ 10⌋ as its own quantity, called intluk:

> effectiveMacc := intluk ÷ 36

And, Thikket & Nekonecat’s work defines the probability of hitting using a formula derived from [quadratic regression](https://en.wikipedia.org/wiki/Polynomial_regression):

> P(magicHit) = −2.5795 ⋅ effectiveMacc² + 5.2343 ⋅ effectiveMacc − 1.6749

Which unfortunately (or, fortunately, if you’re into it) means that we have to do some non-trivial algebra. Thankfully, the [quadratic formula](https://en.wikipedia.org/wiki/Quadratic_formula) will be doing almost all of the work here:

> 1 = −2.5795 ⋅ effectiveMacc² + 5.2343 ⋅ effectiveMacc − 1.6749
>
> 0 = −2.5795 ⋅ effectiveMacc² + 5.2343 ⋅ effectiveMacc − 2.6749

At which point, we can plug this into the quadratic formula: a = −2.5795, b = 5.2343, c = −2.6749.

<!--

So, this leaves us with the following four:

- [Wandginner](https://oddjobs.codeberg.page/odd-jobs.html#wand-beginner)
- [Generic claw-wielding non-rogue](https://oddjobs.codeberg.page/odd-jobs.html#besinner)
- [Generic STR whacker](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner)
- [Generic DEX whacker](https://oddjobs.codeberg.page/odd-jobs.html#dex-beginner)

Equipping our wandginner will look very similar to equipping the [wand warrior](https://oddjobs.codeberg.page/odd-jobs.html#wand-warrior) in the previous installation, with the minor exception that I’ll be taking 3 STR off of the shield to reflect the disadvantage that the wandginner has due to only being able to equip shields that lack class requirements. Equipping the generic claw-wielding non-rogue will also be pretty simple, as we can just assume (like we did for the ranged [permarogue](https://oddjobs.codeberg.page/odd-jobs.html#permarogue)) that they are throwing [Ilbis](https://maplelegends.com/lib/use?id=2070006).

This brings us to the generic STR and DEX whackers. Picking a weapon here is inevitably bound to be controversial, as there are various weapons considered to be “endgame-worthy” for STRginners and their ilk: the [Crimson Arcglaive](https://maplelegends.com/lib/equip?id=01442068), the [Purple Surfboard (PSB)](https://maplelegends.com/lib/equip?id=01442057), the [Fan](https://maplelegends.com/lib/equip?id=01332030), and even the [Toy of 101](https://maplelegends.com/lib/equip?id=01402038). Which weapon(s) pans out to be “the best” is _very_ sensitive to the circumstances and assumptions that surround the choice of weapon. That being said, I want to immediately throw the Crimson Arcglaive out due to its extremely steep STR requirement of 300, which renders it unusable for DEX whackers. It’s easier and a bit more fair of a comparison if we can use the same weapon for both our generic STR and DEX whackers. Our other options lack significant stat requirements, with the slight exception of the Fan, but it requires just ≥31 STR from equipment for DEX whackers, which is pretty doable. That being said, choosing the Fan means pairing it with a shield, so we are going to throw out the Fan to keep things simpler. This leaves just the PSB and the Toy of 101; the Toy of 101 has its advantages, but because of the methods used in this analysis, we can nearly guarantee that the PSB comes out ahead of the Toy of 101, so we will be optimistic and pick the PSB.

With all that said, let’s take a look at our models. As usual, they are all roughly level 100, and have pretty good gear:

### The model wandginner

- 547 STR (42 of which is from gear)
- 80 DEX (60 of which is from gear)
- 91 WATK (45 from [wand](https://maplelegends.com/lib/equip?id=01372033) + 13 from [shield](https://maplelegends.com/lib/equip?id=01092028) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### The model generic claw-wielding non-rogue

- 581 LUK (60 of which is from gear)
- 34 STR (30 of which is from gear)
- 34 DEX (30 of which is from gear)
- 70 WATK (10 from [claw](https://maplelegends.com/lib/equip?id=01472088) + 27 from [stars](https://maplelegends.com/lib/use?id=2070006) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### The model generic STR whacker

- 550 STR (45 of which is from gear)
- 80 DEX (60 of which is from gear)
- 144 WATK (111 from [polearm](https://maplelegends.com/lib/equip?id=01442057) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### The model generic DEX whacker

- 566 DEX (45 of which is from gear)
- 64 STR (60 of which is from gear)
- 144 WATK (111 from [polearm](https://maplelegends.com/lib/equip?id=01442057) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### Comparing single-target DPS

As usual, we assume that all player characters and monsters have the same level. And, as in the “R>1 pog ranged for…” series, we will use 600 WDEF to reasonably represent a low- or mid-level boss monster. All models are, of course, limited to only basic-attacking.

| model                           |    DPS |
| :------------------------------ | -----: |
| Generic STR whacker             | 2227.8 |
| Wandginner                      | 1308.7 |
| Generic DEX whacker             |  966.0 |
| Generic claw-wielding non-rogue |  841.5 |

As expected, our generic STR whacker comes out on top here. What might come as somewhat of a surprise to some people is the fact that the wandginner compares very favourably to the other non-STR-based entries here, and the fact that the generic claw-wielding non-rogue is firmly in last place, despite being truly pure LUK (base stats of 4/4/4/`x`). While our wandginner obviously lacks WATK due to being restricted to wands/staves, they can still put as much AP into STR as they want (as long as they still have enough WACC to hit consistently), and benefit from the guaranteed PSM (primary stat multiplier, the primary stat being STR in this case) of 4.4 that wands/staves give. And while our generic claw-wielding non-rogue does have a metric shitton of LUK (even more than an ordinary rogue of the same level), they are _extremely_ starved for WATK due to getting exactly 10 WATK from their weapon (no matter how good their gear is), and they are hurting in the PSM department: claws have a PSM of just 3.6.

We also, of course, want to compare these figures to the figures calculated in the previous series, and in previous installations of this series. I’ve added some class-based (class as in: beginner, warrior, mage, archer, rogue, pirate) emojis to the “model” column to add some readability to this now quite lengthy table (N.B. some or none of these emojis may show up if you are viewing this on the MapleLegends forums, for some reason(‽)):

| model                              |     DPS |
| :--------------------------------- | ------: |
| ⚔️ dagger warrior                  | 14828.4 |
| 🥷 STRmit (SM)                      | 13807.5 |
| ⚔️ DEX WK (fire weak)              | 11856.8 |
| ⚔️ DEXsader                        | 10729.0 |
| ⚔️ DEX WK (lightning weak)         |  9977.1 |
| ⚔️ wand warrior                    |  9446.7 |
| ⚔️ DEX WK (ice weak)               |  8097.5 |
| 🥷 Permarogue                       |  7658.0 |
| ⚔️ DEX WK (fire neutral)           |  7470.9 |
| 🏹 Wood(wo)man (bow)               |  6725.3 |
| 🏹 Wood(wo)man (xbow)              |  6690.6 |
| 🏴‍☠️ Permapirate                     |  6669.1 |
| 🏴‍☠️ Swashbuckler (yes Octo)         |  6598.6 |
| ⚔️ DEX WK (lightning neutral)      |  6217.8 |
| ⚔️ permawarrior                    |  5934.0 |
| 🏴‍☠️ Swashbuckler (no Octo)          |  5614.4 |
| 🏹 Permarcher                      |  5614.0 |
| ⚔️ DEXgon knight                   |  5054.7 |
| ⚔️ LUK WK (fire weak)              |  4353.5 |
| ⚔️ LUKsader                        |  3868.9 |
| ⚔️ LUK WK (lightning weak)         |  3545.8 |
| ⚔️ LUK WK (ice weak)               |  2738.0 |
| ⚔️ LUK WK (fire neutral)           |  2468.8 |
| 🔰 Generic STR whacker             |  2227.8 |
| ⚔️ LUK WK (lightning neutral)      |  1930.3 |
| ⚔️ LUK DK                          |  1350.2 |
| 🔰 Wandginner                      |  1308.7 |
| 🔰 Generic DEX whacker             |   966.0 |
| 🔰 Generic claw-wielding non-rogue |   841.5 |

As expected, our beginners and beginner-a-likes rank pretty low in the ranking that we have so far. But, the generic STR whacker is able to beat certain [LUK warriors](https://oddjobs.codeberg.page/odd-jobs.html#luk-warrior) in certain situations.

**_IMPORTANT REMINDERS BEFORE ANYONE GOES AROUND TOUTING THESE NUMERIC FIGURES:_** Keep in mind (and I cannot stress this enough) that this is a purely one-dimensional — and somewhat shoddy — analysis using dummy models, _and_ that jobs cannot be reduced to raw single-target DPS numbers. The odd jobs that are listed above differ quite a bit in their playstyles and range of abilities. Furthermore, this only considers characters that are roughly level 100.

-->
