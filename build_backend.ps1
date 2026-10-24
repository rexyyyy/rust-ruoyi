# 释放 exe 锁（运行中的服务会锁 target/debug/*.exe，导致 link LNK1104）
# 注意：Stop-Process 对 bat 拉起的进程可能不生效，taskkill /IM 更可靠
& taskkill /F /IM ruoyi_rust_dev.exe 2>$null | Out-Null
Start-Sleep -Seconds 1

# 定位 VS 2022：已知路径优先（powershell.exe -File 子进程里 vswhere 输出偶发为空），vswhere 兜底
$install = "C:\Program Files\Microsoft Visual Studio\2022\Community"
if (-not (Test-Path $install)) {
    $vswhere = "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vswhere) {
        $install = (& $vswhere -latest -property installationPath 2>$null)
    }
}
if (-not $install -or -not (Test-Path $install)) {
    "FATAL: VS installation not resolved" | Out-File "D:/rust-proj/rust-ruoyi/build.log" -Encoding ascii
    exit 1
}

$msvcBase = Join-Path $install "VC\Tools\MSVC"
$msvcVer = (Get-ChildItem $msvcBase -Directory -ErrorAction SilentlyContinue | Sort-Object Name | Select-Object -Last 1).Name

$sdkBase = "C:\Program Files (x86)\Windows Kits\10\Include"
$sdkVer = (Get-ChildItem $sdkBase -Directory -ErrorAction SilentlyContinue | Where-Object { $_.Name -match '^\d+\.\d+\.\d+\.\d+$' } | Sort-Object Name | Select-Object -Last 1).Name

if (-not $msvcVer -or -not $sdkVer) {
    "FATAL: MSVC or SDK version not found (MSVC=$msvcVer SDK=$sdkVer)" | Out-File "D:/rust-proj/rust-ruoyi/build.log" -Encoding ascii
    exit 1
}

$env:PATH = "$install\VC\Tools\MSVC\$msvcVer\bin\Hostx64\x64;C:\Program Files (x86)\Windows Kits\10\bin\$sdkVer\x64;" + $env:PATH
$env:LIB = "$install\VC\Tools\MSVC\$msvcVer\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\$sdkVer\um\x64;C:\Program Files (x86)\Windows Kits\10\Lib\$sdkVer\ucrt\x64"
$env:INCLUDE = "$install\VC\Tools\MSVC\$msvcVer\include;C:\Program Files (x86)\Windows Kits\10\Include\$sdkVer\um;C:\Program Files (x86)\Windows Kits\10\Include\$sdkVer\ucrt;C:\Program Files (x86)\Windows Kits\10\Include\$sdkVer\shared"

"MSVC=$msvcVer SDK=$sdkVer INSTALL=$install" | Out-File "D:/rust-proj/rust-ruoyi/toolchain.log" -Encoding ascii

& "C:/Users/Administrator/.cargo/bin/cargo.exe" build --manifest-path "D:/rust-proj/rust-ruoyi/backend/Cargo.toml" *> "D:/rust-proj/rust-ruoyi/cargo.log"
# cargo 退出码必须显式传播：外层只看 build.log 的 EXIT，且失败时脚本必须以非零退出
"EXIT=$LASTEXITCODE" | Out-File "D:/rust-proj/rust-ruoyi/build.log" -Encoding ascii
if ($LASTEXITCODE -ne 0) { exit 1 }
exit 0
