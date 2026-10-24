<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="mb-[10px]">
        <el-card shadow="hover">
          <el-form ref="queryFormRef" :model="queryParams" :inline="true">
            <el-form-item label="课程编码" prop="courseCode">
              <el-input v-model="queryParams.courseCode" placeholder="请输入课程编码" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="课程名称" prop="courseName">
              <el-input v-model="queryParams.courseName" placeholder="请输入课程名称" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" icon="Search" @click="handleQuery">搜索</el-button>
              <el-button icon="Refresh" @click="resetQuery">重置</el-button>
            </el-form-item>
          </el-form>
        </el-card>
      </div>
    </transition>

    <el-card shadow="hover">
      <template #header>
        <el-row :gutter="10" class="mb8">
          <el-col :span="1.5">
            <el-button v-hasPermi="['demo:course:add']" type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button v-hasPermi="['demo:course:edit']" type="success" plain icon="Edit" :disabled="single" @click="handleUpdate()">修改</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button v-hasPermi="['demo:course:remove']" type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete()">删除</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button v-hasPermi="['demo:course:export']" type="warning" plain icon="Download" @click="handleExport">导出</el-button>
          </el-col>
          <right-toolbar v-model:show-search="showSearch" @query-table="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="courseList" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="课程编号" align="center" prop="courseId" show-overflow-tooltip />
        <el-table-column label="课程编码" align="center" prop="courseCode" show-overflow-tooltip />
        <el-table-column label="课程名称" align="center" prop="courseName" show-overflow-tooltip />
        <el-table-column label="授课教师" align="center" prop="teacher" show-overflow-tooltip />
        <el-table-column label="学分" align="center" prop="credit" show-overflow-tooltip />
        <el-table-column label="开课状态" align="center" prop="openStatus" show-overflow-tooltip />
        <el-table-column label="开课日期" align="center" prop="startDate" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.startDate)"></span>
          </template>
        </el-table-column>
        <el-table-column label="备注" align="center" prop="remark" show-overflow-tooltip />
        <el-table-column label="创建者" align="center" prop="createBy" show-overflow-tooltip />
        <el-table-column label="创建时间" align="center" prop="createTime" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.createTime)"></span>
          </template>
        </el-table-column>
        <el-table-column label="更新者" align="center" prop="updateBy" show-overflow-tooltip />
        <el-table-column label="更新时间" align="center" prop="updateTime" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.updateTime)"></span>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="180" align="center" class-name="small-padding fixed-width">
          <template #default="scope">
            <el-tooltip content="修改" placement="top">
              <el-button v-hasPermi="['demo:course:edit']" link type="primary" icon="Edit" @click="handleUpdate(scope.row)"></el-button>
            </el-tooltip>
            <el-tooltip content="删除" placement="top">
              <el-button v-hasPermi="['demo:course:remove']" link type="primary" icon="Delete" @click="handleDelete(scope.row)"></el-button>
            </el-tooltip>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 添加或修改课程对话框 -->
    <el-dialog v-model="dialog.visible" :title="dialog.title" width="600px" append-to-body>
      <el-form ref="courseFormRef" :model="form" :rules="rules" label-width="100px">
        <el-form-item label="课程编号" prop="courseId">
          <el-input-number v-model="form.courseId" controls-position="right" :min="0" />
        </el-form-item>
        <el-form-item label="课程编码" prop="courseCode">
          <el-input v-model="form.courseCode" placeholder="请输入课程编码" />
        </el-form-item>
        <el-form-item label="课程名称" prop="courseName">
          <el-input v-model="form.courseName" placeholder="请输入课程名称" />
        </el-form-item>
        <el-form-item label="授课教师" prop="teacher">
          <el-input v-model="form.teacher" placeholder="请输入授课教师" />
        </el-form-item>
        <el-form-item label="学分" prop="credit">
          <el-input-number v-model="form.credit" controls-position="right" :min="0" />
        </el-form-item>
        <el-form-item label="开课状态" prop="openStatus">
          <el-input v-model="form.openStatus" placeholder="请输入开课状态" />
        </el-form-item>
        <el-form-item label="开课日期" prop="startDate">
          <el-date-picker clearable v-model="form.startDate" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="请选择开课日期"></el-date-picker>
        </el-form-item>
        <el-form-item label="备注" prop="remark">
          <el-input v-model="form.remark" type="textarea" placeholder="请输入备注" />
        </el-form-item>
      </el-form>
      <template #footer>
        <div class="dialog-footer">
          <el-button type="primary" @click="submitForm">确 定</el-button>
          <el-button @click="cancel">取 消</el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="DemoCourse" lang="ts">
import { listDemoCourse, addDemoCourse, delDemoCourse, getDemoCourse, updateDemoCourse } from '@/api/demo/course';
import { DemoCourseForm, DemoCourseQuery, DemoCourseVO } from '@/api/demo/course/types';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;

const courseList = ref<DemoCourseVO[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref<Array<number | string>>([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const courseFormRef = ref<ElFormInstance>();
const queryFormRef = ref<ElFormInstance>();

const dialog = reactive<DialogOption>({
  visible: false,
  title: ''
});

const initFormData: DemoCourseForm = {
  courseId: undefined,
  courseCode: '',
  courseName: '',
  teacher: '',
  credit: 0,
  openStatus: '',
  startDate: '',
  remark: '',
};

const data = reactive<PageData<DemoCourseForm, DemoCourseQuery>>({
  form: { ...initFormData },
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    courseCode: '',
    courseName: '',
},
  rules: {
  }
});

const { queryParams, form, rules } = toRefs<PageData<DemoCourseForm, DemoCourseQuery>>(data);

/** 查询课程列表 */
const getList = async () => {
  loading.value = true;
  const res = await listDemoCourse(queryParams.value);
  courseList.value = res.rows;
  total.value = res.total;
  loading.value = false;
};

/** 取消按钮 */
const cancel = () => {
  reset();
  dialog.visible = false;
};

/** 表单重置 */
const reset = () => {
  form.value = { ...initFormData };
  courseFormRef.value?.resetFields();
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
const handleSelectionChange = (selection: DemoCourseVO[]) => {
  ids.value = selection.map((item) => item.courseId);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
};

/** 新增按钮操作 */
const handleAdd = () => {
  reset();
  dialog.visible = true;
  dialog.title = '添加课程';
};

/** 修改按钮操作 */
const handleUpdate = async (row?: DemoCourseVO) => {
  reset();
  const _courseId = row?.courseId || ids.value[0];
  const res = await getDemoCourse(_courseId);
  Object.assign(form.value, res.data);
  dialog.visible = true;
  dialog.title = '修改课程';
};

/** 提交按钮 */
const submitForm = () => {
  courseFormRef.value?.validate(async (valid: boolean) => {
    if (valid) {
      form.value.courseId ? await updateDemoCourse(form.value) : await addDemoCourse(form.value);
      proxy?.$modal.msgSuccess('操作成功');
      dialog.visible = false;
      await getList();
    }
  });
};

/** 删除按钮操作 */
const handleDelete = async (row?: DemoCourseVO) => {
  const _courseIds = row?.courseId || ids.value;
  await proxy?.$modal.confirm('是否确认删除课程编号为"' + _courseIds + '"的数据项？');
  await delDemoCourse(_courseIds);
  await getList();
  proxy?.$modal.msgSuccess('删除成功');
};

/** 导出按钮操作 */
const handleExport = () => {
  proxy?.download(
    'demo/course/export',
    {
      ...queryParams.value
    },
    `course_${new Date().getTime()}.xlsx`
  );
};

onMounted(() => {
  getList();
});
</script>
