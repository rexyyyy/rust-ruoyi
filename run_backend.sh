#!/usr/bin/env bash
# run_backend.sh —— 启动 rust-ruoyi backend（用 build_backend.sh 产出的新二进制）
#
# 与编译不同：运行后端不触发编译器产物写出，普通 D: 写被沙箱放行，
# 因此本脚本用【后台 + dangerouslyDisableSandbox=true】常驻即可（srO5oy 验证通过）。
#
# 用法（Bash 工具）:
#   command = "bash run_backend.sh"
#   run_in_background = true, dangerouslyDisableSandbox = true
#
# 重启标准流程:
#   1) taskkill /F /IM ruoyi_rust_dev.exe
#   2) bash build_backend.sh            # 前台 + dangerouslyDisableSandbox
#   3) bash run_backend.sh              # 后台 + dangerouslyDisableSandbox
#   4) 跑 smoke_toolgen.py / smoke_demo.py 验证

BIN="C:/Users/Administrator/WorkBuddy/2026-09-29-17-41-32/rust_rel/debug/ruoyi_rust_dev.exe"
BACKEND="D:/rust-proj/rust-ruoyi/backend"
LOG="D:/rust-proj/rust-ruoyi/be_run_new.log"

cd "$BACKEND" || { echo "cd $BACKEND 失败"; exit 1; }
if [ ! -f "$BIN" ]; then
  echo "未找到二进制: $BIN —— 请先 bash build_backend.sh"
  exit 1
fi
echo "[run] 启动 $BIN profile=dev -> $LOG"
exec "$BIN" profile=dev > "$LOG" 2>&1
