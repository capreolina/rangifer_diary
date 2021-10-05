# rangifer’s diary: pt. lxvii

## Taxonomising odd jobs, pt. iv: Microtaxonomy & encodings. §3

In the previous section (§2) of this part, I compared and contrasted [the list of odd jobs on the **Oddjobs** website](https://oddjobs.codeberg.page/odd-jobs.html) with two other similar lists:

- “[General List of Experimental Classes](https://web.archive.org/web/20200217044826/http://www.basilmarket.com/Other-General-List-of-Experimental-Classes-Wiki-2993)”, posted on 2010-01-13 by [BasilMarket](https://basilmarket.com/) user **GunDelHel**.
- “[Alyssaur’s Kind of Unnecessary Compilation of Unusual Builds](https://mapleroyals.com/forum/threads/alyssaurs-kind-of-unnecessary-compilation-of-unusual-builds.32333/)” ([archived](https://web.archive.org/web/20210101092152/https://mapleroyals.com/forum/threads/alyssaurs-kind-of-unnecessary-compilation-of-unusual-builds.32333/)), posted on 2015-07-17 by [MapleRoyals](https://mapleroyals.com/) user **Alyssaur**.

And we also compared the Oddjobs list to the findings of pt. ii of this series, as well.

I came across a few adjustments that I wanted to make to the Oddjobs list (at the very least, for the purpose of this series):

- More carefully include post-v62-but-pre-BB (“v62” as in version 62 of GMS; “pre-BB” as in pre-Big-Bang) classes, i.e. Cygnus Knights and Legends. The Oddjobs list (at the time of writing) does mention some of these, but only in the aliases and notes for a few jobs like [STR beginner](https://oddjobs.codeberg.page/odd-jobs.html#str-beginner). GunDelHel’s list splits out some of the Cygnus-based and Legend-based odd jobs into their own jobs, but none of them are radically different enough from their pre-v63 counterparts to fall outside of their counterparts’ original definitions. That being said, in the interest of generality, I want to ensure that these later versions are incorporated explicitly into the odd jobs’ definitions.
- Although there are some good reasons why the Oddjobs list currently defines [bowginners](https://oddjobs.codeberg.page/odd-jobs.html#bowginner) (and the other jobbed rangeginners, i.e. [clawginners](https://oddjobs.codeberg.page/odd-jobs.html#clawginner) and [begunners](https://oddjobs.codeberg.page/odd-jobs.html#begunner)) as being perma-firsts (permanently first grade), I want to loosen this restriction on all three of them. Again, this is in the interest of generality, and to make their definitions sit more comfortably with historical definitions.
- Similarly in the interest of generality and historicity, I want to loosen the restriction on [pugilists](https://oddjobs.codeberg.page/odd-jobs.html#pugilist) that they must be totally bare-handed, instead stipulating that they _mustn’t use a main weapon, but may still equip shields_. This still means no knucklers, no spears, etc.; the only difference is that pugilists can now wear shields by definition, making “shieldless pugilist” a subjob of pugilist.
- We have at least two pirate odd jobs that have unclear definitions (well, their definitions are basically clear — it’s just unclear whether these are spiritually the “correct” or “natural” definitions), which we are going to handwavingly gloss over for now:
    - [Summoner](https://oddjobs.codeberg.page/odd-jobs.html#summoner),
    - [Pistol-whipper](https://oddjobs.codeberg.page/odd-jobs.html#pistol-whipper).
- The definition of [HP warrior](https://oddjobs.codeberg.page/odd-jobs.html#hp-warrior) is to be expanded to include HP spear(wo)men/DKs, although HP [permawarriors](https://oddjobs.codeberg.page/odd-jobs.html#permawarrior) will still be excluded. Again, this is for generality and historicity purposes.

Otherwise, there are other difficult issues that don’t arise (at least, not directly) from comparing historical lists side by side.

### Toe-stepping

The fundamental problem of perhaps all taxonomy is, in some ways, very simple: how do we make things (conceptually) separate from one other? In [The Real World™](https://en.wikipedia.org/wiki/Universe) (whatever that is), things are very messy, and nothing is separate from anything else. In some ways, the fact that we are dealing with a [video game](https://en.wikipedia.org/wiki/Video_game) can simplify things — if we want to know what class my character is, we can unambiguously consult the [bit](https://en.wikipedia.org/wiki/Bit)-level representation of my class. Something like `0000 0000 1110 1000`\* (in [decimal](https://en.wikipedia.org/wiki/Decimal): [232][232]) is known as “bishop”. This is because [computer programs](https://en.wikipedia.org/wiki/Computer_program), intentionally or tacitly, and for better or for worse, must formalise things to some degree — in the most basic case, by merely encoding them at the bit level.

Also for better or for worse, this kind of formalisation doesn’t actually take us very far. We bear in mind with seriousness the mechanical details as we go, but when it comes to something like defining “odd jobs”, we are left with an essentially _[anthropological](https://en.wikipedia.org/wiki/Anthropology)_ subject. In Actual Reality™ (again, whatever that is), we are dealing with a _body of practice_ surrounding an unusually specific way of playing a particular [MMORPG](https://en.wikipedia.org/wiki/Massively_multiplayer_online_role-playing_game) from [2003](https://en.wikipedia.org/wiki/2003). [Cultural approaches](https://en.wikipedia.org/wiki/Cultural_anthropology) are useful here, but limited, as the traces of such “culture” are largely lost to the ages, and many players (or groups of players) have decided to play odd jobs _independently_ of one another.

In any case, this leaves us with the task of hardening the definitions on our list of odd jobs. The fundamental problem of taxonomy is one of separation, so that is what we’ll do: get rid of as much toe-stepping as we can. We have a few issues with “odd job” definitions stepping on one another’s toes:

#### “Permabeginner”

In §2 of pt. i of this series, I tried to define “odd job” in a way that is parsimonious, and keeps things simple & pure. I said that “\[d\]efining a particular odd job (e.g. [permawarrior](https://oddjobs.codeberg.page/odd-jobs.html#permawarrior)) is **simple**”, and also that “\[e\]ach particular odd job satisfies some intuitive notion of **natural**”. The simplest definition of the classic “permabeginner” defines it as “the job that never takes first grade advancement”. However, permabeginners are very diverse, and as such, this definition technically encompasses our [campers](https://oddjobs.codeberg.page/odd-jobs.html#camper), [islanders](https://oddjobs.codeberg.page/odd-jobs.html#islander), and even non-locative distinctions like our [STRginners](https://oddjobs.codeberg.page/odd-jobs.html#str-beginner), [besinners](https://oddjobs.codeberg.page/odd-jobs.html#besinner), etc. It is possible to pave over these distinctions as all being subjobs (and subsubjobs, and so on), but at least to me, that would seem to be doing an immense disservice if this results in “hiding” them all within the umbrella of “permabeginner”. If our intent really is that stated in §1 of this part:

> \[W\]e are still imposing the restrictions that our finite set of odd jobs is indeed finite, has a [cardinality](https://en.wikipedia.org/wiki/Cardinality) of at least two, and that — importantly — each member of the set is a well-defined “odd job” that exists on the same conceptual level as, but is distinct from, every other member of the set, in a way that is more or less analogous to [the biological notion of “species”](https://en.wikipedia.org/wiki/Species).

…Then we need a way to chop down our ontology into a single conceptual level, with the understanding that there can always be “subjobs, subsubjobs, etc.” below that level, and possibly even “superjobs” above it. Ultimately, we want our taxonomical structure to co-occupy this upper level alongside where the “superjobs” (c/w)ould be, because we don’t want to organise odd jobs based on their definition (and therefore simply on which definitions are strictly more general, or less general, than some others) and nothing else. It’s pretty clear to me (and should become even more clear when we really start nailing down (a) taxonom(y/ies)) that defining a taxonomy based on the subjob relation is certainly _possible_, but pretty much misses the point, and leaves a lot (most?) of odd jobs stranded, somehow completely unrelated to every other job. So flattening things out gives us a simple and easy starting point (again, as elaborated in §1 of this part), and in doing so, we don’t want to flatten out our beginners to the point of asphyxiation.

If we — to use just one example — wanted to separate besinners from STRginners, it seems that we could easily float out some arguments that justify doing so, even given our definition of “odd job” expounded in pt. i of this series. A [natural-language](https://en.wikipedia.org/wiki/Natural_language) definition of these two jobs can still, arguably, satisfy our “simplicity” and “naturalness” requirements. For example:

- A STRginner is a **STR-based beginner**.
- A besinner is a **beginner that uses claws**.

Unfortunately, we’ve already run into a problem here. “STR-based beginner” still encompasses STRlanders, and “beginner that uses claws” still encompasses sinlanders. So, for better or for worse, we’re going to introduce the term “off-island” into these kinds of definitions, so we would have (for example):

- A STRginner is a **STR-based off-island beginner**.
- A besinner is an **off-island beginner that uses claws**.
- A STRlander is a **STR-based islander**.
- A sinlander is an **islander that uses claws**.

Other ways of phrasing “off-island” would be “outlander” or “mainlander”. The justification here is basically that locative distinctions form such a fundamental part of a job, that we cannot reasonably lump STRginners with STRlanders (although their names and corresponding naïve definitions might have you believe otherwise).

Likewise, distinctions of AP allocation, and of weaponry, form fundamental parts of jobs as well. Some odd jobs seem to (at least, definitionally) differ from non-odd ones simply by AP allocation alone: the [magelet](https://oddjobs.codeberg.page/odd-jobs.html#magelet) is a mage that is LUK-based rather than INT-based. And we can say similar things about weaponry distinctions: the [dagger warrior](https://oddjobs.codeberg.page/odd-jobs.html#dagger-warrior) is a warrior that uses daggers rather than ordinary warrior weapons (e.g. blunt weapons for warriors/pages/WKs/paladins). So this provides a strong argument for separating — reusing the same example — besinners from STRginners.

And furthermore, we have to simply realise the obvious: beginners are, by design, inherently unspecialised. This is why [every character is born a beginner](https://en.wikipedia.org/wiki/Tabula_rasa) (or whatever it may be called, e.g. noblesse; I’m using “beginner” as the catch-all term). And because all characters are _expected_ to leave Maple Island forever, and _expected_ to take first grade advancement at level 10 (or 8), these assumptions can be sneakily implicit in our definitions/names, and also causes “\[any\] job that never takes first grade advancement” to necessarily be odd already.

So, with that, we can expand “permabeginner” (or the list of permabeginner odd jobs on the Oddjobs list) into:

- Camp:
    - Camper (a.k.a. applelander)
- Maple Island:
    - STRlander (a.k.a. STR islander, stronglander)
    - DEXlander (a.k.a. DEX islander)
    - Magelander (a.k.a. wand islander, wandlander)
    - LUKlander (a.k.a. LUK islander, sinlander)
    - Hybridlander (a.k.a. hybrid islander) (this includes “perfectlanders”, and other related concepts)
- Outland:
    - STRginner (a.k.a. STR beginner)
    - DEXginner (a.k.a. DEX beginner, permadexer)
    - Wand beginner (a.k.a. wandginner, permawander)
    - LUKginner (a.k.a. besinner, singinner, LUK beginner)

…for a total of _ten_ odd jobs which necessarily can never take first grade advancement.

#### Thieves with a healthy disrespect for AP allocation

While this doesn’t cover _all_ thieves with a healthy disrespect for AP allocation (others are less problematic), the following are five such odd jobs that can sometimes be difficult to pry apart:

- [Grim reaper](https://oddjobs.codeberg.page/odd-jobs.html#grim-reaper)
- [Carpenter](https://oddjobs.codeberg.page/odd-jobs.html#carpenter)
- [LUKless assassin](https://oddjobs.codeberg.page/odd-jobs.html#lukless-assassin)
- [Brigand](https://oddjobs.codeberg.page/odd-jobs.html#brigand)
- [LUKless bandit](https://oddjobs.codeberg.page/odd-jobs.html#lukless-bandit)

See §3 of pt. i of this series for the arguments as to why grim reaper, carpenter, and brigand count as authentic odd jobs by our definitions, even though this might not be obvious.

First of all, we want to pry grim reaper and carpenter out of the grips of LUKless sin and brigand. Because grim reapers & carpenters are, by definition, restricted to using a one-handed axe & a one-handed sword, respectively, they tend to look like highly-weapon-restricted brigands. Or was it highly-weapon-restricted LUKless sins? Well, obviously, it depends on how they take their second grade advancement. But what if they just don’t take second grade advancement? Well, then they would overlap with [permarogue](https://oddjobs.codeberg.page/odd-jobs.html#permarogue) instead then, wouldn’t they? The point that I want to make is: just because two jobs can easily overlap with one another (“overlap” in the sense of a single character having both jobs), that doesn’t necessarily mean that they conflict, or step on each other’s toes. In particular, as long as the two jobs are defined distinctly enough that neither one encompasses the other (neither is a subjob of the other), and as long as each definition satisfies our requirements to be an “odd job”, there is no real issue. A given carpenter _could_ be a permarogue, or they could just take second grade advancement. A given carpenter _could_ be a brigand, or they could just never take second grade advancement to bandit. A given carpenter _could_ be a LUKless sin, or they could just never take second grade advancement to assassin. The key thing is that the carpenter, as a job, is defined only by the exclusive use of the [Saw](https://maplelegends.com/lib/equip?id=01302001). This happens to imply that they take first grade advancement to rogue (otherwise, they obviously couldn’t use the Saw at all), but there is no particular suggestion of what to do beyond that point. This means that our odd job definitions, when properly formalised/encoded/whatever, will need to include information about weaponry — at the very least, the set of all possible weapons that the job is capable of using. This information is already reflected in the Oddjobs list. I also want to, in addition, include the set of all “canonical” weapons (or rather, weapon types) for each job — so, hopefully I remember to do that!

Second of all, we want to pry apart LUKless sin and brigand from one another. The issue here is essentially that both of these odd jobs are rogues at their core, but don’t differ in any notable way while they are still first grade. Again, upon closer inspection, this isn’t a real issue; many pairs of jobs (when played/built “canonically”, if you will) are more or less indistinguishable from one another at _some_ point. Sometimes this happens within the first grade — take, as another example, a [permawarrior](https://oddjobs.codeberg.page/odd-jobs.html#permawarrior) versus any normal warrior. For obvious reasons, this is even more likely to happen within the zeroth grade… The point is, only diverging from second grade advancement and onwards is not really an issue for us, thanks to our definition of “atemporality” in pt. i of this series (see §4 and §5 in particular). Furthermore, LUKless sins and brigands actually do differ in a subtle way during even first grade, which I will get into below.

More to the point, we need to fix up how we represent the relationship between classes (“class” meaning in-game class name, like beginner, rogue, gunslinger, white knight, marksman, etc.; see §1 of pt. i of this series) and our odd jobs. Right now, on the Oddjobs list, each entry in the list has an associated list of “possible job progressions”. Each such “possible job progression” is full and complete; it always starts with “beginner” and proceeds to the highest possible grade, including everything in between. For many purposes, this works well enough. It’s supposed to be implied that some characters with a given odd job — and a given “job progression” thereof — may not make it to the highest possible grade listed, either because they simply never make it that far, or because they exist within a version of the game that lacks the fourth grade (and possibly also lower grades), or because they are self-imposing a grade restriction. And it’s also supposed to be implied that — while it’s obviously impossible to “skip” lower grades to get to higher ones more quickly — a character with a given job may not differentiate itself immediately, especially not while still a beginner. So lower grades might be irrelevant in some cases as well. When producing our own formalisation/encoding of odd jobs for the purpose of this series, “possible job progressions” just won’t do. Instead, we need to encode classes into our odd jobs by associating each odd job with a corresponding set of classes. This set would only contain exactly those classes _c_ such that it’s possible for a character whose class is _c_ to be a genuine specimen of the given odd job. In this example, a first grade brigand would not be considered a “genuine specimen” yet, as we are intentionally making a three-way distinction between kinds of non-dagger-using melee thieves: permarogues, brigands, and LUKless sins. None of these can be truly differentiated until level \>30, so before that point, they are in kind of a weird state where we consider them to be odd (as they are non-dagger-using melee thieves), but we don’t yet conclusively assign a job to them.

Third of all, we want to pry apart brigand and LUKless dit from one another. Both are LUKless dits/CBs/shads by definition, which makes the nomenclature somewhat confusing, as only one of them is actually _called_ “LUKless dit”. Furthermore, they are both melee combatants — barring the possible use of claws, which neither are very good at using anyways, although claws can still be useful in niche circumstances. What separates them is purely a matter of weaponry. Brigands are defined such that they can use essentially any weapon type, except for the canonical weapon type of their class (dit/CB/shad): daggers. On the other hand, LUKless dits are essentially limited to only ordinary thief weapons: daggers (and claws…). This gives them a playstyle very different from brigands, as there are many daggers (read: warrior-thief daggers, e.g. [Dragon Kanzir](https://maplelegends.com/lib/equip?id=01332049)) which don’t have LUK requirements, and their damage with daggers still scales reasonably well with STR+DEX (read: non-LUK stats) in all of their attacking skills. This also means that their AP allocation strategies tend to _usually_ be quite different in practice, although this is not part of their definitions _per se_. In line with the weaponry sets discussed above, this might look like:

- **Brigand**
    - Possible weapons:
        - {one-handed swords} [∪][union]
        - {one-handed axes} ∪
        - {one-handed blunt weapons} ∪
        - {wands} ∪
        - {staves} ∪
        - {two-handed swords} ∪
        - {two-handed axes} ∪
        - {two-handed blunt weapons} ∪
        - {spears} ∪
        - {polearms} ∪
        - {claws}.
    - Canonical weapons:
        - {one-handed swords} ∪
        - {one-handed axes} ∪
        - {one-handed blunt weapons} ∪
        - {two-handed swords} ∪
        - {two-handed axes} ∪
        - {two-handed blunt weapons} ∪
        - {polearms}.
- **LUKless dit**
    - Possible weapons:
        - {daggers} ∪
        - {claws}.
    - Canonical weapons:
        - {daggers}.

<details>
<summary>Footnotes for “Toe-stepping”</summary>

\*I’m using 16 bits here because, in general, these numbers can be as large as 999, thus requiring at least two [bytes](https://en.wikipedia.org/wiki/Byte). At best, [DPD](https://en.wikipedia.org/wiki/Densely_packed_decimal) gets you down to ten bits :)

</details>

[232]: https://en.wikipedia.org/wiki/232_(number)
[union]: https://en.wikipedia.org/wiki/Union_(set_theory)

## Adventures on Victoria Island

![Helm HP 10%s](helm-hp-10s.webp "Helm HP 10%s")

![d33r’s 12 INT cape~](d33r-12-int-cape.webp "d33r’s 12 INT cape~")

![d34r vs. special fierry](d34r-vs-special-fierry.webp "d34r vs. special fierry")

## The big levelup party

![](ducklings-100-xbowtjuhnl-harlez-gruzz-120-party.webp "")

## Moar MPQ w/ panolia

![](espontanea-tootle-applebao-and-panolia-vs-franken-lloyd.webp "")

![](panolia-pengxiang-and-stink3r-trio-mpq.webp "")

![](xchouri-panolia-espontanea-and-siramik-vs-angy-fanky.webp "")

![](rip-siramik.webp "")

## Cards on cards on cards

![](capre-ghost-pirate-card-get.webp "")

![](capre-buffoon-card-get.webp "")

![](capre-dual-ghost-pirate-card-get.webp "")

![](capre-finds-an-unknown-letter.webp "")

![](capre-master-death-teddy-card-get.webp "")

![](capre-spirit-viking-card-get.webp "")

![](capre-125-spirit-vikings-epm-test.webp "")

![](capre-grim-phantom-watch-card-get.webp "")

![](capre-thanatos-card-get.webp "")

![](capre-gatekeeper-card-get.webp "")

![](capre-timer-card-get.webp "")

![](capre-master-soul-teddy-card-get.webp "")

![](capre-soul-teddy-card-get.webp "")

![](mst-etcs.webp "")

![](capre-scholar-ghost-card-get.webp "")

![](capre-seruf-card-get.webp "")

![](capre-security-camera-card-get-1.webp "")

![](capre-shade-card-get.webp "")

![](capre-zombie-mushmom-card-get-1.webp "")

![](what-in-the-damn-hell.webp "")

## rusa does the Rush quest (again)

![](rusa-helps-jcrry-with-the-rush-quest.webp "")

## rusa slays the Leviathan

## cervine hits the big 110

![](cervine-110.webp "")

![](pirate-guy-wanna-cry.webp "")

![](cervine-s-mon.webp "")

## A special little LPQ session

![](suboptimal-krew-and-best5-vs-alishar.webp "")

![](lpq-mule-suicide-pact.webp "")
