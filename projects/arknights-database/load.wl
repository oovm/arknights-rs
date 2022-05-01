(* ::Package:: *)

SetDirectory@NotebookDirectory[];


config = Import["data/gamedata_const.json", "RawJSON"];


levels = PadRight[config["maxLevel"], {6, 3}, 1];
expMap = config["characterExpMap"] /. {-1 -> Nothing};
cashMap = config["characterUpgradeCostMap"] /. {-1 -> Nothing};
mapping = TemplateApply["\
#[inline]
fn map_exp() -> Self {
    Self {
        elite0: `1`,
        elite1: `2`,
        elite2: `3`,
    }
}
#[inline]
fn map_cash() -> Self {
    Self {
        elite0: `4`,
        elite1: `5`,
        elite2: `6`,
    }
}
",
    Flatten@{asVec /@ expMap, asVec /@ cashMap}
];


asVec[v_] := "vec![" <> StringRiffle[v, ","] <> "]";
f[{e0_, e1_, e2_}, {index_}] := Block[
    {},
    TemplateApply["
/// \:5347\:7ea7 1 \:661f\:5e72\:5458\:6240\:9700\:8981\:7684\:7ecf\:9a8c\:503c
pub fn star`1`_exp() -> Self {
    let map = Self::map_exp();
    Self {
        elite0: map.elite0.into_iter().take(`2`).collect(),
        elite1: map.elite0.into_iter().take(`3`).collect(),
        elite2: map.elite0.into_iter().take(`4`).collect(),
    }
}

/// \:5347\:7ea7 `1` \:661f\:5e72\:5458\:6240\:9700\:8981\:7684\:9f99\:95e8\:5e01
pub fn star`1`_cash() -> Self {
    let map = Self::map_cash();
    Self {
        elite0: map.elite0.into_iter().take(`2`).collect(),
        elite1: map.elite0.into_iter().take(`3`).collect(),
        elite2: map.elite0.into_iter().take(`4`).collect(),
    }
}
",
        {
            index,
            e0 - 1,
            e1 - 1,
            e2 - 1
        }
    ]
];


levelUpCostDB = TemplateApply["\
use super::*;

//noinspection DuplicatedCode
impl LevelUpCostDB {
`1`
`2`
}
",
    {
        mapping,
        StringJoin[MapIndexed[f, levels]]
    }
];
Export["src/cost/stars.rs", levelUpCostDB, "Text"]
