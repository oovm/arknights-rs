(* ::Package:: *)

SetDirectory@NotebookDirectory[]





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


selectMatrix[matrix_]:=Block[
{},
matrix
]


matrix["matrix"]
