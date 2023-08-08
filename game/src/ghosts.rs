use strum::EnumIter;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumIter)]
pub enum HintGhost {
    // Hyrule Overworld Hint Ghosts (36) -----------------------------------------------------------
    LostWoodsMaze1,
    LostWoodsMaze2,
    LostWoodsMaze3,
    LostWoods,
    SpectacleRock,
    TowerOfHeraOutside,
    FloatingIsland,
    FireCave,
    MoldormCave,
    ZorasDomain,
    FortuneTellerHyrule,
    Sanctuary,
    GraveyardHyrule,
    WaterfallCave,
    Well,
    ShadyGuy,
    StylishWoman,
    BlacksmithCave,
    EasternRuinsPegs,
    EasternRuinsCave,
    EasternRuinsEntrance,
    RupeeRushHyrule,
    Cuccos,
    SouthBridge,
    SouthernRuins,
    HouseOfGalesIsland,
    HyruleHotfoot,
    Letter,
    StreetPassTree,
    BlacksmithBehind,
    GraveyardLedge,
    DesertEast,
    DesertCenter,
    DesertSouthWest,
    HyruleCastleRocks,
    WitchsHouse,

    // Lorule Overworld Hint Ghosts (20) -----------------------------------------------------------
    SkullWoodsCuccos,
    TreacherousTower,
    IceRuinsOutside,
    LoruleGraveyard,
    DarkRuinsNorth,
    SkullWoodsSouth,
    FortunesChoice,
    VeteranThief,
    FortuneTellerLorule,
    DarkMaze,
    RupeeRushLorule,
    GreatRupeeFairy,
    OctoballDerby,
    VacantHouse,
    MiseryMireLedge,
    SwampPalaceOutsideLeft,
    TurtleBullied,
    TurtleWall,
    TurtleRockOutside,
    DarkPalaceOutside,
    SwampPalaceOutsideRight,
    MiseryMireBridge,
}
