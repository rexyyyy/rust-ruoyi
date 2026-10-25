<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="search">
        <el-form ref="queryFormRef" :model="queryParams" :inline="true">
          <el-form-item label="流程编码" prop="flowCode">
            <el-input v-model="queryParams.flowCode" placeholder="请输入流程编码" clearable @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="流程状态" prop="flowStatus">
            <el-select v-model="queryParams.flowStatus" placeholder="请选择流程状态" clearable style="width: 160px">
              <el-option v-for="dict in flow_status" :key="dict.value" :label="dict.label" :value="dict.value" />
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
            <el-radio-group v-model="scope" @change="handleQuery">
              <el-radio-button :value="false">我的已办</el-radio-button>
              <el-radio-button :value="true">全部已办</el-radio-button>
            </el-radio-group>
          </el-col>
          <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="taskList">
        <el-table-column label="流程名称" align="center" prop="flowName" show-overflow-tooltip />
        <el-table-column label="业务主键" align="center" prop="businessId" width="170" show-overflow-tooltip />
        <el-table-column label="办理节点" align="center" prop="nodeName" width="120" />
        <el-table-column label="流转至" align="center" prop="targetNodeName" width="120" />
        <el-table-column label="办理人" align="center" width="110">
          <template #default="scope">{{ scope.row.approverName || scope.row.approver }}</template>
        </el-table-column>
        <el-table-column label="流转类型" align="center" width="100">
          <template #default="scope">
            <el-tag :type="skipTag(scope.row.skipType)">{{ skipLabel(scope.row.skipType) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="流程状态" align="center" width="110">
          <template #default="scope">
            <dict-tag :options="flow_status" :value="scope.row.flowStatus" />
          </template>
        </el-table-column>
        <el-table-column label="审批意见" align="center" prop="message" show-overflow-tooltip />
        <el-table-column label="办理时间" align="center" prop="updateTime" width="160" />
        <el-table-column label="操作" fixed="right" align="center" width="160" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" icon="View" @click="handleDetail(scope.row)">详情</el-button>
            <el-button link type="primary" icon="Picture" @click="handleChart(scope.row)">流程图</el-button>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 详情 -->
    <el-dialog v-model="detailVisible" title="办理详情" width="860px" append-to-body>
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
    </el-dialog>

    <!-- 流程图 -->
    <el-dialog v-model="chartVisible" title="流程图" width="900px" append-to-body>
      <flow-chart :def="chartData" />
    </el-dialog>
  </div>
</template>

<script setup name="TaskFinish" lang="ts">
import { flowHisTaskList, listAllDone, listDone } from '@/api/workflow/task';
import { FlowHisTaskVO, FlowTaskQuery } from '@/api/workflow/task/types';
import { flowChart } from '@/api/workflow/instance';
import { DesignJson } from '@/api/workflow/definition/types';
import { getDefinition } from '@/api/workflow/definition';
import type { Component } from 'vue';
import FlowChart from '../components/FlowChart.vue';
import LeaveEdit from '../leaveEdit/index.vue';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;

const { flow_status } = toRefs<any>(proxy?.useDict('flow_status'));

const taskList = ref<FlowHisTaskVO[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const total = ref(0);
/** 管理员视角：查看全部已办 */
const scope = ref(false);

const queryFormRef = ref<ElFormInstance>();

const queryParams = ref<FlowTaskQuery>({
  pageNum: 1,
  pageSize: 10,
  flowCode: undefined,
  flowStatus: undefined
});

const skipLabel = (skipType: string) => (skipType === 'REJECT' ? '退回' : skipType === 'NONE' ? '无条件' : '通过');
const skipTag = (skipType: string) => (skipType === 'REJECT' ? 'danger' : skipType === 'NONE' ? 'info' : 'success');

/** 查询已办列表 */
const getList = async () => {
  loading.value = true;
  try {
    const res = scope.value ? await listAllDone(queryParams.value) : await listDone(queryParams.value);
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
// 详情
// ---------------------------------------------------------------------------

const detailVisible = ref(false);
const current = ref<FlowHisTaskVO | null>(null);
const hisList = ref<FlowHisTaskVO[]>([]);
/** 已办列表没有 formPath，按定义反查一次，用于挑业务表单 */
const formPathMap = new Map<string, string>();

const FORM_COMPONENTS: Record<string, Component> = {
  '/workflow/leaveEdit/index': LeaveEdit
};

const bizComp = computed(() => FORM_COMPONENTS[current.value ? (formPathMap.get(current.value.definitionId) ?? '') : ''] ?? null);

const handleDetail = async (row: FlowHisTaskVO) => {
  current.value = row;
  if (!formPathMap.has(row.definitionId)) {
    try {
      const def = await getDefinition(row.definitionId);
      formPathMap.set(row.definitionId, def.data?.formPath ?? '');
    } catch {
      formPathMap.set(row.definitionId, '');
    }
  }
  const res = await flowHisTaskList(row.instanceId);
  hisList.value = res.data ?? [];
  detailVisible.value = true;
};

// ---------------------------------------------------------------------------
// 流程图
// ---------------------------------------------------------------------------

const chartVisible = ref(false);
const chartData = ref<DesignJson | null>(null);

const handleChart = async (row: FlowHisTaskVO) => {
  const res = await flowChart(row.instanceId);
  chartData.value = res.data;
  chartVisible.value = true;
};

onMounted(() => {
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
</style>
