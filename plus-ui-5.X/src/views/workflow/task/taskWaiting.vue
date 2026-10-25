<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="search">
        <el-form ref="queryFormRef" :model="queryParams" :inline="true">
          <el-form-item label="流程编码" prop="flowCode">
            <el-input v-model="queryParams.flowCode" placeholder="请输入流程编码" clearable @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="节点编码" prop="nodeCode">
            <el-input v-model="queryParams.nodeCode" placeholder="请输入节点编码" clearable @keyup.enter="handleQuery" />
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
            <el-radio-group v-model="scope" @change="handleQuery">
              <el-radio-button :value="false">我的待办</el-radio-button>
              <el-radio-button :value="true">全部待办</el-radio-button>
            </el-radio-group>
          </el-col>
          <el-col :span="1.5">
            <el-text size="small" type="info">全部待办为管理员视角，可代办未分配给自己的节点</el-text>
          </el-col>
          <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="taskList">
        <el-table-column label="流程名称" align="center" prop="flowName" show-overflow-tooltip />
        <el-table-column label="流程编码" align="center" prop="flowCode" width="120" />
        <el-table-column label="当前节点" align="center" prop="nodeName" width="120" />
        <el-table-column label="业务主键" align="center" prop="businessId" width="170" show-overflow-tooltip />
        <el-table-column label="流程状态" align="center" width="110">
          <template #default="scope">
            <dict-tag :options="flow_status" :value="scope.row.flowStatus" />
          </template>
        </el-table-column>
        <el-table-column label="到达时间" align="center" prop="createTime" width="160" />
        <el-table-column label="操作" fixed="right" align="center" width="260" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="Check" @click="handleOpen(scope.row)">办理</el-button>
            <el-button link type="primary" icon="Share" @click="handleTransfer(scope.row)">转办</el-button>
            <el-button link type="danger" icon="CircleClose" @click="handleTermination(scope.row)">终止</el-button>
            <el-button link type="primary" icon="Picture" @click="handleChart(scope.row)">流程图</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 办理 -->
    <el-dialog v-model="handleVisible" title="任务办理" width="860px" append-to-body>
      <el-row :gutter="16">
        <el-col :span="12">
          <div class="dp-block-title">业务信息</div>
          <component :is="bizComp" v-if="bizComp" :business-id="current?.businessId" />
          <el-empty v-else description="该流程未配置可预览的业务表单" :image-size="70" />
        </el-col>
        <el-col :span="12">
          <div class="dp-block-title">流转历史</div>
          <el-timeline class="mt4">
            <el-timeline-item v-for="item in hisList" :key="item.id" :timestamp="item.updateTime" placement="top">
              <div>{{ item.nodeName }} → {{ item.targetNodeName }}</div>
              <div class="dp-his-sub">{{ item.approverName || item.approver }} · {{ item.message || '（无意见）' }}</div>
            </el-timeline-item>
          </el-timeline>
        </el-col>
      </el-row>
      <el-divider />
      <el-form ref="handleFormRef" :model="handleForm" :rules="handleRules" label-width="90px">
        <el-form-item label="流转动作" prop="skipType">
          <el-radio-group v-model="handleForm.skipType">
            <el-radio-button value="PASS">同意</el-radio-button>
            <el-radio-button value="REJECT">退回</el-radio-button>
          </el-radio-group>
        </el-form-item>
        <el-form-item label="审批意见" prop="message">
          <el-input v-model="handleForm.message" type="textarea" :rows="3" placeholder="请输入审批意见" maxlength="200" show-word-limit />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="handleVisible = false">取 消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitHandle">确 定</el-button>
      </template>
    </el-dialog>

    <!-- 转办 -->
    <el-dialog v-model="transferVisible" title="转办任务" width="520px" append-to-body>
      <el-form label-width="90px">
        <el-form-item label="转办给">
          <el-select v-model="transferHandlers" multiple filterable placeholder="请选择办理人" class="w-full">
            <el-option v-for="u in userOptions" :key="u.userId" :label="u.nickName" :value="String(u.userId)" />
          </el-select>
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="transferMessage" type="textarea" :rows="2" placeholder="转办原因" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="transferVisible = false">取 消</el-button>
        <el-button type="primary" :loading="submitting" @click="submitTransfer">确 定</el-button>
      </template>
    </el-dialog>

    <!-- 流程图 -->
    <el-dialog v-model="chartVisible" title="流程图" width="900px" append-to-body>
      <flow-chart :def="chartData" />
    </el-dialog>
  </div>
</template>

<script setup name="TaskWaiting" lang="ts">
import { completeTask, flowHisTaskList, listAllTodo, listTodo, taskOperation, terminationTask } from '@/api/workflow/task';
import { FlowHisTaskVO, FlowTaskQuery, FlowTaskVO } from '@/api/workflow/task/types';
import { flowChart } from '@/api/workflow/instance';
import { DesignJson } from '@/api/workflow/definition/types';
import { listUser } from '@/api/system/user';
import FlowChart from '../components/FlowChart.vue';
import LeaveEdit from '../leaveEdit/index.vue';
import type { Component } from 'vue';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;

const { flow_status } = toRefs<any>(proxy?.useDict('flow_status'));

const taskList = ref<FlowTaskVO[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const total = ref(0);
/** 管理员视角：查看全部待办 */
const scope = ref(false);

const queryFormRef = ref<ElFormInstance>();

const queryParams = ref<FlowTaskQuery>({
  pageNum: 1,
  pageSize: 10,
  flowCode: undefined,
  nodeCode: undefined
});

/** 查询待办列表 */
const getList = async () => {
  loading.value = true;
  try {
    const res = scope.value ? await listAllTodo(queryParams.value) : await listTodo(queryParams.value);
    taskList.value = res.rows ?? [];
    total.value = res.total ?? 0;
  } finally {
    loading.value = false;
  }
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

// ---------------------------------------------------------------------------
// 办理
// ---------------------------------------------------------------------------

const handleVisible = ref(false);
const submitting = ref(false);
const current = ref<FlowTaskVO | null>(null);
const hisList = ref<FlowHisTaskVO[]>([]);

/** 业务表单：按流程定义里的 formPath 找前端组件 */
const FORM_COMPONENTS: Record<string, Component> = {
  '/workflow/leaveEdit/index': LeaveEdit
};

const bizComp = computed(() => FORM_COMPONENTS[current.value?.formPath ?? ''] ?? null);

const handleFormRef = ref<ElFormInstance>();
const handleForm = reactive({ skipType: 'PASS', message: '' });
const handleRules = {
  skipType: [{ required: true, message: '请选择流转动作', trigger: 'change' }]
};

const handleOpen = async (row: FlowTaskVO) => {
  current.value = row;
  handleForm.skipType = 'PASS';
  handleForm.message = '';
  const res = await flowHisTaskList(row.instanceId);
  hisList.value = res.data ?? [];
  handleVisible.value = true;
};

const submitHandle = async () => {
  await handleFormRef.value?.validate();
  submitting.value = true;
  try {
    await completeTask({
      taskId: current.value!.id,
      skipType: handleForm.skipType,
      message: handleForm.message
    });
    proxy?.$modal.msgSuccess(handleForm.skipType === 'PASS' ? '已同意' : '已退回');
    handleVisible.value = false;
    await getList();
  } finally {
    submitting.value = false;
  }
};

// ---------------------------------------------------------------------------
// 转办
// ---------------------------------------------------------------------------

const transferVisible = ref(false);
const transferHandlers = ref<string[]>([]);
const transferMessage = ref('');
const userOptions = ref<any[]>([]);

const loadUsers = async () => {
  const res = await listUser({ pageNum: 1, pageSize: 500 });
  userOptions.value = res.rows ?? [];
};

const handleTransfer = (row: FlowTaskVO) => {
  current.value = row;
  transferHandlers.value = [];
  transferMessage.value = '';
  transferVisible.value = true;
};

const submitTransfer = async () => {
  if (!transferHandlers.value.length) {
    proxy?.$modal.msgError('请选择转办目标');
    return;
  }
  submitting.value = true;
  try {
    await taskOperation('transfer', {
      taskId: current.value!.id,
      handlerIds: transferHandlers.value,
      message: transferMessage.value
    });
    proxy?.$modal.msgSuccess('已转办');
    transferVisible.value = false;
    await getList();
  } finally {
    submitting.value = false;
  }
};

// ---------------------------------------------------------------------------
// 终止 / 流程图
// ---------------------------------------------------------------------------

const handleTermination = async (row: FlowTaskVO) => {
  const { value } = await proxy?.$modal.prompt(`是否确认终止流程「${row.flowName}」？请填写终止原因`);
  await terminationTask({ taskId: row.id, message: value });
  proxy?.$modal.msgSuccess('已终止');
  await getList();
};

const chartVisible = ref(false);
const chartData = ref<DesignJson | null>(null);

const handleChart = async (row: FlowTaskVO) => {
  const res = await flowChart(row.instanceId);
  chartData.value = res.data;
  chartVisible.value = true;
};

onMounted(() => {
  loadUsers();
  getList();
});
</script>

<style scoped>
.dp-block-title {
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 8px;
}

.dp-his-sub {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.mt4 {
  margin-top: 4px;
}

.w-full {
  width: 100%;
}
</style>
