# 工作流端到端冒烟：登录 -> 请假单提交并发起流程 -> 逐级审批 -> 流程结束
# 依赖：backend 以 `captcha_enabled: false` 启动，示例流程 leave1 已自动导入并发布
# 用法: pwsh -File smoke_workflow.ps1
param(
    [string]$Base = "http://127.0.0.1:60800/dev-api",
    [string]$User = "admin",
    [string]$Pass = "admin123"
)

$ErrorActionPreference = "Continue"
$script:rows = @()

function Call($method, $path, $body, $token) {
    $headers = @{}
    if ($token) { $headers["Authorization"] = "Bearer " + $token }
    $uri = $Base + $path
    try {
        if ($null -ne $body) {
            $json = $body | ConvertTo-Json -Depth 10 -Compress
            $r = Invoke-WebRequest -Method $method -Uri $uri -Headers $headers `
                -ContentType "application/json;charset=utf-8" `
                -Body ([Text.Encoding]::UTF8.GetBytes($json)) -SkipHttpErrorCheck -TimeoutSec 30
        } else {
            $r = Invoke-WebRequest -Method $method -Uri $uri -Headers $headers -SkipHttpErrorCheck -TimeoutSec 30
        }
        return [pscustomobject]@{ Status = [int]$r.StatusCode; Body = $r.Content }
    } catch {
        return [pscustomobject]@{ Status = -1; Body = $_.Exception.Message }
    }
}

function Json($res) {
    try { return ($res.Body | ConvertFrom-Json) } catch { return $null }
}

function Check($name, $res) {
    $j = Json $res
    $code = -1
    if ($null -ne $j -and $null -ne $j.code) { $code = [int]$j.code }
    $ok = ($res.Status -eq 200 -and ($code -eq 200 -or $code -eq 0))
    $detail = ""
    if (-not $ok) { $detail = ($res.Body -replace "\s+", " "); if ($detail.Length -gt 300) { $detail = $detail.Substring(0, 300) } }
    elseif ($null -ne $j.msg) { $detail = $j.msg }
    $script:rows += [pscustomobject]@{ Case = $name; Http = $res.Status; OK = $ok; Detail = $detail }
    return $ok
}

function CheckTrue($name, $cond, $detail) {
    $script:rows += [pscustomobject]@{ Case = $name; Http = 200; OK = [bool]$cond; Detail = $(if ($cond) { "" } else { $detail }) }
}

# ---------- 1. 登录 ----------
$login = Call "POST" "/auth/login" @{ username = $User; password = $Pass; code = ""; uuid = "" } $null
Check "POST /auth/login" $login
$token = $null
try { $token = (Json $login).data.access_token } catch { }
if (-not $token) {
    Write-Host "登录失败，终止: $($login.Body)"
    $script:rows | Format-Table -AutoSize -Wrap | Out-String -Width 200 | Write-Host
    exit 1
}

# ---------- 2. 流程定义 ----------
$defList = Call "GET" "/workflow/definition/list?pageNum=1&pageSize=10&flowCode=leave1" $null $token
Check "GET  /workflow/definition/list" $defList
$def = $null
try { $def = (Json $defList).rows | Select-Object -First 1 } catch { }
CheckTrue "示例流程 leave1 已发布（isPublish=1）" ($null -ne $def -and [int]$def.isPublish -eq 1) "def=$($def | ConvertTo-Json -Compress)"
if ($null -eq $def) {
    $script:rows | Format-Table -AutoSize -Wrap | Out-String -Width 200 | Write-Host
    exit 1
}

$design = Call "GET" "/workflow/definition/design?id=$($def.id)" $null $token
Check "GET  /workflow/definition/design（设计器数据）" $design
$nodeCount = 0
try { $nodeCount = @((Json $design).data.nodeList).Count } catch { }
CheckTrue "设计器返回 5 个节点" ($nodeCount -eq 5) "nodeCount=$nodeCount"

$chart = Call "GET" "/workflow/definition/design" $null $token
Check "GET  /workflow/definition/design（新建壳子）" $chart

# ---------- 3. 请假：保存并提交（发起流程） ----------
$leave = Call "POST" "/workflow/leave/submit" @{
    leaveType = "1"
    startDate = "2026-11-02 09:00:00"
    endDate   = "2026-11-03 18:00:00"
    leaveDays = 2
    remark    = "工作流端到端冒烟"
} $token
Check "POST /workflow/leave/submit（发起流程）" $leave
$start = $null
try { $start = (Json $leave).data } catch { }
if ($null -eq $start) {
    Write-Host "发起失败，终止: $($leave.Body)"
    $script:rows | Format-Table -AutoSize -Wrap | Out-String -Width 200 | Write-Host
    exit 1
}
$instanceId = [string]$start.processInstanceId
$businessId = [string]$start.businessCode
Write-Host "实例 $instanceId / 业务编号 $businessId / 首个待办 $($start.taskId)"
CheckTrue "发起返回流程实例 id" (-not [string]::IsNullOrWhiteSpace($instanceId)) "processInstanceId=$instanceId"
CheckTrue "发起返回业务编号" (-not [string]::IsNullOrWhiteSpace($businessId)) "businessCode=$businessId"

# 提交时系统已自动办结「申请人」，第一条待办应该直接是组长
$first = $null
$myFirst = Call "GET" "/workflow/task/todoList?pageNum=1&pageSize=50" $null $token
try { $first = (Json $myFirst).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
if ($null -eq $first) {
    $allFirst = Call "GET" "/workflow/task/allTodoList?pageNum=1&pageSize=50" $null $token
    try { $first = (Json $allFirst).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
}
CheckTrue "提交后自动办结申请人，首个待办即「组长」" ([string]$first.nodeName -eq "组长") "首个待办=$($first.nodeName)"

# ---------- 4. 逐级审批直到流程结束 ----------
$approvals = 0
for ($i = 1; $i -le 8; $i++) {
    # 先看我的待办，再看全部待办（超管兜底：节点配的角色里可能没有 admin）
    # 注意：实例的 business_id 是请假单主键，不是 LV 开头的申请编号，所以按 instanceId 过滤
    # 「申请人」节点已在提交时自动办结，这里只会轮到组长与部门主管两级
    $todo = $null
    $my = Call "GET" "/workflow/task/todoList?pageNum=1&pageSize=50" $null $token
    try { $todo = (Json $my).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
    if ($null -eq $todo) {
        $all = Call "GET" "/workflow/task/allTodoList?pageNum=1&pageSize=50" $null $token
        try { $todo = (Json $all).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
    }
    if ($null -eq $todo) { break }

    $nodeName = $todo.nodeName
    $done = Call "POST" "/workflow/task/completeTask" @{
        taskId  = [string]$todo.id
        skipType = "PASS"
        message  = "同意（$nodeName）"
    } $token
    $ok = Check "POST /workflow/task/completeTask（$nodeName 通过）" $done
    if (-not $ok) { break }
    $approvals++
}

CheckTrue "手动完成 2 级审批（组长/部门主管，申请人由提交自动办结）" ($approvals -eq 2) "approvals=$approvals"

# ---------- 5. 校验流程与业务状态 ----------
$ins = Call "GET" "/workflow/instance/list?pageNum=1&pageSize=50" $null $token
Check "GET  /workflow/instance/list" $ins
$instance = $null
try { $instance = (Json $ins).rows | Where-Object { [string]$_.id -eq $instanceId } | Select-Object -First 1 } catch { }
CheckTrue "流程实例状态已结束（flowStatus=8）" ($null -ne $instance -and [string]$instance.flowStatus -eq "8") "flowStatus=$($instance.flowStatus)"

$his = Call "GET" "/workflow/task/flowHisTaskList/$instanceId" $null $token
Check "GET  /workflow/task/flowHisTaskList（审批历史）" $his
$hisCount = 0
try { $hisCount = @((Json $his).data).Count } catch { }
CheckTrue "审批历史 >= 4 条（开始+申请人+组长+部门主管）" ($hisCount -ge 4) "hisCount=$hisCount"

$inst = Call "GET" "/workflow/instance/flowChart/$instanceId" $null $token
Check "GET  /workflow/instance/flowChart（流程图）" $inst

$var = Call "GET" "/workflow/instance/variable/$instanceId" $null $token
Check "GET  /workflow/instance/variable（流程变量）" $var

$leaveList = Call "GET" "/workflow/leave/list?pageNum=1&pageSize=10" $null $token
Check "GET  /workflow/leave/list" $leaveList
$leaveRow = $null
try { $leaveRow = (Json $leaveList).rows | Where-Object { $_.applyCode -eq $businessId } | Select-Object -First 1 } catch { }
CheckTrue "请假单状态回写为 finish" ($null -ne $leaveRow -and [string]$leaveRow.status -eq "finish") "status=$($leaveRow.status)"

$doneList = Call "GET" "/workflow/task/doneList?pageNum=1&pageSize=50" $null $token
Check "GET  /workflow/task/doneList（我的已办）" $doneList

# ---------- 6. 汇总 ----------
Write-Host ""
$script:rows | Format-Table -AutoSize -Wrap | Out-String -Width 200 | Write-Host
$fail = ($script:rows | Where-Object { -not $_.OK }).Count
Write-Host ("总计 {0} 项，通过 {1}，失败 {2}" -f $script:rows.Count, ($script:rows.Count - $fail), $fail)
if ($fail -gt 0) {
    Write-Host "`n失败明细:"
    $script:rows | Where-Object { -not $_.OK } | ForEach-Object { Write-Host ("- {0} | http={1} | {2}" -f $_.Case, $_.Http, $_.Detail) }
    exit 1
}
