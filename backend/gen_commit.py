import io
base = r'D:\rust-proj\rust-ruoyi\backend'
gitpath = r'C:\Program Files\Git\cmd\git.EXE'

# ---- commit_msg.txt (UTF-8, 含中文) ----
msg = (
    'refactor(web_data): use request-scoped CurrentUser extractor instead of thread_local global user state\n'
    '\n'
    '- web_data.rs rewritten as CurrentUser FromRequest extractor: no global mutable state, no lock, per-request isolation\n'
    '- Auth middleware inserts CurrentUser into req.extensions() after auth; handlers take current_user: CurrentUser param\n'
    '- 11 controllers + sys_auth_controller::logout migrated to CurrentUser\n'
    '- fix actix-web 4.11 FromRequest payload type (actix_http::Payload) and HttpMessage trait scope\n'
    '- token_auth::check_permit: drop illegal extensions_mut on immutable HttpRequest (path not on live chain)\n'
    '- verified: after admin login, sys_notice add->create_by=1, update->update_by=1 (correct per-request attribution)\n'
)
with io.open(base + r'\commit_msg.txt', 'w', encoding='utf-8', newline='\n') as f:
    f.write(msg)

# ---- commit.bat (GBK/ANSI, pure ASCII) ----
bat = (
    '@echo off\n'
    'cd /d %~dp0\n'
    '\n'
    'REM git absolute path (fallback if not in PATH)\n'
    'set "GIT=' + gitpath + '"\n'
    'where git >nul 2>&1 && set "GIT=git"\n'
    '\n'
    'REM committer identity (skip if already configured globally/locally)\n'
    '%GIT% config user.name >nul 2>&1 || %GIT% config user.name "LaoYang"\n'
    '%GIT% config user.email >nul 2>&1 || %GIT% config user.email "you@example.com"\n'
    '\n'
    'REM init repo if needed (safe to re-run)\n'
    '%GIT% rev-parse --is-inside-work-tree >nul 2>&1 || %GIT% init -q\n'
    '\n'
    '%GIT% add -A\n'
    '\n'
    'echo.\n'
    'echo ===== Files to be committed =====\n'
    '%GIT% status --short\n'
    'echo ==================================\n'
    'echo.\n'
    '\n'
    'if defined DRY (\n'
    '  echo [DRY-RUN] no commit, just preview:\n'
    '  %GIT% commit --dry-run -F commit_msg.txt\n'
    '  goto :end\n'
    ')\n'
    '\n'
    'set /p CONFIRM=Type Y to commit, any other key to cancel: \n'
    'if /I not "%CONFIRM%"=="Y" (\n'
    '  echo Cancelled. Nothing committed.\n'
    '  goto :end\n'
    ')\n'
    '\n'
    '%GIT% commit -F commit_msg.txt\n'
    '\n'
    'echo.\n'
    'echo Done. View log with: git log --oneline\n'
    ':end\n'
    'pause\n'
)
with io.open(base + r'\commit.bat', 'w', encoding='gbk', newline='\r\n') as f:
    f.write(bat)
print('written commit.bat (gbk) + commit_msg.txt (utf-8)')
