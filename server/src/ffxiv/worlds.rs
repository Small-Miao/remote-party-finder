use std::collections::HashMap;
use ffxiv_types_cn::World;

lazy_static::lazy_static! {
    pub static ref WORLDS: HashMap<u32, World> = maplit::hashmap! {
        21 => World::Ravana,
        22 => World::Bismarck,
        23 => World::Asura,
        24 => World::Belias,
        28 => World::Pandaemonium,
        29 => World::Shinryu,
        30 => World::Unicorn,
        31 => World::Yojimbo,
        32 => World::Zeromus,
        33 => World::Twintania,
        34 => World::Brynhildr,
        35 => World::Famfrit,
        36 => World::Lich,
        37 => World::Mateus,
        39 => World::Omega,
        40 => World::Jenova,
        41 => World::Zalera,
        42 => World::Zodiark,
        43 => World::Alexander,
        44 => World::Anima,
        45 => World::Carbuncle,
        46 => World::Fenrir,
        47 => World::Hades,
        48 => World::Ixion,
        49 => World::Kujata,
        50 => World::Typhon,
        51 => World::Ultima,
        52 => World::Valefor,
        53 => World::Exodus,
        54 => World::Faerie,
        55 => World::Lamia,
        56 => World::Phoenix,
        57 => World::Siren,
        58 => World::Garuda,
        59 => World::Ifrit,
        60 => World::Ramuh,
        61 => World::Titan,
        62 => World::Diabolos,
        63 => World::Gilgamesh,
        64 => World::Leviathan,
        65 => World::Midgardsormr,
        66 => World::Odin,
        67 => World::Shiva,
        68 => World::Atomos,
        69 => World::Bahamut,
        70 => World::Chocobo,
        71 => World::Moogle,
        72 => World::Tonberry,
        73 => World::Adamantoise,
        74 => World::Coeurl,
        75 => World::Malboro,
        76 => World::Tiamat,
        77 => World::Ultros,
        78 => World::Behemoth,
        79 => World::Cactuar,
        80 => World::Cerberus,
        81 => World::Goblin,
        82 => World::Mandragora,
        83 => World::Louisoix,
        85 => World::Spriggan,
        86 => World::Sephirot,
        87 => World::Sophia,
        88 => World::Zurvan,
        90 => World::Aegis,
        91 => World::Balmung,
        92 => World::Durandal,
        93 => World::Excalibur,
        94 => World::Gungnir,
        95 => World::Hyperion,
        96 => World::Masamune,
        97 => World::Ragnarok,
        98 => World::Ridill,
        99 => World::Sargatanas,
        400 => World::Sagittarius,
        401 => World::Phantom,
        402 => World::Alpha,
        403 => World::Raiden,
        404 => World::Marilith,
        405 => World::Seraph,
        406 => World::Halicarnassus,
        407 => World::Maduin,
        1042 => World::拉诺西亚,
        1043 => World::紫水栈桥,
        1044 => World::幻影群岛,
        1045 => World::摩杜纳,
        1060 => World::萌芽池,
        1076 => World::白金幻象,
        1081 => World::神意之地,
        1106 => World::静语庄园,
        1113 => World::旅人栈桥,
        1121 => World::拂晓之间,
        1166 => World::龙巢神殿,
        1167 => World::红玉海,
        1169 => World::延夏,
        1170 => World::潮风亭,
        1171 => World::神拳痕,
        1172 => World::白银乡,
        1173 => World::宇宙和音,
        1174 => World::沃仙曦染,
        1175 => World::晨曦王座,
        1176 => World::梦羽宝境,
        1177 => World::海猫茶屋,
        1178 => World::柔风海湾,
        1179 => World::琥珀原,
        1180 => World::太阳海岸,
        1183 => World::银泪湖,
        1186 => World::伊修加德,
        1192 => World::水晶塔,
        1200 => World::亚马乌罗提,
        1201 => World::红茶川,
        1202 => World::萨雷安,
        1203 => World::加雷马,
    };
}
