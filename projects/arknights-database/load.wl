(* ::Package:: *)

SetDirectory@NotebookDirectory[];


config = Import["data/gamedata_const.json", "RawJSON"];


levels = PadRight[config["maxLevel"], {6, 3}, 1]


asVec[v_] := "vec![" <> StringRiffle[v, ","] <> "]";
f[{e0_, e1_, e2_}, {index_}] := Block[
    {},
    TemplateApply["\
pub fn star`1`_exp() -> Self {
        Self {
            elite0: `2`,
            elite1: `3`,
            elite2: `4`,
        }
    }
pub fn star`1`_cash() -> Self {
        Self {
            elite0: `5`,
            elite1: `6`,
            elite2: `7`,
        }
    }
",
        {
            index,
            config["characterExpMap"][[1, 1 ;; e0 - 1]] // asVec,
            config["characterExpMap"][[2, 1 ;; e1 - 1]] // asVec,
            config["characterExpMap"][[3, 1 ;; e2 - 1]] // asVec,
            config["characterUpgradeCostMap"][[1, 1 ;; e0 - 1]] // asVec,
            config["characterUpgradeCostMap"][[2, 1 ;; e1 - 1]] // asVec,
            config["characterUpgradeCostMap"][[3, 1 ;; e2 - 1]] // asVec
        }
    ]
]


levelUpCostDB = TemplateApply["\
use super::*;

impl LevelUpCostDB {`1`}
",
    {StringJoin[MapIndexed[f, levels]]}
];
Export["src/cost/stars.rs", levelUpCostDB, "Text"]


mainStory = FileNames["*.json", "data/gamedata/story/", Infinity];


file = mainStory[[2]];





findMainStageID[file_] := Block[
    {stageID, story},
    stageID = StringTrim[FileNameTake[file], "level_" | "_beg.json" | "_end.json" | ".json"];
    story = Import[file, "RawJSON"];
    stageID -> <|
        "chapter" -> story["eventName"],
        "kind" -> story["entryType"],
        "name" -> story["storyName"],
        "code" -> story["storyCode"]
    |>
]


mainStageID = DeleteDuplicatesBy[ParallelMap[findMainStageID, mainStory], First];


Export["src/story_id/data.json", mainStageID]


matrix = Import["https://penguin-stats.io/PenguinStats/api/v2/result/matrix", "RawJSON"]


data = Normal@Import["data/story_review_table.json", "RawJSON"];
findStory[story_, chapterName_] := <|
    "chapter" -> chapterName,
    "name" -> story["storyName"],
    "code" -> story["storyCode"]
|>
findStories[chapter_] := Block[
    {chapterName, stories},
    chapterName = chapter[[-1, "name"]];
    stories = chapter[[-1, "infoUnlockDatas"]];
    findStory[#, chapterName]& /@ stories
]
findStories /@ data // Flatten


selectMatrix[matrix_] := Block[
    {},
    matrix
]


matrix["matrix"]
