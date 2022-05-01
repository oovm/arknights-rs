(* ::Package:: *)

SetDirectory@NotebookDirectory[]



config=Import["data/stage_index.json","RawJSON"];


config["maxLevel"]


config["characterExpMap"]


mainStory=FileNames["*.json",  "data/gamedata/story/",Infinity];


file=mainStory[[2]];





findMainStageID[file_]:=Block[
{stageID,story},
stageID=StringTrim[FileNameTake[file],"level_"|"_beg.json"|"_end.json"|".json"];
story=Import[file,"RawJSON"];
stageID-><|
"chapter"->story["eventName"],
"kind"->story["entryType"],
"name"->story["storyName"],
"code"->story["storyCode"]
|>
]


mainStageID=DeleteDuplicatesBy[ParallelMap[findMainStageID,mainStory],First];


Export["src/story_id/data.json", mainStageID]


matrix=Import["https://penguin-stats.io/PenguinStats/api/v2/result/matrix","RawJSON"]


data=Normal@Import["data/story_review_table.json","RawJSON"];
findStory[story_,chapterName_]:=<|
"chapter"->chapterName,
"name"->story["storyName"],
"code"->story["storyCode"]
|>
findStories[chapter_]:=Block[
{chapterName,stories},
chapterName=chapter[[-1,"name"]];
stories=chapter[[-1,"infoUnlockDatas"]];
findStory[#,chapterName]&/@stories
]
findStories/@data//Flatten


selectMatrix[matrix_]:=Block[
{},
matrix
]


matrix["matrix"]
