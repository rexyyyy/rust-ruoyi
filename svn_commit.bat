@echo off
cd /d %~dp0
set SVN="C:\Program Files\TortoiseSVN\bin\svn.exe"
set REPO=svn://gitee.com/witgg2001/rust-ruoyi

echo ============================================================
echo  Archive legacy frontend into branch 0.1, then update master
echo ============================================================
echo  [1] commit login/register tweaks of ruoyi-vue3-rust-master (keep in history)
echo  [2] sync those 2 tweaks into branch 0.1 (temp checkout + commit)
echo  [3] svn delete ruoyi-vue3-rust-master from main tree
echo  [4] commit codegen + plus-ui-5.X + deletion  ->  master
echo  NOTE: pushes directly to gitee (irreversible). Need network + credentials.
echo ============================================================
set /p CONFIRM=Confirm execute? (Y/N):
if /I not "%CONFIRM%"=="Y" (
  echo Cancelled.
  pause
  exit /b
)

echo [1/4] commit legacy frontend final tweaks ...
%SVN% commit ruoyi-vue3-rust-master -m "chore: final login/register tweaks of legacy frontend before archiving to branch 0.1"
if errorlevel 1 goto FAIL

echo [2/4] sync tweaks into branch 0.1 ...
rd /s /q _b01_sync 2>nul
%SVN% co "%REPO%/branches/0.1/ruoyi-vue3-rust-master/src/views" _b01_sync
if errorlevel 1 goto FAIL
copy /Y "ruoyi-vue3-rust-master\src\views\login.vue" "_b01_sync\login.vue"
copy /Y "ruoyi-vue3-rust-master\src\views\register.vue" "_b01_sync\register.vue"
%SVN% commit "_b01_sync" -m "sync login/register tweaks from master into branch 0.1 (legacy frontend final state)"
if errorlevel 1 goto FAIL
rd /s /q _b01_sync

echo [3/4] delete ruoyi-vue3-rust-master from main tree ...
%SVN% delete "ruoyi-vue3-rust-master"
if errorlevel 1 goto FAIL

echo [4/4] commit codegen + plus-ui-5.X + deletion -> master ...
%SVN% commit codegen plus-ui-5.X ruoyi-vue3-rust-master -F commit_msg.txt --encoding utf-8
if errorlevel 1 goto FAIL

echo ============================================================
echo  DONE. Legacy frontend archived in branch 0.1; master updated.
echo ============================================================
goto END

:FAIL
echo ERROR occurred, aborted. Check network/credentials and retry.
pause
exit /b 1

:END
pause
