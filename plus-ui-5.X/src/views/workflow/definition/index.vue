<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="search">
        <el-form ref="queryFormRef" :model="queryParams" :inline="true">
          <el-form-item label="流程名称" prop="flowName">
            <el-input v-model="queryParams.flowName" placeholder="请输入流程名称" clearable @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="流程编码" prop="flowCode">
            <el-input v-model="queryParams.flowCode" placeholder="请输入流程编码" clearable @keyup.enter="handleQuery" />
          </el-form-item>
          <el-form-item label="流程分类" prop="category">
            <el-tree-select
              v-model="queryParams.category"
              :data="categoryOptions"
              :props="{ value: 'id', label: 'label', children: 'children' } as any"
              value-key="id"
              placeholder="请选择流程分类"
              check-strictly
              clearable
              style="width: 200px"
            />
          </el-form-item>
          <el-form-item label="发布状态" prop="isPublish">
            <el-select v-model="queryParams.isPublish" placeholder="请选择" clearable style="width: 140px">
              <el-option label="未发布" :value="0" />
              <el-option label="已发布" :value="1" />
              <el-option label="已失效" :value="9" />
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
            <el-button type="primary" plain icon="Plus" @click="handleDesign()" v-hasPermi="['workflow:definition:add']">新建流程</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button type="warning" plain icon="Upload" @click="handleImport" v-hasPermi="['workflow:definition:import']">导入流程</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete()" v-hasPermi="['workflow:definition:remove']"
              >删除</el-button
            >
          </el-col>
          <right-toolbar v-model:showSearch="showSearch" @queryTable="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="definitionList" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="流程编号" align="center" prop="flowCode" show-overflow-tooltip />
        <el-table-column label="流程名称" align="center" prop="flowName" show-overflow-tooltip />
        <el-table-column label="流程分类" align="center" prop="categoryName" width="110" />
        <el-table-column label="版本号" align="center" prop="version" width="80" />
        <el-table-column label="发布状态" align="center" width="100">
          <template #default="scope">
            <el-tag :type="publishTag(scope.row.isPublish).type">{{ publishTag(scope.row.isPublish).label }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="激活状态" align="center" width="100">
          <template #default="scope">
            <el-tag :type="scope.row.activityStatus === 1 ? 'success' : 'info'">{{ scope.row.activityStatus === 1 ? '激活' : '挂起' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="表单地址" align="center" prop="formPath" show-overflow-tooltip />
        <el-table-column label="创建时间" align="center" prop="createTime" width="160" />
        <el-table-column label="操作" fixed="right" align="center" width="230" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-tooltip content="设计" placement="top">
              <el-button link type="primary" icon="EditPen" @click="handleDesign(scope.row)" v-hasPermi="['workflow:definition:edit']" />
            </el-tooltip>
            <el-tooltip v-if="scope.row.isPublish !== 1" content="发布" placement="top">
              <el-button link type="primary" icon="Promotion" @click="handlePublish(scope.row)" v-hasPermi="['workflow:definition:publish']" />
            </el-tooltip>
            <el-tooltip v-else content="取消发布" placement="top">
              <el-button link type="warning" icon="RefreshLeft" @click="handleUnPublish(scope.row)" />
            </el-tooltip>
            <el-tooltip v-if="scope.row.isPublish === 1" :content="scope.row.activityStatus === 1 ? '挂起' : '激活'" placement="top">
              <el-button link type="primary" :icon="scope.row.activityStatus === 1 ? 'VideoPause' : 'VideoPlay'" @click="handleActive(scope.row)" />
            </el-tooltip>
            <el-tooltip content="复制新版本" placement="top">
              <el-button link type="primary" icon="CopyDocument" @click="handleCopy(scope.row)" v-hasPermi="['workflow:definition:add']" />
            </el-tooltip>
            <el-tooltip content="导出 JSON" placement="top">
              <el-button link type="primary" icon="Download" @click="handleExport(scope.row)" />
            </el-tooltip>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 导入流程 JSON -->
    <el-dialog v-model="importVisible" title="导入流程定义 JSON" width="640px" append-to-body>
      <el-alert type="info" :closable="false" show-icon class="mb8" title="导入会生成一条新的流程定义（自动递增版本），不会覆盖原流程" />
      <el-input v-model="importText" type="textarea" :rows="16" placeholder="粘贴 warm-flow / 本系统设计器导出的流程 JSON" />
      <template #footer>
        <el-button @click="importVisible = false">取消</el-button>
        <el-button type="primary" :loading="importing" @click="submitImport">确定导入</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="FlowDefinition" lang="ts">
import {
  activeDefinition,
  copyDefinition,
  delDefinition,
  exportDefinition,
  importDefinition,
  listDefinition,
  publishDefinition,
  unActiveDefinition,
  unPublishDefinition
} from '@/api/workflow/definition';
import { FlowDefinitionQuery, FlowDefinitionVO } from '@/api/workflow/definition/types';
import { categoryTree } from '@/api/workflow/category';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;
const router = useRouter();

const definitionList = ref<FlowDefinitionVO[]>([]);
const categoryOptions = ref<any[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref<Array<string | number>>([]);
const multiple = ref(true);
const total = ref(0);

const queryFormRef = ref<ElFormInstance>();

const queryParams = ref<FlowDefinitionQuery>({
  pageNum: 1,
  pageSize: 10,
  flowName: undefined,
  flowCode: undefined,
  // el-tree-select 不接受 undefined 作为 value，空值统一用空串（后端 blank_to_none 会当没传）
  category: '',
  isPublish: undefined
});

const publishTag = (isPublish: number) => {
  if (isPublish === 1) return { label: '已发布', type: 'success' };
  if (isPublish === 9) return { label: '已失效', type: 'danger' };
  return { label: '未发布', type: 'info' };
};

/** 查询流程定义列表 */
const getList = async () => {
  loading.value = true;
  const res = await listDefinition(queryParams.value);
  definitionList.value = res.rows ?? [];
  total.value = res.total ?? 0;
  loading.value = false;
};

/** 分类树下拉 */
const getCategoryTree = async () => {
  const res = await categoryTree();
  categoryOptions.value = res.data ?? [];
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
const handleSelectionChange = (selection: FlowDefinitionVO[]) => {
  ids.value = selection.map((item) => item.id);
  multiple.value = !selection.length;
};

/** 新建 / 修改流程：跳转设计器 */
const handleDesign = (row?: FlowDefinitionVO) => {
  router.push(row?.id ? `/workflow/design/${row.id}` : '/workflow/design');
};

/** 发布 */
const handlePublish = async (row: FlowDefinitionVO) => {
  await proxy?.$modal.confirm(`是否确认发布流程「${row.flowName}」？`);
  await publishDefinition(row.id);
  proxy?.$modal.msgSuccess('发布成功');
  await getList();
};

/** 取消发布 */
const handleUnPublish = async (row: FlowDefinitionVO) => {
  await proxy?.$modal.confirm(`是否确认取消发布流程「${row.flowName}」？`);
  await unPublishDefinition(row.id);
  proxy?.$modal.msgSuccess('已取消发布');
  await getList();
};

/** 激活 / 挂起 */
const handleActive = async (row: FlowDefinitionVO) => {
  if (row.activityStatus === 1) {
    await unActiveDefinition(row.id);
    proxy?.$modal.msgSuccess('已挂起');
  } else {
    await activeDefinition(row.id);
    proxy?.$modal.msgSuccess('已激活');
  }
  await getList();
};

/** 复制为新版本 */
const handleCopy = async (row: FlowDefinitionVO) => {
  await proxy?.$modal.confirm(`是否基于「${row.flowName}」${row.version} 复制一个新版本？`);
  await copyDefinition(row.id);
  proxy?.$modal.msgSuccess('复制成功');
  await getList();
};

/** 删除 */
const handleDelete = async (row?: FlowDefinitionVO) => {
  const delIds = row?.id ? [row.id] : ids.value;
  await proxy?.$modal.confirm(`是否确认删除选中的 ${delIds.length} 条流程定义？`);
  await delDefinition(delIds.join(','));
  proxy?.$modal.msgSuccess('删除成功');
  await getList();
};

/** 导出 JSON */
const handleExport = async (row: FlowDefinitionVO) => {
  const res = await exportDefinition(row.id);
  const blob = new Blob([JSON.stringify(res.data, null, 2)], { type: 'application/json' });
  const a = document.createElement('a');
  a.href = URL.createObjectURL(blob);
  a.download = `${row.flowCode}-${row.version}.json`;
  a.click();
  URL.revokeObjectURL(a.href);
};

/** 导入 JSON */
const importVisible = ref(false);
const importing = ref(false);
const importText = ref('');

const handleImport = () => {
  importText.value = '';
  importVisible.value = true;
};

const submitImport = async () => {
  if (!importText.value.trim()) {
    proxy?.$modal.msgError('请粘贴流程 JSON');
    return;
  }
  importing.value = true;
  try {
    await importDefinition(importText.value);
    proxy?.$modal.msgSuccess('导入成功');
    importVisible.value = false;
    await getList();
  } finally {
    importing.value = false;
  }
};

onMounted(() => {
  getCategoryTree();
  getList();
});
</script>
