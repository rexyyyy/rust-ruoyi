$vswhere = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) { "NO_VSWHERE" | Out-File "D:/rust-proj/rust-ruoyi/vs.log" -Encoding ascii; exit }
$install = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
if (-not $install) { "NO_INSTALL" | Out-File "D:/rust-proj/rust-ruoyi/vs.log" -Encoding ascii; exit }
"INSTALL=$install" | Out-File "D:/rust-proj/rust-ruoyi/vs.log" -Encoding ascii

$msvcBase = Join-Path $install "VC\Tools\MSVC"
$msvcVer = (Get-ChildItem $msvcBase -Directory -ErrorAction SilentlyContinue | Sort-Object Name | Select-Object -Last 1).Name
"MSVC=$msvcVer" | Out-File "D:/rust-proj/rust-ruoyi/vs.log" -Encoding ascii -Append

$sdkBase = "C:\Program Files (x86)\Windows Kits\10\Include"
$sdkVer = (Get-ChildItem $sdkBase -Directory -ErrorAction SilentlyContinue | Where-Object { $_.Name -match '^\d+\.\d+\.\d+\.\d+$' } | Sort-Object Name | Select-Object -Last 1).Name
"SDK=$sdkVer" | Out-File "D:/rust-proj/rust-ruoyi/vs.log" -Encoding ascii -Append
