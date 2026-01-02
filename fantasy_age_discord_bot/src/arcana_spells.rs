pub fn get_arcana_spells(arcana: &str) -> Vec<String> {
    let arcane_spells = match arcana.to_lowercase().as_str() {
        "air" => AIR_ARCANA_SPELLS.iter().map(|&s| s.to_string()).collect(),
        _ => vec!["No spells found for the specified arcana.".to_string()],
    };

    arcane_spells.iter().map(|s| s.to_string()).collect()
}

static AIR_ARCANA_SPELLS: &[&str] = &[
    "***AIR BUBBLE, DEFENSIVE***
    - **Requirements:** Air Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 12
    You create a bubble of fresh, clean, breathable air within a 3-yard radius around you, which moves with you as you move.
    When it comes into being, the bubble pushes away any smoke, mist, dust, toxic gases, or even water (forming a bubble of air underwater)
    and slight positive air pressure keeps out all such things for up to an hour. You can renew an existing air bubble before it expires by recasting the spell,
    extending its duration for another hour.",
    "***PROTECTIVE WINDS, UTILITY***
    - **Requirements:** Air Arcana (Novice) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 10
    You summon a whipping wind that circles about you and protects you from harm. You gain a +2 to your Defense vs. ranged attacks. 
    Anyone adjacent to you (friend or foe) suffers a -1 penalty when making melee attacks. Protective winds last until the end of the encounter.",
    "***VOICES OF THE WIND, UTILITY***
    - **Requirements:** Air Arcana (Novice) - **MP Cost:** 4+
    - **Casting Time:** 1 Minute - **Target Number:** 11
    You can focus in on any conversation happening within 100 yards of you, as long as you can see it taking place.
    The wind carries the conversation to your ears, and you can hear it as clearly as if you were sitting next to the participants.
    You can switch to a different conversation as a major action. Voices on the wind lasts for 10 minutes, and you can extend it
    for 10 minutes for each 2 MP you spend.",
    "***WIND NET, UTILITY***
    - **Requirements:** Air Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Free Action - **Target Number:** 11
    At your command, the winds form an invisible net to slow and cushion a fall, allowing you to drop from any height and 
    land on the ground without harm. You can also cast wind net to catch a falling object or creature so long as you can see them, 
    and their landing point is within 30 yards of you. Wind net does not affect direct missile weapon attacks, except for things like large falling objects.
    Each target object requires a new casting of the spell.",
    "***WALL OF MIST, UTILITY***
    - **Requirements:** Air Arcana (Expert) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 12
    You summon up a wall of thick swirling mist within 30 yards of you. The wall is up to 10 yards long, 4 yards tall, and 2 yards
    thick, following whatever path you wish. The mist does not impede movement but does block all sight through and within it. 
    Those moving through the mist lose half (rounded down) of any move distance remaining to them in this action.
    The wall of mist lasts for up to 1 minute. You can pay the spell's MP cost again to extend its duration an additional minute.",
    "***WIND BLAST, ATTACK***
    - **Requirements:** Air Arcana (Expert) - **MP Cost:** 8
    - **Casting Time:** Major Action - **Target Number:** 14
    - **Test:** Strength (Might) vs. Spellpower
    You send a powerful gust of wind from your outstretched hands or arcane device. The wind blast is 6 yards long and 4 yards wide. 
    Anyone caught inside it must make a Strength (Might) test vs. your Spellpower or be knocked Prone after sliding away from you a number of yards equal to the spell's degrees ofsuccess. 
    The wind will also send light objects flying, blow out candles, etc.",
    "***WHIRLWIND, ATTACK***
    - **Requirements:** Air Arcana (Master) - **MP Cost:** 10
    - **Casting Time:** Major Action - **Target Number:** 15
    - **Test:** Dextarity (Acrobatics) vs. Spellpower
    A powerful vortex of air with a 4-yard radius springs into being at a point you choose within 50 yards of you.
    Anyone caught in the area of the whirlwind takes 3d6 + Willpower damage and may be knocked Prone. 
    Those who succeed on a Dexterity (Acrobatics) test vs. your Spellpower are able to remain standing.",
    "***WINDS OF FLIGHT, UTILITY***
    - **Requirements:** Air Arcana (Master) - **MP Cost:** 10+
    - **Casting Time:** Major Action - **Target Number:** 15
    You summon winds that bear you aloft. You gain a flying Speed of 8 + Willpower. 
    You must use the rules for flying found in Chapter 2, though you retain your land Speed while on the ground. 
    Winds of flight lasts for 1 hour and you can extend it for up to 5 hours for a cost of 2 MP per hour."
];

static BEAST_ARCANA_SPELLS: &[&str] = &[
    "***BEAST SENSES, UTILITY***
    - **Requirements:** Beast Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 12
    You can access the senses of a creature, using them as if they were your own. Choose a beast within 20 yards of you that you can perceive 
    or that you know well. You may either gain one of that beast's Perception focuses or sensory special qualities or perceive the 
    environment through the beast's senses as if you were that beast. 
    Either effect lasts for the remainder of the encounter, even if the beast moves more than the initial 20 yards away from you. 
    If you are perceiving through the beast's senses, your own are shut down and you are unaware of what is happening around you. 
    You can pay the spell's MP cost again to extend its duration for another encounter, even if the beast is some distance from you. 
    At Expert degree, you can cast beast senses on any beast you have summoned with beast summoning or charmed with charm beasts automatically for half MP cost.",
    "***BEAST SPEECH, UTILITY***
    - **Requirements:** Beast Arcana (Novice) - **MP Cost:** 2
    - **Casting Time:** Major Action - **Target Number:** 12
    You can channel your magic to communicate with members of the beast world. For the remainder of the encounter your speech becomes 
    intelligible to all beasts and you can likewise understand their vocalizations as if they were speaking to you. Most natural 
    beasts aren't really great conversationalists and cannot understand or convey complex concepts, but any beast that is not 
    hostile toward you may at least be persuaded to tell you things it has recently seen or experienced, or things it knows about the local area. 
    Communication tests may be required, and social stunts involving beasts are possible as a result of these tests.",
    "***BEAST SUMMONING, UTILITY***
    - **Requirements:** Beast Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 11
    You send out a mystic call that summons a nearby beast or group of beasts. A successful casting roll means the beasts hear your call from 
    up to your level in miles away and come to you as quickly as they can under their own power. Summoned beasts are not 
    under your control and behave normally according to their nature when they arrive at your location. You can use other 
    spells to communicate with the beasts or convince them to do as you wish. You may limit your summons to a particular
    type of beast (such as only birds of prey, horses, or wolves, for example) or even to a particular beast known to you, such as a pet or mount.",
    "***POWER OF THE WILD, ENCHANTMENT***
    - **Requirements:** Beast Arcana (Novice) - **MP Cost:** 8
    - **Casting Time:** Major Action - **Target Number:** 12
    You magically “borrow” an aspect of a beast to improve your own abilities. Choose a type of beast with an ability rating greater than one of your abilities. 
    For the duration, you gain a +2 bonus to that ability, up to a maximum equal to the beast's ability rating. You also gain one focus the beast possesses for that ability. 
    If you already have that focus, then your bonus with it increases from +2 to +3. If you are level 11 or higher, your +1 bonus to all focuses applies to the focus you gained with this spell.
    *Power of the wild* normally lasts for one minute, but you can pay an additional 5 MP upon casting it for it to last two minutes instead.
    At Expert degree, you can cast power of the wild on a subject other than yourself by touching them. The spell's effects are the same otherwise.",
    "***CHARM BEASTS, UTILITY***
    - **Requirements:** Beast Arcana (Expert) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 13
    - **Test:** Willpower (Morale) vs. Spellpower
    You magically charm a beast within 6 yards merely by making eye contact with it. The beast must make a Willpower (Morale) test against your Spellpower. 
    If it fails, its attitude toward you for the remainder of the encounter becomes friendly and agreeable, as if you had trained it with the Novice degree of the Animal Training talent (see **Chapter 3**). 
    At Master degree, you can influence a charmed beast as if you had trained it to the Expert degree of Animal Training. You can also cast *charm beasts* on multiple beasts at once by 
    paying an additional +2 MP per additional beast, up to a maximum of your Willpower.",
    "***LESSER BEAST FORM, UTILITY***
    - **Requirements:** Beast Arcana (Expert) - **MP Cost:** 10
    - **Casting Time:** Major Action - **Target Number:** 12
    You take on the physical form of a beast, gaining all its physical abilities. For the rest of the encounter, or until you choose to 
    reassume your normal form, you transform into a natural animal of your choice that is rated as a Minor Threat (See **Chapter 9**). 
    Examples include a bat, a boar, a cat, a dog, an eagle, or a horse. While in beast form, you retain your own Intelligence and Willpower 
    (and focuses) and gain all of the beast's other abilities and focuses. You gain the higher of the beast's Perception and Health or your own. 
    You cannot use your own physical or class abilities while in beast form, including casting spells and any carried or held equipment 
    either drops to the ground where you transform or else disappears, melding into your own form and is unusable until you assume your normal form again.",
    "***CURSE OF THE BEAST, ATTACK***
    - **Requirements:** Beast Arcana (Master) - **MP Cost:** 12
    - **Casting Time:** Major Action - **Target Number:** 15
    - **Test:** Willpower (Self-Discipline) vs. Spellpower
    Your sorcery transforms a foe into a humble beast. Choose a target within 6 yards of you, who must immediately roll a Willpower (Self-Discipline) 
    test against your Spellpower. If the test fails, the target is transformed into a Minor Threat beast of your choice (See **Chapter 9**). 
    The target loses all their previous abilities, focuses, talents, and qualities, except for Willpower and Health. Replace them with those of 
    the beast's abilities instead. Each round, the victim of *curse of the beast* may roll a new Willpower (Self-Discipline) test at the start of each of their 
    turns as a free action. Success ends the spell, but four total failures mean the victim is permanently trapped in beast form 
    until you either choose to end the spell or it is removed, such as by *arcane abatement*.",
    "***GREATER BEAST FORM, UTILITY***
    - **Requirements:** Beast Arcana (Master) - **MP Cost:** 15
    - **Casting Time:** Major Action - **Target Number:** 12
    You take on the physical form of a more powerful beast. Like lesser beast form, except you can transform into a Moderate Threat beast of your choice (See **Chapter 9**). 
    At the Game Master's option, you can also transform into a Moderate Threat version of a Minor Threat beast by adding 1 to any three of the beast's abilities, two focuses, and increasing its Health by 5."
];

static COLD_ARCANA_SPELLS: &[&str] = &[
    "***ICE GRIP, ATTACK***
    - **Requirements:** Cold Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 12
    - **Test:** Constitution (Stamina) vs. Spellpower
    A cloud of frost and ice swirls forth to envelop a target of your choice. The target must be within 20 yards, and the cloud lasts for rounds equal to your Willpower. 
    The target takes 1d6 penetrating damage from the cold. Each additional round, at the start of your turn, the target must make a Constitution (Stamina) test vs. your Spellpower. 
    If successful, the icy grip spell ends. If the test fails, the target takes another 1d6 penetrating damage and a -2 cumulative penalty to Speed. 
    The caster decides whether a target reduced to 0 Health by icy grip is frozen solid, in which case they are Dying, or substantially encased in ice, in which case they are Helpless.",
    "***ICE KNIVES, ATTACK***
    - **Requirements:** Cold Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 11
    - **Test:** Dexterity (Acrobatics) vs. Spellpower
    You fling a shower of razor-sharp icicles at your foes. These *ice knives* manifest in an arc from the caster 5 yards long and up to 2 yards wide. 
    Any creature in the area must make a Dexterity (Acrobatics) test against your Spellpower. On a failure, the creature suffers 1d6 + Willpower damage. 
    A successful test results only in damage equal to your Willpower (minimum of 1).",
    "***ICE SHEET, ATTACK***
    - **Requirements:** Cold Arcana (Novice) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 13
    - **Test:** Dexterity (Acrobatics) vs. Spellpower
    You mystically create a slippery layer of frozen water on the ground beneath your opponents. The *ice sheet* can be up to 10 yards in diameter, anywhere 
    within 30 yards of you. Anyone in the area at the time of casting, entering the area subsequently, or moving within the area must make a successful Dexterity (Acrobatics) test 
    vs. your Spellpower or fall Prone. Prone characters can crawl but standing up requires another test to avoid falling. If you cast ice sheet on a body of water, 
    it freezes the surface, creating ice thick enough for a person to walk on, free-floating unless it can touch and anchor to where the water meets land. 
    A 10-yard or smaller diameter area is completely frozen over. The ice sheet persists as long as the local temperature allows, melting normally if it is above freezing.",
    "***WINTER'S WALK, UTILITY***
    - **Requirements:** Cold Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 10
    Icy arcane power speeds your passage through areas of frozen terrain. Choose any creatures within 6 yards of you (including yourself). 
    For the remainder of the encounter, the subjects can all move across ice and snow without any reduction in Speed or any chance of sinking, 
    slipping, or falling. Subjects leave no footprints, even on soft or powdery snow, and are immune to the effects of the *ice sheet* spell.",
    "***IMMUNITY TO COLD, DEFENSE***
    - **Requirements:** Cold Arcana (Expert) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 13
    Your magics protect you or others from cold-based damage. You, or the subject touched, become immune to the effects 
    of cold and freezing temperatures for the rest of the encounter. This includes damage inflicted solely by cold temperatures 
    including other Cold Arcana spells, but not by physical weapons made of ice.",
    "***FROST WEAPONS, ATTACK***
    - **Requirements:** Cold Arcana (Expert) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 15
    Damaging cold energy radiates from the weapons of your allies. The readied melee weapons of all allies within 10 yards of you exude freezing cold, 
    inflicting an additional 2 points of penetrating damage with each successful attack. The spell lasts for one minute; 
    you can extend the duration by spending an additional 2 MP per additional minute. *Frost weapons* does not harm the weapons themselves or their wielders.",
    "***WINTER BLAST, ATTACK***
    - **Requirements:** Cold Arcana (Master) - **MP Cost:** 15
    - **Casting Time:** Major Action - **Target Number:** 17
    - **Test:** Constitution (Stamina) vs. Spellpower
    Freezing mists numb and chill your foes. You project a blast of freezing mist from your outstretched hands that is 2 yards wide and 8 yards long. Anyone in the 
    area suffers 2d6 + Willpower penetrating damage and a -10 penalty to Speed for a number of rounds equal to half your Willpower, rounded down (minimum of 1). 
    Subjects who succeed on the Constitution (Stamina) test vs. your Spellpower only take 1d6 + Willpower penetrating damage and a -5 penalty to Speed.",
    "***BLIZZARD BURST, ATTACK***
    - **Requirements:** Cold Arcana (Master) - **MP Cost:** 20
    - **Casting Time:** Major Action - **Target Number:** 17
    - **Test:** Dexterity (Acrobatics) vs. Spellpower
    The force of a blizzard strikes at your command. You conjure a roaring blizzard of swirling ice and snow in a 5-yard radius, centered anywhere within 50 yards of you. 
    Anyone in the affected area takes 2d6 + Willpower penetrating damage and must succeed on a Dexterity (Acrobatics) test vs. your Spellpower or fall Prone. 
    Targets that start their turn in the area of the blizzard burst take an additional 1d6 penetrating damage and must make a Dexterity (Acrobatics) test to avoid falling Prone. 
    All melee attacks made into or out of the area of the blizzard burst suffer a -2 penalty, while all ranged attacks suffer a -5 penalty. 
    The spell normally lasts for only one round but, for 10 MP, you can extend its duration another round at the start of your turn as a free action."
];

static DEATH_ARCANA_SPELLS: &[&str] = &[
    "***DRAW UPON DEATH, UTILITY***
    - **Requirements:** Death Arcana (Novice) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 15
    You draw upon the fading life force of the dying to replenish and heal yourself. For the rest of the encounter, whenever a living creature 
    dies within 6 yards of you while the spell is in effect, you can choose to regain either 1d6 Health or 1d6 Magic Points. 
    This replenishment cannot increase either score above its normal maximum.",
    "***GHOST STRIKE, ENCHANTMENT***
    - **Requirements:** Death Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** Major Action - **Target Number:** 11
    You empower your allies to combat incorporeal spirits. Choose allies within 6 yards of you equal to your Willpower (including yourself). 
    For the remainder of the encounter, those allies' melee attacks can affect incorporeal spirits of the dead, such as spectres (see Chapter 9), as if they were solid. 
    Allies affected by the spell can also otherwise touch such creatures, but don't gain the benefit of this spell for ranged attacks.",
    "***SPEAK WITH THE DEAD, UTILITY***
    - **Requirements:** Death Arcana (Novice) - **MP Cost:** 10
    - **Casting Time:** 1 Minute - **Target Number:** 15
    You touch a corpse and gain the ability to speak with the spirit of the deceased who once inhabited that body. You may ask questions which the spirit is compelled to answer truthfully, 
    although it cannot relate information it did not know in life. The initial casting of speak with the dead gets you one question, but you can spend an additional 5 MP per 
    additional question, up to a maximum number equal to your Willpower + 1 (minimum 1).",
    "***WARD OFF THE DEAD, DEFENSE***
    - **Requirements:** Death Arcana (Novice) - **MP Cost:** 6
    - **Casting Time:** Major Action - **Target Number:** 11
    - **Test:** Willpower (Morale) vs. Spellpower
    You speak powerful words of command to compel the obedience of the unliving. Any undead creature able to perceive you when you cast ward off the dead must roll a Willpower (Morale) test against 
    your Spellpower. If it fails, the creature cannot approach within 3 yards of you, and must retreat to that distance if it is already closer. Furthermore, the creature cannot use any 
    Special Qualities on you or anyone within 3 yards of you. If the undead creature succeeds on the Willpower (Morale) test, it is unaffected. You can maintain the effects of ward off the 
    dead by taking an Activate action on each of your turns, and so long as you maintain it, the effects persist. If you or your allies attack an affected undead creature, the effect of the spell ends for that creature.",
    "***ANIMATE DEAD, UTILITY***
    - **Requirements:** Death Arcana (Expert) - **MP Cost:** 10
    - **Casting Time:** One Minute - **Target Number:** 17
    You touch a corpse, infusing it with arcane power and causing it to rise as a walking corpse (see **Chapter 9**). The walking corpse created by this spell obeys your verbal 
    commands while it exists. The corpse remains animated for 1 minute, but you can extend the spell's effect by an additional minute by spending 2 MP. 
    At Master degree, you can spend an additional 5 MP when casting this spell for each of the following effects:
    • Shorten the casting time of animate dead to a major action.
    • Raise an additional corpse.
    • Raise one or more corpses within 5 yards of you without touching them. 
    You can apply the second and third effects multiple times. The additional MP costs are cumulative.",
    "***PESTILENCE, ATTACK***
    - **Requirements:** Death Arcana (Expert) - **MP Cost:** 8
    - **Casting Time:** Major Action - **Target Number:** 13
    - **Test:** Constitution (Stamina) vs. Spellpower
    You inflict a terrible, sorcerous wasting disease on a target of your choice. The victim must be within 10 yards of you and makes a Constitution (Stamina) test against your Spellpower. 
    If the test succeeds, there is no effect. If it fails, the victim immediately falls ill, and they have a -2 penalty on all physical ability tests. 
    Each hour after the pestilence takes hold, the victim must make another Constitution (Stamina) test against your Spellpower to avoid taking 1d6 penetrating damage, 
    which cannot be healed so long as the disease persists. Victims reduced to half their Health or less have their Speed halved. The pestilence lasts until the victim succeeds two consecutive 
    Constitution (Stamina) tests against it or the disease is cured by a cure spell or an Intelligence (Healing) test against the caster's Spellpower.",
    "***DEATH CURSE, ATTACK***
    - **Requirements:** Death Arcana (Master) - **MP Cost:** 20
    - **Casting Time:** Major Action - **Target Number:** 21
    - **Test:** Constitution (Stamina) vs. Spellpower
    You invoke a deadly curse upon your enemies. Choose either a single target or a 5-yard radius area within 50 yards of you. If you target a single enemy, they suffer 2d6 + Willpower 
    penetrating damage immediately and 1d6 + Willpower penetrating damage at the start of each of their turns. Additionally, they cannot regain Health by any means while the spell lasts. 
    If the target makes a successful Constitution (Stamina) test vs. your Spellpower, they suffer only 1d6 penetrating damage per turn and can regain Health as usual. 
    If you target an area, any living creature that enters or starts its turn in that area takes 2d6 + Willpower penetrating damage. If the creature makes a successful Constitution 
    (Stamina) test vs. your Spellpower, they suffer only 1d6 penetrating damage. The death curse lasts for a number of rounds equal to your Willpower, but for 10 MP you can extend its duration for one 
    additional round at the start of your turn as a free action.",
    "***STEAL LIFE, ATTACK***
    - **Requirements:** Death Arcana (Master) - **MP Cost:** 12
    - **Casting Time:** Major Action - **Target Number:** 15
    - **Test:** Constitution (Stamina) vs. Spellpower
    You can replenish your life force by mystically stealing it from others. The *steal life* spell has two possible effects, as follows. 
    • The target must make a Constitution (Stamina) test against your Spellpower or take 3d6 + Willpower penetrating damage. If you have lost Health, then steal life 
    allows you to regain it by taking it from a target you touch on a 1-to-1 basis; for each point of Health you regain, the target loses one. 
    When the target is reduced to Health 0, they die. 
    • If you are at full Health, you can steal actual years of life from your target and reduce your physical age accordingly! In this case the damage rolled is the number of years 
    that can be stolen. The target physically ages while you grow younger and more vital. This is a 1-to-1 transfer if the caster and subject belong to creatures with the same 
    biological lifespan, otherwise it is adjusted relative to the caster's lifespan. If you drain a target with a biological lifespan that is half of yours, you gain a year of life for 
    each six months they lose. If you drain a target with ten times your biological lifespan, you gain a year for every 10 they lose, and so forth. 
    A subject aged to their biological maximum lifespan dies. A *rejuvinate body* spell can restore years lost to *steal life*.
    Stealing Health occurs instantly when you touch the subject. Stealing years of life is more gradual, at a rate of one year granted to you per round of contact. 
    This usually means your subject must be restrained or helpless to drain them fully. In either case, if the target succeeds on the initial Constitution (Stamina) test against your Spellpower, there is no effect." 
];

static DIVINATION_ARCANA_SPELLS: &[&str] = &[
    "***BLOOD MARK, UTILITY***
    - **Requirements:** Divination Arcana (Novice) - **MP Cost:** 4+
    - **Casting Time:** Once Minute - **Target Number:** 11
    - **Test:** Perception (Touching) vs. Spellpower
    You inscribe an object or creature with an arcane mark unique to you that remains until you dismiss it. It requires a drop of your blood, although the mark itself is invisible to the naked eye. 
    Some variations of this spell take the form of a whisper emanating from the mark which only you can hear. Once an object or creature has a blood mark on it, you automatically know if it is within a mile of you. 
    You can also “burn” the blood mark so that for 10 minutes you will know the direction of a target with your mark even if you cannot otherwise perceive them—this erases the mark on the target when 
    the duration expires. You may, however, sustain the spell's effect longer, for 10 more minutes per 2 MP, decided when the effect would otherwise end. In any event, when you stop sustaining a “burned” blood mark it expires. 
    Unless burned, a blood mark lasts indefinitely. You can keep a number of active blood marks equal to your Willpower. A living creature can find and remove a blood mark on them by making a successful Perception (Touching) test vs. Spellpower. 
    The creature can make this test again every 10 minutes. You can dismiss a blood mark as a free action.",
    "***DIVINE SPEECH, UTILITY***
    - **Requirements:** Divination Arcana (Novice) - **MP Cost:** 4
    - **Casting Time:** 1 Minute - **Target Number:** 9
    Divine glibness directs your speech, allowing you to say exactly the right things to someone to improve their attitude toward you and achieve your desired ends. 
    Select one subject you are capable of communicating with. For the remainder of the encounter, you gain a +2 bonus to the Stunt Die of Communication tests to 
    change that individual's attitude (see **Impressions and Attitudes** in **Chapter 2**). This bonus increases your total test result and degree of success and, 
    if you score doubles, the number of stunt points you gain, but don't count it when determining if you've scored doubles.",
    "***FOREWARNING, UTILITY***
    - **Requirements:** Divination Arcana (Novice) - **MP Cost:** 2
    - **Casting Time:** Minor Action - **Target Number:** 9
    - **Test:** Dexterity (Stealth) vs. Spellpower
    An arcanely-invoked “sixth-sense” warns you of imminent danger or threats. 
    After successfully casting this spell, you, or another subject you've touched cannot be surprised (see **Surprise** in **Chapter 8: Mastering the Rules**) 
    except by attackers who are themselves magically concealed, in which case they must succeed on a Dexterity (Stealth) test against your Spellpower in order 
    to surprise you. Also, you gain a +2 bonus on all tests to avoid hazards where a moment's forewarning can aid you (see **Hazards** in **Chapter 8**). 
    Both effects last for the rest of the encounter.",
    "***SENTINEL, UTILITY***
    - **Requirements:** Divination Arcana (Novice) - **MP Cost:** 3
    - **Casting Time:** Major Action - **Target Number:** 9
    - **Test:** Dexterity (Stealth) vs. Spellpower
    You weave together arcane energies to place a location under surveillance. When you cast this spell, you place an invisible, intangible arcane watcher to 
    monitor an area no larger than a 3-yard radius. The spell cannot be placed on an object or creature, only to observe a place. 
    If any circumstance you specify when casting the sentinel occurs in that area, you receive a vision of it as if you were standing where the sentinel is placed. 
    Examples include when any humanoid creature enters the area, when a particular door or container opens, or when light enters the area. You can maintain the vision 
    by taking a minor action each round to do so, but your ordinary senses are overridden, so you are unaware of what is happening near you while you are observing 
    events elsewhere. You can maintain a number of active sentinel spells at any one time equal to your Willpower, although you can only perceive through one of them at a time. 
    At Expert degree, you can spend an additional 2 MP to allow your sentinel to deliver a short message (just a sentence or two) in the voice of your choosing when it is activated. 
    At Master degree, you can cast a single spell you know while perceiving through your sentinel as if you were present at that point, but doing so ends the sentinel spell.",
    
];
