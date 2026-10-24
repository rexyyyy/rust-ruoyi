<template>
  <div class="p-2">
    <transition :enter-active-class="proxy?.animate.searchAnimate.enter" :leave-active-class="proxy?.animate.searchAnimate.leave">
      <div v-show="showSearch" class="mb-[10px]">
        <el-card shadow="hover">
          <el-form ref="queryFormRef" :model="queryParams" :inline="true">
            <el-form-item label="用户名" prop="memberUsername">
              <el-input v-model="queryParams.memberUsername" placeholder="请输入用户名" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="支付方式：0->未支付；1->支付宝；2->微信" prop="payType">
              <el-input v-model="queryParams.payType" placeholder="请输入支付方式：0->未支付；1->支付宝；2->微信" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="订单来源：0->PC订单；1->app订单" prop="sourceType">
              <el-input v-model="queryParams.sourceType" placeholder="请输入订单来源：0->PC订单；1->app订单" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单" prop="status">
              <el-input v-model="queryParams.status" placeholder="请输入订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="订单类型：0->正常订单；1->秒杀订单" prop="orderType">
              <el-input v-model="queryParams.orderType" placeholder="请输入订单类型：0->正常订单；1->秒杀订单" clearable @keyup.enter="handleQuery" />
            </el-form-item>
            <el-form-item label="收货人姓名" prop="receiverName">
              <el-input v-model="queryParams.receiverName" placeholder="请输入收货人姓名" clearable @keyup.enter="handleQuery" />
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
            <el-button v-hasPermi="['oms:regorder:add']" type="primary" plain icon="Plus" @click="handleAdd">新增</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button v-hasPermi="['oms:regorder:edit']" type="success" plain icon="Edit" :disabled="single" @click="handleUpdate()">修改</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button v-hasPermi="['oms:regorder:remove']" type="danger" plain icon="Delete" :disabled="multiple" @click="handleDelete()">删除</el-button>
          </el-col>
          <el-col :span="1.5">
            <el-button v-hasPermi="['oms:regorder:export']" type="warning" plain icon="Download" @click="handleExport">导出</el-button>
          </el-col>
          <right-toolbar v-model:show-search="showSearch" @query-table="getList"></right-toolbar>
        </el-row>
      </template>

      <el-table v-loading="loading" border :data="regorderList" @selection-change="handleSelectionChange">
        <el-table-column type="selection" width="55" align="center" />
        <el-table-column label="主键" align="center" prop="regid" show-overflow-tooltip />
        <el-table-column label="订单号" align="center" prop="orderSn" show-overflow-tooltip />
        <el-table-column label="用户名" align="center" prop="memberUsername" show-overflow-tooltip />
        <el-table-column label="订单总金额" align="center" prop="totalAmount" show-overflow-tooltip />
        <el-table-column label="应付金额" align="center" prop="payAmount" show-overflow-tooltip />
        <el-table-column label="支付方式：0->未支付；1->支付宝；2->微信" align="center" prop="payType" show-overflow-tooltip />
        <el-table-column label="订单来源：0->PC订单；1->app订单" align="center" prop="sourceType" show-overflow-tooltip />
        <el-table-column label="订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单" align="center" prop="status" show-overflow-tooltip />
        <el-table-column label="订单类型：0->正常订单；1->秒杀订单" align="center" prop="orderType" show-overflow-tooltip />
        <el-table-column label="物流公司(配送方式)" align="center" prop="deliveryCompany" show-overflow-tooltip />
        <el-table-column label="申请号" align="center" prop="requestSn" show-overflow-tooltip />
        <el-table-column label="回复号" align="center" prop="deliverySn" show-overflow-tooltip />
        <el-table-column label="收货人姓名" align="center" prop="receiverName" show-overflow-tooltip />
        <el-table-column label="收货人电话" align="center" prop="receiverPhone" show-overflow-tooltip />
        <el-table-column label="省份/直辖市" align="center" prop="receiverProvince" show-overflow-tooltip />
        <el-table-column label="城市" align="center" prop="receiverCity" show-overflow-tooltip />
        <el-table-column label="区" align="center" prop="receiverRegion" show-overflow-tooltip />
        <el-table-column label="订单备注" align="center" prop="note" show-overflow-tooltip />
        <el-table-column label="确认收货状态：0->未确认；1->已确认" align="center" prop="confirmStatus" show-overflow-tooltip />
        <el-table-column label="支付时间" align="center" prop="paymentTime" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.paymentTime)"></span>
          </template>
        </el-table-column>
        <el-table-column label="发货时间" align="center" prop="deliveryTime" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.deliveryTime)"></span>
          </template>
        </el-table-column>
        <el-table-column label="确认收货时间" align="center" prop="receiveTime" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.receiveTime)"></span>
          </template>
        </el-table-column>
        <el-table-column label="评价时间" align="center" prop="commentTime" width="180">
          <template #default="scope">
            <span v-text="proxy?.parseTime(scope.row.commentTime)"></span>
          </template>
        </el-table-column>
        <el-table-column label="删除标志（0代表存在 2代表删除）" align="center" prop="delFlag" show-overflow-tooltip />
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
              <el-button v-hasPermi="['oms:regorder:edit']" link type="primary" icon="Edit" @click="handleUpdate(scope.row)"></el-button>
            </el-tooltip>
            <el-tooltip content="删除" placement="top">
              <el-button v-hasPermi="['oms:regorder:remove']" link type="primary" icon="Delete" @click="handleDelete(scope.row)"></el-button>
            </el-tooltip>
          </template>
        </el-table-column>
      </el-table>

      <pagination v-show="total > 0" v-model:page="queryParams.pageNum" v-model:limit="queryParams.pageSize" :total="total" @pagination="getList" />
    </el-card>

    <!-- 添加或修改注册码登记对话框 -->
    <el-dialog v-model="dialog.visible" :title="dialog.title" width="600px" append-to-body>
      <el-form ref="regorderFormRef" :model="form" :rules="rules" label-width="100px">
        <el-form-item label="主键" prop="regid">
          <el-input-number v-model="form.regid" controls-position="right" :min="0" />
        </el-form-item>
        <el-form-item label="订单号" prop="orderSn">
          <el-input v-model="form.orderSn" placeholder="请输入订单号" />
        </el-form-item>
        <el-form-item label="用户名" prop="memberUsername">
          <el-input v-model="form.memberUsername" placeholder="请输入用户名" />
        </el-form-item>
        <el-form-item label="订单总金额" prop="totalAmount">
          <el-input-number v-model="form.totalAmount" controls-position="right" :min="0" />
        </el-form-item>
        <el-form-item label="应付金额" prop="payAmount">
          <el-input-number v-model="form.payAmount" controls-position="right" :min="0" />
        </el-form-item>
        <el-form-item label="支付方式：0->未支付；1->支付宝；2->微信" prop="payType">
          <el-input v-model="form.payType" placeholder="请输入支付方式：0->未支付；1->支付宝；2->微信" />
        </el-form-item>
        <el-form-item label="订单来源：0->PC订单；1->app订单" prop="sourceType">
          <el-input v-model="form.sourceType" placeholder="请输入订单来源：0->PC订单；1->app订单" />
        </el-form-item>
        <el-form-item label="订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单" prop="status">
          <el-input v-model="form.status" placeholder="请输入订单状态：0->待付款；1->待发货；2->已发货；3->已完成；4->已关闭；5->无效订单" />
        </el-form-item>
        <el-form-item label="订单类型：0->正常订单；1->秒杀订单" prop="orderType">
          <el-input v-model="form.orderType" placeholder="请输入订单类型：0->正常订单；1->秒杀订单" />
        </el-form-item>
        <el-form-item label="物流公司(配送方式)" prop="deliveryCompany">
          <el-input v-model="form.deliveryCompany" placeholder="请输入物流公司(配送方式)" />
        </el-form-item>
        <el-form-item label="申请号" prop="requestSn">
          <el-input v-model="form.requestSn" placeholder="请输入申请号" />
        </el-form-item>
        <el-form-item label="回复号" prop="deliverySn">
          <el-input v-model="form.deliverySn" placeholder="请输入回复号" />
        </el-form-item>
        <el-form-item label="收货人姓名" prop="receiverName">
          <el-input v-model="form.receiverName" placeholder="请输入收货人姓名" />
        </el-form-item>
        <el-form-item label="收货人电话" prop="receiverPhone">
          <el-input v-model="form.receiverPhone" placeholder="请输入收货人电话" />
        </el-form-item>
        <el-form-item label="省份/直辖市" prop="receiverProvince">
          <el-input v-model="form.receiverProvince" placeholder="请输入省份/直辖市" />
        </el-form-item>
        <el-form-item label="城市" prop="receiverCity">
          <el-input v-model="form.receiverCity" placeholder="请输入城市" />
        </el-form-item>
        <el-form-item label="区" prop="receiverRegion">
          <el-input v-model="form.receiverRegion" placeholder="请输入区" />
        </el-form-item>
        <el-form-item label="订单备注" prop="note">
          <el-input v-model="form.note" type="textarea" placeholder="请输入订单备注" />
        </el-form-item>
        <el-form-item label="确认收货状态：0->未确认；1->已确认" prop="confirmStatus">
          <el-input v-model="form.confirmStatus" placeholder="请输入确认收货状态：0->未确认；1->已确认" />
        </el-form-item>
        <el-form-item label="支付时间" prop="paymentTime">
          <el-date-picker clearable v-model="form.paymentTime" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="请选择支付时间"></el-date-picker>
        </el-form-item>
        <el-form-item label="发货时间" prop="deliveryTime">
          <el-date-picker clearable v-model="form.deliveryTime" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="请选择发货时间"></el-date-picker>
        </el-form-item>
        <el-form-item label="确认收货时间" prop="receiveTime">
          <el-date-picker clearable v-model="form.receiveTime" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="请选择确认收货时间"></el-date-picker>
        </el-form-item>
        <el-form-item label="评价时间" prop="commentTime">
          <el-date-picker clearable v-model="form.commentTime" type="datetime" value-format="YYYY-MM-DD HH:mm:ss" placeholder="请选择评价时间"></el-date-picker>
        </el-form-item>
        <el-form-item label="删除标志（0代表存在 2代表删除）" prop="delFlag">
          <el-input v-model="form.delFlag" placeholder="请输入删除标志（0代表存在 2代表删除）" />
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

<script setup name="OmsRegorder" lang="ts">
import { listOmsRegorder, addOmsRegorder, delOmsRegorder, getOmsRegorder, updateOmsRegorder } from '@/api/oms/regorder';
import { OmsRegorderForm, OmsRegorderQuery, OmsRegorderVO } from '@/api/oms/regorder/types';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;

const regorderList = ref<OmsRegorderVO[]>([]);
const loading = ref(true);
const showSearch = ref(true);
const ids = ref<Array<number | string>>([]);
const single = ref(true);
const multiple = ref(true);
const total = ref(0);
const regorderFormRef = ref<ElFormInstance>();
const queryFormRef = ref<ElFormInstance>();

const dialog = reactive<DialogOption>({
  visible: false,
  title: ''
});

const initFormData: OmsRegorderForm = {
  regid: undefined,
  orderSn: '',
  memberUsername: '',
  totalAmount: 0,
  payAmount: 0,
  payType: '',
  sourceType: '',
  status: '',
  orderType: '',
  deliveryCompany: '',
  requestSn: '',
  deliverySn: '',
  receiverName: '',
  receiverPhone: '',
  receiverProvince: '',
  receiverCity: '',
  receiverRegion: '',
  note: '',
  confirmStatus: '',
  paymentTime: '',
  deliveryTime: '',
  receiveTime: '',
  commentTime: '',
  delFlag: '',
};

const data = reactive<PageData<OmsRegorderForm, OmsRegorderQuery>>({
  form: { ...initFormData },
  queryParams: {
    pageNum: 1,
    pageSize: 10,
    memberUsername: '',
    payType: '',
    sourceType: '',
    status: '',
    orderType: '',
    receiverName: '',
},
  rules: {
    receiverName: [{ required: true, message: '收货人姓名不能为空', trigger: 'blur' }],
    receiverPhone: [{ required: true, message: '收货人电话不能为空', trigger: 'blur' }],
  }
});

const { queryParams, form, rules } = toRefs<PageData<OmsRegorderForm, OmsRegorderQuery>>(data);

/** 查询注册码登记列表 */
const getList = async () => {
  loading.value = true;
  const res = await listOmsRegorder(queryParams.value);
  regorderList.value = res.rows;
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
  regorderFormRef.value?.resetFields();
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
const handleSelectionChange = (selection: OmsRegorderVO[]) => {
  ids.value = selection.map((item) => item.regid);
  single.value = selection.length != 1;
  multiple.value = !selection.length;
};

/** 新增按钮操作 */
const handleAdd = () => {
  reset();
  dialog.visible = true;
  dialog.title = '添加注册码登记';
};

/** 修改按钮操作 */
const handleUpdate = async (row?: OmsRegorderVO) => {
  reset();
  const _regid = row?.regid || ids.value[0];
  const res = await getOmsRegorder(_regid);
  Object.assign(form.value, res.data);
  dialog.visible = true;
  dialog.title = '修改注册码登记';
};

/** 提交按钮 */
const submitForm = () => {
  regorderFormRef.value?.validate(async (valid: boolean) => {
    if (valid) {
      form.value.regid ? await updateOmsRegorder(form.value) : await addOmsRegorder(form.value);
      proxy?.$modal.msgSuccess('操作成功');
      dialog.visible = false;
      await getList();
    }
  });
};

/** 删除按钮操作 */
const handleDelete = async (row?: OmsRegorderVO) => {
  const _regids = row?.regid || ids.value;
  await proxy?.$modal.confirm('是否确认删除注册码登记编号为"' + _regids + '"的数据项？');
  await delOmsRegorder(_regids);
  await getList();
  proxy?.$modal.msgSuccess('删除成功');
};

/** 导出按钮操作 */
const handleExport = () => {
  proxy?.download(
    'oms/regorder/export',
    {
      ...queryParams.value
    },
    `regorder_${new Date().getTime()}.xlsx`
  );
};

onMounted(() => {
  getList();
});
</script>
