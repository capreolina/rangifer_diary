# rangifer’s diary: pt. xxiii

Sorry for the delay on this one, hehe… Had a bit of difficulty, thanks to some adverse weather conditions.

## R>1 pog ranged for blue mushmom run

Dealing ungodly quantities of single-target DPS to boss monsters has long been a fascination of maplers. Many maplers aspire to mow their oversized enemies to shreds with the most brutal possible efficiency, and in pre-BB (pre-Big-Bang) MapleStory, playing a [terribly squishy](https://tvtropes.org/pmwiki/pmwiki.php/Main/GlassCannon) and [ranged](https://tvtropes.org/pmwiki/pmwiki.php/Main/LongRangeFighter) character is usually the choice way to do it. And if we’re talking about ways _not_ to do it, we might consider instead playing a [peepee poopoo garbo](https://tvtropes.org/pmwiki/pmwiki.php/Main/JokeCharacter) [odd-jobbed](https://oddjobs.codeberg.page/odd-jobs.html) character.

But what if you could do both?

Some [odd jobs](https://oddjobs.codeberg.page/odd-jobs.html) retain the ability to use ranged DPS skills; and if you’ll remember, in pt. iv of this diary, we explored the damaging capabilities of the third-job [STRmit](https://oddjobs.codeberg.page/odd-jobs.html#str-assassin). We also compared the model STRmit to an equivalent model [pugilist](https://oddjobs.codeberg.page/odd-jobs.html#pugilist), although the pugilist is a little out of scope here; the pugilist suffers too much from [a lack of range](https://maplelegends.com/lib/skill?id=5001001) and inability to surmount any significant quantity of WDEF. The pugilist is more specialised in [crowd control](https://maplelegends.com/lib/skill?id=5101004), [survivability](https://maplelegends.com/lib/skill?id=5100000), and general, jack-of-all-trades, ass-whoopin’.

Here, I want to look at one (or two, depending on how you count) other third-job odd-jobber who can also reasonably fill the role of “ranged DPS” in an odd-jobbed context: the [wood(wo)man](https://oddjobs.codeberg.page/odd-jobs.html#woodsman). Woodwomen [have historically been understood as a kind of “fancier” or “jobbed” version of permabeginners](https://oddjobs.codeberg.page/archive/subject/woodsman/), because they are pure STR by definition. But the fact remains that woodmen are, nevertheless, technically capable of using any archery skill in the game, from first through fourth job. This is of utmost importance when differentiating the two varieties of woodwoman: bow-using (hunter/ranger/bowmaster), and crossbow-using (crossbow(wo)man/sniper/marks(wo)man).

Crossbow-using woodmen are usually understood to be somewhat superior, although in second job, [Arrow Bomb](https://maplelegends.com/lib/skill?id=3101005) proves to be more generally applicable than [Iron Arrow](https://maplelegends.com/lib/skill?id=3201005). The reasons to believe that crossbows would be superior to bows here are two:

- Crossbows differ from bows in the general sense that crossbows deal more per-shot damage (better damage “range”), at the cost of having larger attack periods (slower attack rate). This would seem\* to work in favour of the woodwoman — who has a relatively depressed (although considerably [narrower](https://en.wikipedia.org/wiki/Coefficient_of_variation)) damage “range” in comparison to the DEX-based archer — because their target’s WDEF chews away so much of their raw damage before it can be multiplied by their skills (including [crits](https://maplelegends.com/lib/skill?id=3000001)).
- Crossbow-users get [Snipe](https://maplelegends.com/lib/skill?id=3221007) as their main new (“new” excludes [Strafe](https://maplelegends.com/lib/skill?id=3211006), which the woodman already has by third job) attacking skill in fourth job. This is ostensibly\*\* more fruitful than the bow-user’s main new fourth-job attacking skill: [Hurricane](https://maplelegends.com/lib/skill?id=3121004). The reason is simply that Snipe’s efficacy doesn’t depend on the woodwoman’s stats (DEX, STR, WATK); in fact, it doesn’t depend on much at all, save the skill level of Snipe itself.

Oh geez, look at those asterisks. We won’t be going into the double-asterisked (\*\*) bit above, because that’s getting a little too in the weeds with fourth-job stuff. But it’s worth investigating the singly-asterisked (\*) bit above, using similar model characters to the STRmit and pugilist used before.

### The model wood(wo)man

- 525 STR (25 of which is from gear)
- 105 DEX (80 of which is from gear)
- 128|131 WATK (90|93 from [bow](https://maplelegends.com/lib/equip?id=01452045)|[xbow](https://maplelegends.com/lib/equip?id=01462040) + 20 from [Cider](https://maplelegends.com/lib/use?id=2022002) + 3 from [cape](https://maplelegends.com/lib/equip?id=01102084) + 10 from gloves + 5 from [arrows](https://maplelegends.com/lib/use?id=2060004)|[quarrels](https://maplelegends.com/lib/use?id=2061004))

And we take the target to be the same level as the woodman.

Let’s consider how the basic attack damage (assuming no (x)bow-whacks and no crits) compares between bow and xbow, as we modulate the target’s WDEF; the basic attack damage is indicative of the raw advantage that one weapon type has over another, because damage modifiers from skills and crits are applied _after_ WDEF. We won’t be looking at the [dispersion](https://en.wikipedia.org/wiki/Statistical_dispersion) of the values in question here, because the _vast_ majority of the dispersion that matters for woodwomen who are engaging in archery is in the form of crits. The reason for this is that so much of the woodman’s damage comes from STR (not DEX), which does not differ in its damage contribution from the minimum damage to the maximum damage (unlike damage due to DEX, which is subject to [mastery](https://maplelegends.com/lib/skill?id=3100000) as well as a constant 0.9 multiplier when calculating minimum damage).

### Comparing basic attacks

The numbers here are crunched using [my damage calculator](https://oddjobs.codeberg.page/dmg-calc/). The winner of each matchup is bolded.

#### 0 WDEF

Example: [Orange Mushroom](https://maplelegends.com/lib/monster?id=1210102).

Bow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 918~1128 (1023.4)
- DPS: **1550.5**

Xbow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 955~1182 (**1068.5**)
- DPS: 1548.6

#### 200 WDEF

Example: [Red Slime](https://maplelegends.com/lib/monster?id=9400204).

Bow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 798~1028 (913.4)
- DPS: 1383.9

Xbow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 835~1082 (**958.5**)
- DPS: **1389.2**

#### 400 WDEF

Example: [Female Mannequin](https://maplelegends.com/lib/monster?id=9410033).

Bow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 678~928 (803.4)
- DPS: 1217.2

Xbow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 715~982 (**848.5**)
- DPS: **1229.8**

#### 600 WDEF

Example: [Eliza](https://maplelegends.com/lib/monster?id=8220000).

Bow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 558~828 (693.4)
- DPS: 1050.5

Xbow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 595~882 (**738.5**)
- DPS: **1070.3**

#### 800 WDEF

Example: [Angy Fanky](https://maplelegends.com/lib/monster?id=9300140).

Bow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 438~728 (583.4)
- DPS: 883.9

Xbow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 475~782 (**628.5**)
- DPS: **910.9**

#### 1000 WDEF

Example: [Pianus (L)](https://maplelegends.com/lib/monster?id=8520000).

Bow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 318~628 (473.4)
- DPS: 717.2

Xbow:

- Range ([avg](https://en.wikipedia.org/wiki/Expected_value)): 355~682 (**518.5**)
- DPS: **751.5**

Of course, WDEF values can go much higher than 1000, but we will stop there.

As you can see, the bow-user only wins in cases where the target has very low WDEF, and even then, only w.r.t. DPS; their per-hit damage is still inferior on even a 0 WDEF target.

It’s worth noting again that this assumes the use of level 64 maple weapons. Woodwomen who are rangers/bowmasters have another option: the [Bow of Magical Destruction](https://maplelegends.com/lib/equip?id=01452018) (BoMD). Obviously it depends on scrolling outcomes, but the BoMD has, on average, the same WATK as the [Kandiva](https://maplelegends.com/lib/equip?id=01452045), but is one speed category slower than the Kandiva. So using the BoMD will make the bow-user comparatively worse here (in terms of DPS; the per-hit damage would be unchanged), still assuming that the xbow-user is using the [Nishada](https://maplelegends.com/lib/equip?id=01462040).

Oh, remember when I said that “\[t\]he reasons to believe that crossbows would be superior to bows here are two”? Well, maybe they’re _three_ now, in MapleLegends…?

### Comparing Strafes

In MapleLegends, [the sniper version of Strafe](https://maplelegends.com/lib/skill?id=3211006) was changed to fire **5** projectiles, rather than [the ranger version](https://maplelegends.com/lib/skill?id=3111006)’s (and original sniper version’s) 4. The per-projectile damage multiplier was adjusted accordingly, with the sniper version only having a multiplier of 75% at max level, versus the ranger version’s 110%. This would naïvely seem to work out in favour of the ranger, because 110% ⋅ 4 = 440% > 375% = 75% ⋅ 5.

But critical hits work by adding the critical’s damage multiplier to other sources of damage multiplier, and then taking that sum and multiplying it by the after-WDEF damage. For snipers/rangers with no [SE](https://maplelegends.com/lib/skill?id=3121002), their critical multiplier is 100%, so for a critical line of the sniper’s Strafe, that’s a total multiplier of 75% + 100% = 175%, and for the ranger’s Strafe, 110% + 100% = 210%. Because (again, assuming no SE) 50% of lines are crits, we [expect](https://en.wikipedia.org/wiki/Expected_value) the total damage multiplier for a line of the sniper’s Strafe to be (75% + 175%) ÷ 2 = 125%, and for the ranger’s Strafe, (110% + 210%) ÷ 2 = 160%. Taking into account the number of projectiles/lines per Strafe, we have 160% ⋅ 4 = 640% > 625% = 125% ⋅ 5. So **the ranger still wins here, in terms of damage multiplier**, just by a smaller margin (both in absolute and relative terms).

Since, after all, we are worried about shredding boss monsters with as high of a single-target DPS as possible, let’s throw together all of the variables into the calculator here. And this time, we will assuming 600 WDEF, since that is reasonably representative of many low- or mid-level boss monsters:

- Bow DPS: **6725.3**
- Xbow DPS: 6690.6

So, indeed, the ranger still wins here, albeit by a similarly small margin as the xbow-user won when basic-attacking: 6725.3 ÷ 6690.6 = 1.00519. It looks like rangers are still, after all, the (very slight) favourite for single-target DPS in third job. Remember, though, that xbow-users still have a number of advantages, even before fourth job!:

- They deal more single-target DPS in first and second jobs, as we saw above.
- [Iron Arrow](https://maplelegends.com/lib/skill?id=3201005), while somewhat more difficult to pull off than [Arrow Bomb](https://maplelegends.com/lib/skill?id=3101005) (and lacking stun), deals considerably more damage than Arrow Bomb.
- The xbow-user’s access to [Blizzard](https://maplelegends.com/lib/skill?id=3211003) means that they pull ahead of bow-users in terms of crowd control during third job. This generally makes up for their lack of Arrow Bomb (not to mention that Arrow Bomb has a measly 60% proc chance at max level).

## Almost all-Oddjobs LPQ…

Around the time that we had just got a new addition to the guild, a [permabeginner](https://oddjobs.codeberg.page/odd-jobs.html#permabeginner) by the name of **FairRow**, I noticed we had a few people online who could [LPQ](https://maplelegends.com/lib/map?id=221024500):

- FairRow was in level range;
- I had my [DEX spearwoman](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior), **rusa**;
- **Mings** had his [blood](https://oddjobs.codeberg.page/odd-jobs.html#blood-bandit) [brigand](https://oddjobs.codeberg.page/odd-jobs.html#brigand), **MingsToo**;
- **Outside** had his F/P [gishlet](https://oddjobs.codeberg.page/odd-jobs.html#luk-gish), **TestChars**;
- and **doiob**, a [STR cleric](https://oddjobs.codeberg.page/odd-jobs.html#str-mage), was in level range.

Together we actually formed a technically-functional LPQ party: doiob and TestChars had [Teleport](https://maplelegends.com/lib/skill?id=2301001), and MingsToo was capable of covering both the thief portal (due to having [Dark Sight](https://maplelegends.com/lib/skill?id=4001003)) and our range (due to having [Keen Eyes](https://maplelegends.com/lib/skill?id=4000001)). I prepared by gathering up some potions for us, as well as [a claw](https://maplelegends.com/lib/equip?id=01472000), [stars](https://maplelegends.com/lib/use?id=2070006), and even a LUK [earring](https://maplelegends.com/lib/equip?id=01032001) for MingsToo.

We were very nearly ready to go, but unfortunately doiob was having some serious internet connectivity issues with the MapleLegends servers, so we eventually were forced to give up the dream… Oh well, maybe next time.

I did, however, get to try out Outside’s [Maple Scorpio](https://maplelegends.com/lib/equip?id=01442024):

![rusa tries on the Scorpio](rusa-tries-on-the-scorpio.png "rusa tries on the Scorpio")

I love how it looks on rusa~

## capre lays waste to the island we know as Victoria

I recently got a [Duey](https://maplelegends.com/lib/npc?id=9010009) package from **Voxtagrams**, who very kindly supplied me with the jewels necessary to complete various monster book rings:

![Duey from Voxtagrams](voxtagrams-duey.png "Duey from Voxtagrams")

…So it looks like it’s time to go more card-hunting B)

I had already started my [woodwoman](https://oddjobs.codeberg.page/odd-jobs.html#woodsman), **capreolina**, on her way towards the [T2 ring](https://maplelegends.com/lib/equip?id=01119004)! With maxed [Arrow Rain](https://maplelegends.com/lib/skill?id=3111004), it’s much more effortless and relaxing to simply run around and Arrow Rain the shit out of everything. With my [STR priest](https://oddjobs.codeberg.page/odd-jobs.html#str-mage), **cervid**, card-hunting has the possible advantage of being more engaging; in order to be efficient, I have to pay a lot of attention to where I position myself, how I use [Teleport](https://maplelegends.com/lib/skill?id=2301001), and exactly how much damage each one of my hits deals. As a STR priest, my damage is wildly unstable, so the number of hits that it takes me to kill a given monster can vary quite a bit. But when using [a bow](https://maplelegends.com/lib/equip?id=01452045) with capreolina, it’s smooth as butter:

<details>
<summary>capre lays waste to Vic</summary>

![Axe Stump card get!](capre-axe-stump-card-get.png "Axe Stump card get!")

![Bubbling card get!](capre-bubbling-card-get.png "Bubbling card get!")

![Cold Eye card get!](capre-cold-eye-card-get.png "Cold Eye card get!")

![Croco card get!](capre-croco-card-get.png "Croco card get!")

![Curse Eye card get!](capre-curse-eye-card-get.png "Curse Eye card get!")

![Dark Axe Stump card get!](capre-dark-axe-stump-card-get.png "Dark Axe Stump card get!")

![Drake card get!](capre-drake-card-get.png "Drake card get!")

![Evil Eye card get!](capre-evil-eye-card-get.png "Evil Eye card get!")

![Fire Boar card get!](capre-fire-boar-card-get.png "Fire Boar card get!")

![Horny Mushroom card get!](capre-horny-mushroom-card-get.png "Horny Mushroom card get!")

![Iron Pig card get!](capre-iron-pig-card-get.png "Iron Pig card get!")

![Iron Boar card get!](capre-iron-boar-card-get.png "Iron Boar card get!")

![Jr. Boogie card get!](capre-jr.-boogie-card-get.png "Jr. Boogie card get!")

![Ligator card get!](capre-ligator-card-get.png "Ligator card get!")

![Lupin card get!](capre-lupin-card-get.png "Lupin card get!")

![Stirge card get!](capre-stirge-card-get.png "Stirge card get!")

![Wild Boar card get!](capre-wild-boar-card-get.png "Wild Boar card get!")

![Wraith card get!](capre-wraith-card-get.png "Wraith card get!")

![Zombie Lupin card get!](capre-zombie-lupin-card-get.png "Zombie Lupin card get!")

</details>

capre is now at 49 card sets! Sick~~ And I met a few big bad guys along the way:

![Bob spotted](bob-spotted.png "Bob spotted")

Hi, [Bob](https://maplelegends.com/lib/monster?id=9400551).

![Dyle approaches](dyle-approaches.png "Dyle approaches")

Gah! [Dyle](https://maplelegends.com/lib/monster?id=6220000)!!

![Zombie Mushmom approaches](zombie-mushmom-approaches.png "Zombie Mushmom approaches")

A showdown with [Zombie Mushmom](https://maplelegends.com/lib/monster?id=6300005) results in [a card](https://maplelegends.com/lib/use?id=2388008) for a set that I will likely never finish:

![Zombie Mushmom card get!](capre-zombie-mushmom-card-get.png "Zombie Mushmom card get!")

Not going to lie, I expected Zombie Mushmom to put up more of a fight…

## hashishi and Gambolpuddy-chan train in preparation for permanoob KPQing

And then, I was ready for more training fun times with my [besinner](https://oddjobs.codeberg.page/odd-jobs.html#besinner) **hashishi**, and Kelsey’s (a.k.a. **Taima**, **Tacgnol**, **Boymoder**, **Yotsubachan**) [DEXginner](https://oddjobs.codeberg.page/odd-jobs.html#dex-beginner) **Gambolpuddy**:

![hashishi and Gambolpuddy-chan prepare for KPQ](hashishi-and-gambolpuddy-chan-prepare-for-kpq.png "hashishi and Gambolpuddy-chan prepare for KPQ")

So that by our powers combined, we could achieve level 21, for great [KPQ](https://maplelegends.com/lib/map?id=103000800)! Yes~!!
