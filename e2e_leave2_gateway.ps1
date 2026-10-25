# 端到端：用设计器存盘链路「新建」一个带排他网关的请假流程 leave2，
# 然后实际跑通两条分支：
#   leaveDays <= 2 → 组长（role:1）
#   leaveDays >  2 → 部门主管（role:3@@role:4）→ 总经理（role:1）
# 依赖：backend 以 captcha_enabled: false 启动
# 用法: pwsh -File e2e_leave2_gateway.ps1
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
            $json = $body | ConvertTo-Json -Depth 20 -Compress
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

# ---------- 2. 设计器存盘：新建 leave2（互斥网关 nodeType=3） ----------
# 已存在同名流程时带上最新版本的 id 走「更新」，保证脚本可重复执行
$flowCode = "leave2"
$existId = $null
$existList = Call "GET" "/workflow/definition/list?pageNum=1&pageSize=50&flowCode=$flowCode" $null $token
try {
    $latest = (Json $existList).rows | Sort-Object { [int]$_.version } -Descending | Select-Object -First 1
    if ($null -ne $latest) { $existId = [string]$latest.id }
} catch { }

$codeStart = [guid]::NewGuid().ToString("N")
$codeApply = [guid]::NewGuid().ToString("N")
$codeGate  = [guid]::NewGuid().ToString("N")
$codeLead  = [guid]::NewGuid().ToString("N")
$codeDept  = [guid]::NewGuid().ToString("N")
$codeBoss  = [guid]::NewGuid().ToString("N")
$codeEnd   = [guid]::NewGuid().ToString("N")

$node = {
    param($type, $code, $name, $perm, $coord, $skips)
    [ordered]@{
        nodeType      = $type
        nodeCode      = $code
        nodeName      = $name
        permissionFlag = $perm
        nodeRatio     = "0.000"
        formCustom    = "N"
        coordinate    = $coord
        ext           = "[]"
        skipList      = $skips
    }
}
$skip = {
    param($now, $next, $cond, $skipName, $coord)
    [ordered]@{
        nowNodeCode   = $now
        nextNodeCode  = $next
        skipType      = "PASS"
        skipCondition = $cond
        skipName      = $skipName
        coordinate    = $coord
    }
}

$payload = [ordered]@{
    flowCode   = $flowCode
    flowName   = "请假申请-排他网关"
    modelValue = "CLASSICS"
    category   = "103"
    formCustom = "N"
    formPath   = "/workflow/leaveEdit/index"
    nodeList   = @(
        (& $node 0 $codeStart "开始" $null "200,240|200,240" @(
                (& $skip $codeStart $codeApply $null $null "222,240;345,240")))
        (& $node 1 $codeApply "申请人" "" "400,240|400,240" @(
                (& $skip $codeApply $codeGate $null $null "455,240;570,240")))
        (& $node 3 $codeGate "天数判断" $null "600,240|600,240" @(
                (& $skip $codeGate $codeLead "le@@leaveDays|2" "两天以内" "600,216;600,140;765,140"),
                (& $skip $codeGate $codeDept "gt@@leaveDays|2" "大于两天" "600,264;600,340;765,340")))
        (& $node 1 $codeLead "组长" "role:1" "820,140|820,140" @(
                (& $skip $codeLead $codeEnd $null $null "875,140;1010,140;1010,240;1078,240")))
        (& $node 1 $codeDept "部门主管" "role:3@@role:4" "820,340|820,340" @(
                (& $skip $codeDept $codeBoss $null $null "875,340;985,340")))
        (& $node 1 $codeBoss "总经理" "role:1" "1040,340|1040,340" @(
                (& $skip $codeBoss $codeEnd $null $null "1095,340;1100,262")))
        (& $node 2 $codeEnd "结束" $null "1100,240|1100,240" @())
    )
}
if ($existId) { $payload["id"] = $existId }

$save = Call "POST" "/workflow/definition/design" $payload $token
Check "POST /workflow/definition/design（新建 leave2）" $save
$defId = $null
try { $defId = [string](Json $save).data.id } catch { }
CheckTrue "保存后拿到流程定义 id" (-not [string]::IsNullOrWhiteSpace($defId)) "defId=$defId"
$createTime = $null
try { $createTime = (Json $save).data.createTime } catch { }
CheckTrue "createTime 非空（回归：引擎整行覆写会刷成 null）" ($null -ne $createTime -and "$createTime" -ne "") "createTime=[$createTime]"

# 回读设计器数据，确认网关节点与条件按 warm-flow 语义存下来了
$design = Call "GET" "/workflow/definition/design?id=$defId" $null $token
Check "GET  /workflow/definition/design（回读 leave2）" $design
$dj = Json $design
$gate = $null
try { $gate = $dj.data.nodeList | Where-Object { [int]$_.nodeType -eq 3 } | Select-Object -First 1 } catch { }
CheckTrue "回读含互斥网关（nodeType=3）" ($null -ne $gate) "节点总数=$(@($dj.data.nodeList).Count)"
$condCount = 0
try { $condCount = @($gate.skipList | Where-Object { $_.skipCondition }).Count } catch { }
CheckTrue "网关两条出线都带条件表达式" ($condCount -eq 2) "condCount=$condCount"

$pub = Call "PUT" "/workflow/definition/publish/$defId" $null $token
Check "PUT  /workflow/definition/publish（发布 leave2）" $pub

# ---------- 3. 跑分支：提交请假单 -> 逐级审批 -> 校验走向 ----------
function Run-Branch($title, $days, $start, $end, $expectNodes, $rejectNodes) {
    Write-Host ""
    Write-Host "=== $title（leaveDays=$days） ===" -ForegroundColor Cyan

    $submit = Call "POST" "/workflow/leave/submit" @{
        leaveType = "1"
        startDate = $start
        endDate   = $end
        leaveDays = $days
        remark    = "排他网关分支验证-$title"
        flowCode  = $flowCode
    } $token
    Check "POST /workflow/leave/submit（$title）" $submit
    $sj = Json $submit
    $instanceId = $null
    $applyCode = $null
    try { $instanceId = [string]$sj.data.processInstanceId } catch { }
    try { $applyCode = [string]$sj.data.businessCode } catch { }
    CheckTrue "$title 发起成功拿到实例 id" (-not [string]::IsNullOrWhiteSpace($instanceId)) "instanceId=$instanceId body=$($submit.Body)"
    if ([string]::IsNullOrWhiteSpace($instanceId)) { return }

    # 提交即自动办结「申请人」：此刻该实例的第一条待办应该已经是真正的审批节点
    $firstTodo = $null
    $my0 = Call "GET" "/workflow/task/todoList?pageNum=1&pageSize=50" $null $token
    try { $firstTodo = (Json $my0).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
    if ($null -eq $firstTodo) {
        $all0 = Call "GET" "/workflow/task/allTodoList?pageNum=1&pageSize=50" $null $token
        try { $firstTodo = (Json $all0).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
    }
    $wantFirst = [string]$expectNodes[1]
    CheckTrue "$title 提交后无需再点同意，首个待办即「$wantFirst」" ([string]$firstTodo.nodeName -eq $wantFirst) "首个待办=$($firstTodo.nodeName)"

    $visited = @()
    for ($i = 1; $i -le 8; $i++) {
        $todo = $null
        $my = Call "GET" "/workflow/task/todoList?pageNum=1&pageSize=50" $null $token
        try { $todo = (Json $my).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
        if ($null -eq $todo) {
            $all = Call "GET" "/workflow/task/allTodoList?pageNum=1&pageSize=50" $null $token
            try { $todo = (Json $all).rows | Where-Object { [string]$_.instanceId -eq $instanceId } | Select-Object -First 1 } catch { }
        }
        if ($null -eq $todo) { break }

        $nodeName = $todo.nodeName
        $visited += $nodeName
        $done = Call "POST" "/workflow/task/completeTask" @{
            taskId   = [string]$todo.id
            skipType = "PASS"
            message  = "同意（$nodeName / $title）"
        } $token
        $ok = Check "completeTask（$title · $nodeName）" $done
        if (-not $ok) { break }
    }
    Write-Host "经过节点：$($visited -join ' → ')"

    $ins = Call "GET" "/workflow/instance/list?pageNum=1&pageSize=50" $null $token
    $instance = $null
    try { $instance = (Json $ins).rows | Where-Object { [string]$_.id -eq $instanceId } | Select-Object -First 1 } catch { }
    CheckTrue "$title 实例已办结（flowStatus=8）" ($null -ne $instance -and [string]$instance.flowStatus -eq "8") "flowStatus=$($instance.flowStatus)"
    CheckTrue "$title 实例归属 leave2" ($null -ne $instance -and [string]$instance.definitionId -eq $defId) "definitionId=$($instance.definitionId)"

    $his = Call "GET" "/workflow/task/flowHisTaskList/$instanceId" $null $token
    Check "GET  flowHisTaskList（$title）" $his
    $hisNodes = @()
    try { $hisNodes = @((Json $his).data | ForEach-Object { $_.nodeName }) } catch { }
    foreach ($expect in $expectNodes) {
        $hit = $hisNodes -contains $expect
        CheckTrue "$title 历史经过「$expect」" $hit "hisNodes=$($hisNodes -join ',')"
    }
    foreach ($deny in $rejectNodes) {
        $hit = $hisNodes -contains $deny
        CheckTrue "$title 历史不经过「$deny」" (-not $hit) "hisNodes=$($hisNodes -join ',')"
    }

    $chart = Call "GET" "/workflow/instance/flowChart/$instanceId" $null $token
    Check "GET  flowChart（$title）" $chart
    $cj = Json $chart
    $doneNodes = @()
    try { $doneNodes = @($cj.data.nodeList | Where-Object { [int]$_.status -eq 2 } | ForEach-Object { $_.nodeName }) } catch { }
    foreach ($expect in $expectNodes) {
        CheckTrue "$title 流程图「$expect」标已办理(status=2)" ($doneNodes -contains $expect) "doneNodes=$($doneNodes -join ',')"
    }

    $list = Call "GET" "/workflow/leave/list?pageNum=1&pageSize=20" $null $token
    $row = $null
    try { $row = (Json $list).rows | Where-Object { [string]$_.applyCode -eq $applyCode } | Select-Object -First 1 } catch { }
    CheckTrue "$title 请假单状态回写 finish" ($null -ne $row -and [string]$row.status -eq "finish") "status=$($row.status)"
    CheckTrue "$title 请假单显示走的流程 leave2" ($null -ne $row -and [string]$row.flowCode -eq $flowCode) "flowCode=$($row.flowCode)"
}

# 分支 A：1 天 → 只走组长
Run-Branch "短期分支" 1 "2026-11-02 09:00:00" "2026-11-02 18:00:00" @("申请人", "组长") @("部门主管", "总经理")
# 分支 B：5 天 → 走部门主管 + 总经理
Run-Branch "长期分支" 5 "2026-11-02 09:00:00" "2026-11-06 18:00:00" @("申请人", "部门主管", "总经理") @("组长")

# ---------- 4. 汇总 ----------
Write-Host ""
$script:rows | Format-Table -AutoSize -Wrap | Out-String -Width 200 | Write-Host
$fail = @($script:rows | Where-Object { -not $_.OK }).Count
Write-Host "总计 $($script:rows.Count) 项，失败 $fail" -ForegroundColor $(if ($fail -eq 0) { "Green" } else { "Red" })
exit $fail
