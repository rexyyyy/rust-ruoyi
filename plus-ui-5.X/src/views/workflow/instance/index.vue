<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="search">
        <el-form ref="queryFormRef" :model="queryParams" :inline="true">
          <el-form-item label="流程编码" prop="flowCode">
            <el-input v-model="queryParams.flowCode" placeholder="请输入流程编码" clearable @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="业务主键" prop="businessId">
            <el-input v-model="queryParams.businessId" placeholder="请输入业务主键" clearable @keyup.enter="handleQuery" />
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
            <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete()" v-hasPermi="['workflow:instance:remove']"
              >删除</el-button
            >
          </el-col>
          <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="instanceList" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="流程实例号" align="center" prop="id" width="180" show-overflow-tooltip />
        <el-table-column label="流程名称" align="center" prop="flowName" show-overflow-tooltip />
        <el-table-column label="流程编码" align="center" prop="flowCode" show-overflow-tooltip />
        <el-table-column label="业务主键" align="center" prop="businessId" width="170" show-overflow-tooltip />
        <el-table-column label="当前节点" align="center" prop="nodeName" width="120" />
        <el-table-column label="流程状态" align="center" width="110">
          <template #default="scope">
            <dict-tag :options="flow_status" :value="scope.row.flowStatus" />
          </template>
        </el-table-column>
        <el-table-column label="激活状态" align="center" width="100">
          <template #default="scope">
            <el-tag :type="activeTag(scope.row).type">{{ activeTag(scope.row).text }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="发起人" align="center" width="110">
          <template #default="scope">{{ scope.row.createByName || scope.row.createBy }}</template>
        </el-table-column>
        <el-table-column label="发起时间" align="center" prop="createTime" width="160" />
        <el-table-column label="操作" fixed="right" align="center" width="180" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-button link type="primary" @click="handleChart(scope.row)">流程图</el-button>
            <el-button link type="primary" @click="handleVariable(scope.row)">流程变量</el-button>
            <el-button
              link
              type="primary"
              :icon="scope.row.activityStatus === 1 ? 'VideoPause' : 'VideoPlay'"
              :disabled="isFinished(scope.row)"
              @click="handleActive(scope.row)"
              v-hasPermi="['workflow:instance:edit']"
            />
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 流程图 -->
    <el-dialog v-model="chartVisible" title="流程图" width="900px" append-to-body>
      <flow-chart :def="chartData" />
    </el-dialog>

    <!-- 流程变量 -->
    <el-dialog v-model="variableVisible" title="流程变量" width="640px" append-to-body>
      <el-table border :data="variableRows">
        <el-table-column label="变量名" prop="key" width="200" />
        <el-table-column label="变量值" prop="value" show-overflow-tooltip />
      </el-table>
    </el-dialog>
  </div>
</template>

<script setup name="FlowInstance" lang="ts">
import { activeInstance, delInstance, flowChart, instanceVariable, unActiveInstance, listInstance } from '@/api/workflow/instance';
import { FlowInstanceQuery, FlowInstanceVO } from '@/api/workflow/instance/types';
import { DesignJson } from '@/api/workflow/definition/types';
import FlowChart from '../components/FlowChart.vue';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;

const { flow_status } = toRefs<any>(proxy?.useDict('flow_status'));

const instanceList = ref<FlowInstanceVO[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref<Array<string | number>>([]);
const multiple = ref(true);
const total = ref(0);

const queryFormRef = ref<ElFormInstance>();

const queryParams = ref<FlowInstanceQuery>({
  pageNum: 1,
  pageSize: 10,
  flowCode: undefined,
  businessId: undefined,
  flowStatus: undefined
});

/** 查询流程实例列表 */
const getList = async () => {
  loading.value = true;
  const res = await listInstance(queryParams.value);
  instanceList.value = res.rows ?? [];
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
const handleSelectionChange = (selection: FlowInstanceVO[]) => {
  ids.value = selection.map((item) => item.id);
  multiple.value = !selection.length;
};

/** 流程图 */
const chartVisible = ref(false);
const chartData = ref<DesignJson | null>(null);

const handleChart = async (row: FlowInstanceVO) => {
  const res = await flowChart(row.id);
  chartData.value = res.data;
  chartVisible.value = true;
};

/** 流程变量 */
const variableVisible = ref(false);
const variableRows = ref<Array<{ key: string; value: string }>>([]);

const handleVariable = async (row: FlowInstanceVO) => {
  const res = await instanceVariable(row.id);
  variableRows.value = Object.entries(res.data ?? {}).map(([key, value]) => ({
    key,
    value: typeof value === 'string' ? value : JSON.stringify(value)
  }));
  variableVisible.value = true;
};

/** 办结后引擎不再回写 activity_status，这里按流程状态说“已结束”，免得与前一列语义打架 */
const isFinished = (row: FlowInstanceVO) => String(row.flowStatus) === '8';
const activeTag = (row: FlowInstanceVO): { type: 'success' | 'info'; text: string } => {
  if (isFinished(row)) return { type: 'info', text: '已结束' };
  return row.activityStatus === 1 ? { type: 'success', text: '激活' } : { type: 'info', text: '挂起' };
};

/** 激活 / 挂起 */
const handleActive = async (row: FlowInstanceVO) => {
  if (row.activityStatus === 1) {
    await unActiveInstance(row.id);
    proxy?.$modal.msgSuccess('已挂起');
  } else {
    await activeInstance(row.id);
    proxy?.$modal.msgSuccess('已激活');
  }
  await getList();
};

/** 删除 */
const handleDelete = async (row?: FlowInstanceVO) => {
  const delIds = row?.id ? [row.id] : ids.value;
  await proxy?.$modal.confirm(`是否确认删除选中的 ${delIds.length} 条流程实例？删除后待办任务也会一并消失。`);
  await delInstance(delIds.join(','));
  proxy?.$modal.msgSuccess('删除成功');
  await getList();
};

onMounted(() => {
  getList();
});
</script>
