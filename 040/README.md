# rangifer’s diary: pt. xli

## R>1 _nimble_ warrior for blue mushmom run

Readers of this diary may remember the “R>1 pog ranged for \_\_\_\_\_\_ run” series from diary parts xxiii, xxiv, and xxv. The content of that series is based on a mock model of level ≈100 odd jobbers, and focusses **exclusively** of theoretical single-target DPS figures. As a result, it can be somewhat informative, but also a bit goofy, and missing most of the picture when it comes to actually _playing_ such odd jobs. That being said, it can be a lot of fun to work through, or look at, and yields sometimes quite unexpected results. So, in the spirit of carrying on with this series, and in order to not give the wrong impression about ranged DPS (as the previous series only considered odd jobs capable of dealing truly ranged damage), we are taking a look today at [DEX warriors](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior)! I actually have a dex warrior of my own, named **rusa**, who is a level 95 DEXgon knight at the time of this writing :)

For a closer comparison and for ease of choice, both the crusader and the white knight have the [Stonetooth Sword](https://maplelegends.com/lib/equip?id=01402037) as their weapon of choice. For the sader, this is a pretty likely (albeit expensive) choice, as their axe options are much more limited (but definitely [still exist](https://maplelegends.com/lib/equip?id=01412027)) than their sword options. But it’s worth noting that for the white knight, another snazzy level 100 loadout is possible: the [Duck Tube](https://maplelegends.com/lib/equip?id=01322064), combined with a [nice shield](https://maplelegends.com/lib/equip?id=01092046)! The DEXgon knight will be using a similarly expensive weapon: the [Sky Ski](https://maplelegends.com/lib/equip?id=01432018).

With that said, let’s take a look at our models. As usual, they are all roughly level 100, and have pretty good gear:

### The model DEXsader

- 95 STR (60 of which is from gear)
- 535 DEX (45 of which is from gear)
- 146 WATK (113 from [sword](https://maplelegends.com/lib/equip?id=01402037) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### The model DEX white knight

- 95 STR (60 of which is from gear)
- 535 DEX (45 of which is from gear)
- 146 WATK (113 from [sword](https://maplelegends.com/lib/equip?id=01402037) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### The model DEXgon knight

- 95 STR (60 of which is from gear)
- 535 DEX (45 of which is from gear)
- 139 WATK (106 from [spear](https://maplelegends.com/lib/equip?id=01432018) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves)

### Comparing single-target DPS

As usual, we assume that all player characters and monsters have the same level. And, as in the “R>1 pog ranged for…” series, we will use 600 WDEF to reasonably represent a low- or mid-level boss monster. Additionally, because of the DEX white knight, elemental interactions now matter, so we now assume _by default_ that all monsters are neutral towards all elements; however, I will _also_ list the non-neutral cases separately. The DEXsader is using [Power Strike](https://maplelegends.com/lib/skill?id=1001004) with maxed [Combo Attack](https://maplelegends.com/lib/skill?id=1111002), the DEX white knight is using Power Strike with maxed charge (that is, whichever charge they are using, e.g. [Fire Charge](https://maplelegends.com/lib/skill?id=1211003)), and the DEXgon knight is using maxed [Spear Crusher](https://maplelegends.com/lib/skill?id=1311001):

| model                      |     DPS |
| :------------------------- | ------: |
| DEX WK (fire weak)         | 11856.8 |
| DEXsader                   | 10729.0 |
| DEX WK (lightning weak)    |  9977.1 |
| DEX WK (ice weak)          |  8097.5 |
| DEX WK (fire neutral)      |  7470.9 |
| DEX WK (lightning neutral) |  6217.8 |
| DEXgon knight              |  5054.7 |
| DEX WK (ice neutral)       |  4964.7 |
| DEX WK (fire strong)       |  3085.1 |
| DEX WK (lightning strong)  |  2458.5 |
| DEX WK (ice strong)        |  1832.0 |

So, in this comparison, it’s pretty clear that the DEXgon knight is firmly in last place in the vast majority of realistic situations. Poor rusa. But, on the other hand, DEXsaders and DEX WKs appear to be so ridiculously powerful that being beaten by them in terms of single-target DPS is hard to feel very bad about. The DEX WK using [Fire Charge](https://maplelegends.com/lib/skill?id=1211003) versus a fire-weak monster comes out on top here, and while that is a fairly specific situation, there are even some boss monsters that are fire-weak: [Scarlion](https://maplelegends.com/lib/monster?id=9420549), [Blue Mushmom](https://maplelegends.com/lib/monster?id=9400205), [Male Boss](https://maplelegends.com/lib/monster?id=9400120), [Pianus (both)](https://maplelegends.com/lib/monster?id=8510000), [Bigfoot](https://maplelegends.com/lib/monster?id=9400575), [Blue King Goblin](https://maplelegends.com/lib/monster?id=7130401), [Bodyguard A](https://maplelegends.com/lib/monster?id=9400112), [The Boss](https://maplelegends.com/lib/monster?id=9400300), and even [one of Vergamot’s bodies](https://maplelegends.com/lib/monster?id=9400264)(‽). Otherwise, DEXsader is the most consistently high single-target DPS here, at a whopping 10.7k(!!) DPS.

I had actually speculated earlier (before I did any of these calculations) that DEXgon knights are generally the worst single-target DPS of the three main species (there are other species, like e.g. the DEX [permawarrior](https://oddjobs.codeberg.page/odd-jobs.html#permawarrior)) of DEX warrior; my intuition here was that DKs never gain access to pre-defence damage modifiers. [Combo Attack](https://maplelegends.com/lib/skill?id=1111002) and [WK](https://maplelegends.com/lib/skill?id=1211003)/[pally](https://maplelegends.com/lib/skill?id=1221003) charges grant damage multipliers that are applied _directly to the raw damage range_, even before WDEF is taken into account. This helps to surmount the defence difficulties that DEX warriors have due to their rather low (but quite stable) damage ranges. And, for better or worse, even in fourth job, the DK still has no access to these kinds of multipliers; [Berserk](https://maplelegends.com/lib/skill?id=1320006) is an _after-modifier_ that applies even after attacking-skill/critical damage multipliers are taken into account (level 30 Berserk really does exactly what it says on the tin: it simply doubles your overall damage).

We also, of course, want to compare these figures to the figures calculated in the previous series. For that, I will first eliminate the particularly unlikely cases from above, namely the DEX WK using [Ice Charge](https://maplelegends.com/lib/skill?id=1211005) versus any monster that is not ice-weak, and the DEX WK using any charge versus any monster that is strong against the charge’s element. This gets rid of some of the needlessly low DPS figures for the DEX WK, who would presumably be smart enough to check the elemental weaknesses/strengths of their opponents on [the library](https://maplelegends.com/lib/monster), and who is unlikely to be fighting a monster that is strong/immune against both lightning and fire, but also _not_ strong/immune to ice.

| model                      |     DPS |
| :------------------------- | ------: |
| STRmit (SM)                | 13807.5 |
| DEX WK (fire weak)         | 11856.8 |
| DEXsader                   | 10729.0 |
| DEX WK (lightning weak)    |  9977.1 |
| DEX WK (ice weak)          |  8097.5 |
| Permarogue                 |  7658.0 |
| DEX WK (fire neutral)      |  7470.9 |
| Wood(wo)man (bow)          |  6725.3 |
| Wood(wo)man (xbow)         |  6690.6 |
| Permapirate                |  6669.1 |
| Swashbuckler (yes Octo)    |  6598.6 |
| DEX WK (lightning neutral) |  6217.8 |
| Swashbuckler (no Octo)     |  5614.4 |
| Permarcher                 |  5614.0 |
| DEXgon knight              |  5054.7 |

As we can see here, the DEXsader, as well as the DEX WK when fighting any monster that is weak to their charge’s element, actually beat all of the ranged odd jobs in this comparison… With the exception of STRmit, of course, thanks to [SM](https://maplelegends.com/lib/skill?id=4111004). This suggests that warriors are a very strong choice for odd-jobbers who want to be good at (at least, “good at” by odd job standards) bossing; warriors have useful skills in boss fights, lots of survivability, and some of them ([DEX warriors](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior), in this case) have some pretty darn good damage!

**_IMPORTANT REMINDERS BEFORE ANYONE GOES AROUND TOUTING THESE NUMERIC FIGURES:_** Keep in mind (and I cannot stress this enough) that this is a purely one-dimensional — and somewhat shoddy — analysis using dummy models, _and_ that jobs cannot be reduced to raw single-target DPS numbers. The odd jobs that are listed above differ quite a bit in their playstyles and range of abilities (e.g. we did not even factor [Threaten](https://maplelegends.com/lib/skill?id=1201006) into the analysis). Furthermore, this only considers characters that are roughly level 100.

## GM boofs for woosa~

A GM announced GM buffs while I was playing my [DEXgon knight](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior), **rusa**, so I headed to [the FM](https://maplelegends.com/lib/map?id=910000000) to get the boofs on rusa, and on my [woodwoman](https://oddjobs.codeberg.page/odd-jobs.html#woodsman), **capreolina**. I’ve mostly just been card-hunting/questing on capreolina, but I told myself I’d grind CDs for an hour for EXP any time I had GM boofs to accompany me. So after getting the boofs, I stashed capre away in the cash shop, and headed to CDs with rusa:

![rusa solo CDs @epm test](rusa-solo-cds-epm-test.png "rusa solo CDs @epm test")

I may have chewed up a lot of [Honsters](https://maplelegends.com/lib/use?id=2002021) over the course of this `@epm` test, as I recorded this EPM entirely solo, but wow… 7.26M EPH! And the loot from the CDs can really make up for the Honsters spent :O

But after some solo training, a priest by the name of **Madarin** stumbled across my map and asked if I wanted to party. I of course said yes, eager to save myself some mesos, and Madarin was even able to cast [Maple Warrior](https://maplelegends.com/lib/skill?id=3221000) and [Sharp Eyes](https://maplelegends.com/lib/skill?id=3221002) on the both of us, via another one of their characters idling in the FM!:

![CDs with Madarin](cds-with-madarin.png "CDs with Madarin")

You can see the effect of SE on my damage; there’s 3 critical hits in that image, I think. We had a jolly good time finishing up the GM boofs! Thanks for the party, Madarin!!

And I used the GM boofs on capreolina almost immediately afterwards, which got her 26% EXP or so out of just 60 minutes of grinding~ ^^

## LPQ for hashishi~

I’ve been needing to [LPQ](https://maplelegends.com/lib/map?id=221024500) more on my [besinner](https://oddjobs.codeberg.page/odd-jobs.html#besinner), **hashishi**, so I can unstick her from LPQ and get her to level 50, to catch up with her [DEXginner](https://oddjobs.codeberg.page/odd-jobs.html#dex-beginner) friend **Gambolpuddy** (**Taima**, **Tacgnol**, **Boymoder**, **Numidium**, **Yotsubachan**)! I have set a goal for myself to get two pairs of [Broken Glasses](https://bbb.hidden-street.net/eq/eye-accessory/broken-glasses) on hashishi, as there is no other way to get LUK from eye accessories, so I want to (hopefully) land at least one [30%](https://maplelegends.com/lib/use?id=2040203) (which gives DEX, thus also contributing to her damage) on one of the pairs. This is, obviously, a bit of a micro-optimisation, considering that anything that isn’t a WATK increase is fairly minor, and considering that I can’t play hashishi super seriously anyways (just imagine card-hunting on a besinner… _ouch_). But I like LPQ, okay!?!

I was able to do a few LPQs with **maebee** (**drainer**, **xX17Xx**, **strainer**), a [woodwoman](https://oddjobs.codeberg.page/odd-jobs.html#woodsman) of **Oddjobs**!:

![hashishi and maebee vs. Alishar](hashishi-and-maebee-vs-alishar.png "hashishi and maebee vs. Alishar")

And later, some LPQs with my pals in **Pals**!:

![Pals vs. Alishar](pals-vs-alishar.png "Pals vs. Alishar")

Our party was not too hot, though… I was by far the highest level (at 46), and everyone else was either [washing](https://maplelegends.com/lib/cash?id=5050000) or quite new to the game :X

So one [Alishar](https://maplelegends.com/lib/monster?id=9300012) fight didn’t turn out too well…:

![Terrible Alishar fight](terrible-alishar-fight.png "Terrible Alishar fight")

Now, believe me when I say that I had all of my protective gear on, by which I mean my 71 MAXHP [Newspaper Hat](https://maplelegends.com/lib/equip?id=01002418). But when you’re a besinner in a situation where the floor is so crowded with various Chronos species that you’re the only one really doing much damage (desperately and slowly finishing off Alishar), and Alishar deals enough damage to nearly (but not quite) one-shot you, and you’re just trying to get in the right spots so that your fake-ranged attacks hit the damn thing, it’s easy to make a slight HP-potion-using slip-up >.<

After that PQ, though, I decided that it was worth simply multi-clienting (considering that our party size dropped below 6) and bringing along my OPQ-mule-to-be, **sets**, to pick up the slack. After that, it was more arduous for me personally (as I am not the world’s foremost expert at multi-clienting and actually _playing_ both characters), but the runs were a lot quicker and more painless overall. hashishi is now a healthy bit closer to that second pair of glasses, and I even [EPQ](https://maplelegends.com/lib/map?id=300030100)ed a bit on sets afterward, with **GishGallop** (**Cortical**, **dendrite**, **Cerebellum**, **MageFP**, **Medulla**, **WizetWizard**), an I/L [gish](https://oddjobs.codeberg.page/odd-jobs.html#gish) of **Oddjobs**. That was enough to get sets to 51, so now she can finally be a real OPQ mule… LOL

## capreolina is still on the hunt

You know what time it is: card huntin’ time! My [woodwoman](https://oddjobs.codeberg.page/odd-jobs.html#woodsman), **capreolina**, has been my main focus for card-hunting these days, and so I headed to the [Hidden Tower](https://maplelegends.com/lib/map?id=221020701) to start where I left off in the Eos Tower:

<details>
<summary>Cards. Cards. Cards.</summary>

![Block Golem card get!](capre-block-golem-card-get.png "Block Golem card get!")

![King Block Golem card get!](capre-king-block-golem-card-get.png "King Block Golem card get!")

![Rombot card get!](capre-rombot-card-get.png "Rombot card get!")

I hunted for [Block Golem](https://maplelegends.com/lib/monster?id=4230109) and [King Block Golem](https://maplelegends.com/lib/monster?id=4230110) cards in the Hidden Tower, which is also the only map where [Rombot](https://maplelegends.com/lib/monster?id=4130103) spawns. There are only two spawn points in the entire giant map, and they only respawn every 20 minutes. The card drop rate is kind of well-known to be crappy, and I found that out for myself after grinding there for quite a while and still being 0/5 Rombot cards.

**Cortical** challenged me to finish the set, and I really wasn’t going to… except that I had terrible luck with the King Block Golem cards, so by the time I got 5/5 of those, I had already been 4/5 Rombot cards! So I set out to just finish the damn set… and it did take a while, I must say >.<

But I eventually did finish it! Take that!!:

![Rombot card get! F555555](capre-rombot-card-get-f5.png "Rombot card get! F555555")

Finally, I could leave that god forsaken block golem tower and finish up the Eos tower by doing the [Tweeter](https://maplelegends.com/lib/monster?id=3230308) card set:

![Tweeter card get!](capre-tweeter-card-get.png "Tweeter card get!")

And so off I was to the [Ludibrium Clocktower](https://maplelegends.com/lib/map?id=220010500) to finish up the card sets from the terrace, toy factory, and time (but not deep Ludi) zones (I already had some of the card sets, like [Brown Teddy](https://maplelegends.com/lib/monster?id=3000005) and [Toy Trojan](https://maplelegends.com/lib/monster?id=3230305)):

![Roloduck card get!](capre-roloduck-card-get.png "Roloduck card get!")

![Panda Teddy card get!](capre-panda-teddy-card-get.png "Panda Teddy card get!")

![Robo card get!](capre-robo-card-get.png "Robo card get!")

![Master Robo card get!](capre-master-robo-card-get.png "Master Robo card get!")

![Tick card get!](capre-tick-card-get.png "Tick card get!")

![Chronos card get!](capre-chronos-card-get.png "Chronos card get!")

![Platoon Chronos card get!](capre-platoon-chronos-card-get.png "Platoon Chronos card get!")

![Master Chronos card get!](capre-master-chronos-card-get.png "Master Chronos card get!")

And then I went off to do the Helios Tower, which is pretty easy; it’s just one card set!:

![Retz card get!](capre-retz-card-get.png "Retz card get!")

And so I headed down the elevator towards [KFT](https://maplelegends.com/lib/map?id=222000000)/[Aqua Road](https://maplelegends.com/lib/map?id=230000000):

![Pinboom card get!](capre-pinboom-card-get.png "Pinboom card get!")

(I found the [Pinboom](https://maplelegends.com/lib/monster?id=2230108) card just as I was mucking about in the area; I’ve yet to actually finish the set.)

</details>

## Noam’s level 100 party!

**OmokTeacher** (**Slime**, **JumpQuest**, **Slimu**) planned an elaborate level 100 party for their [STRginner](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner)! I was glad to be able to attend:

![Noam’s level 100 party](noam-s-100-party.png "Noam’s level 100 party")

And to see the levelup itself!! Unfortunately, I had some technical issues duing the minigames that ensued, so I only got to attend part of the party. But congrats again on 100~!!!

## capreolina is a legend of hometown

In planning to start card-hunting in [KFT](https://maplelegends.com/lib/map?id=222000000), I stumbled across a quest that I was previously unaware of: “[Legends of Hometown](https://bbb.hidden-street.net/quest/ludus-lake/legends-of-hometown)”. This quest belongs to the same set of quests that I’m already fond of: the area boss quests, like [that of Eliza](https://bbb.hidden-street.net/quest/el-nath-mt-aquaroad/goddess-pet), [that of Snowman](https://bbb.hidden-street.net/quest/el-nath-mt-aquaroad/snowfield-giant), [that of Capt. Latanica](https://bbb.hidden-street.net/quest/singapore/the-secret-of-ghostship), etc. For some reason, I just really like the setup of these quests: some pretty normal questing leading up to the climax of a boss fight, with a gob of EXP and fame at the end to serve as an actual reward (quite refreshing, as many quests in this game tend to lack a real reward…). Think of it as a solo PQ, maybe. I love PQs! :P

So, naturally enough, even though capre would be very overleveled (being level 112) for the quest at this point, I set out to do the quest anyways (and get some cards along the way):

![Hodori card get!](capre-hodori-card-get.png "Hodori card get!")

![Samiho card get!](capre-samiho-card-get.png "Samiho card get!")

Unfortunately, the gotcha of these quests is that area bosses tend to be virtually nonexistent in MapleLegends. Even just dropping [a card](https://maplelegends.com/lib/use?id=2388009) is enough to make an area boss get hunted to extinction 24/7. So I was, pretty quickly, at the point that I dread: waiting, for up to three hours, for an [Old Fox](https://maplelegends.com/lib/monster?id=7220001) to actually show up. But, I figured I would hunt the [Samiho](https://maplelegends.com/lib/monster?id=5100004) cards in the meantime.

As I was waiting, I met **Guitarmaster**, a level 70 outlaw who was also looking to do the same quest! So we partied up and chatted while we got 5/5 Samiho cards each, and then simply waited for an Old Fox to show up on one of the channels. Guitarmaster was, thankfully, more dilligent with periodically channel-surfing to look for new spawns (I tend to just pick a channel and squat there, for fear of losing mapowner and losing any and all Old Foxes to someone who has all the timers), and eventually found one in another channel after an hour or two. And, conveniently, the one Old Fox that we killed dropped enough [tails](https://maplelegends.com/lib/etc?id=4031793) for both of us to complete the quest~!
