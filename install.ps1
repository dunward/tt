$targetDir = "$env:USERPROFILE\.tt"
$exePath = "$targetDir\tt.exe"

$url = "https://github.com/dunward/tt/releases/latest/download/tt.exe"

if (!(Test-Path -Path $targetDir)) {
    New-Item -ItemType Directory -Path $targetDir | Out-Null
}

Invoke-WebRequest -Uri $url -OutFile $exePath

$envPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if (-not ($envPath -split ";" | Where-Object { $_ -eq $targetDir })) {
    [Environment]::SetEnvironmentVariable("Path", "$envPath;$targetDir", [EnvironmentVariableTarget]::User)
    Write-Host "`nAdded $targetDir to PATH (will take effect in new PowerShell window)"
}

Write-Host "`n tt.exe installation complete! You can now use the 'tt' command."
