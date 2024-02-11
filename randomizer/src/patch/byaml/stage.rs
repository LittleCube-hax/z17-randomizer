use crate::patch::Patcher;
use crate::{patch::util::*, Result, SeedInfo};
use game::tower_stage::TowerStage;
use game::Course::{self, *};
use log::info;
use macros::fail;
use modinfo::settings::keysy::Keysy;
use modinfo::settings::portal_shuffle::PortalShuffle;
use modinfo::settings::Settings;
use rom::flag::Flag;
use rom::scene::{Arg, Obj, SpawnPoint, Transform, Vec3};

macro_rules! apply {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(::game::Course::$course, $stage - 1)?.stage_mut().get_mut();
            $(action!((stage
                .get_obj_mut($unq)
                .ok_or_else(|| $crate::Error::game("Could not find scene."))?
            ).$action $value);)+
        })+
    };
}

macro_rules! apply_system {
    ($patcher:expr, $($course:ident $stage:literal {
        $([$unq:literal].$action:ident $value:tt,)+
    },)+) => {
        $({
            let stage = $patcher.scene(::game::Course::$course, $stage - 1)?.stage_mut().get_mut();
            $(action!((stage
                .get_system_mut($unq)
                .ok_or_else(|| $crate::Error::game("Could not find scene."))?
            ).$action $value);)+
        })+
    };
}

macro_rules! action {
    ($unq:tt.id($id:literal)) => {
        $unq.set_id($id);
    };
    ($unq:tt.each [$($action:ident $value:tt,)+]) => {
        $(action!($unq.$action $value);)+
    };
    ($unq:tt.active(0)) => {
        $unq.set_active_flag(None);
    };
    ($unq:tt.active($flag:literal)) => {
        $unq.set_active_flag(Flag::Event($flag));
    };
    ($unq:tt.inactive($flag:literal)) => {
        $unq.set_inactive_flag(Flag::Event($flag));
    };
    ($unq:tt.enable($flag:expr)) => {
        $unq.set_enable_flag($flag);
    };
    ($unq:tt.disable($flag:expr)) => {
        $unq.set_disable_flag($flag);
    };
    ($unq:tt.clear_enable_flag()) => {
        $unq.clear_enable_flag();
    };
    ($unq:tt.clear_disable_flag()) => {
        $unq.clear_disable_flag();
    };
    ($unq:tt.clear_active_args()) => {
        $unq.clear_active_args();
    };
    ($unq:tt.clear_inactive_args()) => {
        $unq.clear_inactive_args();
    };
    ($unq:tt.enable()) => {
        $unq.enable();
    };
    ($unq:tt.disable()) => {
        $unq.disable();
    };
    ($unq:tt.call $fn:block) => {
        ($fn)($unq);
    };
    ($unq:tt.set_translate($x:literal, $y:literal, $z:literal)) => {
        $unq.set_translate($x, $y, $z);
    };
    ($unq:tt.redirect($spawn_point:literal, $scene:literal, $scene_index:literal)) => {
        $unq.redirect($spawn_point, $scene, $scene_index);
    };
}

/// Patch Stage BYAML Files
pub fn patch(
    patcher: &mut Patcher, SeedInfo { settings, treacherous_tower_floors: tower_floors, .. }: &SeedInfo,
) -> Result<()> {
    info!("Patching BYAML Files...");

    do_dev_stuff(patcher, settings)?;

    patch_flag_510_effects(patcher)?;
    patch_ravios_shop(patcher)?;
    patch_maiamai_cave(patcher);
    patch_treacherous_tower(patcher, tower_floors)?;
    patch_big_problem_chests(patcher, settings);
    patch_blacksmith_hyrule(patcher);
    patch_castles(patcher);
    patch_chamber_of_sages(patcher);
    patch_dark_maze(patcher);
    patch_kus_domain(patcher);
    patch_letter_in_a_bottle(patcher);
    patch_master_sword(patcher);
    patch_gales_softlock(patcher, settings);
    patch_thief_girl_cave(patcher);
    patch_treasure_dungeons(patcher, settings);
    patch_zora(patcher);
    patch_swamp_palace(patcher);
    patch_hint_ghosts_overworld(patcher)?;
    patch_hint_ghosts_dungeons(patcher)?;

    patch_blacksmith_lorule(patcher);
    patch_trials_door(patcher);
    patch_hildas_study(patcher, settings);

    patch_portal_shuffle(patcher, settings);
    patch_keysy_small(patcher, settings);
    patch_keysy_big(patcher, settings);
    // patch_reverse_desert_palace(patcher, settings);

    patch_big_bomb_flower_skip(patcher, settings);
    patch_no_progression_enemies(patcher, settings);
    patch_lost_woods(patcher);
    // patch_open_lost_woods(patcher);
    patch_magic_shop(patcher);
    patch_ice_ruins(patcher);

    patcher.modify_objs(FieldLight, 18, [disable(529)]);

    // TODO convert to new approach
    apply!(patcher,

        // East Death Mountain
        FieldLight 4 {
            [36].disable(), // Remove Bouldering Guy (pre-Letter in a Bottle)
            [157].clear_active_args(), // Not 100% sure what this does, but removing the association to the 916 flag
            [157].enable(), // Keep Bouldering Guy around
        },

        // Outside Sanctuary
        FieldLight 11 {
            [101].disable(), // Dampe
            [102].disable(), // Seres
            [133].active(1), // Close Church Door by default
            [133].disable(Flag::Event(523)), // Church Door rigged to open when Sanc left switch pulled
        },

        // Sanctuary Dungeon
        CaveLight 18 {
            // 523 is a repurposed flag to control this
            [35].active(523), // Pull Switch
            [37].inactive(523), // Door
            [107].active(523), // TagCameraFocus
            [107].disable(Flag::Event(523)), // TagCameraFocus
        },

        // Sanctuary Church
        IndoorLight 11 {
            [14].clear_enable_flag(), // Church Door
            [14].disable(Flag::Event(523)), // Church Door
            [16].disable(), // Early game Priest
            [20].active(523),
        },

        // Outside witch's house
        FieldLight 14 {
            [123].disable(), // Disable surprised Zora
        },

        // Cucco Ranch
        FieldLight 24 {
            [32].disable(), // Buzz Blob
            [33].disable(), // Buzz Blob
            [34].disable(), // Buzz Blob
            [38].enable(), // Dagger Soldier
            [40].enable(), // Blue Soldier
            [194].disable(), // Buzz Blob
        },

        // StreetPass Tree
        FieldLight 26 {
            [83].disable(), // Buzz Blob
            [84].disable(), // Buzz Blob
        },
        // Outside woods
        FieldLight 32 {
            [47].disable(), // Buzz Blob
            [48].disable(), // Buzz Blob
            [49].disable(), // Buzz Blob
            [50].disable(), // Buzz Blob
            [51].disable(), // Buzz Blob
            [76].disable(), // Disable Blacksmith's Wife
        },
        // Southern Ruins
        FieldLight 33 {
            [69].enable(), // Blue Soldier
            [70].enable(), // Blue Soldier
            [128].enable(), // Blue Soldier
            [206].disable(), // Buzz Blob
            [208].disable(), // Buzz Blob
            [342].disable(), // Buzz Blob
            [344].disable(), // Buzz Blob
            [345].disable(), // Buzz Blob
            [346].disable(), // Buzz Blob
        },

        // Sacred Realm
        FieldLight 43 {
            //[23].disable(), // seichi - "Sanctuary" - Initial text
            //[32].disable(), // Remove Clouds
            [26].disable(), // zelda_talk - Chat after standing up
            [33].disable(), // zelda_talk_b - Wait for Zelda
            [34].disable(), // zelda_talk_c - Last chat before triangles
        },

        // Hyrule Castle
        IndoorLight 12 {
            [23].clear_disable_flag(), // Zelda
            //[24].disable(), // Entry Impa
            [26].disable(), // NPC Soldier
            [28].disable(), // NPC Soldier
            [29].disable(), // NPC Soldier
            [37].disable(), // NPC Soldier
            [38].disable(), // NPC Soldier
            [39].disable(), // NPC Soldier
            // [40].disable(), // Textbox trigger FieldLight_1B_Impa_ACT03_01 (left)
            // [41].disable(), // Textbox trigger FieldLight_1B_Impa_ACT03_02 (right)
            // [43].disable(), // Textbox trigger FieldLight_1B_Impa_ACT03_00 (main exit)
            // [45].disable(), // Disable ZeldaFirstTimeEvent_01 (Charm)
            [46].disable(), // NPC Soldier
            [47].disable(), // NPC Soldier

            [53].clear_enable_flag(), // Blue Soldier
            [54].clear_enable_flag(), // Arrow Soldier
            [56].clear_enable_flag(), // Arrow Soldier
            [57].clear_enable_flag(), // Shooter Spear
            [58].clear_enable_flag(), // Red Spear Soldier
            [60].clear_enable_flag(), // Green Spear Soldier
            [61].clear_enable_flag(), // Green Soldier
            [63].clear_enable_flag(), // Dagger Soldier
            [77].clear_enable_flag(), // Red Spear Soldier
            [78].disable(), // EnemyGreenSoldier
            [79].disable(), // EnemySoldierBlue
            [80].disable(), // EnemySoldierDagger
            [81].disable(), // EnemySoldierGreenSpear
            [82].disable(), // EnemySoldierRedSpear

            //[92].disable(), // NPC Soldier (lower right)
            //[93].disable(), // NPC Soldier (lower left)
            [94].disable(), // Scholar

            //[99].disable(), // Text box trigger FieldLight_1B_Impa_ACT_03_05
            //[100].disable(), // NpcZeldaDemo
            //[101].disable(), // TIMER

            [103].clear_enable_flag(), // Hyrule Paint Soldier
            [104].clear_enable_flag(), // Hyrule Paint Soldier
            [105].clear_enable_flag(), // Hyrule Paint Soldier
            [106].clear_enable_flag(), // Hyrule Paint Soldier
            [107].clear_enable_flag(), // Hyrule Paint Soldier
            [108].clear_enable_flag(), // Hyrule Paint Soldier
            [109].clear_enable_flag(), // Hyrule Paint Soldier
            [110].clear_enable_flag(), // Hyrule Paint Soldier

            [125].disable(), // NPC Solider (upper right)
            [126].disable(), // NPC Solider (upper left)
            [127].disable(), // FieldLight_Right_Soldier_Area
            [128].disable(), // FieldLight_Left_Soldier_Area
            [131].disable(), // NPC Soldier ACT 3
            [132].disable(), // NPC Soldier ACT 3
            [133].disable(), // NPC Soldier
            [134].disable(), // NPC Soldier
            [135].disable(), // NPC Soldier
            [136].disable(), // NPC Soldier
            [137].clear_enable_flag(), // Hyrule Paint Soldier
            [138].clear_enable_flag(), // Hyrule Paint Soldier
            [139].clear_enable_flag(), // Hyrule Paint Soldier
            [140].clear_enable_flag(), // Hyrule Paint Soldier
            [141].clear_enable_flag(), // Hyrule Paint Soldier
            [142].clear_enable_flag(), // Hyrule Paint Soldier
            [143].clear_enable_flag(), // Hyrule Paint Soldier

            // [145].disable(), // Impa stops makes you wait and lets you go see Zelda
            [146].clear_enable_flag(), // Blue Soldier

            // Fix chest to not respawn
            [48].call {|obj: &mut Obj| {
                obj.arg_mut().5 = 3;
            }},
        },

        // Milk Bar
        IndoorLight 15 {
            [12].disable(), // Bouldering Guy stays on the mountain, so remove him from here
            [15].disable(), // Disable post Climber dialogue?
        },
        // Blacksmith's House
        IndoorLight 19 {
            [0x10].disable(), // Remove captain
        },

        // Donkey Cave
        CaveLight 1 {
            [84].disable(), // Remove a MojVolcanicRock to fix a vanilla softlock
        },

        // Eastern Palace
        DungeonEast 3 {
            // Open door after defeating Yuga
            [0x5D].each [
                inactive(250),
                enable(),
            ],
        },

        // Skull Woods B2
        DungeonDokuro 2 {
            [363].disable(), // Remove door that can softlock player
        },

        // Thieves' Hideout
        DungeonHagure 1 {
            [595].disable(), // Thief Girl "We're locked in!" camera
            [1371].disable(), // Spear Boy AreaEventTalk
            [1372].disable(), // Spear Boy
            [1345].disable(), // Thief Girl Text - 1st Zazak Fight
        },

        // Swamp Palace 1F
        // DungeonWater 1 {
        //     [326].disable(), // SE Room shutter door, removed for softlock prevention
        //     [385].disable(), // SW Room shutter door, removed for softlock prevention
        // },

        // Swamp Palace B1
        // DungeonWater 2 {
        //     [255].disable(), // Remove crystal switch, forces merge requirement to complete room to prevent softlock
        // },
    );

    // Change 'System' properties
    apply_system!(patcher,
        // Link's House
        IndoorLight 1 {
            // Default Spawn Point
            [47].call {|obj: &mut Obj| {
                obj.srt_mut().rotate.y = 0.0;
                obj.set_translate(0.0, 0.0, -6.5);
            }},
        },
    );

    Ok(())
}

/// Patch things affected by Flag 510 so that picking up Quake (or starting with it) neatly swaps the world state from
/// post-Eastern Palace to after arriving in Lorule.
fn patch_flag_510_effects(patcher: &mut Patcher) -> Result<()> {
    // Outside Rosso's House
    patcher.modify_objs(
        FieldLight,
        2,
        [
            disable(130), // NpcGameRaceGoal HyruleRace_Goal duplicate TODO GO BACK
        ],
    );

    // Outside Fortune-Teller
    patcher.modify_objs(
        FieldLight,
        9,
        [
            set_disable_flag(86, Flag::SAGE_IRENE), // Buzz Blob
            set_disable_flag(87, Flag::SAGE_IRENE), // Buzz Blob
            set_disable_flag(88, Flag::SAGE_IRENE), // Buzz Blob
            set_disable_flag(89, Flag::SAGE_IRENE), // Buzz Blob
        ],
    );

    // Small Pond
    patcher.modify_objs(
        FieldLight,
        10,
        [
            // Buzz Blob - 54 appears on 246
            // Buzz Blob - 55 appears on 246

            // EnemySoldierDagger - 56 appears on 246, disappears on 510
            // EnemySoldierDagger - 57 appears on 246, disappears on 510
            // EnemySoldierBlue   - 58 appears on 246, disappears on 510
            set_disable_flag(70, Flag::SAGE_IRENE), // Buzz Blob - disappears on 251
            set_disable_flag(71, Flag::SAGE_IRENE), // Buzz Blob - disappears on 251
            set_disable_flag(72, Flag::SAGE_IRENE), // Buzz Blob - disappears on 251

                                                    // EnemySoldierBlue   - 107 appears on 510
                                                    // EnemySoldierGreen  - 108 appears on 510
                                                    // EnemySoldierGreen  - 109 appears on 510
        ],
    );

    // Outside Sanctuary
    patcher.modify_objs(
        FieldLight,
        11,
        [
            set_disable_flag(81, Flag::QUAKE),  // Buzz Blob - disappears on 251
            set_disable_flag(82, Flag::QUAKE),  // Buzz Blob - disappears on 251
            set_disable_flag(83, Flag::QUAKE),  // Buzz Blob - disappears on 251
            set_disable_flag(84, Flag::QUAKE),  // Buzz Blob - disappears on 251
            set_enable_flag(85, Flag::QUAKE),   // EnemySoldierGreenSpear - appears on 251
            set_enable_flag(86, Flag::QUAKE),   // EnemySoldierGreenSpear - appears on 251
            set_enable_flag(87, Flag::QUAKE),   // EnemySoldierBlue       - appears on 251
            set_disable_flag(144, Flag::QUAKE), // Buzz Blob - disappears on 251
            set_enable_flag(145, Flag::QUAKE),  // Buzz Blob - appears on 251
            set_enable_flag(146, Flag::QUAKE),  // Buzz Blob - appears on 251
            set_enable_flag(147, Flag::QUAKE),  // Buzz Blob - appears on 251
        ],
    );

    // Hyrule Graveyard
    patcher.modify_objs(
        FieldLight,
        12,
        [
            set_disable_flag(89, Flag::QUAKE),  // Crowly - disappears on 251
            set_disable_flag(91, Flag::QUAKE),  // Buzz Blob - disappears on 251 (105?)
            set_disable_flag(92, Flag::QUAKE),  // Buzz Blob - disappears on 251 (105?)
            set_enable_flag(93, Flag::QUAKE),   // EnemyShooterArrow - appears on 251
            set_enable_flag(94, Flag::QUAKE),   // EnemyShooterArrow - appears on 251
            set_disable_flag(162, Flag::QUAKE), // EnemyCrowly - disappears on 251
        ],
    );

    // Kakariko Village
    patcher.modify_objs(
        FieldLight,
        16,
        [
            // Papa
            clear_enable_flag(259), // Papa #2 - appears on 310, disappears on 235
            // 287 - Papa #3 - appears on 235 (buy anything from Ravio)
            disable(416), // Papa #1 - appears on 210, disappears on 310
            // Girl
            clear_enable_flag(260), // Girl #2 - appears on 310, disappears on 235
            // 288 - Girl #3 - appears on 235
            disable(415), // Girl #1 - appears on 210, disappears on 310
            // Cuccos
            // 241 - disappears on 235
            // 242 - disappears on 235
            // 246
            // 289 - appears on 235
            // 302
            // 312 - appears on 235
            // 313 - appears on 235
            disable(413), // appears on 210, disappears on 310
            disable(414), // appears on 210, disappears on 310
            // misc.
            clear_disable_flag(264), // MojSignboardRental
            disable(197),            // Disable merchant's Smooth Gem text
            disable(265),            // Disable girl/dad text
            disable(299),            // Disable merchant's bottle text
        ],
    );

    // Blacksmith's Backyard
    patcher.modify_objs(
        FieldLight,
        17,
        [
            set_disable_flag(47, Flag::QUAKE), // Buzz Blob
            set_disable_flag(48, Flag::QUAKE), // Buzz Blob
            set_disable_flag(49, Flag::QUAKE), // Buzz Blob
            set_enable_flag(55, Flag::QUAKE),  // EnemySoldierDagger
            set_enable_flag(56, Flag::QUAKE),  // EnemySoldierGreenSpear
            set_enable_flag(57, Flag::QUAKE),  // EnemySoldierDagger
            set_disable_flag(58, Flag::QUAKE), // Buzz Blob
            set_disable_flag(59, Flag::QUAKE), // Buzz Blob
            set_disable_flag(60, Flag::QUAKE), // Buzz Blob
            set_disable_flag(61, Flag::QUAKE), // Buzz Blob
            disable(75),                       // NpcMapleFlying_1A
            disable(76),                       // NpcMaple
            set_enable_flag(80, Flag::QUAKE),  // Buzz Blob
            set_enable_flag(81, Flag::QUAKE),  // Buzz Blob
            set_enable_flag(82, Flag::QUAKE),  // Buzz Blob
            set_enable_flag(83, Flag::QUAKE),  // Buzz Blob
        ],
    );

    // Hyrule Castle Exterior
    patcher.modify_objs(
        FieldLight,
        18,
        [
            disable(167), // EnemyCrowly
            disable(168), // EnemyCrowly
            disable(175), // Buzz Blob
            disable(177), // Buzz Blob
            disable(178), // Buzz Blob
            disable(179), // Buzz Blob
            // EnemySoldierBlue
            call(186, |obj| {
                obj.clear_enable_flag();
                obj.set_disable_flag(Flag::QUAKE)
            }),
            // EnemySoldierDagger
            call(189, |obj| {
                obj.clear_enable_flag();
                obj.set_disable_flag(Flag::QUAKE)
            }),
            // EnemySoldierBlue
            call(207, |obj| {
                obj.clear_enable_flag();
                obj.set_disable_flag(Flag::QUAKE)
            }),
            set_enable_flag(263, Flag::QUAKE), // EnemyShooterSpear
            set_enable_flag(536, Flag::QUAKE), // EnemyShooterSpear
            // --- //
            // FIXME go through below:
            // --- //
            clear_enable_flag(187), // Dagger Soldier
            clear_enable_flag(190), // Blue Soldier
            disable(194),           // NPC Soldier
            disable(195),           // NPC Soldier
            disable(198),           // NPC Soldier
            clear_enable_flag(204), // Arrow Soldier
            disable(225),           // Paint Soldier
            disable(234),           // Scarecrow
            disable(235),           // Scarecrow
            clear_enable_flag(258), // Bomb Soldier
            clear_enable_flag(260), // Bomb Soldier
            disable(274),           // NPC Soldier
            disable(278),           // NPC Soldier
            disable(279),           // NPC Soldier
            disable(280),           // NPC Soldier
            disable(281),           // Paint Soldier
            disable(282),           // Paint Soldier
            disable(301),           // Paint Soldier
            disable(302),           // Paint Soldier
            disable(303),           // Paint Soldier
            enable(308),            // Paint Soldier
            disable(309),           // Paint Soldier
            disable(369),           // Scarecrow
            disable(370),           // Scarecrow
            disable(371),           // NPC Soldier
            disable(372),           // NPC Soldier
            disable(373),           // NPC Soldier
            disable(395),           // AreaSimpleTalk - Hekiga_Green_Soldier
            disable(401),           // AreaSimpleTalk - Hekiga_fueta_Red
            disable(402),           // AreaSimpleTalk - Hekiga_fueta_Green
            disable(403),           // AreaSimpleTalk - Hekiga_Green_Soldier
            disable(404),           // AreaSimpleTalk - Hekiga_fueta_Green
            disable(488),           // Paint Soldier
            enable(491),            // Paint Soldier
            enable(492),            // Paint Soldier
            enable(493),            // Paint Soldier
            enable(495),            // Paint Soldier
            enable(496),            // Paint Soldier
            enable(497),            // Paint Soldier
            enable(498),            // Paint Soldier
            clear_enable_flag(501), // TagDisableWallIn, prevent merging into barrier
            disable(532),           // Buzz Blob
            disable(533),           // AreaSimpleTalk - Hekiga_fueta_Green
            disable(534),           // AreaSimpleTalk - Hekiga_Blue_Soldier
            disable(535),           // AreaSimpleTalk - Hekiga_Blue_Soldier
        ],
    );

    // Wooden Bridge
    patcher.modify_objs(
        FieldLight,
        19,
        [
            set_disable_flag(27, Flag::QUAKE), // Buzz Blob
            set_disable_flag(28, Flag::QUAKE), // Buzz Blob
            set_disable_flag(29, Flag::QUAKE), // Buzz Blob
            set_disable_flag(30, Flag::QUAKE), // Buzz Blob
            set_disable_flag(32, Flag::QUAKE), // Buzz Blob
            clear_enable_flag(35),             // Arrow Soldier
            clear_enable_flag(36),             // Arrow Soldier
            clear_enable_flag(37),             // Green Spear Solider
            clear_enable_flag(38),             // EnemySoldierDagger
                                               // 83 - EnemySoldierGreenSpear - appears on 510
        ],
    );

    // Eastern Ruins

    // TODO 21
    // TODO 22
    // TODO 23
    // TODO 24
    // TODO 25
    // TODO 26

    // Outside Link's house
    patcher.modify_objs(
        FieldLight,
        27,
        [
            set_enable_flag(154, Flag::QUAKE), // EnemySoldierGreen
            set_enable_flag(155, Flag::QUAKE), // EnemySoldierBlue
            set_enable_flag(156, Flag::QUAKE), // EnemySoldierGreen
            disable(158),                      // Blacksmith's Wife
        ],
    );

    // Stone Bridge
    patcher.modify_objs(
        FieldLight,
        28,
        [
            clear_enable_flag(18), // River Zora
            clear_enable_flag(24), // Octorok
            clear_enable_flag(25), // Octorok
            clear_enable_flag(29), // EnemySoldierDagger (disappears with 510)
            clear_enable_flag(30), // EnemySoldierBlue (disappears with 510)
            disable(58),           // Buzz Blob
            disable(59),           // Buzz Blob
            disable(60),           // Buzz Blob
            disable(61),           // Octorok
            disable(62),           // Octorok
        ],
    );

    // TODO 29
    // TODO 30
    // TODO 31
    // TODO 32
    // TODO 33

    // Hyrule Hotfoot Area
    patcher.modify_objs(
        FieldLight,
        36,
        [
            disable(40), // Post-Irene Hyrule Hotfoot guy (duplicate)
            disable(43), // Letter in a Bottle text
        ],
    );

    Ok(())
}

fn patch_ravios_shop(patcher: &mut Patcher) -> Result<()> {
    patcher.modify_objs(
        IndoorLight,
        1,
        [
            call(15, |obj| obj.arg.3 = 20), // Tornado Slot - Set to 20 Rupee sale price
            call(17, |obj| obj.arg.3 = 10), // Bow Slot     - Set to 10 Rupee sale price
            call(19, |obj| obj.arg.3 = 20), // Hammer Slot  - Set to 20 Rupee sale price
            disable(31),                    // Disable first time goodbye text
            disable(34),                    // Disable 1st Ravio
            disable(35),                    // Disable 1st Sheerow
            disable(46),                    // Disable Ravio's bye-bye
            disable(54),                    // Disable Ravio's welcome
            // Move 2nd Ravio to where 1st Ravio was
            call(56, |obj| {
                obj.clear_enable_flag();
                obj.set_translate(0.0, 0.0, -7.0);
            }),
            // Move 2nd Sheerow to where 1st Ravio was
            call(57, |obj| {
                obj.clear_enable_flag();
                obj.set_translate(-1.0, 0.0, -6.5);
            }),
            disable(58), // Disable Ravio's welcome
            disable(59), // Disable Ravio's welcome
        ],
    );

    Ok(())
}

fn patch_treacherous_tower(patcher: &mut Patcher, tower_floors: &Vec<TowerStage>) -> Result<()> {
    let mut i = 1;
    while i < tower_floors.len() {
        // i - 1 = previous floor, i = current floor
        let floor_prev = tower_floors.get(i - 1).unwrap();
        let floor_cur = tower_floors.get(i).unwrap();

        patcher.modify_objs(
            floor_prev.course,
            floor_prev.stage as u16,
            [redirect(10, SpawnPoint::new(floor_cur.course, floor_cur.stage as i32, 0))],
        );

        i += 1;
    }

    // Give 1000 rupee prize instead of 5000
    patcher.modify_objs(EnemyAttackL, 50, [call(23, |obj| obj.arg.0 = 1)]);

    Ok(())
}

/// Lost Woods
fn patch_lost_woods(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldLight,
        38,
        [
            // Repurpose Flag 375 loading zone to go directly to Pedestal, skipping the Poes and the maze
            call(134, |obj| obj.redirect(SpawnPoint::new(FieldLight, 34, 0))),
            // Move Ghosts from the Maze to the entrance area so they can still give their hints
            call(266, |obj| {
                obj.clp = 4;
                obj.set_translate(7.5, 0.0, 113.5);
            }),
            call(267, |obj| {
                obj.clp = 4;
                obj.set_translate(10.0, 0.0, 108.5);
            }),
            call(268, |obj| {
                obj.clp = 4;
                obj.set_translate(-2.5, 0.0, 109.0);
            }),
        ],
    );
}

#[allow(unused)]
fn patch_open_lost_woods(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldLight,
        1,
        [
            disable(34), // Keep Lost Woods Maze from disappearing after getting Pedestal
        ],
    );

    patcher.modify_objs(
        FieldLight,
        38,
        [
            // Allow entry to maze without All Pendants Flag (375) set
            redirect(259, SpawnPoint::new(FieldLight, 38, 5)),
            // 1st Fork - Enable all Loading Zones
            clear_active_args(137), // North
            clear_active_args(138), // West
            clear_active_args(139), // East
            // 2nd Fork - Enable all Loading Zones
            clear_active_args(168), // North
            clear_active_args(91),  // West
            clear_active_args(89),  // South
            // 3rd Fork - Make all Loading Zones correct
            redirect(110, SpawnPoint::new(FieldLight, 38, 6)), // West
            redirect(111, SpawnPoint::new(FieldLight, 38, 6)), // East
            redirect(112, SpawnPoint::new(FieldLight, 38, 6)), // North
            // 1st Poes
            disable(132),
            disable(133),
            // 2nd Poes
            disable(170),
            disable(185),
            // 3rd Poes
            disable(175),
            disable(186),
            // Redirect normal loading zone to Pedestal to kick player out
            call(127, |obj| {
                obj.redirect(SpawnPoint::new(FieldLight, 38, 0));
                obj.set_translate(-80.25, -1.5, -200.5); // move back slightly
            }),
            // Repurpose Flag 375 loading zone to appear at end of maze, allowing Pedestal access
            call(134, |obj| {
                obj.redirect(SpawnPoint::new(FieldLight, 34, 0));
                obj.set_translate(-80.25, -1.5, -200.0); // take position of OG loading zone
                obj.clp = 5;
            }),
        ],
    );
}

/// Witch's House
fn patch_magic_shop(patcher: &mut Patcher) {
    patcher.modify_objs(
        IndoorLight,
        2,
        [
            disable(19), // Entry_FieldLight16_Obaba_MissingMaple_00
            disable(20), // MagicShopKeeper_StoneBeauty
            disable(21), // Entry_FieldLight16_Obaba_HelpMaple
        ],
    );
}

/// Ice Ruins
fn patch_ice_ruins(patcher: &mut Patcher) {
    // Add extra torch as alternative way to open the annoying door
    patcher.add_obj(
        DungeonIce,
        1,
        Obj {
            arg: Arg(0, 1, 0, 1, 3, 0, 35, 0, 0, 0, 0, 0, 0, 0.0),
            clp: 16,
            flg: (0, 0, 0, 0),
            id: 112,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser: Some(408),
            srt: Transform { scale: Vec3::UNIT, rotate: Vec3::ZERO, translate: Vec3 { x: 15.5, y: 47.5, z: -7.0 } },
            typ: 1,
            unq: 1214,
        },
    );
}

/// Hyrule Blacksmith
fn patch_blacksmith_hyrule(patcher: &mut Patcher) {
    patcher.modify_objs(
        IndoorLight,
        19,
        [
            // Make PackageSword a Chest
            call(12, |obj| {
                obj.clear_active_args();
                obj.set_inactive_flag(Flag::Event(26));
                //obj.clear_disable_flag();
                obj.set_typ(1);
                obj.srt.translate.x = -1.957;
                obj.srt.translate.y = 0.6;
                obj.srt.scale = match obj.id {
                    35 => Vec3 { x: 1.00000, y: 2.00000, z: 2.22222 },
                    34 => Vec3 { x: 0.52632, y: 2.00000, z: 1.66667 },
                    _ => {
                        fail!("PackageSword wasn't a chest")
                    },
                }
            }),
            disable(19), // Map attention
        ],
    );
}

/// Lorule Blacksmith
fn patch_blacksmith_lorule(patcher: &mut Patcher) {
    patcher.modify_objs(
        IndoorDark,
        4,
        [
            clear_active_args(5), // Prevent Blacksmith's Wife making things as if Link just woke up
            disable(7),           // Disable Blacksmith's Wife's dialog
        ],
    );

    patcher.add_obj(
        IndoorDark,
        4,
        Obj::green_warp(
            Flag::Event(430),
            0,
            Some(13),
            22,
            SpawnPoint::new(IndoorDark, 5, 5),
            Vec3 { x: -0.5, y: 0.0, z: -6.0 },
        ),
    )
}

// Chamber of Sages
fn patch_chamber_of_sages(patcher: &mut Patcher) {
    patcher.modify_objs(
        CaveDark,
        10,
        [
            set_46_args(5, Flag::Event(1)),  // Skip needing Flag 430 to function
            set_46_args(35, Flag::Event(1)), // Skip needing Flag 430 to function
            set_46_args(74, Flag::Event(0)), // Staircase
        ],
    );
}

// Ku's Domain
fn patch_kus_domain(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldDark,
        7,
        [
            call(55, |obj| {
                obj.set_typ(4); // changed to chest automatically, set typ here
            }),
            disable(66), // rupee throw camera
        ],
    );
}

// Mini-Dungeons
fn patch_treasure_dungeons(patcher: &mut Patcher, settings: &Settings) {
    // Remove Mini-Dungeon entry cutscenes only when CSMC is off (since they show the chests)
    if !settings.chest_size_matches_contents {
        patcher.modify_objs(AttractionLight, 1, [disable(15)]);
        patcher.modify_objs(AttractionLight, 2, [disable(54)]);
        patcher.modify_objs(AttractionLight, 3, [disable(47)]);
        patcher.modify_objs(AttractionLight, 4, [disable(118)]);
        patcher.modify_objs(AttractionLight, 5, [disable(26)]);
    }
}

// Zora
fn patch_zora(patcher: &mut Patcher) {
    // Lake Hylia
    patcher.modify_objs(
        FieldLight,
        35,
        [
            enable(151), // Zora outside House of Gales
        ],
    );
}

// Swamp Palace
fn patch_swamp_palace(patcher: &mut Patcher) {
    patcher.modify_objs(
        DungeonWater,
        2,
        [call(633, |obj| {
            obj.clp = 3; // Fix the impossible Rupee
        })],
    );
}

// Enable All Overworld Hint Ghosts
fn patch_hint_ghosts_overworld(patcher: &mut Patcher) -> Result<()> {
    patcher.modify_objs(FieldLight, 14, [enable(126)]); // Witch's House
    patcher.modify_objs(FieldLight, 16, [enable(407)]); // Shady Guy (Kakariko)
    patcher.modify_objs(FieldLight, 17, [enable(96)]); // Behind Blacksmith
    patcher.modify_objs(FieldDark, 35, [enable(205)]); // Bullied Turtle
    // todo add 510 ghosts (HFI, etc.)

    Ok(())
}

// Hide All Dungeon Hint Ghosts
fn patch_hint_ghosts_dungeons(patcher: &mut Patcher) -> Result<()> {
    // Eastern
    patcher.modify_objs(
        DungeonEast,
        1,
        [disable(251), disable(252), disable(253), disable(254), disable(255), disable(256), disable(257)],
    );
    patcher.modify_objs(
        DungeonEast,
        2,
        [
            disable(235),
            disable(236),
            disable(237),
            disable(238),
            disable(239),
            disable(240),
            disable(241),
            disable(243),
        ],
    );
    patcher.modify_objs(DungeonEast, 3, [disable(92)]);

    // Gales
    patcher.modify_objs(DungeonWind, 1, [disable(390), disable(391), disable(392), disable(393), disable(394)]);
    patcher.modify_objs(DungeonWind, 2, [disable(327), disable(328), disable(329), disable(474)]);
    patcher.modify_objs(DungeonWind, 3, [disable(509), disable(510), disable(511), disable(512)]);

    // Hera
    patcher.modify_objs(
        DungeonHera,
        1,
        [
            disable(862),
            disable(863),
            disable(864),
            disable(865),
            disable(866),
            disable(867),
            disable(868),
            disable(869),
            disable(870),
            disable(871),
        ],
    );

    // Hyrule Castle
    patcher.modify_objs(DungeonCastle, 2, [disable(64)]);

    // Dark
    patcher.modify_objs(
        DungeonDark,
        1,
        [
            disable(208),
            disable(209),
            disable(210),
            disable(211),
            disable(212),
            disable(213),
            disable(214),
            disable(216),
            disable(217),
            disable(218),
        ],
    );
    patcher.modify_objs(
        DungeonDark,
        2,
        [
            disable(170),
            disable(171),
            disable(172),
            disable(173),
            disable(174),
            disable(175),
            disable(176),
            disable(177),
            disable(204),
        ],
    );
    patcher.modify_objs(
        DungeonDark,
        3,
        [disable(225), disable(226), disable(227), disable(228), disable(229), disable(230), disable(231)],
    );

    // Swamp
    patcher.modify_objs(DungeonWater, 1, [disable(446), disable(447), disable(448), disable(449)]);
    patcher.modify_objs(DungeonWater, 2, [disable(565), disable(566), disable(567), disable(589), disable(660)]);

    // Skull
    patcher.modify_objs(DungeonDokuro, 1, [disable(765), disable(766), disable(767), disable(768), disable(776)]);
    patcher.modify_objs(DungeonDokuro, 2, [disable(480), disable(481)]);

    // Thieves'
    patcher.modify_objs(
        DungeonHagure,
        1,
        [disable(1364), disable(1365), disable(1366), disable(1367), disable(1368), disable(1416)],
    );

    // Turtle
    patcher.modify_objs(DungeonKame, 1, [disable(247), disable(248), disable(249), disable(250)]);
    patcher.modify_objs(DungeonKame, 2, [disable(234), disable(235), disable(236), disable(237), disable(263)]);

    // Desert
    patcher.modify_objs(
        DungeonSand,
        1,
        [disable(598), disable(599), disable(600), disable(601), disable(602), disable(616)],
    );
    patcher.modify_objs(DungeonSand, 2, [disable(668), disable(669), disable(670), disable(671)]);
    patcher.modify_objs(DungeonSand, 3, [disable(293), disable(294)]);

    // Ice
    patcher.modify_objs(
        DungeonIce,
        1,
        [
            disable(900),
            disable(901),
            disable(902),
            disable(903),
            disable(904),
            disable(906),
            disable(907),
            disable(908),
            disable(909),
            disable(910),
            disable(911),
            disable(1145),
        ],
    );

    // Lorule Castle
    patcher.modify_objs(
        DungeonGanon,
        1,
        [
            disable(1230),
            disable(1232),
            disable(1233),
            disable(1234),
            disable(1235),
            disable(1236),
            disable(1237),
            disable(1238),
            disable(1239),
            disable(1241),
            disable(1242),
            disable(1371),
            disable(1602),
            disable(1607),
        ],
    );

    Ok(())
}

/// Small Keysy - Remove all Small Key-locked doors
fn patch_keysy_small(patcher: &mut Patcher, settings: &Settings) {
    match settings.keysy {
        Keysy::SmallKeysy | Keysy::AllKeysy => {},
        _ => return,
    }

    patcher.modify_objs(CaveLight, 18, [disable(60)]); // Hyrule Sewers

    patcher.modify_objs(DungeonEast, 1, [disable(114)]); // Eastern Palace 1F
    patcher.modify_objs(DungeonEast, 2, [disable(34)]); // Eastern Palace 2F

    patcher.modify_objs(DungeonWind, 1, [disable(74)]); // House of Gales 1F
    patcher.modify_objs(DungeonWind, 2, [disable(150)]); // House of Gales 2F
    patcher.modify_objs(DungeonWind, 2, [disable(153)]); // House of Gales 2F
    patcher.modify_objs(DungeonWind, 3, [disable(54)]); // House of Gales 3F

    patcher.modify_objs(DungeonHera, 1, [disable(245)]); // Tower of Hera 3F
    patcher.modify_objs(DungeonHera, 1, [disable(335)]); // Tower of Hera 7F

    patcher.modify_objs(AttractionDark, 2, [disable(60)]); // Lorule Sewers

    patcher.modify_objs(DungeonDark, 2, [disable(26)]); // Dark Palace 1F
    patcher.modify_objs(DungeonDark, 2, [disable(231)]); // Dark Palace 1F
    patcher.modify_objs(DungeonDark, 1, [disable(108)]); // Dark Palace B1
    patcher.modify_objs(DungeonDark, 1, [disable(142)]); // Dark Palace B1

    // Swamp B1 Key doors are two sided, so there are twice as many
    patcher.modify_objs(DungeonWater, 2, [disable(65)]); // Swamp Palace B1
    patcher.modify_objs(DungeonWater, 2, [disable(205)]); // Swamp Palace B1 - Center Room
    patcher.modify_objs(DungeonWater, 2, [disable(207)]); // Swamp Palace B1 - Center Room
    patcher.modify_objs(DungeonWater, 2, [disable(208)]); // Swamp Palace B1
    patcher.modify_objs(DungeonWater, 2, [disable(209)]); // Swamp Palace B1 - Center Room
    patcher.modify_objs(DungeonWater, 2, [disable(210)]); // Swamp Palace B1

    patcher.modify_objs(DungeonDokuro, 1, [disable(240)]); // Skull Woods B1
    patcher.modify_objs(DungeonDokuro, 1, [disable(332)]); // Skull Woods B1
    patcher.modify_objs(DungeonDokuro, 2, [disable(223)]); // Skull Woods B2

    patcher.modify_objs(DungeonHagure, 1, [disable(542)]); // Thieves' Hideout B2

    patcher.modify_objs(DungeonKame, 2, [disable(116)]); // Turtle Rock B1
    patcher.modify_objs(DungeonKame, 2, [disable(118)]); // Turtle Rock B1
    patcher.modify_objs(DungeonKame, 2, [disable(229)]); // Turtle Rock B1

    patcher.modify_objs(DungeonSand, 1, [disable(77)]); // Desert Palace 1F
    patcher.modify_objs(DungeonSand, 1, [disable(419)]); // Desert Palace 1F
    patcher.modify_objs(DungeonSand, 2, [disable(259)]); // Desert Palace 2F
    patcher.modify_objs(DungeonSand, 2, [disable(463)]); // Desert Palace 2F
    patcher.modify_objs(DungeonSand, 3, [disable(156)]); // Desert Palace 3F

    patcher.modify_objs(DungeonIce, 1, [disable(116)]); // Ice Ruins B1
    patcher.modify_objs(DungeonIce, 1, [disable(169)]); // Ice Ruins B1
    patcher.modify_objs(DungeonIce, 1, [disable(230)]); // Ice Ruins B2

    patcher.modify_objs(DungeonGanon, 1, [disable(416)]); // Lorule Castle
    patcher.modify_objs(DungeonGanon, 1, [disable(990)]); // Lorule Castle
    patcher.modify_objs(DungeonGanon, 1, [disable(1090)]); // Lorule Castle
    patcher.modify_objs(DungeonGanon, 1, [disable(1104)]); // Lorule Castle
    patcher.modify_objs(DungeonGanon, 1, [disable(1307)]); // Lorule Castle
}

/// Big Keysy - Remove all huge doors
fn patch_keysy_big(patcher: &mut Patcher, settings: &Settings) {
    match settings.keysy {
        Keysy::BigKeysy | Keysy::AllKeysy => {},
        _ => return,
    }

    patcher.modify_objs(DungeonEast, 2, [disable(26)]); // Eastern Palace 2F
    patcher.modify_objs(DungeonWind, 3, [disable(401)]); // House of Gales 3F
    patcher.modify_objs(DungeonHera, 1, [disable(740)]); // Tower of Hera 11F
    patcher.modify_objs(DungeonDark, 1, [disable(38)]); // Dark Palace B1
    patcher.modify_objs(DungeonWater, 1, [disable(29)]); // Swamp Palace 1F
    patcher.modify_objs(DungeonDokuro, 2, [disable(106)]); // Skull Woods B2
    patcher.modify_objs(DungeonHagure, 1, [disable(531)]); // Thieves' Hideout
    patcher.modify_objs(DungeonKame, 2, [disable(28)]); // Turtle Rock B1
    patcher.modify_objs(DungeonSand, 3, [disable(9)]); // Desert Palace 3F
    patcher.modify_objs(DungeonIce, 1, [disable(291)]); // Ice Ruins B4
}

fn patch_portal_shuffle(patcher: &mut Patcher, settings: &Settings) {
    if settings.portal_shuffle == PortalShuffle::Off {
        return;
    }

    // Remove the Curtain
    //patcher.modify_objs(IndoorLight, 7, [disable(26)]);

    // Eastern Ruins SE Portal Blockage
    patcher.modify_objs(
        FieldLight,
        30,
        [call(57, |obj| {
            obj.set_active_flag(Flag::PORTAL_EASTERN_RUINS_SE);
            obj.set_disable_flag(Flag::PORTAL_EASTERN_RUINS_SE);
        })],
    );

    // Dark Ruins SE Portal
    patcher.modify_objs(
        FieldDark,
        30,
        [call(37, |obj| {
            obj.set_active_flag(Flag::PORTAL_DARK_MAZE_SE);
            obj.set_enable_flag(Flag::QUAKE);
            obj.set_disable_flag(Flag::PORTAL_DARK_MAZE_SE);
        })],
    );

    // Desert North Portal
    patcher.modify_objs(
        FieldLight,
        31,
        [call(65, |obj| {
            obj.set_active_flag(Flag::PORTAL_DESERT_NORTH);
            obj.set_enable_flag(Flag::QUAKE);
            obj.set_disable_flag(Flag::PORTAL_DESERT_NORTH);
        })],
    );

    // Lorule Graveyard Ledge Portal
    patcher.modify_objs(
        FieldDark,
        12,
        [call(19, |obj| {
            obj.set_active_flag(Flag::PORTAL_GRAVEYARD_LEDGE_LORULE);
            obj.set_enable_flag(Flag::QUAKE);
            obj.set_disable_flag(Flag::PORTAL_GRAVEYARD_LEDGE_LORULE);
        })],
    );
}

fn patch_trials_door(patcher: &mut Patcher) {
    let door_flag = Flag::Event(421);

    // Lorule Castle side
    patcher.modify_objs(
        DungeonGanon,
        1,
        [
            set_46_args(158, door_flag),
            //set_disable_flag(158, door_flag),
        ],
    );

    // Hilda's Study side
    patcher.modify_objs(IndoorDark, 5, [set_46_args(4, door_flag), clear_disable_flag(4)]);
}

fn patch_hildas_study(patcher: &mut Patcher, settings: &Settings) {
    // Warp back to Demo4 so player still has path to Lorule Blacksmith.
    patcher.add_obj(
        IndoorDark,
        5,
        Obj::green_warp(
            Flag::Event(708),
            1,
            Some(14),
            48,
            SpawnPoint::new(Demo, 4, 0),
            Vec3 { x: 63.0, y: 0.0, z: -14.5 },
        ),
    );

    // Add spawn point for the warp (index 5)
    patcher.add_system(IndoorDark, 5, Obj::spawn_point(5, 1, 15, 49, Vec3 { x: 63.0, y: 0.0, z: -14.5 }));

    if settings.progressive_bow_of_light {
        return;
    }

    // Bow of Light Hint Ghost
    patcher.add_obj(
        IndoorDark,
        5,
        Obj {
            arg: Arg(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0),
            clp: 1,
            flg: (0, 0, 0, 0),
            id: 235,
            lnk: vec![],
            nme: Some("HintGhostDark/HintGhost_FieldDark_2C_014".to_owned()),
            ril: vec![],
            ser: Some(16),
            srt: Transform {
                scale: Vec3::UNIT,
                rotate: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                translate: Vec3 { x: 67.0, y: 0.0, z: -14.5 },
            },
            typ: 3,
            unq: 50,
        },
    );
}

/// Reverse Desert Palace
///
/// Make Desert Palace make sense as a dungeon for glitchless players who enter from 3F.
/// This mostly involves making Key doors (small + boss) two-sided so that keys are required to pass through them no
/// matter which way the player approach them.
///
/// Currently not being used as I'm keeping the DP/Z Portals vanilla for the first release.
#[allow(unused)]
fn patch_reverse_desert_palace(patcher: &mut Patcher, settings: &Settings) {
    if settings.portal_shuffle == PortalShuffle::Off {
        return;
    }

    // 1F Small Key Door
    let (unq, ser) = patcher.find_objs_unq_ser(DungeonSand, 1);
    patcher.add_obj(
        DungeonSand,
        1,
        Obj {
            arg: Arg(0, 0, 0, 0, 0, 3, 0, 13, 0, 0, 0, 0, 0, 0.0),
            clp: 6,
            flg: (0, 0, 0, 0),
            id: 37,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser,
            srt: Transform {
                scale: Vec3::UNIT,
                rotate: Vec3 { x: 0.0, y: 270.0, z: 0.0 },
                translate: Vec3 { x: -28.5, y: 0.0, z: -21.0 },
            },
            typ: 1,
            unq,
        },
    );

    // 1F Large Rock in front of Key Door
    let (unq_objs, ser_objs) = patcher.find_objs_unq_ser(DungeonSand, 1);
    let (unq_system, ser_system) = (2, None);
    patcher.add_obj(
        DungeonSand,
        1,
        Obj {
            arg: Arg(0, 0, 0, 0, 3, 0, 13, 0, 0, 0, 0, 0, 0, 0.0),
            clp: 6,
            flg: (0, 0, 0, 0),
            id: 249,
            lnk: vec![(unq_system, -1, -1)],
            nme: None,
            ril: vec![],
            ser: ser_objs,
            srt: Transform { scale: Vec3::UNIT, rotate: Vec3::ZERO, translate: Vec3 { x: -29.5, y: 0.0, z: -21.0 } },
            typ: 9,
            unq: unq_objs,
        },
    );
    patcher.add_system(
        DungeonSand,
        1,
        Obj {
            arg: Arg(0, 0, 0, 0, 3, 0, 13, 0, 0, 0, 0, 0, 0, 0.0),
            clp: 6,
            flg: (0, 0, 0, 0),
            id: 249,
            lnk: vec![(unq_system, -1, -1)],
            nme: None,
            ril: vec![],
            ser: ser_system,
            srt: Transform { scale: Vec3::UNIT, rotate: Vec3::ZERO, translate: Vec3 { x: -29.5, y: 0.0, z: -21.0 } },
            typ: 9,
            unq: unq_system,
        },
    );

    // 2F Small Key Door
    let (unq, ser) = patcher.find_objs_unq_ser(DungeonSand, 2);
    patcher.add_obj(
        DungeonSand,
        2,
        Obj {
            arg: Arg(0, 0, 0, 0, 0, 3, 0, 34, 0, 0, 0, 0, 0, 0.0),
            clp: 2,
            flg: (0, 0, 0, 0),
            id: 37,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser,
            srt: Transform {
                scale: Vec3::UNIT,
                rotate: Vec3 { x: 0.0, y: 270.0, z: 0.0 },
                translate: Vec3 { x: -19.5, y: 5.0, z: -46.0 },
            },
            typ: 1,
            unq,
        },
    );

    // 3F Small Key Door
    let (unq, ser) = patcher.find_objs_unq_ser(DungeonSand, 3);
    patcher.add_obj(
        DungeonSand,
        3,
        Obj {
            arg: Arg(0, 0, 0, 0, 0, 3, 0, 25, 0, 0, 0, 0, 0, 0.0),
            clp: 4,
            flg: (0, 0, 0, 0),
            id: 37,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser,
            srt: Transform { scale: Vec3::UNIT, rotate: Vec3::ZERO, translate: Vec3 { x: -20.5, y: 5.0, z: -67.5 } },
            typ: 1,
            unq,
        },
    );

    // 3F Boss Key Door
    let (unq, ser) = patcher.find_objs_unq_ser(DungeonSand, 3);
    patcher.add_obj(
        DungeonSand,
        3,
        Obj {
            arg: Arg(0, 0, 0, 0, 3, 3, 25, 29, 0, 0, 0, 0, 0, 0.0),
            clp: 1,
            flg: (0, 0, 0, 0),
            id: 2,
            lnk: vec![],
            nme: None,
            ril: vec![],
            ser,
            srt: Transform {
                scale: Vec3::UNIT,
                rotate: Vec3 { x: 0.0, y: 90.0, z: 0.0 },
                translate: Vec3 { x: 19.5, y: 4.92188, z: -45.5 },
            },
            typ: 1,
            unq,
        },
    );
}

fn patch_castles(patcher: &mut Patcher) {
    let yuga_defeated = Flag::Event(420); // Set after Yuga 2 defeated
    let hc_31 = Flag::Course(31); // Also set after Yuga 2 defeated

    // Hyrule Castle (exterior)
    patcher.modify_objs(
        FieldLight,
        18,
        [
            // Barrier
            set_46_args(165, Flag::Event(1)), // Enable Barrier from game start
            disable(505),                     // Barrier "would you like to save?" text
                                              // TODO Sahas still talking at us after Master Sword get
        ],
    );

    // Open IHC Dungeon entrance
    patcher.modify_objs(
        FieldLight,
        18,
        [
            enable(155),  // HC dungeon loading zone
            disable(393), // HC dungeon door
        ],
    );

    // 2F (there is no 1F of the dungeon)
    patcher.modify_objs(
        DungeonCastle,
        1,
        [
            set_disable_flag(19, hc_31), // Armos Statue
            call(35, move |obj| {
                // Warp
                obj.set_active_flag(hc_31);
                obj.set_enable_flag(hc_31);
            }),
        ],
    );

    // 4F
    patcher.modify_objs(
        DungeonCastle,
        7,
        [
            enable(19), // Green Soldier
            enable(20), // Green Soldier
            enable(21), // Red Spear Soldier
            enable(22), // Red Spear Soldier
        ],
    );

    // 7F
    patcher.modify_objs(
        DungeonCastle,
        5,
        [call(18, move |obj| {
            // warp
            obj.set_active_flag(hc_31);
            obj.set_enable_flag(hc_31);
        })],
    );

    // 8F
    patcher.modify_objs(
        DungeonCastle,
        6,
        [
            set_disable_flag(20, hc_31), // Rewire entrance door to stay open
            disable(28),                 // no revisits door
        ],
    );

    // Zelda's Study
    patcher.modify_objs(
        IndoorLight,
        7,
        [
            // No backtracking door
            call(27, move |obj| {
                obj.clear_enable_flag();
                obj.set_disable_flag(yuga_defeated);
            }),
            //set_disable_flag(26, hacky_flag), // Curtain
            //set_disable_flag(29, hacky_flag), // AreaDisableWallIn
            // disable(26), // Curtain
            // disable(29), // AreaDisableWallIn
            // Portal
            call(10, move |obj| {
                obj.arg.3 = 0; // Prevent Long Portal Transition
            }),
            // Fairies
            clear_enable_flag(18),
            clear_enable_flag(19),
            clear_enable_flag(20),
            clear_enable_flag(21),
            // Hearts (Painted)
            clear_disable_flag(36),
            clear_disable_flag(41),
            clear_disable_flag(42),
            clear_disable_flag(43),
        ],
    );
    const ZELDA_SPAWN_INDEX: i32 = 5;
    const GATE_SPAWN_INDEX: i32 = 64;

    // Hyrule Castle Gate Convenience Warp
    patcher.add_obj(
        FieldLight,
        18,
        Obj::green_warp(
            Flag::HC_YUGA_DEFEATED,
            0,
            Some(164),
            537,
            SpawnPoint::new(IndoorLight, 7, ZELDA_SPAWN_INDEX),
            Vec3 { x: 0.0, y: 0.0, z: 8.0 },
        ),
    );
    // Zelda's Study Convenience Warp
    patcher.add_obj(
        IndoorLight,
        7,
        Obj::green_warp(
            Flag::HC_YUGA_DEFEATED,
            0,
            Some(19),
            44,
            SpawnPoint::new(FieldLight, 18, GATE_SPAWN_INDEX),
            Vec3 { x: 0.0, y: 0.0, z: -5.8 },
        ),
    );
    // Hyrule Castle Gate spawn point
    patcher.add_system(
        FieldLight,
        18,
        Obj::spawn_point(GATE_SPAWN_INDEX, 0, 165, 538, Vec3 { x: 0.0, y: 0.0, z: 8.0 }),
    );
    // Zelda's Study spawn point
    patcher.add_system(
        IndoorLight,
        7,
        Obj::spawn_point(ZELDA_SPAWN_INDEX, 0, 20, 45, Vec3 { x: 0.0, y: 0.0, z: -5.8 }),
    );

    // Hilda's Study
    patcher.modify_objs(
        IndoorDark,
        5,
        [
            set_disable_flag(4, Flag::TRIFORCE_OF_COURAGE),  // Trial's Door
            clear_enable_flag(12),                           // Yuga revives Ganon cutscene
            set_disable_flag(12, Flag::TRIFORCE_OF_COURAGE), // Yuga revives Ganon cutscene
            set_enable_flag(34, Flag::TRIFORCE_OF_COURAGE),  // Throne Room Loading Zone
            set_enable_flag(23, Flag::TRIFORCE_OF_COURAGE),  // Skull (top right, controller obj)
        ],
    );

    // Hilda's Study (system)
    patcher.modify_system(
        IndoorDark,
        5,
        [
            set_enable_flag(23, Flag::TRIFORCE_OF_COURAGE), // Skull (top right, controller system obj)
            set_enable_flag(24, Flag::TRIFORCE_OF_COURAGE), // Skull (middle right)
            set_enable_flag(25, Flag::TRIFORCE_OF_COURAGE), // Skull (bottom right)
            set_enable_flag(41, Flag::TRIFORCE_OF_COURAGE), // Skull (bottom left)
            set_enable_flag(46, Flag::TRIFORCE_OF_COURAGE), // Skull (middle left)
            set_enable_flag(47, Flag::TRIFORCE_OF_COURAGE), // Skull (top left)
        ],
    );

    // Lorule Castle
    patcher.modify_objs(
        DungeonGanon,
        1,
        [
            disable(265),            // Trial's Door camera pan
            clear_enable_flag(1193), // Respawn Trial's Skip big rock upon leaving the room
        ],
    );

    // Throne Room
    patcher.modify_objs(
        DungeonBoss,
        1,
        [
            // fight start trigger
            call(10, move |obj| {
                obj.set_enable_flag(Flag::TRIFORCE_OF_COURAGE);
                obj.set_active_flag(Flag::TRIFORCE_OF_COURAGE);
            }),
            clear_enable_flag(27), // Hilda
            clear_enable_flag(41), // camera offset
            clear_enable_flag(43), // NpcAttention1
            clear_enable_flag(48), // ObjPictureZelda
        ],
    );
}

// Change Letter in a Bottle to a Heart Piece object
fn patch_letter_in_a_bottle(patcher: &mut Patcher) {
    patcher.modify_objs(
        FieldLight,
        36,
        [call(38, |obj| {
            obj.clear_disable_flag();
            obj.set_inactive_flag(Flag::Event(916));
            obj.set_id(99);
            obj.set_typ(1);
        })],
    );
}

fn patch_master_sword(patcher: &mut Patcher) {
    // Master Sword Pedestal
    patcher.modify_objs(
        FieldLight,
        34,
        [call(71, |obj| {
            obj.clear_active_args();
            obj.set_inactive_flag(Flag::Course(150));
            obj.enable();
        })],
    );
}

fn patch_dark_maze(patcher: &mut Patcher) {
    // Remove dialog
    patcher.modify_objs(
        FieldDark,
        20,
        [
            disable(63),  // AreaEventTalk
            disable(115), // AreaEventTalk
            disable(116), // AreaEventTalk
            disable(119), // AreaEventTalk
            disable(122), // AreaEventTalk
            disable(188), // AreaEventTalk
            disable(195), // NpcGuardMan
            disable(196), // NpcGuardMan
            disable(231), // AreaEventTalk
            disable(235), // Hilda Text
        ],
    );
}

fn patch_thief_girl_cave(patcher: &mut Patcher) {
    patcher.modify_objs(
        CaveDark,
        15,
        [
            // Thief Girl w/ Mask
            // set_enable_flag(8, prize_flag), // Thief Girl
            // set_enable_flag(9, prize_flag), // Chest
            disable(10), // Entrance text
            disable(11), // AreaSwitchCube
            disable(13), // It's a secret to everybody
        ],
    );
}

/// Mother Maiamai's Cave
fn patch_maiamai_cave(patcher: &mut Patcher) {
    // Open automatically, without need for Bombs
    patcher.modify_objs(
        FieldLight,
        35,
        [
            disable(233), // Open Maiamai Cave
            disable(235), // Remove the Sign
        ],
    );
}

/// Modify the hitboxes of select big chests that could negatively affect gameplay
fn patch_big_problem_chests(patcher: &mut Patcher, settings: &Settings) {
    if !settings.chest_size_matches_contents {
        return;
    }

    const BIG_PROBLEM_CHESTS: [(Course, u16, u16); 21] = [
        (FieldLight, 3, 303),  // Death Mountain West Ledge
        (FieldLight, 34, 71),  // Master Sword Pedestal
        (FieldLight, 35, 155), // Lake Hylia Ledge
        (FieldLight, 33, 320), // Southern Ruins Ledge
        // (FieldLight, 1, 133),  // Lost Woods Big Rock
        (AttractionLight, 2, 33), // Southern Ruins Mini-Dungeon
        (DungeonEast, 2, 52),     // Eastern 2F 4 Switches
        (DungeonDark, 2, 127),    // Dark 1F Fall from 2F
        (DungeonDark, 3, 269),    // Dark 2F East
        (DungeonWater, 1, 170),   // Swamp 1F West Room
        (DungeonWater, 1, 299),   // Swamp 1F East Room
        (DungeonWater, 1, 373),   // Swamp 1F SW Room
        (DungeonWater, 2, 620),   // Swamp B1 Raft Room (Left)
        (DungeonWater, 2, 621),   // Swamp B1 Raft Room (Right)
        (DungeonDokuro, 2, 105),  // Skull B2 Moving Platform Room
        (FieldDark, 1, 515),      // Skull Outdoor Chest
        (DungeonKame, 1, 173),    // Turtle 1F SE Room
        (DungeonKame, 2, 183),    // Turtle B1 East Platform
        (DungeonSand, 1, 78),     // Desert 1F Entrance
        (DungeonSand, 1, 565),    // Desert 1F South Sand Room
        (DungeonSand, 2, 462),    // Desert 2F Below Big Chest
        // (DungeonIce, 1, 1122), // Ice Ruins B4 SW Fall
        (DungeonGanon, 1, 882), // Lorule Castle Tile Trial #2
    ];

    // Change collision scaling to effectively match the small chests
    for (stage, stage_index, unq) in BIG_PROBLEM_CHESTS {
        patcher.modify_objs(
            stage,
            stage_index,
            [call(unq, |obj| {
                if obj.id == 34 {
                    obj.srt.scale.x = 0.52632; // 0.52632 * 1.9 (actor profile) ~= 1.0
                    obj.srt.scale.z = 0.75; // 0.75 * 1.2 (actor profile) = 0.9
                }
            })],
        );
    }
}

/// Gales Softlock Prevention - Add trigger to drop wall if player entered miniboss without hitting switch
fn patch_gales_softlock(patcher: &mut Patcher, _settings: &Settings) {
    patcher.add_obj(
        DungeonWind,
        1,
        Obj::trigger_cube(Flag::Course(60), 2, 146, 454, Vec3 { x: 16.5, y: 2.5, z: -19.0 }),
    );
}

/// Big Bomb Flower Skip
fn patch_big_bomb_flower_skip(patcher: &mut Patcher, settings: &Settings) {
    if !settings.skip_big_bomb_flower {
        return;
    }

    // Big Bomb Flower Field
    patcher.modify_objs(
        FieldDark,
        24,
        [
            disable(86), // Unlock Big Bomb Flower
            disable(93), // Great Rupee Fairy
        ],
    );

    // South of Octoball Derby
    patcher.modify_objs(
        FieldDark,
        32,
        [
            disable(89), // Boulder of Destiny
        ],
    );

    // Lorule Southern Ruins
    patcher.modify_objs(
        FieldDark,
        33,
        [
            /* Swamp Palace gets drained by setting Flag 541 */
            disable(201), // Swamp Cave
        ],
    );
}

/// No Progression Enemies
fn patch_no_progression_enemies(patcher: &mut Patcher, settings: &Settings) {
    if !settings.no_progression_enemies {
        return;
    }

    // Swamp
    patcher.modify_objs(
        DungeonWater,
        1,
        [
            disable(451), // Bawb (west)
            disable(452), // Bawb (east)
        ],
    );

    // Skull
    patcher.modify_objs(
        DungeonDokuro,
        1,
        [
            disable(271), // Wall Master (North B1)
        ],
    );

    // Thieves'
    patcher.modify_objs(
        DungeonHagure,
        1,
        [
            disable(707),  // Bawb (center)
            disable(1057), // Bawb (west)
            disable(1133), // Sluggula
        ],
    );

    // Desert
    patcher.modify_objs(
        DungeonSand,
        3,
        [
            disable(234), // Bawb
            disable(240), // Bawb
            disable(252), // Bawb
        ],
    );

    // Ice
    patcher.modify_objs(
        DungeonIce,
        1,
        [
            disable(234), // Keelon
            disable(235), // Keelon
        ],
    );
}

//noinspection ALL
#[rustfmt::skip]
#[allow(unused)]
/// Development Sandbox
/// Make changes here for dev & testing we don't want to risk making it into the actual release.
fn do_dev_stuff(patcher: &mut Patcher, settings: &Settings) -> Result<()> {
    if !settings.dev_mode {
        return Ok(());
    }

    // Ravio's Shop Exit Door
    patcher.modify_objs(IndoorLight, 1, [call(24, |obj| {
        obj.redirect(SpawnPoint::new(
            FieldLight, 27, 5,  // No Redirect
            // Demo, 4, 0,
            // IndoorDark, 4, 0,  // Lorule Blacksmith
            // DungeonSand, 1, 16,  // Desert Palace 1F Exit
            // FieldLight, 4, 8,  // Floating Island
            // IndoorLight, 14, 0,  // Stylish Woman's House
            // IndoorLight, 12, 4,  // Hyrule Castle
            // FieldDark, 29, 5,  // Lorule River Portal
            // FieldLight, 16, 5,  // Kakariko Village
            // CaveLight, 15, 0, // Maiamai Cave
            // IndoorLight, 17, 0, // Bee Guy's House
            // CaveLight, 30, 0, // Witch Cave
            // DungeonKame, 1, 0,  // Turtle Rock
            // DungeonHagure, 1, 0,  // Thieves' Hideout
            // DungeonHagure, 1, 15,  // Thief Girl's Cell
            // DungeonHagure, 1, 30,  // Thieves' Miniboss outside
            // DungeonIce, 1, 0,  // Ice Ruins
            // FieldDark, 3, 0, // Lorule Death Mountain West
            // IndoorLight, 2, 0, // Witch's House
            // IndoorLight, 14, 0, // Stylish Woman's House
            // CaveLight, 15, 0, // Maiamai Cave
            // IndoorLight, 10, 0, // Rosso's House
            // FieldLight, 43, 0, // Sacred Realm
            // FieldLight, 36, 0,  // Hotfoot Area
            // FieldLight, 4, 3,
            // FieldLight, 18, 10, // Hyrule Castle Front Door
            // CaveLight, 7, 0, // Zora's Domain
            // IndoorLight, 15, 0, // Osfala Portrait
            // DungeonGanon, 1, 18, // LC 3F Center Warp Tile
            // CaveDark, 8, 0,     // Mysterious Man Cave
            // FieldDark, 31, 0, // Misery Mire
            // IndoorDark, 5, 0, // Hilda's Study
            // IndoorLight, 7, 0, // Zelda's Study (lighting gets weird)
            // DungeonCastle, 6, 0, // Yuga 2 Boss
        ));
    })]);

    // Ravio's Shop Front Door
    // patcher.modify_objs(FieldLight, 27, [
    //     call(51, |obj| {
    //         obj.redirect(Dest::new(
    //         // IndoorLight, 1, 1,  // No Redirect
    //         FieldDark, 29, 5,  // Lorule River Portal
    //     ));
    // })]);

    // Osfala Portrait House
    // patcher.modify_objs(IndoorDark, 15, [
    //     redirect(6, 20, 1, 0), // Seres Portrait
    // ]);

    Ok(())
}
