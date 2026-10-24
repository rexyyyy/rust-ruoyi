# MySQL 方言冒烟测试：登录取 token -> 逐个打接口 -> 汇总 PASS/FAIL
# 用法: pwsh -File smoke_mysql.ps1
param(
    [string]$Base   = "http://127.0.0.1:60800/dev-api",
    [string]$Redis  = "C:\Program Files\Redis\redis-cli.exe",
    [string]$User   = "admin",
    [string]$Pass   = "admin123"
)

$ErrorActionPreference = "Continue"
$script:rows = @()

function Call($method, $path, $body, $token) {
    $headers = @{}
    if ($token) { $headers["Authorization"] = "Bearer " + $token }
    $uri = $Base + $path
    try {
        if ($null -ne $body) {
            $json = $body | ConvertTo-Json -Depth 8 -Compress
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

function Check($name, $res, [switch]$AllowNon200) {
    $ok = $false
    $code = -1
    $msg  = ""
    try {
        $j = $res.Body | ConvertFrom-Json
        if ($null -ne $j.code) { $code = [int]$j.code }
        if ($j.msg) { $msg = [string]$j.msg }
    } catch { }
    if ($res.Status -eq 200 -and ($code -eq 200 -or $code -eq 0)) { $ok = $true }
    if ($AllowNon200 -and $res.Status -eq 200) { $ok = $true }
    $snippet = ""
    if (-not $ok) { $snippet = ($res.Body -replace "\s+", " ") ; if ($snippet.Length -gt 240) { $snippet = $snippet.Substring(0, 240) } }
    $script:rows += [pscustomobject]@{ Case = $name; Http = $res.Status; Code = $code; OK = $ok; Detail = $msg + $snippet }
}

function CheckGone($name, $res) {
    $ok = $false
    $code = -1
    try {
        $j = $res.Body | ConvertFrom-Json
        if ($null -ne $j.code) { $code = [int]$j.code }
    } catch { }
    if ($res.Status -eq 200 -and $code -ne 200) { $ok = $true }
    $snippet = ""
    if (-not $ok) { $snippet = ($res.Body -replace "\s+", " ") }
    $script:rows += [pscustomobject]@{ Case = $name; Http = $res.Status; Code = $code; OK = $ok; Detail = $snippet }
}

function CheckTrue($name, $cond, $detail) {
    $script:rows += [pscustomobject]@{ Case = $name; Http = 200; Code = 200; OK = [bool]$cond; Detail = $(if ($cond) { "" } else { $detail }) }
}

# ---------- 1. 验证码 + 登录 ----------
$cap = Call "GET" "/captchaImage" $null $null
$uuid = $null
$codeVal = ""
try {
    $capJson = $cap.Body | ConvertFrom-Json
    if ($capJson.data.captchaEnabled -eq $false) {
        Write-Host "captcha 已关闭（captchaEnabled=false），直接密码登录"
        $uuid = ""
    } else {
        $uuid = $capJson.data.uuid
        $codeVal = (& $Redis get "login_captcha:$uuid").Trim()
    }
} catch { }
if ($uuid -eq $null) { Write-Host "captchaImage 异常: $($cap.Status) $($cap.Body)"; exit 1 }
Write-Host "captcha uuid=$uuid code=$codeVal"

$login = Call "POST" "/auth/login" @{ username = $User; password = $Pass; code = $codeVal; uuid = $uuid } $null
$token = $null
try { $token = ($login.Body | ConvertFrom-Json).data.access_token } catch { }
Check "POST /auth/login (验证码+密码+写登录日志)" $login
if (-not $token) { Write-Host "登录失败，终止: $($login.Body)"; $script:rows | Format-Table -AutoSize | Out-String -Width 220; exit 1 }

# ---------- 2. 只读接口 ----------
$gets = @(
    @("/getInfo",                                "GET  用户信息+权限+角色"),
    @("/getRouters",                             "GET  动态路由（菜单树）"),
    @("/system/user/list?pageNum=1&pageSize=10", "GET  用户分页"),
    @("/system/user/list?pageNum=1&pageSize=5&userName=ad", "GET  用户分页-条件查询"),
    @("/system/user/getInfo",                    "GET  用户信息(镜像路径)"),
    @("/system/user/deptTree",                   "GET  用户部门树"),
    @("/system/role/list?pageNum=1&pageSize=10", "GET  角色分页"),
    @("/system/role/optionselect",               "GET  角色下拉"),
    @("/system/menu/list",                       "GET  菜单列表"),
    @("/system/menu/treeselect",                 "GET  菜单树"),
    @("/system/dept/list",                       "GET  部门列表"),
    @("/system/dept/optionselect",                  "GET  部门下拉"),
    @("/system/post/list?pageNum=1&pageSize=10", "GET  岗位分页"),
    @("/system/dict/type/list?pageNum=1&pageSize=10", "GET  字典类型分页"),
    @("/system/dict/data/type/sys_user_sex",     "GET  字典数据按类型"),
    @("/system/dict/data/list?pageNum=1&pageSize=10", "GET  字典数据分页"),
    @("/system/config/list?pageNum=1&pageSize=10","GET  参数配置分页"),
    @("/system/config/configKey/sys.account.captcha_enabled", "GET  参数按 key 查询"),
    @("/system/notice/list?pageNum=1&pageSize=10","GET  通知公告分页"),
    @("/system/user/profile",                    "GET  个人中心"),
    @("/monitor/operlog/list?pageNum=1&pageSize=10", "GET  操作日志分页"),
    @("/monitor/logininfor/list?pageNum=1&pageSize=10", "GET  登录日志分页"),
    @("/monitor/online/list",                    "GET  在线用户"),
    @("/monitor/server",                         "GET  服务器监控"),
    @("/monitor/cache",                          "GET  缓存监控"),
    @("/job/list?pageNum=1&pageSize=10",         "GET  定时任务分页"),
    @("/job/log/list?pageNum=1&pageSize=10",     "GET  定时任务日志"),
    @("/tool/gen/list?pageNum=1&pageSize=10",    "GET  代码生成-已导入表"),
    @("/tool/gen/db/list?pageNum=1&pageSize=10", "GET  代码生成-数据库表元数据"),
    @("/demo/course/list?pageNum=1&pageSize=10", "GET  演示课程分页")
)
foreach ($g in $gets) { Check -name $g[1] -res (Call "GET" $g[0] $null $token) }

# ---------- 3. 写接口（新增->修改->删除） ----------
$dictType = "zz_mysql_probe"
$addDict = Call "POST" "/system/dict/type" @{
    dictName = "MySQL冒烟"; dictType = $dictType; status = "0"; remark = "mysql smoke"
} $token
Check "POST /system/dict/type 新增字典类型" $addDict
# 注意：add 返回的是影响行数，不是新主键（与 RuoYi 一致），主键必须回查列表
$dictId = $null
$listDict = Call "GET" "/system/dict/type/list?pageNum=1&pageSize=10&dictType=$dictType" $null $token
Check "GET  字典类型回查" $listDict
try { $dictId = ($listDict.Body | ConvertFrom-Json).rows[0].dictId } catch { }
if ($dictId) {
    Check "PUT  修改字典类型" (Call "PUT" "/system/dict/type" @{
        dictId = [string]$dictId; dictName = "MySQL冒烟改"; dictType = $dictType; status = "0"; remark = "mysql smoke2"
    } $token)
    Check "DELETE 删除字典类型" (Call "DELETE" "/system/dict/type/$dictId" $null $token)
}

$addNotice = Call "POST" "/system/notice" @{
    noticeTitle = "MySQL冒烟公告"; noticeType = "1"; noticeContent = "hello mysql"; status = "0"
} $token
Check "POST /system/notice 新增公告（含 text 列/时间列）" $addNotice
$noticeList = Call "GET" "/system/notice/list?pageNum=1&pageSize=10&noticeTitle=MySQL冒烟公告" $null $token
Check "GET  公告回查" $noticeList
$noticeId = $null
try { $noticeId = ($noticeList.Body | ConvertFrom-Json).rows[0].noticeId } catch { }
if ($noticeId) {
    Check "PUT  修改公告" (Call "PUT" "/system/notice" @{
        noticeId = [string]$noticeId; noticeTitle = "MySQL冒烟公告-已改"; noticeType = "2";
        noticeContent = "hello mysql v2"; status = "0"
    } $token)
    Check "DELETE 删除公告" (Call "DELETE" "/system/notice/$noticeId" $null $token)
    CheckGone "GET  确认公告已删" (Call "GET" "/system/notice/$noticeId" $null $token)
}

$courseCode = "MYSQL-" + (Get-Random -Maximum 99999)
$course = Call "POST" "/demo/course" @{
    courseCode = $courseCode
    courseName = "MySQL 方言验证课程"
    teacher = "tester"
    credit = 3.5
    openStatus = "0"
    startDate = "2026-10-24"
    remark = "smoke"
} $token
Check "POST /demo/course 新增课程（雪花ID+double+date）" $course
# add 同样只返回影响行数，主键要回查（按唯一 course_code）
$courseList = Call "GET" "/demo/course/list?pageNum=1&pageSize=10&courseCode=$courseCode" $null $token
Check "GET  课程回查" $courseList
$courseId = $null
try { $courseId = ($courseList.Body | ConvertFrom-Json).rows[0].courseId } catch { }
if ($courseId) {
    Check "PUT  修改课程" (Call "PUT" "/demo/course" @{
        courseId = [string]$courseId; courseCode = $courseCode; courseName = "MySQL 方言验证课程-改";
        teacher = "tester2"; credit = 4.0; openStatus = "1"; startDate = "2026-10-25"; remark = "smoke-upd"
    } $token)
    $courseDetail = Call "GET" "/demo/course/$courseId" $null $token
    $teacherOk = $false
    try { $teacherOk = (($courseDetail.Body | ConvertFrom-Json).data.teacher -eq "tester2") } catch { }
    $script:rows += [pscustomobject]@{ Case = "GET  课程详情校验 teacher=tester2"; Http = $courseDetail.Status; Code = 200; OK = $teacherOk; Detail = "" }
    Check "DELETE 删除课程" (Call "DELETE" "/demo/course/$courseId" $null $token)
    CheckGone "GET  确认课程已删" (Call "GET" "/demo/course/$courseId" $null $token)
}

Check "POST /tool/gen/importTable 导入表(demo_course)" (Call "POST" "/tool/gen/importTable?tables=demo_course" $null $token)
$genList = Call "GET" "/tool/gen/list?pageNum=1&pageSize=10&tableName=demo_course" $null $token
Check "GET  代码生成-已导入表回查" $genList
$tableId = $null
try { $tableId = ($genList.Body | ConvertFrom-Json).rows[0].tableId } catch { }
if ($tableId) {
    Check "GET  代码生成-表详情" (Call "GET" "/tool/gen/$tableId" $null $token)
    Check "GET  代码生成-预览"   (Call "GET" "/tool/gen/preview/$tableId" $null $token)
}

# ---------- 4. 组织 / 权限写接口（覆盖 char(1)/bool 列） ----------
# 岗位
$postCode = "zz_probe_post"
Check "POST /system/post 新增岗位" (Call "POST" "/system/post" @{
    postCode = $postCode; postName = "MySQL探针岗位"; postSort = 99; status = "0"; remark = "smoke"
} $token)
$postList = Call "GET" "/system/post/list?pageNum=1&pageSize=10&postCode=$postCode" $null $token
Check "GET  岗位回查" $postList
$postId = $null
try { $postId = ($postList.Body | ConvertFrom-Json).rows[0].postId } catch { }
if ($postId) { Check "DELETE 删除岗位" (Call "DELETE" "/system/post/$postId" $null $token) }

# 部门
try {
    $deptName = "MySQL探针部门"
    Check "POST /system/dept 新增部门" (Call "POST" "/system/dept" @{
        parentId = "100"; ancestors = "0,100"; deptName = $deptName; orderNum = 99;
        leader = "1"; phone = ""; email = ""
    } $token)
    $deptList = Call "GET" "/system/dept/list" $null $token
    Check "GET  部门回查" $deptList
    $deptId = $null
    try { $deptId = (($deptList.Body | ConvertFrom-Json).rows | Where-Object { $_.deptName -eq $deptName } | Select-Object -First 1).deptId } catch { }
    if ($deptId) { Check "DELETE 删除部门" (Call "DELETE" "/system/dept/$deptId" $null $token) }
} catch { }

# 角色（menu_check_strictly/dept_check_strictly 为 bool 列）
$roleKey = "zz_probe_role"
Check "POST /system/role 新增角色（含菜单授权）" (Call "POST" "/system/role" @{
    roleName = "MySQL探针角色"; roleKey = $roleKey; roleSort = 99; dataScope = "1";
    menuCheckStrictly = $true; deptCheckStrictly = $true; menuIds = @(1); status = "0"; remark = "smoke"
} $token)
$roleList = Call "GET" "/system/role/list?pageNum=1&pageSize=10&roleKey=$roleKey" $null $token
Check "GET  角色回查" $roleList
$roleId = $null
try { $roleId = ($roleList.Body | ConvertFrom-Json).rows[0].roleId } catch { }
if ($roleId) {
    $rt = Call "GET" "/system/menu/roleMenuTreeselect/$roleId" $null $token
    Check "GET  角色菜单授权回查" $rt
    $checked1 = @()
    try { $checked1 = @(($rt.Body | ConvertFrom-Json).data.checkedKeys) } catch { }
    CheckTrue "角色新增后已绑定菜单1" ($checked1 -contains 1) "checkedKeys=$($checked1 -join ',')"

    # 状态开关只带 roleId+status：既不写 NULL 到 role_name，也不清空菜单授权
    Check "PUT  角色改状态" (Call "PUT" "/system/role/changeStatus" @{ roleId = [string]$roleId; status = "1" } $token)
    $roleList2 = Call "GET" "/system/role/list?pageNum=1&pageSize=10&roleKey=$roleKey" $null $token
    $r2 = $null
    try { $r2 = ($roleList2.Body | ConvertFrom-Json).rows[0] } catch { }
    CheckTrue "改状态后 roleName 未被写空" ($null -ne $r2 -and $r2.roleName -eq "MySQL探针角色") "roleName=$($r2.roleName)"
    CheckTrue "改状态后 status 已生效" ($null -ne $r2 -and $r2.status -eq "1") "status=$($r2.status)"
    $rt2 = Call "GET" "/system/menu/roleMenuTreeselect/$roleId" $null $token
    $checked2 = @()
    try { $checked2 = @(($rt2.Body | ConvertFrom-Json).data.checkedKeys) } catch { }
    CheckTrue "改状态后菜单授权未被清空" ($checked2 -contains 1) "checkedKeys=$($checked2 -join ',')"

    Check "DELETE 删除角色" (Call "DELETE" "/system/role/$roleId" $null $token)
}

# 菜单（is_frame/is_cache/visible/status 为 char(1)）
$menuName = "MySQL探针菜单"
Check "POST /system/menu 新增菜单" (Call "POST" "/system/menu" @{
    menuName = $menuName; parentId = 0; orderNum = 99; path = "zzprobe"; component = "";
    query = ""; isFrame = "1"; isCache = "0"; menuType = "C"; visible = "0"; status = "0";
    perms = "zz:probe:list"; icon = "#"; remark = "smoke"
} $token)
$menuList = Call "GET" "/system/menu/list" $null $token
Check "GET  菜单回查" $menuList
$menuId = $null
try { $menuId = (($menuList.Body | ConvertFrom-Json).data | Where-Object { $_.menuName -eq $menuName } | Select-Object -First 1).menuId } catch { }
if ($menuId) { Check "DELETE 删除菜单" (Call "DELETE" "/system/menu/$menuId" $null $token) }

# 用户（sex 为 char(1)，关联角色/岗位）
$probeUser = "zz_probe_user"
Check "POST /system/user 新增用户" (Call "POST" "/system/user" @{
    deptId = "103"; userName = $probeUser; nickName = "MySQL探针"; email = "zz@test.com";
    phonenumber = "13800000001"; sex = "0"; password = "Aa123456"; status = "0";
    remark = "smoke"; roleIds = @("2"); postIds = @("1")
} $token)
$userList = Call "GET" "/system/user/list?pageNum=1&pageSize=10&userName=$probeUser" $null $token
Check "GET  用户回查" $userList
$probeUserId = $null
try { $probeUserId = ($userList.Body | ConvertFrom-Json).rows[0].userId } catch { }
if ($probeUserId) {
    Check "PUT  用户改状态" (Call "PUT" "/system/user/changeStatus" @{ userId = [string]$probeUserId; status = "0" } $token)
    Check "DELETE 删除用户" (Call "DELETE" "/system/user/$probeUserId" $null $token)
}

# ---------- 5. 汇总 ----------
Write-Host ""
$script:rows | Format-Table -AutoSize -Wrap | Out-String -Width 200 | Write-Host
$fail = ($script:rows | Where-Object { -not $_.OK }).Count
Write-Host ("总计 {0} 项，通过 {1}，失败 {2}" -f $script:rows.Count, ($script:rows.Count - $fail), $fail)
if ($fail -gt 0) {
    Write-Host "`n失败明细:"
    $script:rows | Where-Object { -not $_.OK } | ForEach-Object { Write-Host ("- {0} | http={1} code={2} | {3}" -f $_.Case, $_.Http, $_.Code, $_.Detail) }
    exit 1
}
