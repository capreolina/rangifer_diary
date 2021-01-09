# rangifer’s diary: pt. iii

Today I logged on with rangifer, at level 31, and saw that **Boymoder** (Kelsey) had already gotten level 35! And she even did what she called “the slow 🐌 way”: Ludibrium quests & grinding. So it was my duty to catch up those four levels so that we could LPQ 🛑🐀🐙👀🐋 together!!

## Questing in the KPQ-LPQ interim

I decided to finish up just a few (really just a few; I left many behind) of the Victoria Island-based quests… basically, the middling-level [Nautilus Harbour 🚢](https://maplelegends.com/lib/map?id=120000000) quests. This includes some nice quests like [Bartol’s Request](https://bbb.hidden-street.net/quest/victoria-island/bartols-request), [Strange Dish 1](https://bbb.hidden-street.net/quest/victoria-island/strange-dish), [Lazy Little Calico](https://bbb.hidden-street.net/quest/victoria-island/lazy-little-calico), and [For Kyrin](https://bbb.hidden-street.net/quest/victoria-island/for-kyrin) (the dress 👗 is not exactly my style, but I’m sure [Kyrin](https://maplelegends.com/lib/npc?id=1090000) rocks it!). Then, it was off to [Ariant 🏜️](https://maplelegends.com/lib/map?id=260000000) to do the first saga of the Ariant questline!

The Ariant quests proved to be a breeze, with the help of [**Nise**’s “Quick/Precise Levelling for New Chars!” guide](https://forum.maplelegends.com/index.php?threads/guide-quick-precise-levelling-for-new-chars-lvl-35.17122/). By the time that I hit level 34 or so, Boymoder logged on and was raring for some LPQs! She was having some difficulty getting a full party together, so the pressure was _really_ on for me to hit level 35 ASAP 😅. Speedrunning 🏃🏾‍♀️ the last few quests in the questline (at least, within the first saga, which is all that I planned on doing anyways) was just enough to get me to level 35, so I was soon [VIP Teleport Rock](https://maplelegends.com/lib/cash?id=5041000)ing to Boymoder so that we could get an LPQ gang assembled.

## rangifer LPQs for the first time

During our first LPQ, **Outside** (known on the MapleLegends forums as [**braidgame**](https://forum.maplelegends.com/index.php?members/braidgame.3082/)) logged on and saw that Boymoder and I were both in LPQ level range. Outside is in the **Flow** guild, but also has characters in **Oddjobs**, including a level 35 F/P [gishlet](https://oddjobs.codeberg.page/odd-jobs.html#luk-gish) by the name of **TestChars**! Unfortunately, they had to log off before a spot opened up 🙁 — it wasn’t until we finished that LPQ and the one after it that our party size dropped below six. But we were able to get in quite a few LPQs (enough to get me from roughly the bottom of level 35 to the bottom of level 38), and we did so with a number of very nice people 💖, including [one who was live-streaming their LPQ escapades on Twitch 📺](https://www.twitch.tv/sparkgy37)!

![rangifer gleefully does the LPQ thief portal](rangifer-does-thief-portal.png "rangifer gleefully does the LPQ thief portal")

Pictured: rangifer gleefully does the LPQ thief portal.

![The gang battles LPQ rombots](rangifer-battles-lpq-rombots.png "The gang battles LPQ rombots")

Pictured: The gang battles LPQ rombots.

![The gang battles Alishar as Kelsey makes gleeful weeb noises](rangifer-battles-alishar.png "The gang battles Alishar as Kelsey makes gleeful weeb noises")

Pictured: The gang battles Alishar as Kelsey makes gleeful weeb 🗾 noises.

As we were preparing for another run, the MapleLegends servers took a collective nosedive 💥☠️, so that marked the conclusion of our first string of LPQ runs.

## Pugilist math pt. ii: electric boogaloo

If you'll recall from the previous diary entry, the formulae for unarmed melee damage look like:

>     maxDmgUnarmed(PSM, STR, DEX, WATK) := (STR * PSM + DEX) * WATK / 100
>
>     minDmgUnarmed(PSM, STR, DEX, WATK) := (STR * PSM * 0.1 * 0.9 + DEX) * WATK / 100

The “`WATK`” in the formulae above is calculated as you would expect: the sum of the WATK from your equipment items, plus any extra WATK from skills (not applicable here), plus any extra WATK from buffs. Usually, the bulk of the WATK from equipment is to be found on the weapon… but we’re not using a weapon! Does that mean that we get 0 WATK from weaponry? Well, you would think so, but you would also be wrong:

    bareHandsWatk(LEVEL) := min{(2 * LEVEL + 31) / 3, 31}

“`LEVEL`” here is the player character’s level, “`/`” here signifies [integer division](https://mathworld.wolfram.com/IntegerDivision.html), and “`min`” stands for “[minimum](https://en.wikipedia.org/wiki/Maxima_and_minima) \[value of a [set][set]\]”. As you can see, bare hands 👊🏾 grant 31 WATK for _all pirates that are above level 30_. Once you hit level 31, your “weapon” of choice never gets upgraded ever again. 31 WATK is, by the way, equivalent to a [Leather Arms](https://maplelegends.com/lib/equip?id=01482001) (the level 15 knuckler) with average WATK that has then been scrolled with seven [60% WATK scrolls](https://maplelegends.com/lib/use?id=2044801), and had an average scrolling outcome (roughly 4 or 5 successes).

If you thought that (in combination with the observation in the previous diary entry that mastery does not apply to unarmed attacks, i.e. their mastery is always effectively 10%) was bad enough… it gets worse. [Knuckler Booster](https://maplelegends.com/lib/skill?id=5101006) requires a (proper) knuckler to be equipped, so pugilists are left boosterless until fourth job (at which point they can access [Speed Infusion](https://maplelegends.com/lib/skill?id=5121009)). It turns out that unarmed strikes have an effective speed of 4 (“fast”, although speed 5 is confusingly also listed as “fast”), which is not bad, but worse than using a speed 5 knuckler with booster; booster adds −2 to your weapon’s speed value, so the result is 5 − 2 = 3. And most knuckles (with the exception of the [level 15](https://maplelegends.com/lib/equip?id=01482001), [level 25](https://maplelegends.com/lib/equip?id=01482003), [level 35](https://maplelegends.com/lib/equip?id=01482005), and [level 40](https://maplelegends.com/lib/equip?id=01482006) knucklers, which are all speed 6) are speed 5, so knuckle-users tend to have an attack speed advantage in the absence of Speed Infusion. Actually, I don’t even know if Speed Infusion works on unarmed pirates 🙃…

And it should also be noted that in addition to the mastery given by [Knuckler Mastery](https://maplelegends.com/lib/skill?id=5100001) not applying to unarmed attacks, the WACC 🎯 granted by Knuckler Mastery also does not apply. So pugilists are, furthermore, at a 20 WACC disadvantage when compared to knuckle-using brawlers/marauders/buccaneers.

All of this may seem a bit depressing, but these are the sacrifices that we make as pugilists in exchange for the valuable asset that is stellar self-defense (and even lethality) in any situation — even in the total absence of tools and weaponry. Pugilists don’t bring knives 🔪 to gunfights 🔫… because all they need is their bare hands 👊🏾.

[set]: https://en.wikipedia.org/wiki/Set_(mathematics)
