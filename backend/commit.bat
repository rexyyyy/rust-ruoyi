@echo off
cd /d %~dp0

REM git absolute path (fallback if not in PATH)
set "GIT=C:\Program Files\Git\cmd\git.EXE"
where git >nul 2>&1 && set "GIT=git"

REM committer identity (skip if already configured globally/locally)
%GIT% config user.name >nul 2>&1 || %GIT% config user.name "LaoYang"
%GIT% config user.email >nul 2>&1 || %GIT% config user.email "you@example.com"

REM init repo if needed (safe to re-run)
%GIT% rev-parse --is-inside-work-tree >nul 2>&1 || %GIT% init -q

%GIT% add -A

echo.
echo ===== Files to be committed =====
%GIT% status --short
echo ==================================
echo.

if defined DRY (
  echo [DRY-RUN] no commit, just preview:
  %GIT% commit --dry-run -F commit_msg.txt
  goto :end
)

set /p CONFIRM=Type Y to commit, any other key to cancel: 
if /I not "%CONFIRM%"=="Y" (
  echo Cancelled. Nothing committed.
  goto :end
)

%GIT% commit -F commit_msg.txt

echo.
echo Done. View log with: git log --oneline
:end
pause
