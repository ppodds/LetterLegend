$outBasePath = "apps/frontend/Assets/Scripts/Protos/"
$protoBasePath = "apps/backend/src/proto/"

$protoFolderMappingList = @(@("Control", "control/*.proto"), @("Error", "error/*.proto"), @("Game", "game/*.proto"), @("Lobby", "lobby/*.proto"), @("Player", "player/*.proto"))

for ($i = 0; $i -lt $protoFolderMappingList.Length; $i++) {
    $outPath = $outBasePath + $protoFolderMappingList[$i][0]
    if (!(Test-Path -Path $outPath)) {
        New-Item -ItemType Directory -Path $outPath
    }
    $protoPath = $protoBasePath + $protoFolderMappingList[$i][1]
    protoc --csharp_out $outPath -I $protoBasePath $protoPath
}
