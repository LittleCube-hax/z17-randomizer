use crate::filler::portals::Portal;
use crate::hints::{hint_color::HintColor::*, hint_ghost_name};
use crate::patch::lms::msbf::MsbfKey;
use crate::Result;
use game::ghosts::HintGhost;
use modinfo::settings::portal_shuffle::PortalShuffle;
use modinfo::settings::weather_vanes::WeatherVanes;
use modinfo::Settings;
use rom::flag::Flag;
use serde::{Serialize, Serializer};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FillerItem {
    Item(Item),
    Goal(Goal),
    HintGhost(HintGhost),
    Vane(Vane),
    Portal(Portal),
}

impl FillerItem {
    pub fn as_item(&self) -> Option<Item> {
        match self {
            Self::Item(item) => Some(*item),
            _ => None,
        }
    }

    pub fn normalize(self) -> game::Item {
        use game::Item::*;
        use FillerItem::*;
        match self {
            Item(item) => match item.to_game_item() {
                PackageSword | ItemSwordLv1 | ItemSwordLv3 | ItemSwordLv4 => ItemSwordLv2,
                ItemRentalIceRod => ItemIceRod,
                ItemRentalSandRod => ItemSandRod,
                ItemRentalTornadeRod => ItemTornadeRod,
                ItemRentalBomb => ItemBomb,
                ItemRentalFireRod => ItemFireRod,
                ItemRentalHookShot => ItemHookShot,
                ItemRentalBoomerang => ItemBoomerang,
                ItemRentalHammer => ItemHammer,
                ItemRentalBow => ItemBow,
                PowerfulGlove => PowerGlove,
                ClothesRed => ClothesBlue,
                // Item::RingRental => Item::RingHekiga,
                ItemKandelaarLv2 => ItemKandelaar,
                ItemInsectNetLv2 => ItemInsectNet,
                item => item,
            },
            _ => unreachable!(),
        }
    }

    pub fn as_item_index(&self) -> u32 {
        self.normalize() as u32
    }

    pub fn is_hint_ghost(self) -> bool {
        matches!(self, Self::HintGhost(_))
    }

    pub fn include_in_sphere_search(self, settings: &Settings) -> bool {
        match self {
            Self::Item(item) => item.include_in_sphere_search(),
            Self::Vane(_) => settings.weather_vanes == WeatherVanes::Shuffled,
            Self::Portal(_) => settings.portal_shuffle != PortalShuffle::Off,
            Self::Goal(Goal::Triforce) => true,
            _ => false,
        }
    }

    pub fn goes_in_csmc_large_chest(&self, _settings: &Settings) -> bool {
        use crate::filler::filler_item::Item::*;
        if let FillerItem::Item(item) = self {
            match item {
                Bow01
                | Bow02
                | Bow03
                | Boomerang01
                | Boomerang02
                | Hookshot01
                | Hookshot02
                | Bombs01
                | Bombs02
                | FireRod01
                | FireRod02
                | IceRod01
                | IceRod02
                | Hammer01
                | Hammer02
                | SandRod01
                | SandRod02
                | TornadoRod01
                | TornadoRod02
                | Bell
                | StaminaScroll
                | BowOfLight
                | PegasusBoots
                | Flippers
                | RaviosBracelet01
                | RaviosBracelet02
                | HylianShield
                | SmoothGem
                | LetterInABottle
                | PremiumMilk
                | Pouch
                | BeeBadge
                | HintGlasses
                | GreatSpin
                | Bottle01
                | Bottle02
                | Bottle03
                | Bottle04
                | Bottle05
                | Lamp01
                | Lamp02
                | Sword01
                | Sword02
                | Sword03
                | Sword04
                | Glove01
                | Glove02
                | Net01
                | Net02
                | Mail01
                | Mail02
                | OreYellow
                | OreGreen
                | OreBlue
                | OreRed
                | HyruleSanctuaryKey
                | LoruleSanctuaryKey
                | EasternKeyBig
                | EasternKeySmall01
                | EasternKeySmall02
                | GalesKeyBig
                | GalesKeySmall01
                | GalesKeySmall02
                | GalesKeySmall03
                | GalesKeySmall04
                | HeraKeyBig
                | HeraKeySmall01
                | HeraKeySmall02
                | DarkKeyBig
                | DarkKeySmall01
                | DarkKeySmall02
                | DarkKeySmall03
                | DarkKeySmall04
                | SwampKeyBig
                | SwampKeySmall01
                | SwampKeySmall02
                | SwampKeySmall03
                | SwampKeySmall04
                | SkullKeyBig
                | SkullKeySmall01
                | SkullKeySmall02
                | SkullKeySmall03
                | ThievesKeyBig
                | ThievesKeySmall
                | IceKeyBig
                | IceKeySmall01
                | IceKeySmall02
                | IceKeySmall03
                | DesertKeyBig
                | DesertKeySmall01
                | DesertKeySmall02
                | DesertKeySmall03
                | DesertKeySmall04
                | DesertKeySmall05
                | TurtleKeyBig
                | TurtleKeySmall01
                | TurtleKeySmall02
                | TurtleKeySmall03
                | LoruleCastleKeySmall01
                | LoruleCastleKeySmall02
                | LoruleCastleKeySmall03
                | LoruleCastleKeySmall04
                | LoruleCastleKeySmall05
                | PendantOfPower
                | PendantOfWisdom
                | Charm
                | PendantOfCourage
                | SageGulley
                | SageOren
                | SageSeres
                | SageOsfala
                | SageRosso
                | SageIrene
                | SageImpa
                | ScootFruit01
                | FoulFruit01
                | Shield01
                | ScootFruit02
                | FoulFruit02
                | Shield02
                | Shield03
                | Shield04 => return true,
                _ => {},
            }
        };

        false
    }

    pub fn msbf_key(self) -> Option<&'static str> {
        use crate::filler::filler_item::FillerItem::Item;
        use crate::filler::filler_item::Item::*;
        match self {
            Item(SageGulley) => Some(MsbfKey::Dark),
            Item(SageOren) => Some(MsbfKey::Water),
            Item(SageSeres) => Some(MsbfKey::Dokuro),
            Item(SageOsfala) => Some(MsbfKey::Hagure),
            Item(SageIrene) => Some(MsbfKey::Sand),
            Item(SageRosso) => Some(MsbfKey::Ice),
            Item(SageImpa) => None, // Impa special
            Item(PendantOfPower) | Item(PendantOfWisdom) | Item(PendantOfCourage) => None,
            _ => macros::fail!("Not an MSBF Key: {:?}", self),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Item(item) => item.as_str(),
            Self::Goal(goal) => goal.as_str(),
            Self::HintGhost(ghost) => hint_ghost_name(&ghost),
            Self::Vane(vane) => vane.as_str(),
            Self::Portal(portal) => portal.as_str(),
        }
    }

    pub fn as_str_colorized(&self) -> String {
        match self {
            Self::Goal(goal) => goal.as_str_colorized(),
            _ => Name.format(self.as_str()),
        }
    }
}

impl From<Item> for FillerItem {
    fn from(item: Item) -> Self {
        Self::Item(item)
    }
}

impl From<Goal> for FillerItem {
    fn from(goal: Goal) -> Self {
        Self::Goal(goal)
    }
}

impl From<HintGhost> for FillerItem {
    fn from(hint_ghost: HintGhost) -> Self {
        Self::HintGhost(hint_ghost)
    }
}

impl From<Vane> for FillerItem {
    fn from(vane: Vane) -> Self {
        Self::Vane(vane)
    }
}

impl From<Portal> for FillerItem {
    fn from(portal: Portal) -> Self {
        Self::Portal(portal)
    }
}

impl Serialize for FillerItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Item {
    Empty,

    Bow01,
    Bow02,
    Bow03, // Only used when Progressive Bow of Light option is enabled

    Boomerang01,
    Boomerang02,

    Hookshot01,
    Hookshot02,

    Bombs01,
    Bombs02,

    FireRod01,
    FireRod02,

    IceRod01,
    IceRod02,

    Hammer01,
    Hammer02,

    SandRod01,
    SandRod02,

    TornadoRod01,
    TornadoRod02,

    Bell,
    StaminaScroll,
    BowOfLight,
    PegasusBoots,
    Flippers,
    RaviosBracelet01,
    RaviosBracelet02,
    HylianShield,
    SmoothGem,
    LetterInABottle,
    PremiumMilk,
    Pouch,
    BeeBadge,
    HintGlasses,
    Charm,
    GreatSpin,

    Quake,

    RupeeGreen,
    RupeeBlue,
    RupeeRed,

    RupeePurple01,
    RupeePurple02,
    RupeePurple03,
    RupeePurple04,
    RupeePurple05,
    RupeePurple06,
    RupeePurple07,
    RupeePurple08,
    RupeePurple09,
    RupeePurple10,
    RupeePurple11,
    RupeePurple12,
    RupeePurple13,
    RupeePurple14,
    RupeePurple15,
    RupeePurple16,
    RupeePurple17,
    RupeePurple18,
    RupeePurple19,
    RupeePurple20,

    RupeeSilver01,
    RupeeSilver02,
    RupeeSilver03,
    RupeeSilver04,
    RupeeSilver05,
    RupeeSilver06,
    RupeeSilver07,
    RupeeSilver08,
    RupeeSilver09,
    RupeeSilver10,
    RupeeSilver11,
    RupeeSilver12,
    RupeeSilver13,
    RupeeSilver14,
    RupeeSilver15,
    RupeeSilver16,
    RupeeSilver17,
    RupeeSilver18,
    RupeeSilver19,
    RupeeSilver20,
    RupeeSilver21,
    RupeeSilver22,
    RupeeSilver23,
    RupeeSilver24,
    RupeeSilver25,
    RupeeSilver26,
    RupeeSilver27,
    RupeeSilver28,
    RupeeSilver29,
    RupeeSilver30,
    RupeeSilver31,
    RupeeSilver32,
    RupeeSilver33,
    RupeeSilver34,
    RupeeSilver35,
    RupeeSilver36,
    RupeeSilver37,
    RupeeSilver38,
    RupeeSilver39,
    RupeeSilver40,
    RupeeSilver41,

    RupeeGold01,
    RupeeGold02,
    RupeeGold03,
    RupeeGold04,
    RupeeGold05,
    RupeeGold06,
    RupeeGold07,
    RupeeGold08,
    RupeeGold09,
    RupeeGold10,

    Maiamai001,
    Maiamai002,
    Maiamai003,
    Maiamai004,
    Maiamai005,
    Maiamai006,
    Maiamai007,
    Maiamai008,
    Maiamai009,
    Maiamai010,
    Maiamai011,
    Maiamai012,
    Maiamai013,
    Maiamai014,
    Maiamai015,
    Maiamai016,
    Maiamai017,
    Maiamai018,
    Maiamai019,
    Maiamai020,
    Maiamai021,
    Maiamai022,
    Maiamai023,
    Maiamai024,
    Maiamai025,
    Maiamai026,
    Maiamai027,
    Maiamai028,
    Maiamai029,
    Maiamai030,
    Maiamai031,
    Maiamai032,
    Maiamai033,
    Maiamai034,
    Maiamai035,
    Maiamai036,
    Maiamai037,
    Maiamai038,
    Maiamai039,
    Maiamai040,
    Maiamai041,
    Maiamai042,
    Maiamai043,
    Maiamai044,
    Maiamai045,
    Maiamai046,
    Maiamai047,
    Maiamai048,
    Maiamai049,
    Maiamai050,
    Maiamai051,
    Maiamai052,
    Maiamai053,
    Maiamai054,
    Maiamai055,
    Maiamai056,
    Maiamai057,
    Maiamai058,
    Maiamai059,
    Maiamai060,
    Maiamai061,
    Maiamai062,
    Maiamai063,
    Maiamai064,
    Maiamai065,
    Maiamai066,
    Maiamai067,
    Maiamai068,
    Maiamai069,
    Maiamai070,
    Maiamai071,
    Maiamai072,
    Maiamai073,
    Maiamai074,
    Maiamai075,
    Maiamai076,
    Maiamai077,
    Maiamai078,
    Maiamai079,
    Maiamai080,
    Maiamai081,
    Maiamai082,
    Maiamai083,
    Maiamai084,
    Maiamai085,
    Maiamai086,
    Maiamai087,
    Maiamai088,
    Maiamai089,
    Maiamai090,
    Maiamai091,
    Maiamai092,
    Maiamai093,
    Maiamai094,
    Maiamai095,
    Maiamai096,
    Maiamai097,
    Maiamai098,
    Maiamai099,
    Maiamai100,

    MonsterGuts,
    MonsterHorn,
    MonsterTail,

    // 28 Heart Pieces
    HeartPiece01,
    HeartPiece02,
    HeartPiece03,
    HeartPiece04,
    HeartPiece05,
    HeartPiece06,
    HeartPiece07,
    HeartPiece08,
    HeartPiece09,
    HeartPiece10,
    HeartPiece11,
    HeartPiece12,
    HeartPiece13,
    HeartPiece14,
    HeartPiece15,
    HeartPiece16,
    HeartPiece17,
    HeartPiece18,
    HeartPiece19,
    HeartPiece20,
    HeartPiece21,
    HeartPiece22,
    HeartPiece23,
    HeartPiece24,
    HeartPiece25,
    HeartPiece26,
    HeartPiece27,
    HeartPiece28,

    // 10 Heart Containers
    HeartContainer01,
    HeartContainer02,
    HeartContainer03,
    HeartContainer04,
    HeartContainer05,
    HeartContainer06,
    HeartContainer07,
    HeartContainer08,
    HeartContainer09,
    HeartContainer10,

    // 5 Bottles
    Bottle01,
    Bottle02,
    Bottle03,
    Bottle04,
    Bottle05,

    // 2 Lamps
    Lamp01,
    Lamp02,

    // 4 Swords (Adventures!)
    Sword01,
    Sword02,
    Sword03,
    Sword04,
    //Sword05,

    // 2 Gloves
    Glove01,
    Glove02,

    // 2 Nets
    Net01,
    Net02,

    // 2 Mails
    Mail01,
    Mail02,

    // 4 Master Ore
    OreYellow,
    OreGreen,
    OreBlue,
    OreRed,

    // Sanctuary Keys
    HyruleSanctuaryKey,
    LoruleSanctuaryKey,

    // Eastern Palace
    EasternCompass,
    EasternKeyBig,
    EasternKeySmall01,
    EasternKeySmall02,

    // House of Gales
    GalesCompass,
    GalesKeyBig,
    GalesKeySmall01,
    GalesKeySmall02,
    GalesKeySmall03,
    GalesKeySmall04,

    // Tower of Hera
    HeraCompass,
    HeraKeyBig,
    HeraKeySmall01,
    HeraKeySmall02,

    // Dark Palace
    DarkCompass,
    DarkKeyBig,
    DarkKeySmall01,
    DarkKeySmall02,
    DarkKeySmall03,
    DarkKeySmall04,

    // Swamp Palace
    SwampCompass,
    SwampKeyBig,
    SwampKeySmall01,
    SwampKeySmall02,
    SwampKeySmall03,
    SwampKeySmall04,

    // Skull Woods
    SkullCompass,
    SkullKeyBig,
    SkullKeySmall01,
    SkullKeySmall02,
    SkullKeySmall03,

    // Thieves' Hideout
    ThievesCompass,
    ThievesKeyBig,
    ThievesKeySmall,

    // Ice Ruins
    IceCompass,
    IceKeyBig,
    IceKeySmall01,
    IceKeySmall02,
    IceKeySmall03,

    // Desert Palace
    DesertCompass,
    DesertKeyBig,
    DesertKeySmall01,
    DesertKeySmall02,
    DesertKeySmall03,
    DesertKeySmall04,
    DesertKeySmall05,

    // Turtle Rock
    TurtleCompass,
    TurtleKeyBig,
    TurtleKeySmall01,
    TurtleKeySmall02,
    TurtleKeySmall03,

    // Lorule Castle
    LoruleCastleCompass,
    LoruleCastleKeySmall01,
    LoruleCastleKeySmall02,
    LoruleCastleKeySmall03,
    LoruleCastleKeySmall04,
    LoruleCastleKeySmall05,

    // Dungeon Prizes
    PendantOfPower,
    PendantOfWisdom,
    PendantOfCourage,
    SageGulley,
    SageOren,
    SageSeres,
    SageOsfala,
    SageRosso,
    SageIrene,
    SageImpa,

    // Shop Items (treated as Quest Items) ---------------------------------------------------------

    // Kakariko
    ScootFruit01,
    FoulFruit01,
    Shield01,

    // Lakeside
    ScootFruit02,
    FoulFruit02,
    Shield02,

    // Mysterious Man
    GoldBee01,

    // Thieves' Town
    Bee01,
    GoldBee02,
    Fairy01,
    Shield03,

    // Lorule Lakeside
    Bee02,
    GoldBee03,
    Fairy02,
    Shield04,
}

impl Item {
    pub fn to_game_item(&self) -> game::Item {
        use Item::*;
        match self {
            Empty => game::Item::Empty,
            Bow01 | Bow02 | Bow03 => game::Item::ItemBow,
            Boomerang01 | Boomerang02 => game::Item::ItemBoomerang,
            Hookshot01 | Hookshot02 => game::Item::ItemHookShot,
            Bombs01 | Bombs02 => game::Item::ItemBomb,
            FireRod01 | FireRod02 => game::Item::ItemFireRod,
            IceRod01 | IceRod02 => game::Item::ItemIceRod,
            Hammer01 | Hammer02 => game::Item::ItemHammer,
            Bell => game::Item::ItemBell,
            StaminaScroll => game::Item::GanbariPowerUp,
            SandRod01 | SandRod02 => game::Item::ItemSandRod,
            TornadoRod01 | TornadoRod02 => game::Item::ItemTornadeRod,
            BowOfLight => game::Item::ItemBowLight,
            PegasusBoots => game::Item::DashBoots,
            Flippers => game::Item::ItemMizukaki,
            RaviosBracelet01 => game::Item::RingRental,
            RaviosBracelet02 => game::Item::RingRental,
            HylianShield => game::Item::HyruleShield,
            SmoothGem => game::Item::ItemStoneBeauty,
            LetterInABottle => game::Item::MessageBottle,
            PremiumMilk => game::Item::MilkMatured,
            Pouch => game::Item::Pouch,
            BeeBadge => game::Item::BadgeBee,
            HintGlasses => game::Item::HintGlasses,

            HeartPiece01 | HeartPiece02 | HeartPiece03 | HeartPiece04 | HeartPiece05 | HeartPiece06 | HeartPiece07
            | HeartPiece08 | HeartPiece09 | HeartPiece10 | HeartPiece11 | HeartPiece12 | HeartPiece13
            | HeartPiece14 | HeartPiece15 | HeartPiece16 | HeartPiece17 | HeartPiece18 | HeartPiece19
            | HeartPiece20 | HeartPiece21 | HeartPiece22 | HeartPiece23 | HeartPiece24 | HeartPiece25
            | HeartPiece26 | HeartPiece27 | HeartPiece28 => game::Item::HeartPiece,

            HeartContainer01 | HeartContainer02 | HeartContainer03 | HeartContainer04 | HeartContainer05
            | HeartContainer06 | HeartContainer07 | HeartContainer08 | HeartContainer09 | HeartContainer10 => {
                game::Item::HeartContainer
            },

            Bottle01 | Bottle02 | Bottle03 | Bottle04 | Bottle05 => game::Item::ItemBottle,

            Lamp01 | Lamp02 => game::Item::ItemKandelaar,

            Sword01 | Sword02 | Sword03 | Sword04 => game::Item::ItemSwordLv2,

            Glove01 | Glove02 => game::Item::PowerGlove,

            Net01 | Net02 => game::Item::ItemInsectNet,

            Mail01 | Mail02 => game::Item::ClothesBlue,

            OreYellow => game::Item::OreYellow,
            OreGreen => game::Item::OreGreen,
            OreBlue => game::Item::OreBlue,
            OreRed => game::Item::OreRed,

            // Small Keys
            HyruleSanctuaryKey
            | LoruleSanctuaryKey
            | EasternKeySmall01
            | EasternKeySmall02
            | GalesKeySmall01
            | GalesKeySmall02
            | GalesKeySmall03
            | GalesKeySmall04
            | HeraKeySmall01
            | HeraKeySmall02
            | DarkKeySmall01
            | DarkKeySmall02
            | DarkKeySmall03
            | DarkKeySmall04
            | SwampKeySmall01
            | SwampKeySmall02
            | SwampKeySmall03
            | SwampKeySmall04
            | SkullKeySmall01
            | SkullKeySmall02
            | SkullKeySmall03
            | ThievesKeySmall
            | IceKeySmall01
            | IceKeySmall02
            | IceKeySmall03
            | DesertKeySmall01
            | DesertKeySmall02
            | DesertKeySmall03
            | DesertKeySmall04
            | DesertKeySmall05
            | TurtleKeySmall01
            | TurtleKeySmall02
            | TurtleKeySmall03
            | LoruleCastleKeySmall01
            | LoruleCastleKeySmall02
            | LoruleCastleKeySmall03
            | LoruleCastleKeySmall04
            | LoruleCastleKeySmall05 => game::Item::KeySmall,

            // Big Keys
            EasternKeyBig | GalesKeyBig | HeraKeyBig | DarkKeyBig | SwampKeyBig | SkullKeyBig | ThievesKeyBig
            | IceKeyBig | DesertKeyBig | TurtleKeyBig => game::Item::KeyBoss,

            // Compasses
            EasternCompass | GalesCompass | HeraCompass | DarkCompass | SwampCompass | SkullCompass
            | ThievesCompass | IceCompass | DesertCompass | TurtleCompass | LoruleCastleCompass => game::Item::Compass,

            GreatSpin => game::Item::SpecialMove,
            RupeeGreen => game::Item::RupeeG,
            RupeeBlue => game::Item::RupeeB,
            RupeeRed => game::Item::RupeeR,

            RupeePurple01 | RupeePurple02 | RupeePurple03 | RupeePurple04 | RupeePurple05 | RupeePurple06
            | RupeePurple07 | RupeePurple08 | RupeePurple09 | RupeePurple10 | RupeePurple11 | RupeePurple12
            | RupeePurple13 | RupeePurple14 | RupeePurple15 | RupeePurple16 | RupeePurple17 | RupeePurple18
            | RupeePurple19 | RupeePurple20 => game::Item::RupeePurple,

            RupeeSilver01 | RupeeSilver02 | RupeeSilver03 | RupeeSilver04 | RupeeSilver05 | RupeeSilver06
            | RupeeSilver07 | RupeeSilver08 | RupeeSilver09 | RupeeSilver10 | RupeeSilver11 | RupeeSilver12
            | RupeeSilver13 | RupeeSilver14 | RupeeSilver15 | RupeeSilver16 | RupeeSilver17 | RupeeSilver18
            | RupeeSilver19 | RupeeSilver20 | RupeeSilver21 | RupeeSilver22 | RupeeSilver23 | RupeeSilver24
            | RupeeSilver25 | RupeeSilver26 | RupeeSilver27 | RupeeSilver28 | RupeeSilver29 | RupeeSilver30
            | RupeeSilver31 | RupeeSilver32 | RupeeSilver33 | RupeeSilver34 | RupeeSilver35 | RupeeSilver36
            | RupeeSilver37 | RupeeSilver38 | RupeeSilver39 | RupeeSilver40 | RupeeSilver41 => game::Item::RupeeSilver,

            RupeeGold01 | RupeeGold02 | RupeeGold03 | RupeeGold04 | RupeeGold05 | RupeeGold06 | RupeeGold07
            | RupeeGold08 | RupeeGold09 | RupeeGold10 => game::Item::RupeeGold,

            Maiamai001 | Maiamai002 | Maiamai003 | Maiamai004 | Maiamai005 | Maiamai006 | Maiamai007 | Maiamai008
            | Maiamai009 | Maiamai010 | Maiamai011 | Maiamai012 | Maiamai013 | Maiamai014 | Maiamai015 | Maiamai016
            | Maiamai017 | Maiamai018 | Maiamai019 | Maiamai020 | Maiamai021 | Maiamai022 | Maiamai023 | Maiamai024
            | Maiamai025 | Maiamai026 | Maiamai027 | Maiamai028 | Maiamai029 | Maiamai030 | Maiamai031 | Maiamai032
            | Maiamai033 | Maiamai034 | Maiamai035 | Maiamai036 | Maiamai037 | Maiamai038 | Maiamai039 | Maiamai040
            | Maiamai041 | Maiamai042 | Maiamai043 | Maiamai044 | Maiamai045 | Maiamai046 | Maiamai047 | Maiamai048
            | Maiamai049 | Maiamai050 | Maiamai051 | Maiamai052 | Maiamai053 | Maiamai054 | Maiamai055 | Maiamai056
            | Maiamai057 | Maiamai058 | Maiamai059 | Maiamai060 | Maiamai061 | Maiamai062 | Maiamai063 | Maiamai064
            | Maiamai065 | Maiamai066 | Maiamai067 | Maiamai068 | Maiamai069 | Maiamai070 | Maiamai071 | Maiamai072
            | Maiamai073 | Maiamai074 | Maiamai075 | Maiamai076 | Maiamai077 | Maiamai078 | Maiamai079 | Maiamai080
            | Maiamai081 | Maiamai082 | Maiamai083 | Maiamai084 | Maiamai085 | Maiamai086 | Maiamai087 | Maiamai088
            | Maiamai089 | Maiamai090 | Maiamai091 | Maiamai092 | Maiamai093 | Maiamai094 | Maiamai095 | Maiamai096
            | Maiamai097 | Maiamai098 | Maiamai099 | Maiamai100 => game::Item::Kinsta,

            MonsterGuts => game::Item::LiverPurple,
            MonsterHorn => game::Item::LiverYellow,
            MonsterTail => game::Item::LiverBlue,

            // Dungeon Items
            PendantOfPower => game::Item::PendantPower,
            PendantOfWisdom => game::Item::PendantWisdom,
            PendantOfCourage => game::Item::PendantCourage,
            Charm => game::Item::ZeldaAmulet,
            SageGulley => game::Item::SageGulley,
            SageOren => game::Item::SageOren,
            SageSeres => game::Item::SageSeres,
            SageOsfala => game::Item::SageOsfala,
            SageImpa => game::Item::SageImpa,
            SageIrene => game::Item::SageIrene,
            SageRosso => game::Item::SageRosso,

            // Shop Items
            ScootFruit01 | ScootFruit02 => game::Item::EscapeFruit,
            FoulFruit01 | FoulFruit02 => game::Item::StopFruit,
            Shield01 | Shield02 | Shield03 | Shield04 => game::Item::ItemShield,
            Bee01 | Bee02 => game::Item::Bee,
            GoldBee01 | GoldBee02 | GoldBee03 => game::Item::GoldenBeeForSale,
            Fairy01 | Fairy02 => game::Item::Fairy,
            Quake => game::Item::ItemRentalShield, // Repurposed
        }
    }

    pub fn include_in_sphere_search(self) -> bool {
        matches!(
            self,
            Self::Bow01
                | Self::Bow02
                | Self::Bow03
                | Self::Boomerang01
                | Self::Boomerang02
                | Self::Hookshot01
                | Self::Hookshot02
                | Self::Bombs01
                | Self::Bombs02
                | Self::FireRod01
                | Self::FireRod02
                | Self::IceRod01
                | Self::IceRod02
                | Self::Hammer01
                | Self::Hammer02
                | Self::SandRod01
                | Self::SandRod02
                | Self::TornadoRod01
                | Self::TornadoRod02
                | Self::Bell
                | Self::StaminaScroll
                | Self::BowOfLight
                | Self::PegasusBoots
                | Self::Flippers
                | Self::RaviosBracelet01
                | Self::RaviosBracelet02
                | Self::HylianShield
                | Self::SmoothGem
                | Self::LetterInABottle
                | Self::PremiumMilk
                | Self::GreatSpin
                | Self::Bottle01
                | Self::Bottle02
                | Self::Bottle03
                | Self::Bottle04
                | Self::Lamp01
                | Self::Lamp02
                | Self::Sword01
                | Self::Sword02
                | Self::Sword03
                | Self::Sword04
                | Self::Glove01
                | Self::Glove02
                | Self::Net01
                | Self::Net02
                | Self::Mail01
                | Self::Mail02
                | Self::OreYellow
                | Self::OreGreen
                | Self::OreBlue
                | Self::OreRed
                | Self::HyruleSanctuaryKey
                | Self::LoruleSanctuaryKey
                | Self::EasternKeyBig
                | Self::EasternKeySmall01
                | Self::EasternKeySmall02
                | Self::GalesKeyBig
                | Self::GalesKeySmall01
                | Self::GalesKeySmall02
                | Self::GalesKeySmall03
                | Self::GalesKeySmall04
                | Self::HeraKeyBig
                | Self::HeraKeySmall01
                | Self::HeraKeySmall02
                | Self::DarkKeyBig
                | Self::DarkKeySmall01
                | Self::DarkKeySmall02
                | Self::DarkKeySmall03
                | Self::DarkKeySmall04
                | Self::SwampKeyBig
                | Self::SwampKeySmall01
                | Self::SwampKeySmall02
                | Self::SwampKeySmall03
                | Self::SwampKeySmall04
                | Self::SkullKeyBig
                | Self::SkullKeySmall01
                | Self::SkullKeySmall02
                | Self::SkullKeySmall03
                | Self::ThievesKeyBig
                | Self::ThievesKeySmall
                | Self::IceKeyBig
                | Self::IceKeySmall01
                | Self::IceKeySmall02
                | Self::IceKeySmall03
                | Self::DesertKeyBig
                | Self::DesertKeySmall01
                | Self::DesertKeySmall02
                | Self::DesertKeySmall03
                | Self::DesertKeySmall04
                | Self::DesertKeySmall05
                | Self::TurtleKeyBig
                | Self::TurtleKeySmall01
                | Self::TurtleKeySmall02
                | Self::TurtleKeySmall03
                | Self::LoruleCastleKeySmall01
                | Self::LoruleCastleKeySmall02
                | Self::LoruleCastleKeySmall03
                | Self::LoruleCastleKeySmall04
                | Self::LoruleCastleKeySmall05
                | Self::Charm
                | Self::PendantOfPower
                | Self::PendantOfWisdom
                | Self::PendantOfCourage
                | Self::SageGulley
                | Self::SageOren
                | Self::SageSeres
                | Self::SageOsfala
                | Self::SageRosso
                | Self::SageIrene
                | Self::SageImpa
                | Self::ScootFruit01
                | Self::ScootFruit02
                | Self::GoldBee01
        )
    }

    pub fn get_article(self) -> &'static str {
        use Item::*;
        match self {
            Empty => "",

            Bow01 | Bow02 | Bow03 | Boomerang01 | Boomerang02 | Hookshot01 | Hookshot02 | FireRod01 | FireRod02
            | IceRod01 | IceRod02 | Hammer01 | Hammer02 | SandRod01 | SandRod02 | TornadoRod01 | TornadoRod02 => "the",

            Bombs01 | Bombs02 => "",

            Bell | StaminaScroll | BowOfLight | PegasusBoots => "the",

            Flippers => "",

            RaviosBracelet01 | RaviosBracelet02 => "a",

            HylianShield | SmoothGem | LetterInABottle | PremiumMilk | Pouch | BeeBadge | HintGlasses | GreatSpin => {
                "the"
            },

            RupeeGreen | RupeeBlue | RupeeRed | RupeePurple01 | RupeePurple02 | RupeePurple03 | RupeePurple04
            | RupeePurple05 | RupeePurple06 | RupeePurple07 | RupeePurple08 | RupeePurple09 | RupeePurple10
            | RupeePurple11 | RupeePurple12 | RupeePurple13 | RupeePurple14 | RupeePurple15 | RupeePurple16
            | RupeePurple17 | RupeePurple18 | RupeePurple19 | RupeePurple20 | RupeeSilver01 | RupeeSilver02
            | RupeeSilver03 | RupeeSilver04 | RupeeSilver05 | RupeeSilver06 | RupeeSilver07 | RupeeSilver08
            | RupeeSilver09 | RupeeSilver10 | RupeeSilver11 | RupeeSilver12 | RupeeSilver13 | RupeeSilver14
            | RupeeSilver15 | RupeeSilver16 | RupeeSilver17 | RupeeSilver18 | RupeeSilver19 | RupeeSilver20
            | RupeeSilver21 | RupeeSilver22 | RupeeSilver23 | RupeeSilver24 | RupeeSilver25 | RupeeSilver26
            | RupeeSilver27 | RupeeSilver28 | RupeeSilver29 | RupeeSilver30 | RupeeSilver31 | RupeeSilver32
            | RupeeSilver33 | RupeeSilver34 | RupeeSilver35 | RupeeSilver36 | RupeeSilver37 | RupeeSilver38
            | RupeeSilver39 | RupeeSilver40 | RupeeSilver41 | RupeeGold01 | RupeeGold02 | RupeeGold03 | RupeeGold04
            | RupeeGold05 | RupeeGold06 | RupeeGold07 | RupeeGold08 | RupeeGold09 | RupeeGold10 | Maiamai001
            | Maiamai002 | Maiamai003 | Maiamai004 | Maiamai005 | Maiamai006 | Maiamai007 | Maiamai008 | Maiamai009
            | Maiamai010 | Maiamai011 | Maiamai012 | Maiamai013 | Maiamai014 | Maiamai015 | Maiamai016 | Maiamai017
            | Maiamai018 | Maiamai019 | Maiamai020 | Maiamai021 | Maiamai022 | Maiamai023 | Maiamai024 | Maiamai025
            | Maiamai026 | Maiamai027 | Maiamai028 | Maiamai029 | Maiamai030 | Maiamai031 | Maiamai032 | Maiamai033
            | Maiamai034 | Maiamai035 | Maiamai036 | Maiamai037 | Maiamai038 | Maiamai039 | Maiamai040 | Maiamai041
            | Maiamai042 | Maiamai043 | Maiamai044 | Maiamai045 | Maiamai046 | Maiamai047 | Maiamai048 | Maiamai049
            | Maiamai050 | Maiamai051 | Maiamai052 | Maiamai053 | Maiamai054 | Maiamai055 | Maiamai056 | Maiamai057
            | Maiamai058 | Maiamai059 | Maiamai060 | Maiamai061 | Maiamai062 | Maiamai063 | Maiamai064 | Maiamai065
            | Maiamai066 | Maiamai067 | Maiamai068 | Maiamai069 | Maiamai070 | Maiamai071 | Maiamai072 | Maiamai073
            | Maiamai074 | Maiamai075 | Maiamai076 | Maiamai077 | Maiamai078 | Maiamai079 | Maiamai080 | Maiamai081
            | Maiamai082 | Maiamai083 | Maiamai084 | Maiamai085 | Maiamai086 | Maiamai087 | Maiamai088 | Maiamai089
            | Maiamai090 | Maiamai091 | Maiamai092 | Maiamai093 | Maiamai094 | Maiamai095 | Maiamai096 | Maiamai097
            | Maiamai098 | Maiamai099 | Maiamai100 => "a",

            MonsterGuts => "some",

            MonsterHorn | MonsterTail => "a",

            HeartPiece01 | HeartPiece02 | HeartPiece03 | HeartPiece04 | HeartPiece05 | HeartPiece06 | HeartPiece07
            | HeartPiece08 | HeartPiece09 | HeartPiece10 | HeartPiece11 | HeartPiece12 | HeartPiece13
            | HeartPiece14 | HeartPiece15 | HeartPiece16 | HeartPiece17 | HeartPiece18 | HeartPiece19
            | HeartPiece20 | HeartPiece21 | HeartPiece22 | HeartPiece23 | HeartPiece24 | HeartPiece25
            | HeartPiece26 | HeartPiece27 | HeartPiece28 | HeartContainer01 | HeartContainer02 | HeartContainer03
            | HeartContainer04 | HeartContainer05 | HeartContainer06 | HeartContainer07 | HeartContainer08
            | HeartContainer09 | HeartContainer10 => "a",

            Bottle01 | Bottle02 | Bottle03 | Bottle04 | Bottle05 => "an",

            Lamp01 | Lamp02 => "the",

            Sword01 | Sword02 | Sword03 | Sword04 => "a",

            Glove01 | Glove02 => "a",

            Net01 | Net02 => "the",

            Mail01 | Mail02 => "an",

            OreYellow | OreGreen | OreBlue | OreRed => "some",

            HyruleSanctuaryKey | LoruleSanctuaryKey => "the",

            EasternCompass | EasternKeyBig => "the",

            EasternKeySmall01 | EasternKeySmall02 => "an",

            GalesCompass | GalesKeyBig => "the",

            GalesKeySmall01 | GalesKeySmall02 | GalesKeySmall03 | GalesKeySmall04 => "a",

            HeraCompass | HeraKeyBig => "the",

            HeraKeySmall01 | HeraKeySmall02 => "a",

            DarkCompass | DarkKeyBig => "the",

            DarkKeySmall01 | DarkKeySmall02 | DarkKeySmall03 | DarkKeySmall04 => "a",

            SwampCompass | SwampKeyBig => "the",

            SwampKeySmall01 | SwampKeySmall02 | SwampKeySmall03 | SwampKeySmall04 => "a",

            SkullCompass | SkullKeyBig => "the",

            SkullKeySmall01 | SkullKeySmall02 | SkullKeySmall03 => "a",

            ThievesCompass | ThievesKeyBig | ThievesKeySmall => "the",

            IceCompass | IceKeyBig => "the",

            IceKeySmall01 | IceKeySmall02 | IceKeySmall03 => "an",

            DesertCompass | DesertKeyBig => "the",

            DesertKeySmall01 | DesertKeySmall02 | DesertKeySmall03 | DesertKeySmall04 | DesertKeySmall05 => "a",

            TurtleCompass | TurtleKeyBig => "the",

            TurtleKeySmall01 | TurtleKeySmall02 | TurtleKeySmall03 => "a",

            LoruleCastleCompass => "the",

            LoruleCastleKeySmall01
            | LoruleCastleKeySmall02
            | LoruleCastleKeySmall03
            | LoruleCastleKeySmall04
            | LoruleCastleKeySmall05 => "a",

            PendantOfPower | PendantOfWisdom | PendantOfCourage => "the",

            Charm => "a",

            SageGulley | SageOren | SageSeres | SageOsfala | SageRosso | SageIrene | SageImpa => "",

            ScootFruit01 | FoulFruit01 | Shield01 | ScootFruit02 | FoulFruit02 | Shield02 | GoldBee01 | Bee01
            | GoldBee02 | Fairy01 | Shield03 | Bee02 | GoldBee03 | Fairy02 | Shield04 => "a",
            Quake => "the",
        }
    }

    pub fn as_str(&self) -> &'static str {
        use Item::*;
        match self {
            Empty => "Empty",
            Bow01 | Bow02 | Bow03 => "Bow+",
            Boomerang01 | Boomerang02 => "Boomerang+",
            Hookshot01 | Hookshot02 => "Hookshot+",
            Bombs01 | Bombs02 => "Bombs+",
            FireRod01 | FireRod02 => "Fire Rod+",
            IceRod01 | IceRod02 => "Ice Rod+",
            Hammer01 | Hammer02 => "Hammer+",
            SandRod01 | SandRod02 => "Sand Rod+",
            TornadoRod01 | TornadoRod02 => "Tornado Rod+",
            Bell => "Bell",
            StaminaScroll => "Stamina Scroll",
            BowOfLight => "Bow of Light",
            PegasusBoots => "Pegasus Boots",
            Flippers => "Zora's Flippers",
            RaviosBracelet01 | RaviosBracelet02 => "Ravio's Bracelet+",
            HylianShield => "Hylian Shield",
            SmoothGem => "Smooth Gem",
            LetterInABottle => "Letter in a Bottle",
            PremiumMilk => "Premium Milk",
            Pouch => "Pouch",
            BeeBadge => "Bee Badge",
            HintGlasses => "Hint Glasses",
            GreatSpin => "Great Spin",
            RupeeGreen => "Green Rupee",
            RupeeBlue => "Blue Rupee",
            RupeeRed => "Red Rupee",
            RupeePurple01 | RupeePurple02 | RupeePurple03 | RupeePurple04 | RupeePurple05 | RupeePurple06
            | RupeePurple07 | RupeePurple08 | RupeePurple09 | RupeePurple10 | RupeePurple11 | RupeePurple12
            | RupeePurple13 | RupeePurple14 | RupeePurple15 | RupeePurple16 | RupeePurple17 | RupeePurple18
            | RupeePurple19 | RupeePurple20 => "Purple Rupee",
            RupeeSilver01 | RupeeSilver02 | RupeeSilver03 | RupeeSilver04 | RupeeSilver05 | RupeeSilver06
            | RupeeSilver07 | RupeeSilver08 | RupeeSilver09 | RupeeSilver10 | RupeeSilver11 | RupeeSilver12
            | RupeeSilver13 | RupeeSilver14 | RupeeSilver15 | RupeeSilver16 | RupeeSilver17 | RupeeSilver18
            | RupeeSilver19 | RupeeSilver20 | RupeeSilver21 | RupeeSilver22 | RupeeSilver23 | RupeeSilver24
            | RupeeSilver25 | RupeeSilver26 | RupeeSilver27 | RupeeSilver28 | RupeeSilver29 | RupeeSilver30
            | RupeeSilver31 | RupeeSilver32 | RupeeSilver33 | RupeeSilver34 | RupeeSilver35 | RupeeSilver36
            | RupeeSilver37 | RupeeSilver38 | RupeeSilver39 | RupeeSilver40 | RupeeSilver41 => "Silver Rupee",
            RupeeGold01 | RupeeGold02 | RupeeGold03 | RupeeGold04 | RupeeGold05 | RupeeGold06 | RupeeGold07
            | RupeeGold08 | RupeeGold09 | RupeeGold10 => "Gold Rupee",
            Maiamai001 | Maiamai002 | Maiamai003 | Maiamai004 | Maiamai005 | Maiamai006 | Maiamai007 | Maiamai008
            | Maiamai009 | Maiamai010 | Maiamai011 | Maiamai012 | Maiamai013 | Maiamai014 | Maiamai015 | Maiamai016
            | Maiamai017 | Maiamai018 | Maiamai019 | Maiamai020 | Maiamai021 | Maiamai022 | Maiamai023 | Maiamai024
            | Maiamai025 | Maiamai026 | Maiamai027 | Maiamai028 | Maiamai029 | Maiamai030 | Maiamai031 | Maiamai032
            | Maiamai033 | Maiamai034 | Maiamai035 | Maiamai036 | Maiamai037 | Maiamai038 | Maiamai039 | Maiamai040
            | Maiamai041 | Maiamai042 | Maiamai043 | Maiamai044 | Maiamai045 | Maiamai046 | Maiamai047 | Maiamai048
            | Maiamai049 | Maiamai050 | Maiamai051 | Maiamai052 | Maiamai053 | Maiamai054 | Maiamai055 | Maiamai056
            | Maiamai057 | Maiamai058 | Maiamai059 | Maiamai060 | Maiamai061 | Maiamai062 | Maiamai063 | Maiamai064
            | Maiamai065 | Maiamai066 | Maiamai067 | Maiamai068 | Maiamai069 | Maiamai070 | Maiamai071 | Maiamai072
            | Maiamai073 | Maiamai074 | Maiamai075 | Maiamai076 | Maiamai077 | Maiamai078 | Maiamai079 | Maiamai080
            | Maiamai081 | Maiamai082 | Maiamai083 | Maiamai084 | Maiamai085 | Maiamai086 | Maiamai087 | Maiamai088
            | Maiamai089 | Maiamai090 | Maiamai091 | Maiamai092 | Maiamai093 | Maiamai094 | Maiamai095 | Maiamai096
            | Maiamai097 | Maiamai098 | Maiamai099 | Maiamai100 => "Lost Maiamai",
            MonsterGuts => "Monster Guts",
            MonsterHorn => "Monster Horn",
            MonsterTail => "Monster Tail",
            HeartPiece01 | HeartPiece02 | HeartPiece03 | HeartPiece04 | HeartPiece05 | HeartPiece06 | HeartPiece07
            | HeartPiece08 | HeartPiece09 | HeartPiece10 | HeartPiece11 | HeartPiece12 | HeartPiece13
            | HeartPiece14 | HeartPiece15 | HeartPiece16 | HeartPiece17 | HeartPiece18 | HeartPiece19
            | HeartPiece20 | HeartPiece21 | HeartPiece22 | HeartPiece23 | HeartPiece24 | HeartPiece25
            | HeartPiece26 | HeartPiece27 | HeartPiece28 => "Heart Piece",
            HeartContainer01 | HeartContainer02 | HeartContainer03 | HeartContainer04 | HeartContainer05
            | HeartContainer06 | HeartContainer07 | HeartContainer08 | HeartContainer09 | HeartContainer10 => {
                "Heart Container"
            },
            Bottle01 | Bottle02 | Bottle03 | Bottle04 | Bottle05 => "Empty Bottle",
            Lamp01 | Lamp02 => "Lamp+",
            Sword01 | Sword02 | Sword03 | Sword04 => "Sword+",
            Glove01 | Glove02 => "Strength+",
            Net01 | Net02 => "Net+",
            Mail01 | Mail02 => "Mail+",
            OreYellow | OreGreen | OreBlue | OreRed => "Master Ore",
            HyruleSanctuaryKey => "Hyrule Sewers Key",
            LoruleSanctuaryKey => "Lorule Sewers Key",
            EasternCompass => "Eastern Palace Compass",
            EasternKeyBig => "Eastern Palace Big Key",
            EasternKeySmall01 | EasternKeySmall02 => "Eastern Palace Small Key",
            GalesCompass => "House of Gales Compass",
            GalesKeyBig => "House of Gales Big Key",
            GalesKeySmall01 | GalesKeySmall02 | GalesKeySmall03 | GalesKeySmall04 => "House of Gales Small Key",
            HeraCompass => "Tower of Hera Compass",
            HeraKeyBig => "Tower of Hera Big Key",
            HeraKeySmall01 | HeraKeySmall02 => "Tower of Hera Small Key",
            DarkCompass => "Dark Palace Compass",
            DarkKeyBig => "Dark Palace Big Key",
            DarkKeySmall01 | DarkKeySmall02 | DarkKeySmall03 | DarkKeySmall04 => "Dark Palace Small Key",
            SwampCompass => "Swamp Palace Compass",
            SwampKeyBig => "Swamp Palace Big Key",
            SwampKeySmall01 | SwampKeySmall02 | SwampKeySmall03 | SwampKeySmall04 => "Swamp Palace Small Key",
            SkullCompass => "Skull Woods Compass",
            SkullKeyBig => "Skull Woods Big Key",
            SkullKeySmall01 | SkullKeySmall02 | SkullKeySmall03 => "Skull Woods Small Key",
            ThievesCompass => "Thieves' Hideout Compass",
            ThievesKeyBig => "Thieves' Hideout Big Key",
            ThievesKeySmall => "Thieves' Hideout Small Key",
            IceCompass => "Ice Ruins Compass",
            IceKeyBig => "Ice Ruins Big Key",
            IceKeySmall01 | IceKeySmall02 | IceKeySmall03 => "Ice Ruins Small Key",
            DesertCompass => "Desert Palace Compass",
            DesertKeyBig => "Desert Palace Big Key",
            DesertKeySmall01 | DesertKeySmall02 | DesertKeySmall03 | DesertKeySmall04 | DesertKeySmall05 => {
                "Desert Palace Small Key"
            },
            TurtleCompass => "Turtle Rock Compass",
            TurtleKeyBig => "Turtle Rock Big Key",
            TurtleKeySmall01 | TurtleKeySmall02 | TurtleKeySmall03 => "Turtle Rock Small Key",
            LoruleCastleCompass => "Lorule Castle Compass",
            LoruleCastleKeySmall01
            | LoruleCastleKeySmall02
            | LoruleCastleKeySmall03
            | LoruleCastleKeySmall04
            | LoruleCastleKeySmall05 => "Lorule Castle Small Key",
            PendantOfPower => "Pendant of Power",
            PendantOfWisdom => "Pendant of Wisdom",
            PendantOfCourage => "Pendant of Courage",
            Charm => "Charm",
            SageGulley => "Sage Gulley",
            SageOren => "Sage Oren",
            SageSeres => "Sage Seres",
            SageOsfala => "Sage Osfala",
            SageRosso => "Sage Rosso",
            SageIrene => "Sage Irene",
            SageImpa => "Sage Impa",
            ScootFruit01 | ScootFruit02 => "Scoot Fruit",
            FoulFruit01 | FoulFruit02 => "Foul Fruit",
            Bee01 | Bee02 => "Bee",
            GoldBee01 | GoldBee02 | GoldBee03 => "Golden Bee",
            Fairy01 | Fairy02 => "Fairy",
            Shield01 | Shield02 | Shield03 | Shield04 => "Shield",
            Quake => "Quake Medallion",
        }
    }

    pub fn as_str_colorized(&self) -> String {
        Name.format(self.as_str())
    }
}

impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

// Quest Items ---------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Goal {
    // Bosses -------
    Yuga,
    Margomill,
    Moldorm,
    GemesaurKing,
    Arrghus,
    Knucklemaster,
    Stalblind,
    Grinexx,
    Zaganaga,
    Dharkstare,

    // The rest ------
    RavioSigns,
    RavioShopOpen,
    OpenSanctuaryDoors,
    ShadyGuyTrigger,
    BigBombFlower,
    StylishWomansHouseOpen,
    WomanRoofMaiamai,
    SkullEyeRight,
    SkullEyeLeft,
    ThievesB1DoorOpen,
    ThievesB2DoorOpen,
    ThievesB3WaterDrained,
    TurtleFlipped,
    TurtleAttacked,
    TurtleWall,
    AccessPotionShop,
    AccessMilkBar,
    #[allow(unused)]
    AccessFairyFountain, // todo add to world graph
    AccessHyruleBlacksmith,
    AccessLoruleCastleField,
    LcBombTrial,
    LcTileTrial,
    LcLampTrial,
    LcHookTrial,
    Triforce,
}

impl Goal {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Yuga => "Yuga",
            Self::Margomill => "Margomill",
            Self::Moldorm => "Moldorm",
            Self::GemesaurKing => "Gemesaur King",
            Self::Arrghus => "Arrghus",
            Self::Knucklemaster => "Knucklemaster",
            Self::Stalblind => "Stalblind",
            Self::Grinexx => "Grinexx",
            Self::Zaganaga => "Zaganaga",
            Self::Dharkstare => "Dharkstare",

            Self::RavioSigns => "Ravio's Signs Seen",
            Self::RavioShopOpen => "Ravio's Shop Open",
            Self::OpenSanctuaryDoors => "Sanctuary Doors Opened",
            Self::ShadyGuyTrigger => "Shady Guy Trigger",
            Self::BigBombFlower => "Big Bomb Flower",
            Self::StylishWomansHouseOpen => "Stylish Woman's House Opened",
            Self::WomanRoofMaiamai => "Woman's Roof Maiamai",
            Self::SkullEyeRight => "Skull Woods Right Eye",
            Self::SkullEyeLeft => "Skull Woods Left Eye",
            Self::ThievesB1DoorOpen => "Thieves' Hideout B1 Door Open",
            Self::ThievesB2DoorOpen => "Thieves' Hideout B2 Door Open",
            Self::ThievesB3WaterDrained => "Thieves' Hideout B3 Water Drained",
            Self::TurtleFlipped => "Turtle Flipped",
            Self::TurtleAttacked => "Turtle Bullied",
            Self::TurtleWall => "Turtle Wall",
            Self::AccessPotionShop => "Potion Shop Access",
            Self::AccessMilkBar => "Milk Bar Access",
            Self::AccessFairyFountain => "Fairy Fountain Access",
            Self::AccessHyruleBlacksmith => "Hyrule Blacksmith Access",
            Self::AccessLoruleCastleField => "Lorule Castle Field Access",
            Self::LcBombTrial => "Bomb Trial Complete",
            Self::LcTileTrial => "Tile Trial Complete",
            Self::LcLampTrial => "Lamp Trial Complete",
            Self::LcHookTrial => "Hook Trial Complete",
            Self::Triforce => "Triforce",
        }
    }

    pub fn as_str_colorized(&self) -> String {
        match self {
            Self::Yuga => Green,
            Self::Margomill => Blue,
            Self::Moldorm => Attention,
            Self::GemesaurKing => Green,
            Self::Arrghus => Beige,
            Self::Knucklemaster => Blue,
            Self::Stalblind => Beige,
            Self::Grinexx => Purple,
            Self::Zaganaga => Name,
            Self::Dharkstare => Attention,
            _ => Name,
        }
        .format(self.as_str())
    }
}

/// Weather Vane Item
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum Vane {
    BlacksmithWV,
    DarkPalaceWV,
    DeathMountainHyruleWV,
    DeathMountainLoruleWV,
    DesertPalaceWV,
    EasternPalaceWV,
    GraveyardWV,
    HouseOfGalesWV,
    IceRuinsWV,
    KakarikoVillageWV,
    LoruleCastleWV,
    MiseryMireWV,
    SanctuaryWV,
    SkullWoodsWV,
    SwampPalaceWV,
    ThievesTownWV,
    TowerOfHeraWV,
    TreacherousTowerWV,
    TurtleRockWV,
    VacantHouseWV,
    WitchsHouseWV,
    YourHouseWV,
}

impl From<FillerItem> for Vane {
    fn from(filler_item: FillerItem) -> Self {
        match filler_item {
            FillerItem::Vane(vane) => vane,
            _ => unreachable!("Not a Vane: {:?}", filler_item),
        }
    }
}

impl From<&str> for Vane {
    fn from(vane: &str) -> Self {
        use Vane::*;
        match vane {
            "Your House Weather Vane" => YourHouseWV,
            "Kakariko Village Weather Vane" => KakarikoVillageWV,
            "Eastern Palace Weather Vane" => EasternPalaceWV,
            "House of Gales Weather Vane" => HouseOfGalesWV,
            "Tower of Hera Weather Vane" => TowerOfHeraWV,
            "Witch's House Weather Vane" => WitchsHouseWV,
            "Death Mountain (Hyrule) Weather Vane" => DeathMountainHyruleWV,
            "Desert Palace Weather Vane" => DesertPalaceWV,
            "Sanctuary Weather Vane" => SanctuaryWV,
            "Skull Woods Weather Vane" => SkullWoodsWV,
            "Treacherous Tower Weather Vane" => TreacherousTowerWV,
            "Ice Ruins Weather Vane" => IceRuinsWV,
            "Lorule Castle Weather Vane" => LoruleCastleWV,
            "Graveyard Weather Vane" => GraveyardWV,
            "Thieves' Town Weather Vane" => ThievesTownWV,
            "Dark Palace Weather Vane" => DarkPalaceWV,
            "Blacksmith Weather Vane" => BlacksmithWV,
            "Vacant House Weather Vane" => VacantHouseWV,
            "Misery Mire Weather Vane" => MiseryMireWV,
            "Swamp Palace Weather Vane" => SwampPalaceWV,
            "Turtle Rock Weather Vane" => TurtleRockWV,
            "Death Mountain (Lorule) Weather Vane" => DeathMountainLoruleWV,

            vane => panic!("No Weather Vane named: {}", vane),
        }
    }
}

impl Serialize for Vane {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl Vane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::YourHouseWV => "Your House Weather Vane",
            Self::KakarikoVillageWV => "Kakariko Village Weather Vane",
            Self::EasternPalaceWV => "Eastern Palace Weather Vane",
            Self::HouseOfGalesWV => "House of Gales Weather Vane",
            Self::TowerOfHeraWV => "Tower of Hera Weather Vane",
            Self::WitchsHouseWV => "Witch's House Weather Vane",
            Self::DeathMountainHyruleWV => "Death Mountain (Hyrule) Weather Vane",
            Self::DesertPalaceWV => "Desert Palace Weather Vane",
            Self::SanctuaryWV => "Sanctuary Weather Vane",
            Self::SkullWoodsWV => "Skull Woods Weather Vane",
            Self::TreacherousTowerWV => "Treacherous Tower Weather Vane",
            Self::IceRuinsWV => "Ice Ruins Weather Vane",
            Self::LoruleCastleWV => "Lorule Castle Weather Vane",
            Self::GraveyardWV => "Graveyard Weather Vane",
            Self::ThievesTownWV => "Thieves' Town Weather Vane",
            Self::DarkPalaceWV => "Dark Palace Weather Vane",
            Self::BlacksmithWV => "Blacksmith Weather Vane",
            Self::VacantHouseWV => "Vacant House Weather Vane",
            Self::MiseryMireWV => "Misery Mire Weather Vane",
            Self::SwampPalaceWV => "Swamp Palace Weather Vane",
            Self::TurtleRockWV => "Turtle Rock Weather Vane",
            Self::DeathMountainLoruleWV => "Death Mountain (Lorule) Weather Vane",
        }
    }

    pub fn get_world(self) -> game::World {
        match self {
            Self::YourHouseWV
            | Self::KakarikoVillageWV
            | Self::EasternPalaceWV
            | Self::HouseOfGalesWV
            | Self::TowerOfHeraWV
            | Self::WitchsHouseWV
            | Self::DeathMountainHyruleWV
            | Self::DesertPalaceWV
            | Self::SanctuaryWV => game::World::Hyrule,

            Self::SkullWoodsWV
            | Self::TreacherousTowerWV
            | Self::IceRuinsWV
            | Self::LoruleCastleWV
            | Self::GraveyardWV
            | Self::ThievesTownWV
            | Self::DarkPalaceWV
            | Self::BlacksmithWV
            | Self::VacantHouseWV
            | Self::MiseryMireWV
            | Self::SwampPalaceWV
            | Self::TurtleRockWV
            | Self::DeathMountainLoruleWV => game::World::Lorule,
        }
    }
    pub fn flag(self) -> Flag {
        match self {
            Self::YourHouseWV => Flag::WV_YOUR_HOUSE,
            Self::KakarikoVillageWV => Flag::WV_KAKARIKO_VILLAGE,
            Self::EasternPalaceWV => Flag::WV_EASTERN_PALACE,
            Self::HouseOfGalesWV => Flag::WV_HOUSE_OF_GALES,
            Self::TowerOfHeraWV => Flag::WV_TOWER_OF_HERA,
            Self::WitchsHouseWV => Flag::WV_WITCHS_HOUSE,
            Self::DeathMountainHyruleWV => Flag::WV_DEATH_MTN_HYRULE,
            Self::DesertPalaceWV => Flag::WV_DESERT_PALACE,
            Self::SanctuaryWV => Flag::WV_SANCTUARY,
            Self::SkullWoodsWV => Flag::WV_SKULL_WOODS,
            Self::TreacherousTowerWV => Flag::WV_TREACHEROUS_TOWER,
            Self::IceRuinsWV => Flag::WV_ICE_RUINS,
            Self::LoruleCastleWV => Flag::WV_LORULE_CASTLE,
            Self::GraveyardWV => Flag::WV_GRAVEYARD,
            Self::ThievesTownWV => Flag::WV_THIEVES_TOWN,
            Self::DarkPalaceWV => Flag::WV_DARK_PALACE,
            Self::BlacksmithWV => Flag::WV_BLACKSMITH,
            Self::VacantHouseWV => Flag::WV_VACANT_HOUSE,
            Self::MiseryMireWV => Flag::WV_MISERY_MIRE,
            Self::SwampPalaceWV => Flag::WV_SWAMP_PALACE,
            Self::TurtleRockWV => Flag::WV_TURTLE_ROCK,
            Self::DeathMountainLoruleWV => Flag::WV_DEATH_MTN_LORULE,
        }
    }
}