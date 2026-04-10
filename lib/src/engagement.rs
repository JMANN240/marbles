use std::fmt::Display;

use rand::{Rng, seq::IndexedRandom};
use sqlx::SqlitePool;

use crate::{ball::Ball, scene::Scene};

#[derive(Debug, Clone)]
pub struct Marble {
    pub name: String,
    pub pronouns: Pronouns,
}

#[derive(Debug, Copy, Clone)]
pub enum Pronouns {
    Masculine,
    Feminine,
    Neutral,
}

impl Pronouns {
    pub fn from_name(name: &str) -> Self {
        match name {
            "Blue's Wife" | "Black Hole" | "Molly" | "Perl" => Self::Feminine,
            "IKEA" | "Beebo" | "Canada" | "Creepy" | "Estonia" | "Amogus" => Self::Neutral,
            _ => Self::Masculine,
        }
    }

    pub fn subject(&self) -> String {
        match self {
            Self::Masculine => String::from("he"),
            Self::Feminine => String::from("she"),
            Self::Neutral => String::from("it"),
        }
    }

    pub fn object(&self) -> String {
        match self {
            Self::Masculine => String::from("him"),
            Self::Feminine => String::from("her"),
            Self::Neutral => String::from("it"),
        }
    }

    pub fn posessive(&self) -> String {
        match self {
            Self::Masculine => String::from("his"),
            Self::Feminine => String::from("hers"),
            Self::Neutral => String::from("its"),
        }
    }
}

pub enum Engagement {
    WhoWillWin,
    OkEpic,
    NeverGuess,
    LastPlaceOverall,
    DecidesTheChampion,
    OneRaceLeft,
    WinnerStays,
    WhoSurvives,
    NoOneExpected,
    FinalCornerDecides,
    BiggestUpsetInHistory,
    OutOfControl,
    GetsIntense,
    FinalStretch,
    DontBlink,
    WinToLossTodayTheDay(Marble),
    HasntSeenPodium(Marble),
    WinOrCrashOut(Marble),
    HasntLostInStraight(Marble),
    HistoricLosingStreak(Marble),
    HasntFinishedTopHalfInAttempts(Marble),
    HuntingNthWin(Marble),
    RunsThisTrack(Marble),
    NeverLostHere(Marble),
    DoesntLoseCloseRaces(Marble),
    AlwaysFindsAWay(Marble),
    CompletelyUnpredictable(Marble),
    SuspiciouslyConsistent(Marble),
    EitherFirstOrLast(Marble),
    JustDoesOwnThing(Marble),
    ChokedLastNRedemption(Marble),
    DesperateForAWin(Marble),
    HasntBeenRelevant(Marble),
    QuietlyClimbing(Marble),
    BackAfterBrutalLosingStreak(Marble),
    WinsAgainTakesLead(Marble),
    AllEyes(Marble),
    AnyoneStop(Marble),
    VsTied(Marble, Marble),
    HateEachOther(Marble, Marble),
    VsControl(Marble, Marble),
    VsOppositeForces(Marble, Marble),
    AlwaysSabotages(Marble, Marble),
}

impl Display for Engagement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WhoWillWin => write!(f, "Who will win?"),
            Self::OkEpic => write!(f, "Ok, now THIS is epic!"),
            Self::NeverGuess => write!(f, "You'll never guess!"),
            Self::LastPlaceOverall => write!(f, "Loser of this race drops to last place overall"),
            Self::DecidesTheChampion => write!(f, "This race decides the champion"),
            Self::OneRaceLeft => write!(f, "One race left... everything comes down to this"),
            Self::WinnerStays => write!(f, "Winner stays. Loser is eliminated."),
            Self::WhoSurvives => write!(
                f,
                "Three marbles wiped out in the last race... who survives this one?"
            ),
            Self::NoOneExpected => write!(f, "No one expected THIS finish"),
            Self::FinalCornerDecides => write!(f, "The final corner decides everything"),
            Self::BiggestUpsetInHistory => write!(f, "Biggest upset in marble history?"),
            Self::OutOfControl => write!(f, "This race got out of control FAST"),
            Self::GetsIntense => write!(f, "This one gets intense"),
            Self::FinalStretch => write!(f, "Watch the final stretch..."),
            Self::DontBlink => write!(f, "Don't blink"),
            Self::WinToLossTodayTheDay(marble) => {
                write!(f, "{} is 0-27... is today finally the day?", marble.name)
            }
            Self::HasntSeenPodium(marble) => {
                write!(f, "{} hasn't seen a podium in 15 races", marble.name)
            }
            Self::WinOrCrashOut(marble) => write!(
                f,
                "{} either wins... or crashes out completely",
                marble.name
            ),
            Self::HasntLostInStraight(marble) => {
                write!(f, "{} hasn't lost in 6 straight", marble.name)
            }
            Self::HistoricLosingStreak(marble) => {
                write!(f, "{} is on a historic losing streak", marble.name)
            }
            Self::HasntFinishedTopHalfInAttempts(marble) => write!(
                f,
                "{} hasn't finished a race in the top half in 20 attempts",
                marble.name
            ),
            Self::HuntingNthWin(marble) => write!(
                f,
                "{} is hunting {} 8th win",
                marble.name,
                marble.pronouns.posessive()
            ),
            Self::RunsThisTrack(marble) => write!(
                f,
                "{} runs this track... nobody has stopped him yet",
                marble.name
            ),
            Self::NeverLostHere(marble) => {
                write!(f, "{} is back-and he's never lost here", marble.name)
            }
            Self::DoesntLoseCloseRaces(marble) => {
                write!(f, "{} doesn't lose close races", marble.name)
            }
            Self::AlwaysFindsAWay(marble) => write!(f, "{} always finds a way to win", marble.name),
            Self::CompletelyUnpredictable(marble) => {
                write!(f, "{} is completely unpredictable", marble.name)
            }
            Self::SuspiciouslyConsistent(marble) => {
                write!(f, "{} is... suspiciously consistent", marble.name)
            }
            Self::EitherFirstOrLast(marble) => {
                write!(f, "{} is either first or last. No in-between.", marble.name)
            }
            Self::JustDoesOwnThing(marble) => write!(
                f,
                "{} just does {} own thing",
                marble.name,
                marble.pronouns.posessive()
            ),
            Self::ChokedLastNRedemption(marble) => write!(
                f,
                "{} choked the last 3 finishes... redemption today?",
                marble.name
            ),
            Self::DesperateForAWin(marble) => write!(f, "{} is desperate for a win", marble.name),
            Self::HasntBeenRelevant(marble) => write!(
                f,
                "{} hasn't been relevant in weeks... until now?",
                marble.name
            ),
            Self::QuietlyClimbing(marble) => {
                write!(f, "{} is quietly climbing the ranks", marble.name)
            }
            Self::BackAfterBrutalLosingStreak(marble) => {
                write!(f, "{} is back after a brutal streak of losses", marble.name)
            }
            Self::WinsAgainTakesLead(marble) => {
                write!(f, "If {} wins again, he takes the lead", marble.name)
            }
            Self::AllEyes(marble) => write!(f, "All eyes on {}", marble.name),
            Self::AnyoneStop(marble) => write!(f, "Can anyone stop {}?", marble.name),
            Self::VsTied(marble_1, marble_2) => {
                write!(f, "{} vs {} - tied 4-4", marble_1.name, marble_2.name)
            }
            Self::HateEachOther(marble_1, marble_2) => {
                write!(f, "{} and {} hate each other", marble_1.name, marble_2.name)
            }
            Self::VsControl(marble_1, marble_2) => write!(
                f,
                "{} vs {} - who takes control?",
                marble_1.name, marble_2.name
            ),
            Self::VsOppositeForces(marble_1, marble_2) => write!(
                f,
                "{} vs {} - opposite forces collide",
                marble_1.name, marble_2.name
            ),
            Self::AlwaysSabotages(marble_1, marble_2) => {
                write!(f, "{} always sabotages {}", marble_1.name, marble_2.name)
            }
        }
    }
}

const OPPOSITES: [(&str, &str); 7] = [
    ("White Light", "Black Hole"),
    ("Fireball", "Deep Blue"),
    ("Mastermind", "Jokester"),
    ("Giftbringer", "Psycho"),
    ("Blue's Wife", "White's Brother"),
    ("Trump Card", "Joe Mama"),
    ("Homelander", "The Butcher"),
];

pub fn are_opposites(ball_1: &Ball, ball_2: &Ball) -> bool {
    OPPOSITES.contains(&(ball_1.get_name(), ball_2.get_name()))
}

pub fn get_opposites(balls: &[Ball]) -> Vec<(&Ball, &Ball)> {
    let mut opposites = Vec::new();

    for (i, ball_1) in balls.iter().enumerate() {
        for ball_2 in balls[i + 1..].iter() {
            if are_opposites(ball_1, ball_2) {
                opposites.push((ball_1, ball_2));
            }
        }
    }

    opposites
}

pub async fn get_engagement_for_scene(
    _pool: &SqlitePool,
    rng: &mut impl Rng,
    scene: &Scene,
) -> sqlx::Result<String> {
    let mut possible_engagements = vec![
        Engagement::LastPlaceOverall,
        Engagement::DecidesTheChampion,
        Engagement::OneRaceLeft,
        Engagement::WinnerStays,
        Engagement::WhoSurvives,
        Engagement::NoOneExpected,
        Engagement::FinalCornerDecides,
        Engagement::BiggestUpsetInHistory,
        Engagement::OutOfControl,
        Engagement::GetsIntense,
        Engagement::FinalStretch,
        Engagement::DontBlink,
    ];

    for ball in scene.get_balls() {
        let marble = Marble {
            name: ball.get_name().to_string(),
            pronouns: Pronouns::from_name(ball.get_name()),
        };

        possible_engagements.append(&mut vec![
            Engagement::WinToLossTodayTheDay(marble.clone()),
            Engagement::HasntSeenPodium(marble.clone()),
            Engagement::WinOrCrashOut(marble.clone()),
            Engagement::HasntLostInStraight(marble.clone()),
            Engagement::HistoricLosingStreak(marble.clone()),
            Engagement::HasntFinishedTopHalfInAttempts(marble.clone()),
            Engagement::HuntingNthWin(marble.clone()),
            Engagement::RunsThisTrack(marble.clone()),
            Engagement::NeverLostHere(marble.clone()),
            Engagement::DoesntLoseCloseRaces(marble.clone()),
            Engagement::AlwaysFindsAWay(marble.clone()),
            Engagement::CompletelyUnpredictable(marble.clone()),
            Engagement::SuspiciouslyConsistent(marble.clone()),
            Engagement::EitherFirstOrLast(marble.clone()),
            Engagement::JustDoesOwnThing(marble.clone()),
            Engagement::ChokedLastNRedemption(marble.clone()),
            Engagement::DesperateForAWin(marble.clone()),
            Engagement::HasntBeenRelevant(marble.clone()),
            Engagement::QuietlyClimbing(marble.clone()),
            Engagement::BackAfterBrutalLosingStreak(marble.clone()),
            Engagement::WinsAgainTakesLead(marble.clone()),
            Engagement::AllEyes(marble.clone()),
            Engagement::AnyoneStop(marble.clone()),
        ]);
    }

    let opposites = get_opposites(scene.get_balls());

    for (ball_1, ball_2) in opposites {
        let marble_1 = Marble {
            name: ball_1.get_name().to_string(),
            pronouns: Pronouns::from_name(ball_1.get_name()),
        };

        let marble_2 = Marble {
            name: ball_2.get_name().to_string(),
            pronouns: Pronouns::from_name(ball_2.get_name()),
        };

        possible_engagements.append(&mut vec![
            Engagement::HateEachOther(marble_1.clone(), marble_2.clone()),
            Engagement::VsControl(marble_1.clone(), marble_2.clone()),
            Engagement::VsOppositeForces(marble_1.clone(), marble_2.clone()),
            Engagement::AlwaysSabotages(marble_1.clone(), marble_2.clone()),
        ]);
    }

    let engagement = possible_engagements.choose(rng).unwrap();

    Ok(engagement.to_string())
}
