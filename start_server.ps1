$exe = "D:/rust-proj/rust-ruoyi/backend/target/debug/ruoyi_rust_dev.exe"
Set-Location "D:/rust-proj/rust-ruoyi/backend"
Start-Process -FilePath $exe -WorkingDirectory "D:/rust-proj/rust-ruoyi/backend" `
    -RedirectStandardOutput "D:/rust-proj/rust-ruoyi/server_out.log" `
    -RedirectStandardError "D:/rust-proj/rust-ruoyi/server_err.log" -NoNewWindow
"STARTED $(Get-Date)" | Out-File "D:/rust-proj/rust-ruoyi/server_start.log" -Encoding ascii
