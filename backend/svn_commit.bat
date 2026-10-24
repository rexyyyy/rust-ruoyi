@echo off
cd /d %~dp0

REM svn absolute path (fallback if not in PATH)
set "SVN=C:\Program Files\TortoiseSVN\bin\svn.EXE"
where svn >nul 2>&1 && set "SVN=svn"

echo ===== Scheduled changes (A=add D=delete M=modify) =====
%SVN% status
echo ======================================================
echo NOTE: svn commit pushes directly to gitee (irreversible). Unversioned files are NOT committed.
echo.
set /p CONFIRM=Type Y to COMMIT and push to gitee: 
if /I not "%CONFIRM%"=="Y" (
  echo Cancelled. Nothing pushed.
  goto :end
)

%SVN% commit -F commit_msg.txt --encoding utf-8

echo.
echo Done. View with: svn log -l 5
:end
pause
