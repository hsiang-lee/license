$REPO = "http://192.168.100.12/artifactory/CI-Releases/"
$PROJ = "smlicense"
$VERSION = $env:DRONE_TAG

$USER = "ciuploader"
$PWD = ConvertTo-SecureString "ciuploader" -AsPlainText -Force
$CREDS = New-Object System.Management.Automation.PSCredential ($USER, $PWD)

$SOURCE = "Release.zip"
Compress-Archive -LiteralPath "Release","include" -DestinationPath $SOURCE
$URI = New-Object System.Uri("$REPO/$PROJ/$VERSION/Win64.zip")

Invoke-WebRequest -Uri $URI -InFile $SOURCE -Method Put -Credential $CREDS -UseBasicParsing