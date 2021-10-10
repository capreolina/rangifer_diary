# rangifer’s diary: pt. lxviii

## Taxonomising odd jobs, pt. iv: Microtaxonomy & encodings. §4

In the previous section (§3) of this part (pt. iv), I tried to narrow down some refinements to [the list of odd jobs on the Oddjobs website](https://oddjobs.codeberg.page/odd-jobs.html). The purpose of these refinements is mostly to iron out the kinks that we come across when trying to flatten out our “[universe][universe]” of odd jobs into a single structureless [set][set] of objects. We want each one of these objects (i.e. each odd job) to essentially follow the principles laid out in pt. i of this series, and also fit nicely with our historical/anthropological perspective as explored in pt. ii of this series. Furthermore, we want to ensure that these objects don’t overlap in undesirable ways — for example, we don’t want one of the odd jobs to be a subjob (strict or otherwise) of another.

With that in mind, and with some of the kinks having already been ironed out in the previous section, I want to start explicitly constructing the set that we’re after, element by element. Unfortunately, it seems that we need _even more_ prelude here — we need to decide how to describe each odd job. This is already a kind of “encoding”, as we’re using “encoding” loosely to include formalisations in general, in addition to actual concrete [codes](https://en.wikipedia.org/wiki/Code). As before, I’m going to be using the list of odd jobs on the Oddjobs website (at the time of writing) as a starting point here.

### Names

The name of an odd job is only used to identify the odd job. Although obviously these names tend to actually mean something(s), and have some [etymology](https://en.wikipedia.org/wiki/Etymology), we don’t actually care for our purposes here. Thus, a name is just an arbitrary nonempty [string][string], with the constraint that that string is not used as a name for any other odd job.

We _could_ encode names by associating with each odd job a nonempty set of nonempty strings, subject to the constraint that the collection of all such sets is pairwise [disjoint](https://en.wikipedia.org/wiki/Disjoint_sets). This would allow for a given odd job to have multiple names (as they often do, in practice). But, again, we are really only using the names for identification purposes, so it suffices to just pick one.

### Classes

In §3 of this part, I said:

> \[W\]e need to encode classes into our odd jobs by associating each odd job with a corresponding set of classes. This set would only contain exactly those classes _c_ such that it’s possible for a character whose class is _c_ to be a genuine specimen of the given odd job. In this example, a first grade brigand would not be considered a “genuine specimen” yet, as we are intentionally making a three-way distinction between kinds of non-dagger-using melee thieves: permarogues, brigands, and LUKless sins. None of these can be truly differentiated until level \>30, so before that point, they are in kind of a weird state where we consider them to be odd (as they are non-dagger-using melee thieves), but we don’t yet conclusively assign a job to them.

This notion of “genuine specimen” is a kind of… “nonlocal” property, if you will. What I mean is that, when determining whether or not a character is a “genuine specimen” of a particular odd job, it doesn’t always suffice to just look at the character & at the basic definition of the odd job. We might _also_ need to know how _other_ odd jobs are defined, because the notion of “genuine specimen” is tied up with our need to keep the odd jobs separate from one another. We’re perfectly okay with a single character fitting the definition of more than one odd job at the same time: for example, a [permarcher](https://oddjobs.codeberg.page/odd-jobs.html#permarcher) who is also a [woods(wo)man](https://oddjobs.codeberg.page/odd-jobs.html#woodsman) is perfectly conceivable (in fact, I wouldn’t mind playing one myself…). What we’re _not_ okay with is when a pair of odd jobs _necessarily collide_ at some point. Using the example above: if we allowed a rogue (first grade thief) to be considered a [brigand](https://oddjobs.codeberg.page/odd-jobs.html#brigand), and also allowed them to be considered a [LUKless sin](https://oddjobs.codeberg.page/odd-jobs.html#lukless-assassin), then any level 27 rogue who aspires to advance to second grade brigand (with a class of “bandit”) _or_ to second grade LUKless sin (with a class of “assassin”) is somehow considered to be _both_, at least for the time being. And worse yet, if they instead went the [permarogue](https://oddjobs.codeberg.page/odd-jobs.html#permarogue) route — while still being a non-dagger-using melee thief — they would presumably be a brigand, a LUKless sin, and a permarogue all at once. These collisions are not the product of the player’s choice (as was the case with the permarcher-woods(wo)man), but rather, are a product of sloppy definitions.

So, with that in mind, the above quotation summarises how we wish to formalise classes as part of our odd job definitions. The basic structure is thus: a single nonempty set of classes being associated with each odd job. And the constraint is: each set contains all classes _c_ such that it’s possible for a character whose class is _c_ to be a “genuine specimen” (in the above sense) of the associated odd job.

Oh, and also, we will only be considering classes up to, and including, grade two. This is for generality; classes that have a grade higher than two can simply be collapsed into the lowest-grade class that is a member of their throughclass. For example, we can conflate “chief bandit” and “shadower” with bandit, thus eliminating the need to use the terms “chief bandit” and “shadower” entirely.

### Locations

Every odd job is associated with a location of some kind, in order to accomodate our [islanders](https://oddjobs.codeberg.page/odd-jobs.html#islander) and our [campers](https://oddjobs.codeberg.page/odd-jobs.html#camper) (and their exact analogues from later pre-BB versions, e.g. roadies and snowlanders). There are plenty of interesting locations that can be associated with a given character build, but our focus is narrow: we only want to define some odd jobs. So, for that purpose, there are only three possible values for the location of a particular odd job:

- Camp
- Maple Island
- Outland

…Which should be self-explanatory. The only slight caveat is that we are using these terms broadly:

- “Camp” refers to _any_ camp from _any_ pre-BB version of MapleStory. This includes, for example:
    - The “classic camp”, which looks visually similar to Maple Island (grass, dirt, mushrooms, sky, etc.), with the exception of being populated by [Tutorial Jr. Sentinels](https://maplelegends.com/lib/monster?id=9300018).
    - Later versions of the same camp, which are visually indoors (featuring [conveyor belts](https://en.wikipedia.org/wiki/Conveyor_belt), [image projectors](https://en.wikipedia.org/wiki/Projector), etc.) and force the player to wear a full-body green apple suit (whence “applelander”).
    - Empress’ Road (whence “roadie”).
    - The tutorial region of Snow Island (populated by Tutorial Murus), whence “snowlander”.
- “Maple Island” refers to _any_ version of Maple Island, as well as _any_ Maple-Island-equivalent region from _any_ pre-BB version of MapleStory.
- “Outland” refers to the [union][union] of all regions that aren’t already included within the above.

### Stats

Within the list of odd jobs on the Oddjobs website, the stats used by a given odd job are addressed in two ways:

- A list of “primary” stats, and a list of “secondary” stats.
- A list of stat constraints.

And, in particular, the stats in question are:

- STR
- DEX
- INT
- LUK
- MAXHP
- MAXMP

#### Primary stats, secondary stats

Defining what “primary” and “secondary” mean above can be a little tricky. But this should suffice:

- A stat _s_ is “primary” for a job _j_ when one or more of the following are true:
    - _j_ requires, by definition, having a base _s_ that is significantly more than the minimum amount of _s_. The minimum amount of _s_ might depend on _j_’s classes, e.g. 4 is usually the minimum for INT, but that’s raised to 20 in the case that _j_ enforces being a mage. The minimum amount of _s_ also might depend on _j_’s required equipment, in the case that _j_ requires some particular equipment item that has stat requirements. For example, [magelanders](https://oddjobs.codeberg.page/odd-jobs.html#magelander) are forced to use the [Metal Wand](https://maplelegends.com/lib/equip?id=01372002), which has a minimum total INT requirement of 55.
    - _m_ is a mode of attacking that is a primary method of dealing damage for _j_ (many odd jobs have multiple such modes), and _s_ is the stat (out of all six stats) that contributes the most positively to _m_’s [expected][expectation] damage output. In the case of a tie (multiple stats contribute the most positively to _m_’s expected damage output), all stats _s_ that are tied and would otherwise qualify as secondary stats (if they weren’t primary stats) are considered primary stats. Other stats _s_ that are tied are considered to be secondary (unless they are already primary anyways!).
- A stat _s_ is “secondary” for a job _j_ when _s_ is not primary for _j_, and one or more of the following are true:
    - Nontrivial amounts of total _s_ are required to equip one or more of the items that are idiomatic equipment for _j_.
    - _m_ is a mode of attacking that is a primary method of dealing damage for _j_ (many odd jobs have multiple such modes), and _s_ is a stat that contributes positively to _m_’s expected damage output.
- A stat _s_ is “tertiary” for a job _j_ when _s_ is neither primary nor secondary for _j_. It is rare to explicitly name tertiary stats for a job, as knowing the primary and secondary stats is enough to know that all other stats are tertiary.

Slightly wonky, but MapleStory has some rather strange mechanics and weird corner cases that force us to word this very carefully. This is also, obviously, far from the only possible way to reasonably define these terms — but it should be good enough for our purposes.

With that in mind, we can formalise the primary and secondary stats of a given odd job as simply being two sets (each possibly empty) of stats, selected from the universe {STR, DEX, INT, LUK, MAXHP, MAXMP}. We don’t necessarily have to specify that this pair of sets is disjoint, but because of our definitions of “primary” and “secondary” above, they will end up being disjoint anyways.

#### Stat constraints

Within the list of odd jobs on the Oddjobs website, stat constraints are given as a list of constraints, and the implication is that we take the [conjunction](https://en.wikipedia.org/wiki/Logical_conjunction) of these constraints. Each item in the list is generally written as *a*ℛ*b*, where ℛ is one of the following [binary relations](https://en.wikipedia.org/wiki/Binary_relation):

- [=][equality]
- [\>][inequality]
- ≫

This latter symbol means “is much greater than”, and is used to denote that the left-hand side has to be _considerably_ larger than the right-hand side, not merely mathematically larger. And _a_ and _b_ are usually(!) either a constant [natural number](https://en.wikipedia.org/wiki/Natural_number) (e.g. 4), or some stat (e.g. DEX). It gets a little hairier with, for example, the stat constraints of [gish](https://oddjobs.codeberg.page/odd-jobs.html#gish) and of [gishlet](https://oddjobs.codeberg.page/odd-jobs.html#gishlet) — but we can gloss over that, because I don’t want to actually define our stat constraints in that way for our purposes.

Rather than adopting this method, I want to do something fundamentally different, albeit loosely inspired by the way that I’ve done it for the list on the Oddjobs website. Like the aforementioned method, I want to express stat constraints as a conjunction of zero or more (but still a finite number, of course) constraints. But, instead of framing each individual constraint in terms of a binary relation, I want to break the constraints into a few possible cases. Each possible case is formalised as a [higher-order function](https://en.wikipedia.org/wiki/Higher-order_function) that [takes in](https://en.wikipedia.org/wiki/Domain_of_a_function) a stat (i.e. an element of the set **Stat** = {STR, DEX, INT, LUK, MAXHP, MAXMP}), and [yields](https://en.wikipedia.org/wiki/Codomain) a [predicate][predicate]\*. The predicate’s sole [argument](https://en.wikipedia.org/wiki/Argument_of_a_function) is a natural number representing the value of the corresponding base (i.e. _without_ any equipment/buffs) stat of the character. In other words, each case is some [function][function] ℱ [:](https://en.wikipedia.org/wiki/Type_theory) **Stat** [→][function] ([ℕ](https://en.wikipedia.org/wiki/Natural_number) → [{0, 1}](https://en.wikipedia.org/wiki/Boolean_domain)). Each concrete constraint, then, is the result of [applying](https://en.wikipedia.org/wiki/Function_application) such a higher-order function, and the constraint is satisfied [iff](https://en.wikipedia.org/wiki/If_and_only_if) it yields true (or 1, or whatever you wanna call it) for the given character. In the following list of cases, _s_ is any arbitrary stat:

- **Less**(_s_)

  This function gets its name from the [English](https://en.wikipedia.org/wiki/English_language) [suffix](https://en.wikipedia.org/wiki/Suffix) “[-less](https://en.wiktionary.org/wiki/-less#English)”, which is commonly used to describe character builds in MapleStory: “DEXless”, “LUKless”, etc. This expresses constraints that are, in the Oddjobs list, expressed by using equality (“=”) to equate the stat (e.g. STR) with its minimum (e.g. 4 for most classes, 35 for warriors). The constraint is simply that the character cannot have a base _s_ that is significantly higher than the minimum value of their _s_.
- **Ful**(_s_)

  This function gets its name from the English suffix “[-ful](https://en.wiktionary.org/wiki/-ful#English)”, which is the opposite of “-less” in some sense. If **Less**(_s_) expresses that the character mustn’t have any base _s_, then **Ful**(_s_) expresses that the character must have some significant amount (above the minimum) of base _s_. This expresses constraints that are, in the Oddjobs list, expressed by using “≫” to constrain the stat (e.g. STR) to be considerably larger its minimum (e.g. 4 for most classes, 35 for warriors).
- **Pure**(_s_)

  This function is a stronger version of **Ful**(_s_). Rather than expressing that the character has a significantly high base _s_, **Pure**(_s_) expresses that the character has put all of their AP into _s_ (at least, as much as possible). The term “pure” is also in common parlance: a “pure LUK thief” is understood to be a thief that is STRless and DEXless. We can allow some special exceptions to the constraint that the character has “put all of their AP into _s_”, for the purpose of HP/MP washing (if such a mechanic exists in the game implementation), and/or for the purpose of “bloodwashing”/“bluewashing” to some degree\*.

A smol, but significant, addition here is that we actually want each constraint not to be just one predicate, but rather, to be the [disjunction](https://en.wikipedia.org/wiki/Logical_disjunction) of one of more predicates.† This lets us easily cover a few edge cases (e.g. gishes), but is usually unnecessary (i.e. it will usually be the disjunction of just one predicate, which is the same as just the predicate itself).

<details>
<summary>Footnotes for “Stat constraints”</summary>

\*I say “to some degree” because we can’t allow _purely_ washed characters to qualify for this exception. Characters that spend all of their AP to further some combination of HP/MP washing, “bloodwashing”, and “bluewashing” would then be able to satisfy **Pure**(_s_) and **Less**(_s_) at the same time, which we obviously want to disallow.

†We’re getting what is called a “[product](https://en.wikipedia.org/wiki/Logical_conjunction) of [sums](https://en.wikipedia.org/wiki/Logical_disjunction) form” in [digital](https://en.wikipedia.org/wiki/Digital_electronics) logic.

</details>

### Equipment

Because we are going to allow [pugilists](https://oddjobs.codeberg.page/odd-jobs.html#pugilist) to wear shields (see §2 and §3 of this part), the only constraints that we need to place on equipment is on weapons. So, to specify the equipment that is allowed for a given odd job, we can simply specify the set of all weapons that they are “allowed” to use. The key word here is “allow” — this still includes weapons that the odd job is _spiritually_ allowed to use, but that the odd job is actually incapable of wielding in practice. For example, [brigands](https://oddjobs.codeberg.page/odd-jobs.html#brigand) are allowed to use any polearm; nevertheless, they can never wield the [Eclipse](https://maplelegends.com/lib/equip?id=01442019) — despite it being a polearm — due to it being a warrior-only equipment item. This makes equipment constraints more comparable, easier to define, and more general.

There are basically three ways that we might define weapon constraints. For any given odd job, we will pick exactly one of them:

- No constraints. This is just [the set of all weapons in the game][universe].
- A nonempty set of weapon types. Each such weapon type is the set of all weapons that are of that type, and we simply take the [union][union] of all of these sets. The types that exist are as follows:
    - One-handed sword (item [ID](https://en.wikipedia.org/wiki/Identifier)s: 130*dddd*\*)
    - One-handed axe (item IDs: 131*dddd*)
    - One-handed BW† (item IDs: 132*dddd*)
    - Dagger (item IDs: 133*dddd*)
    - Wand (item IDs: 137*dddd*)
    - Staff (item IDs: 138*dddd*)
    - Two-handed sword (item IDs: 140*dddd*)
    - Two-handed axe (item IDs: 141*dddd*)
    - Two-handed BW (item IDs: 142*dddd*)
    - Spear (item IDs: 143*dddd*)
    - Polearm (item IDs: 144*dddd*)
    - Bow (item IDs: 145*dddd*)
    - Crossbow (item IDs: 146*dddd*)
    - Claw (item IDs: 147*dddd*)
    - Knuckler (item IDs: 148*dddd*‡)
    - Gun (item IDs: 149*dddd*)
- A nonempty set of individual weapons (i.e. individual item IDs).

There is just one weird corner case here: attacking with no weapon equipped. Attacking with no weapon is similar enough to using a knuckler that we consider “no weapon at all” to be a member of the set of all knucklers. Obviously, it has no item ID, but we can just pretend that it has an ID of 0. Thus, the weapon constraints for [pugilist](https://oddjobs.codeberg.page/odd-jobs.html#pugilist) are expressed as the [singleton][singleton] set {0}.

Beyond just constraints, we also (as per §3 of this part) want to accommodate “canonical” weapons as part of each odd job’s definition. While the weapon constraints tell us the set of all weapons that the odd job is spiritually _allowed_ to use, the set of canonical weapons tells us which weapons the odd job _idiomatically_ uses. The set of canonical weapons is always a [subset](https://en.wikipedia.org/wiki/Subset) (although, not necessarily a strict one) of the set of allowed weapons. Otherwise, the set of canonical weapons is defined very similarly to the set of allowed weapons. One caveat is that we want to disallow defining the canonical weapons in terms of individual IDs, unless the allowed weapons are also defined in terms of individual IDs. Again, this makes things more comparable, easier to define, and more general.

<details>
<summary>Footnotes for “Equipment”</summary>

\*Here, _dddd_ is any string of four [decimal](https://en.wikipedia.org/wiki/Decimal) digits.

†Here, “BW” stands for “blunt weapon”, and is synonymous with “mace”.

‡We make an exception for fighting with no weapon equipped, which has an ID of 0 despite being considered a knuckler. See the main text.

</details>

### Skills & attacks

Weapon constraints (and class constraints, obviously) naturally put limitations on the abilities (active or passive) that a job can make use of. For example, despite the fact that most definitions of brigand don’t explicitly disallow the use of [Savage Blow](https://maplelegends.com/lib/skill?id=4201005), brigands cannot use Savage Blow. Using Savage Blow requires that the user be wielding a dagger, and brigands cannot wield daggers. That being said, some odd jobs feature constraints on the abilities that they can use which _cannot_ be captured simply by a combination of class constraints & equipment constraints. We can express these skill/attack constraints with the use of just one predicate, in addition to associating yet another set with each odd job. Our new predicate is (in the definition of this predicate, _j_ is any arbitrary job):

- **Ammo**(_j_)

  This predicate is satisfied iff _j_ is allowed to use ammunition. Like with weapon constraints, the word “allowed” here is spiritual — even a job that would likely never want to use ammunition (e.g. [DEX warrior](https://oddjobs.codeberg.page/odd-jobs.html#dex-warrior)), or a job that is _de facto_ incapable of using ammunition (e.g. [dagger warrior](https://oddjobs.codeberg.page/odd-jobs.html#dagger-warrior)), can still satisfy this predicate.

And the set that we want to associate with each job is the set of all non-zeroth-grade skills that that job is allowed to make use of. We ignore zeroth-grade skills because none of our odd job definitions disallow any particular zeroth-grade skills. For jobs that have no skill constraints, it is immaterial to our definition whether the corresponding set of skills contains all skills that are possibly available to that job (based solely on the job’s possible classes), or just all skills in general. For jobs that _do_ have explicit skill constraints, we consider all and only those skills that are possibly available to the job in question (again, based solely on the job’s possible classes). We also consider skills from grades one, two, three, and four, for maximum generality. Furthermore, we collapse multiple skills that only really differ in what class and/or weapon type they are available to, into a single skill with a single ID. An extreme example of this kind of collapsing is [Maple Warrior (MW)](https://maplelegends.com/lib/skill?id=1121000); there is (internally) a different MW skill for every fourth-grade class in the game, but we simply take the smallest skill ID out of all of them (1121000, in the MW case; heroes just happen to have the smallest class ID of any fourth-grade class) and conflate the rest with it. This same logic applies to masteries, boosters, final attack skills, etc.

[universe]: https://en.wikipedia.org/wiki/Universe_%28mathematics%29
[set]: https://en.wikipedia.org/wiki/Set_(mathematics)
[string]: https://en.wikipedia.org/wiki/String_(computer_science)
[union]: https://en.wikipedia.org/wiki/Union_(set_theory)
[expectation]: https://en.wikipedia.org/wiki/Expected_value
[equality]: https://en.wikipedia.org/wiki/Equality_(mathematics)
[inequality]: https://en.wikipedia.org/wiki/Inequality_(mathematics)
[function]: https://en.wikipedia.org/wiki/Function_(mathematics)
[predicate]: https://en.wikipedia.org/wiki/Predicate_(mathematical_logic)
[relation]: https://en.wikipedia.org/wiki/Relation_(mathematics)
[singleton]: https://en.wikipedia.org/wiki/Singleton_%28mathematics%29
