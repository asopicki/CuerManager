Param(
    [Parameter(Mandatory=$true)]
    [String]
    $Build
)

Set-Location $PSScriptRoot

Remove-Item dist -Recurse -ErrorAction Ignore

Set-Location cuer_manager_backend\ui

Write-Host "Start building the frontnend"

ng build --prod --deploy-url=static/

Write-Host "Frontend build done."

Set-Location ..\..\

Set-Item -Path Env:SQLITE3_LIB_DIR -Value $PWD\win

Write-Host "Start building the backend"

cargo build --release

Write-Host "Backend build done"

Write-Host "Creating distribution folder"

New-Item -ItemType Directory -Path dist | Out-Null
New-Item -ItemType Directory -Path dist\CuerManager | Out-Null
New-Item -ItemType Directory -Path dist\CuerManager\bin | Out-Null 
New-Item -ItemType Directory -Path dist\CuerManager\cuecards | Out-Null 
New-Item -ItemType Directory -Path dist\CuerManager\music_files | Out-Null 

Copy-Item .\cuer_manager_backend\ui\dist\cuer-manager-ui dist\CuerManager\public -Recurse
Copy-Item .\target\release\cuecard_indexer.exe dist\CuerManager\bin
Copy-Item .\target\release\cuer_manager.exe dist\CuerManager\bin
Copy-Item .\library.empty.db dist\CuerManager
Copy-Item .\migrations dist\CuerManager -Recurse
Copy-Item .\win\sqlite3.lib dist\CuerManager 
Copy-Item .\win\sqlite3.dll dist\CuerManager
Copy-Item .\win\sqlite3.def dist\CuerManager
Copy-Item .\win\sqlite3.exe dist\CuerManager
Copy-Item .\win\sqldiff.exe dist\CuerManager
Copy-Item .\win\sqlite3_analyzer.exe dist\CuerManager
Copy-Item .\win\Rocket.toml.default dist\CuerManager
Copy-Item .\win\cuer_manager.bat dist\CuerManager

Write-Host "Creating distribution folder complete"

Write-Host "Creating zip file"

$compress = @{
LiteralPath= "dist\CuerManager"
CompressionLevel = "Fastest"
DestinationPath = ".\CuerManager-$Build.zip"
}

Compress-Archive @compress

Write-Host "Finished"
