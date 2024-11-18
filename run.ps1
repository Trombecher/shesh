cargo build --release

if ($LASTEXITCODE -ne 0) {
    exit 1
} else {
    Start-Process -Wait -FilePath "C:\Program Files\WindowsApps\Microsoft.WindowsTerminal_1.21.2911.0_x64__8wekyb3d8bbwe\WindowsTerminal.exe"
}