@echo off
REM Double-click to run cleanup: moves audited redundant files to Recycle Bin.
REM List audited 2026-10-08 by xiaoyun. See cleanup_20261008.ps1 for details.
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0cleanup_20261008.ps1"
pause
