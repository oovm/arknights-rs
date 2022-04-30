(* ::Package:: *)

SetDirectory@NotebookDirectory[]


FileNames["data/gamedata/story/activities/*"]


mainStory=FileNames["data/gamedata/story/obt/main/*"];


file=mainStory[[2]];





findMainStageID[file_]:=Block[
{stageID,story},
stageID=StringTrim[FileNameTake[file],"level_"|"_beg.json"|"_end.json"|".json"];
story=Import[file,"RawJSON"];
stageID-><|
"charpter"->story["eventName"],
"kind"->story["entryType"],
"name"->story["storyName"],
"code"->story["storyCode"]
|>
]


mainStageID=ParallelMap[findMainStageID,mainStory]


Export["src/story_id/data.json", mainStageID]
