param(
    [Parameter(Mandatory=$true)]
    [String]$ImageName,

    [Parameter()]
    [String]$DockerfilePath
)

if (-not $DockerfilePath) {
    $DockerfilePath = "."
}

Write-Host "Building Docker image '$ImageName'..."
docker build -f "$DockerfilePath/Dockerfile" -t "$ImageName" .

Write-Host "Starting Docker container '$ImageName'..."
docker run --name appContainer -p 3000:3000 -it $ImageName