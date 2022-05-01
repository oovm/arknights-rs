(* ::Package:: *)

SetDirectory@NotebookDirectory[];


config = Import["data/gamedata_const.json", "RawJSON"];


levels = PadRight[config["maxLevel"], {6, 3}, 1];
expMap = config["characterExpMap"] /. {-1 -> Nothing};
cashMap = config["characterUpgradeCostMap"] /. {-1 -> Nothing};
goldExtra = config["evolveGoldCost"] /. {-1 -> 0}
mapping = TemplateApply["\
#[inline]
fn map_exp() -> Self {
    Self {
        stage0: `1`,
		elite1: 0,
        stage1: `2`,
		elite2: 0,
        stage2: `3`,
    }
}
#[inline]
fn map_cash() -> Self {
    Self {
        stage0: `4`,
		elite1: 0,
        stage1: `5`,
		elite2: 0,
        stage2: `6`,
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
        stage0: map.stage0.into_iter().take(`2`).collect(),
        stage1: map.stage1.into_iter().take(`3`).collect(),
        stage2: map.stage2.into_iter().take(`4`).collect(),
		elite1: 0,
		elite2: 0,
    }
}

/// \:5347\:7ea7 `1` \:661f\:5e72\:5458\:6240\:9700\:8981\:7684\:9f99\:95e8\:5e01
pub fn star`1`_cash() -> Self {
    let map = Self::map_cash();
    Self {
        stage0: map.stage0.into_iter().take(`2`).collect(),
        stage1: map.stage1.into_iter().take(`3`).collect(),
        stage2: map.stage2.into_iter().take(`4`).collect(),
		elite1: `5`,
		elite2: `6`,
    }
}
",
        {
            index,
            e0 - 1,
            e1 - 1,
            e2 - 1,
            goldExtra[[index, 1]],
            goldExtra[[index, 2]]
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
