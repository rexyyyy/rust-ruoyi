<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="search">
        <el-form ref="queryFormRef" :model="queryParams" :inline="true">
          <el-form-item label="请假类型" prop="leaveType">
            <el-select v-model="queryParams.leaveType" placeholder="请选择请假类型" clearable style="width: 160px">
              <el-option v-for="dict in leave_type" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="单据状态" prop="status">
            <el-select v-model="queryParams.status" placeholder="请选择单据状态" clearable style="width: 160px">
              <el-option v-for="dict in wf_business_status" :key="dict.value" :label="dict.label" :value="dict.value" />
            </el-select>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
            <el-button icon="Refresh" @click="resetQuery">重置</el-button>
          </el-form-item>
        </el-form>
      </div>
    </transition>

    <el-card shadow="never">
      <template #header>
        <el-row :gutter="10" class="mb8">
          <el-col :span="1.5">
            <el-button type="primary" plain icon="Plus" @click="handleAdd()" v-hasPermi="['workflow:leave:add']">新增请假</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete()" v-hasPermi="['workflow:leave:remove']"
              >删除</el-button
            >
          </el-col>
          <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="leaveList" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="申请编号" align="center" prop="applyCode" width="160" />
        <el-table-column label="请假类型" align="center" width="110">
          <template #default="scope">
            <dict-tag :options="leave_type" :value="scope.row.leaveType" />
          </template>
        </el-table-column>
        <el-table-column label="开始时间" align="center" prop="startDate" width="160" />
        <el-table-column label="结束时间" align="center" prop="endDate" width="160" />
        <el-table-column label="天数" align="center" prop="leaveDays" width="80" />
        <el-table-column label="单据状态" align="center" width="110">
          <template #default="scope">
            <dict-tag :options="wf_business_status" :value="scope.row.status" />
          </template>
        </el-table-column>
        <el-table-column label="办理流程" align="center" width="110">
          <template #default="scope">
            <el-tag v-if="scope.row.flowCode" size="small" type="info">{{ scope.row.flowCode }}</el-tag>
            <el-text v-else size="small" type="info">未发起</el-text>
          </template>
        </el-table-column>
        <el-table-column label="申请事由" align="center" prop="remark" show-overflow-tooltip />
        <el-table-column label="操作" fixed="right" align="center" width="230" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button
              v-if="scope.row.status === 'draft' || scope.row.status === 'back'"
              link
              type="primary"
              @click="handleUpdate(scope.row)"
              v-hasPermi="['workflow:leave:edit']"
              >编辑</el-button
            >
            <el-button
              v-if="scope.row.status === 'draft' || scope.row.status === 'back'"
              link
              type="primary"
              @click="handleSubmit(scope.row)"
              v-hasPermi="['workflow:leave:submit']"
              >提交审批</el-button
            >
            <el-tooltip v-else content="查看流程图与审批历史" placement="top">
              <el-button link type="primary" @click="handleTrack(scope.row)">审批轨迹</el-button>
            </el-tooltip>
            <el-button link type="danger" @click="handleDelete(scope.row)" v-hasPermi="['workflow:leave:remove']">删除</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 请假单编辑 -->
    <el-dialog :title="dialog.title" v-model="dialog.visible" width="620px" append-to-body>
      <el-form ref="leaveFormRef" :model="form" :rules="rules" label-width="100px">
        <el-form-item label="请假类型" prop="leaveType">
          <el-select v-model="form.leaveType" placeholder="请选择请假类型" class="w-full">
            <el-option v-for="dict in leave_type" :key="dict.value" :label="dict.label" :value="dict.value" />
          </el-select>
        </el-form-item>
        <el-row :gutter="20">
          <el-col :span="12">
            <el-form-item label="开始时间" prop="startDate">
              <el-date-picker v-model="form.startDate" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="选择开始时间" class="w-full" />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="结束时间" prop="endDate">
              <el-date-picker v-model="form.endDate" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="选择结束时间" class="w-full" />
            </el-form-item>
          </el-col>
        </el-row>
        <el-form-item label="请假天数" prop="leaveDays">
          <el-input-number v-model="form.leaveDays" :min="1" :max="365" controls-position="right" />
          <span class="ml2">留空则按起止日期自动计算</span>
        </el-form-item>
        <el-form-item label="申请事由" prop="remark">
          <el-input v-model="form.remark" type="textarea" :rows="3" placeholder="请输入申请事由" maxlength="200" show-word-limit />
        </el-form-item>
        <el-form-item label="办理流程" prop="flowCode">
          <el-select v-model="form.flowCode" clearable placeholder="不选则走默认流程" class="w-full">
            <el-option
              v-for="item in flowOptions"
              :key="item.id"
              :label="`${item.flowName}（${item.flowCode} v${item.version}）`"
              :value="item.flowCode"
            />
          </el-select>
          <div class="form-tip">候选项只列已发布的流程，新设计的流程需先发布才能发起</div>
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="cancel">取 消</el-button>
          <el-button type="primary" :loading="buttonLoading" @click="submitForm('draft')">存为草稿</el-button>
          <el-button type="success" :loading="buttonLoading" @click="submitForm('submit')">提交审批</el-button>
        </div>
      </template>
    </el-dialog>

    <!-- 审批轨迹 -->
    <el-dialog v-model="trackVisible" title="审批轨迹" width="900px" append-to-body>
      <flow-chart :def="chartData" />
      <el-table class="mt8" border :data="hisList">
        <el-table-column label="节点" align="center" prop="nodeName" width="130" />
        <el-table-column label="流转" align="center" width="150">
          <template #default="scope">{{ scope.row.nodeName }} → {{ scope.row.targetNodeName }}</template>
        </el-table-column>
        <el-table-column label="办理人" align="center" width="120">
          <template #default="scope">{{ scope.row.approverName || scope.row.approver }}</template>
        </el-table-column>
        <el-table-column label="状态" align="center" width="110">
          <template #default="scope">
            <dict-tag :options="flow_status" :value="scope.row.flowStatus" />
          </template>
        </el-table-column>
        <el-table-column label="意见" align="center" prop="message" show-overflow-tooltip />
        <el-table-column label="办理时间" align="center" prop="updateTime" width="160" />
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup name="FlowLeave" lang="ts">
import { addLeave, delLeave, getLeave, listLeave, submitLeave, updateLeave } from '@/api/workflow/leave';
import { LeaveForm, LeaveQuery, LeaveVO } from '@/api/workflow/leave/types';
import { flowChart, instanceByBusinessId } from '@/api/workflow/instance';
import { flowHisTaskList } from '@/api/workflow/task';
import { publishedList } from '@/api/workflow/definition';
import { DesignJson, FlowDefinitionVO } from '@/api/workflow/definition/types';
import { FlowHisTaskVO } from '@/api/workflow/task/types';
import FlowChart from '../components/FlowChart.vue';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;

const { leave_type, wf_business_status, flow_status } = toRefs<any>(proxy?.useDict('leave_type', 'wf_business_status', 'flow_status'));

const leaveList = ref<LeaveVO[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const buttonLoading = ref(false);
const ids = ref<Array<string | number>>([]);
const multiple = ref(true);
const total = ref(0);

const queryFormRef = ref<ElFormInstance>();
const leaveFormRef = ref<ElFormInstance>();

const dialog = reactive<DialogOption>({ visible: false, title: '' });

const initFormData: LeaveForm = {
  id: undefined,
  leaveType: '1',
  startDate: undefined,
  endDate: undefined,
  leaveDays: undefined,
  remark: undefined,
  flowCode: undefined
};

const data = reactive<PageData<LeaveForm, LeaveQuery>>({
  form: { ...initFormData },
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    leaveType: undefined,
    status: undefined
  },
  rules: {
    leaveType: [{ required: true, message: '请选择请假类型', trigger: 'change' }],
    startDate: [{ required: true, message: '请选择开始时间', trigger: 'change' }],
    endDate: [{ required: true, message: '请选择结束时间', trigger: 'change' }]
  }
});

const { queryParams, form, rules } = toRefs(data);

/** 查询请假单列表 */
const getList = async () => {
  loading.value = true;
  const res = await listLeave(queryParams.value);
  leaveList.value = res.rows ?? [];
  total.value = res.total ?? 0;
  loading.value = false;
};

/** 搜索按钮操作 */
const handleQuery = () => {
  queryParams.value.pageNum = 1;
  getList();
};

/** 重置按钮操作 */
const resetQuery = () => {
  queryFormRef.value?.resetFields();
  handleQuery();
};

/** 多选框选中数据 */
const handleSelectionChange = (selection: LeaveVO[]) => {
  ids.value = selection.map((item) => item.id);
  multiple.value = !selection.length;
};

const reset = () => {
  form.value = { ...initFormData };
  leaveFormRef.value?.resetFields();
};

const cancel = () => {
  reset();
  dialog.visible = false;
};

const handleAdd = () => {
  reset();
  dialog.visible = true;
  dialog.title = '新增请假申请';
};

const handleUpdate = async (row: LeaveVO) => {
  reset();
  const res = await getLeave(row.id);
  Object.assign(form.value, res.data);
  dialog.visible = true;
  dialog.title = '修改请假申请';
};

/** 按起止日期估算天数（含首尾） */
watch(
  () => [form.value.startDate, form.value.endDate],
  ([start, end]) => {
    if (!start || !end) return;
    const days = Math.max(1, Math.round((new Date(end).getTime() - new Date(start).getTime()) / 86400000) + 1);
    form.value.leaveDays = days;
  }
);

/** 存草稿 / 提交审批 */
const submitForm = async (mode: 'draft' | 'submit') => {
  await leaveFormRef.value?.validate();
  buttonLoading.value = true;
  try {
    if (mode === 'submit') {
      const res = await submitLeave(form.value);
      proxy?.$modal.msgSuccess(`已提交审批，申请编号：${res.data?.businessCode ?? ''}`);
    } else if (form.value.id) {
      await updateLeave(form.value);
      proxy?.$modal.msgSuccess('草稿已保存');
    } else {
      await addLeave(form.value);
      proxy?.$modal.msgSuccess('草稿已保存');
    }
    dialog.visible = false;
    await getList();
  } finally {
    buttonLoading.value = false;
  }
};

/** 列表行提交：先把单据读进弹窗，由用户确认走哪个流程后再发起 */
const handleSubmit = async (row: LeaveVO) => {
  reset();
  const res = await getLeave(row.id);
  Object.assign(form.value, res.data);
  form.value.flowCode = row.flowCode ?? undefined;
  dialog.visible = true;
  dialog.title = '提交审批';
};

/** 已发布流程（发起时可选） */
const flowOptions = ref<FlowDefinitionVO[]>([]);
const getFlowOptions = async () => {
  const res = await publishedList();
  flowOptions.value = res.data ?? [];
};

/** 删除 */
const handleDelete = async (row?: LeaveVO) => {
  const delIds = row?.id ? [row.id] : ids.value;
  await proxy?.$modal.confirm(`是否确认删除选中的 ${delIds.length} 条请假单？`);
  await delLeave(delIds.join(','));
  proxy?.$modal.msgSuccess('删除成功');
  await getList();
};

/** 审批轨迹 */
const trackVisible = ref(false);
const chartData = ref<DesignJson | null>(null);
const hisList = ref<FlowHisTaskVO[]>([]);

const handleTrack = async (row: LeaveVO) => {
  const ins = await instanceByBusinessId(row.id);
  if (!ins.data?.id) {
    proxy?.$modal.msgWarning('该请假单尚未发起流程');
    return;
  }
  const [chartRes, hisRes] = await Promise.all([flowChart(ins.data.id), flowHisTaskList(ins.data.id)]);
  chartData.value = chartRes.data;
  hisList.value = hisRes.data ?? [];
  trackVisible.value = true;
};

onMounted(() => {
  getList();
  getFlowOptions();
});
</script>

<style scoped>
.w-full {
  width: 100%;
}

.ml2 {
  margin-left: 8px;
  color: var(--el-text-color-secondary);
  font-size: 12px;
}

.form-tip {
  width: 100%;
  color: var(--el-text-color-secondary);
  font-size: 12px;
  line-height: 1.5;
}

.mt8 {
  margin-top: 8px;
}
</style>
