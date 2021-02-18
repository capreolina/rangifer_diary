# rangifer’s diary: pt. xxv

## R>1 pog ranged for snowman run

Oh, you thought I was done with this series? No, no, no, we cannot stop until all ranged odd-jobbers have been covered!

In the diary entry before this one, we took a look at [permarogues](https://oddjobs.codeberg.page/odd-jobs.html#permarogue) and [permarchers](https://oddjobs.codeberg.page/odd-jobs.html#permarcher), comparing odd jobs based solely on idealised single-target ranged DPS numbers at roughly level 100. Before that, we looked at [wood(wo)men](https://oddjobs.codeberg.page/odd-jobs.html#woodsman) and compared bow-users (rangers, because the focus is on third job) to crossbow-users (snipers) when it comes to ranged DPS — although wood(wo)men are, of course, also adept at melee, like their [STR mage](https://oddjobs.codeberg.page/odd-jobs.html#str-mage) and [permabeginner](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner) cousins. And in a much older diary entry (pt. iv), we even took a look at how [STRmits](https://oddjobs.codeberg.page/odd-jobs.html#str-assassin) stack up in terms of ranged DPS. In this entry, I want to take a look at (and compare) _two more_ odd jobs who are proficient in ranged DPS as well: the [permapirate](https://oddjobs.codeberg.page/odd-jobs.html#permapirate) and the [swashbuckler](https://oddjobs.codeberg.page/odd-jobs.html#swashbuckler).

Permapirates and swashbucklers can look somewhat similar; they are both pirates, and both switch between melee and ranged combat. Actually, two other closely related pirate odd jobs, the [DEX brawler](https://oddjobs.codeberg.page/odd-jobs.html#dex-brawler) and [bullet bucc](https://oddjobs.codeberg.page/odd-jobs.html#bullet-bucc), are worth mentioning here. These DEX-focussed buccaneers are more than capable of using guns, but for our purposes here, we don’t care about the differences between permapirates, DEX brawlers, and bullet buccs. Our focus is very narrow: just raw ranged DPS. Only DEX bucc is really at a disadvantage in this case, because they can’t put AP into STR in order to equip higher-level guns.

A quirk of permapirates is that, like permarogues, they are proficient in both [melee](https://maplelegends.com/lib/skill?id=4001334) ([Somersault Kick](https://maplelegends.com/lib/skill?id=5001002), [Flash Fist](https://maplelegends.com/lib/skill?id=5001001)) and [ranged](https://maplelegends.com/lib/skill?id=4001344) ([Double Shot](https://maplelegends.com/lib/skill?id=5001003)) combat; yet, the permarogue has the same primary stat for both modes of combat (LUK), while the permapirate is pulled between _two_ primary stats, STR (for melee) and DEX (for ranged). Here, we will assume that the permapirate specialises in gun-using, and thus favours DEX over STR. Swashbucklers, on the other hand, have no choice; they are pure STR by definition, and so their ranged damage is largely built on their STR (and to a lesser extent, whatever DEX they may have). This comparison may be somewhat unfair to swashbucklers, if only because third job doesn’t improve their raw ranged DPS all that much. [Burst Fire](https://maplelegends.com/lib/skill?id=5210000) is a bit underwhelming — it’s actually a _downgrade_ at essentially any skill level below 11, and it maxes out at level 20 — but they do at least have their [Octopus](https://maplelegends.com/lib/skill?id=5211001) by their side to help them out a bit. We’ll take the Octopus into account here, but will separate it out into a no-Octopus swashbuckler and yes-Octopus swashbuckler, if only to see how much difference the lil guy makes.

With that said, let’s take a look at our models. As usual, they are all roughly level 100, and have pretty good gear:

### The model permapirate

- 530 DEX (80 of which is from gear)
- 100 STR (25 of which is from gear)
- 137 WATK (86 from [gun](https://maplelegends.com/lib/equip?id=01492012) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves + 18 from [bullets](https://maplelegends.com/lib/use?id=2330004))

### The model swashbuckler

- 530 STR (25 of which is from gear)
- 105 DEX (85 of which is from gear; 85 because the [Maple Canon Shooter](https://maplelegends.com/lib/equip?id=01492022) has an average of 5 more DEX than the [Concerto](https://maplelegends.com/lib/equip?id=01492012))
- 123 WATK (72 from [gun](https://maplelegends.com/lib/equip?id=01492022) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves + 18 from [bullets](https://maplelegends.com/lib/use?id=2330004))

### Comparing ranged DPS

As usual, we assume that all player characters and monsters have the same level. And as in the previous diary entry, we will use 600 WDEF to reasonably represent a low- or mid-level boss monster. The permapirate is using [Double Shot](https://maplelegends.com/lib/skill?id=5001003), and the swashbuckler is using [Burst Fire](https://maplelegends.com/lib/skill?id=5210000).

We also want to consider the swashbuckler’s use of [Octopus](https://maplelegends.com/lib/skill?id=5211001), so we will calculate its DPS separately: 984.2.

| model                                                                    |     DPS |
| :----------------------------------------------------------------------- | ------: |
| STRmit ([SM](https://maplelegends.com/lib/skill?id=4111004))             | 13807.5 |
| Permarogue                                                               |  7658.0 |
| Wood(wo)man (bow)                                                        |  6725.3 |
| Wood(wo)man (xbow)                                                       |  6690.6 |
| Permapirate                                                              |  6669.1 |
| Swashbuckler (yes [Octo](https://maplelegends.com/lib/skill?id=5211001)) |  6598.6 |
| Swashbuckler (no [Octo](https://maplelegends.com/lib/skill?id=5211001))  |  5614.4 |
| Permarcher                                                               |  5614.0 |

As you can see, without the Octopus, the swashbuckler is essentially tied for last with the permarcher. Taking into account the Octopus’s damage places the swashbuckler alongside the rest of the, if you will, middle bracket: swashbuckler (yes Octo), permapirate, wood(wo)man (xbow), and wood(wo)man (bow) are all roughly around the same DPS here. The swashbuckler (no Octo) and permarcher sit below this bracket, the permarogue sits above, and the STRmit ([SM](https://maplelegends.com/lib/skill?id=4111004)) completely obliterates everything else.

**_IMPORTANT REMINDERS BEFORE ANYONE GOES AROUND TOUTING THESE NUMERIC FIGURES:_** Keep in mind (and I cannot stress this enough) that this is a purely one-dimensional — and somewhat shoddy — analysis using dummy models, _and_ that jobs cannot be reduced to raw ranged DPS numbers. The odd jobs that are listed above differ wildly in their playstyles and range of abilities. Furthermore, this only considers characters that are roughly level 100.

## EPQ with Dizz

My [DEX spearwoman](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior), **rusa**, did some dozen or so [EPQ](https://maplelegends.com/lib/map?id=300030100)s with ally (member of the **Suboptimal** alliance) and [permabeginner](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner) **Dizz**. We were both in search of the [Glittering Altaire Earrings](https://maplelegends.com/lib/equip?id=01032061), and because Dizz had already done quite a number of EPQs, he was just able to get his pair by the time we finished! Congrats! That’s a total of 50 EPQs… whew.

Playing a [DEX warrior](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior) and having ungodly quantities of WACC (second only to [DEX brawlers](https://oddjobs.codeberg.page/odd-jobs.html#dex-brawler), who gain a whopping 0.9 WACC, instead of 0.8 WACC, per point of DEX) is pretty fun. You can fight all kinds of silly stuff when you’re well under the appropriate level, and so I’ve been recording rusa soloing the PQ bosses that she’s fought so far. Here she is, soloing the [Big Bad Rock Guy](https://maplelegends.com/lib/monster?id=9300178) (BBRG/BBRM):

![rusa vs. BBRG](rusa-vs-bbrg.png "rusa vs. BBRG")

## rusa gets her Ellin Savior’s Ring

Once everyone was done EPQing, I decided that it was a good time for rusa to get her [Ellin Savior’s Ring](https://maplelegends.com/lib/equip?id=01119001). In honesty, this was the first time that I actually did the questline at an appropriate level (I usually procrastinate until roughly level 70), but rusa made quick work of it:

<details>
<summary>rusa gettin’ that Ellin Ring~</summary>

![rusa getting Hardened Pieces of Steel](rusa-getting-steel-pieces.png "rusa getting Hardened Pieces of Steel")

![Dark Stone Golem card get!](rusa-dark-stone-golem-card-get.png "Dark Stone Golem card get!")

![rusa vs. Wild Kargo](rusa-vs-wild-kargo.png "rusa vs. Wild Kargo")

![rusa hunts for Lazy Buffy Marbles](rusa-hunts-lazy-buffy-marbles.png "rusa hunts for Lazy Buffy Marbles")

![rusa at the spa](rusa-at-the-spa.png "rusa at the spa")

![rusa vs. tree thingies](rusa-vs-tree-things.png "rusa vs. tree thingies")

![rusa vs. giga shrooms](rusa-vs-giga-shrooms.png "rusa vs. giga shrooms")

![rusa vs. stone buggos](rusa-vs-stone-buggos.png "rusa vs. stone buggos")

![Stone Bug card get!](rusa-stone-bug-card-get.png "Stone Bug card get!")

![rusa vs. Primitive Boar](rusa-vs-primitive-boar.png "rusa vs. Primitive Boar")

![Primitive Boar card get!](rusa-primitive-boar-card-get.png "Primitive Boar card get!")

</details>

Ta-da!!! Ring acquired~

![rusa gets her Ellin Ring~!](rusa-gets-her-ellin-ring.png "rusa gets her Ellin Ring~!")

rusa now has a bit over 5.1k(!) MAXHP with [HB](https://maplelegends.com/lib/skill?id=1301007), making her my first-ever character that I’ll be using [Honsters](https://maplelegends.com/lib/use?id=2002021) on! :O

## KPQing with Brains, the DEXdit

Dizz decided to join **Oddjobs** with another odd-jobbed character of his own: **Brains**, the [DEXdit](https://oddjobs.codeberg.page/odd-jobs.html#lukless-bandit). I’ve actually just added DEXdit (a.k.a. LUKless dit, unfortundit) to [the list of odd jobs on the Oddjobs website](https://oddjobs.codeberg.page/odd-jobs.html), and one of their defining features (and one of the things that Brains is excited about) is their _extremely_ stable damage. Being LUKless makes their minimum and maximum damages roughly equal, and with a skill like [Savage Blow](https://maplelegends.com/lib/skill?id=4201005) that has so many damage lines (6 at max level), they tend to deal just about the same damage on every attack. DEX is, for obvious reasons, the main stat of DEXdits (hence the name “DEXdit”), but they are really only constrained to be LUKless, so some STR may be added as well, in order to equip daggers like e.g. the [Dragon Kanzir](https://maplelegends.com/lib/equip?id=01332049) and some shields like e.g. the [Dragon Khanjar](https://maplelegends.com/lib/equip?id=01092049). DEXdits who are truly pure DEX (4 base STR, in addition to 4 base LUK) are considered “pure DEXdits”, which is its own subspecies, if you will.

When Brains hit level 21, I asked if he wanted to [KPQ](https://maplelegends.com/lib/map?id=103000800), to which he said yes. I could have KPQed on my [besinner](https://oddjobs.codeberg.page/odd-jobs.html#besinner), **hashishi**, but I didn’t want to outlevel her [DEXginner](https://oddjobs.codeberg.page/odd-jobs.html#dex-beginner) friend **Gambolpuddy**-chan, so I took my ordinary F/P mage [MPQ](https://maplelegends.com/lib/map?id=261000021)-mule-to-be, **potpan**, instead:

![KPQing with Brains](kpqing-with-brains.png "KPQing with Brains")

Brains was wearing some equipment borrowed from his permabeginner Dizz, including a WATK [Stolen Fence](https://maplelegends.com/lib/equip?id=01092003), which nearly fooled our party members into thinking he was an ordinary bandit:

![Brains is strong](brains-is-strong.png "Brains is strong")

And potpan hit level 31 in the first stage of one of our KPQs! Here she is, [Fire Arrow](https://maplelegends.com/lib/skill?id=2101004)ing a [Ligator](https://maplelegends.com/lib/monster?id=9300001) to death:

![Level 31 potpan in KPQ](potpan-level-31-in-kpq.png "Level 31 potpan in KPQ")
