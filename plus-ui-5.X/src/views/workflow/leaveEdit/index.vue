<template>
  <div class="leave-edit">
    <el-descriptions v-loading="loading" :column="2" border>
      <el-descriptions-item label="申请编号">{{ detail.applyCode ?? '-' }}</el-descriptions-item>
      <el-descriptions-item label="单据状态">
        <dict-tag :options="wf_business_status" :value="detail.status" />
      </el-descriptions-item>
      <el-descriptions-item label="请假类型">
        <dict-tag :options="leave_type" :value="detail.leaveType" />
      </el-descriptions-item>
      <el-descriptions-item label="请假天数">{{ detail.leaveDays ?? '-' }} 天</el-descriptions-item>
      <el-descriptions-item label="开始时间">{{ detail.startDate ?? '-' }}</el-descriptions-item>
      <el-descriptions-item label="结束时间">{{ detail.endDate ?? '-' }}</el-descriptions-item>
      <el-descriptions-item label="申请事由" :span="2">{{ detail.remark ?? '-' }}</el-descriptions-item>
    </el-descriptions>
  </div>
</template>

<script setup name="LeaveEdit" lang="ts">
import { leaveByBusinessId } from '@/api/workflow/leave';
import { LeaveVO } from '@/api/workflow/leave/types';

const props = defineProps<{ businessId?: string | number }>();

const { proxy } = getCurrentInstance() as ComponentInternalInstance;
const { leave_type, wf_business_status } = toRefs<any>(proxy?.useDict('leave_type', 'wf_business_status'));

const loading = ref(false);
const detail = ref<LeaveVO>({} as LeaveVO);

const getDetail = async () => {
  if (!props.businessId) {
    detail.value = {} as LeaveVO;
    return;
  }
  loading.value = true;
  try {
    const res = await leaveByBusinessId(props.businessId);
    detail.value = res.data ?? ({} as LeaveVO);
  } finally {
    loading.value = false;
  }
};

watch(() => props.businessId, getDetail, { immediate: true });
</script>
