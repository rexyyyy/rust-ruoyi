# cleanup_20261008.ps1
# rust-ruoyi project redundant-file cleanup (audited 2026-10-08)
# All deletions go to the Recycle Bin (recoverable). ASCII paths only.
# FREES ~6.9 GB (two stale build-cache dirs) + ~120 root temp files.

$ErrorActionPreference = 'Continue'
$root = 'D:\rust-proj\rust-ruoyi'
Add-Type -AssemblyName Microsoft.VisualBasic

$deleted = 0
$failed = 0

function Remove-ToBin([string]$path) {
    if (-not (Test-Path $path)) { return }
    try {
        if (Test-Path $path -PathType Container) {
            [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteDirectory($path, 'OnlyErrorDialogs', 'SendToRecycleBin')
        } else {
            [Microsoft.VisualBasic.FileIO.FileSystem]::DeleteFile($path, 'OnlyErrorDialogs', 'SendToRecycleBin')
        }
        Write-Host "DEL  $path"
        $script:deleted++
    } catch {
        Write-Host "FAIL $path : $($_.Exception.Message)"
        $script:failed++
    }
}

Push-Location $root

# ---------- A) underscore probe/temp files in root ----------
Get-ChildItem $root -File | Where-Object { $_.Name -like '_*' } | ForEach-Object { Remove-ToBin $_.FullName }

# ---------- B) build / cargo logs ----------
$groupB = @('build.log','cargo.log','toolchain.log','check.log','vs.log',
            'pnpm_install.log','pnpm_install_done.txt',
            'verify_log.txt','verify_out.txt','verify_report.txt','vrun3.txt','smoke_done.txt')
Get-ChildItem $root -File -Filter 'build*.log' | ForEach-Object { Remove-ToBin $_.FullName }
Get-ChildItem $root -File -Filter 'cargo*.log' | ForEach-Object { Remove-ToBin $_.FullName }
foreach ($f in $groupB) { Remove-ToBin (Join-Path $root $f) }
Remove-ToBin (Join-Path $root 'test_b64.log')
Remove-ToBin (Join-Path $root 'test_lib_b64.log')

# ---------- C) old server/vite logs (KEEP server_out/err, frontend log/err) ----------
Get-ChildItem $root -File -Filter 'server_*.log' |
    Where-Object { $_.Name -notin @('server_out.log','server_err.log') } |
    ForEach-Object { Remove-ToBin $_.FullName }
Get-ChildItem $root -File -Filter 'vite_run*.log' | ForEach-Object { Remove-ToBin $_.FullName }
foreach ($f in @('be_run_new.log','smoke_demo.out','smoke_toolgen.out')) { Remove-ToBin (Join-Path $root $f) }

# ---------- D) svn snapshot txt / commit msg ----------
Get-ChildItem $root -File -Filter 'svn_*.txt'  | ForEach-Object { Remove-ToBin $_.FullName }
Get-ChildItem $root -File -Filter 'svn_*.log'  | ForEach-Object { Remove-ToBin $_.FullName }
foreach ($f in @('commit_msg.txt','git_probe.txt')) { Remove-ToBin (Join-Path $root $f) }

# ---------- E) old debug / one-off scripts ----------
$groupE = @('debug_put.py','debug_toolgen.py','migrate_toolgen.py','migrate_tool_gen.py',
            'smoke_demo.py','smoke_job.py','smoke_job_clean.py','smoke_job_dynamic.py',
            'smoke_toolgen.py','smoke_watch.py','fix_web_data.py')
foreach ($f in $groupE) { Remove-ToBin (Join-Path $root $f) }

# ---------- F) misc temp ----------
$groupF = @('captcha_fixed.png','captcha_std.png','master',
            'port60800.txt','portcheck.txt','psqlpath.txt','svc.txt','node_procs.txt',
            'errblock.txt','errlines.txt','diag.txt','diag2.txt','diag3.txt','listdbg.txt',
            'db_createby.txt','db_updateby.txt','backend_probe.txt','probe_sandbox.txt',
            'meta.err','meta.json')
foreach ($f in $groupF) { Remove-ToBin (Join-Path $root $f) }

# ---------- G) stale build caches & empty/extract dirs (~6.9 GB) ----------
$groupG = @(
    '_b01_sync',
    '_genout',
    'apalis-main',            # extracted from apalis-main.zip (zip kept, re-extractable)
    'apalis-v1.0.0-rc.9',     # extracted from zip (zip kept)
    'apalis_extract',
    'apalis_core_extract',
    'backend\target\rel_old_20261001_085505',  # 3.15 GB old target backup
    'codegen\target'                            # 3.6 GB standalone build cache (workspace uses backend\target)
)
foreach ($d in $groupG) { Remove-ToBin (Join-Path $root $d) }

Pop-Location

Write-Host ''
Write-Host "=== DONE: deleted=$deleted failed=$failed (all to Recycle Bin) ==="
Write-Host 'KEPT: ref/  script/  LICENSE  README.md  *.bat/*.ps1/*.sh launchers'
Write-Host '      RuoYi-Rust-*.md  *.md  apalis*.zip  predawn-main.zip'
Write-Host '      server_out.log server_err.log frontend.log frontend_err.log backend\target\debug'
