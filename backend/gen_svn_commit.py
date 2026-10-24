import io
base = r'D:\rust-proj\rust-ruoyi\backend'
svn = r'C:\Program Files\TortoiseSVN\bin\svn.EXE'
msg = (
    'refactor(web_data): use request-scoped CurrentUser extractor instead of thread_local global user state\n'
    '\n'
    '- web_data.rs rewritten as CurrentUser FromRequest extractor: no global mutable state, no lock, per-request isolation\n'
    '- Auth middleware inserts CurrentUser into req.extensions() after auth; handlers take current_user: CurrentUser param\n'
    '- 11 controllers + sys_auth_controller::logout migrated to CurrentUser\n'
    '- fix actix-web 4.11 FromRequest payload type (actix_http::Payload) and HttpMessage trait scope\n'
    '- token_auth::check_permit: drop illegal extensions_mut on immutable HttpRequest (path not on live chain)\n'
    '- wc tidy: controllers moved controller/ -> controller/system/, domain/dto reorganized into domain/bo\n'
    '- verified: after admin login, sys_notice add->create_by=1, update->update_by=1 (correct per-request attribution)\n'
)
with io.open(base + r'\commit_msg.txt', 'w', encoding='utf-8', newline='\n') as f:
    f.write(msg)
bat = (
    '@echo off\n'
    'cd /d %~dp0\n'
    '\n'
    'REM svn absolute path (fallback if not in PATH)\n'
    'set "SVN=' + svn + '"\n'
    'where svn >nul 2>&1 && set "SVN=svn"\n'
    '\n'
    'echo ===== Scheduled changes (A=add D=delete M=modify) =====\n'
    '%SVN% status\n'
    'echo ======================================================\n'
    'echo NOTE: svn commit pushes directly to gitee (irreversible). Unversioned files are NOT committed.\n'
    'echo.\n'
    'set /p CONFIRM=Type Y to COMMIT and push to gitee: \n'
    'if /I not "%CONFIRM%"=="Y" (\n'
    '  echo Cancelled. Nothing pushed.\n'
    '  goto :end\n'
    ')\n'
    '\n'
    '%SVN% commit -F commit_msg.txt --encoding utf-8\n'
    '\n'
    'echo.\n'
    'echo Done. View with: svn log -l 5\n'
    ':end\n'
    'pause\n'
)
with io.open(base + r'\svn_commit.bat', 'w', encoding='gbk', newline='\r\n') as f:
    f.write(bat)
print('written svn_commit.bat (gbk) + commit_msg.txt (utf-8)')
