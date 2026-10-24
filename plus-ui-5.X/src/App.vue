<template>
  <el-config-provider :locale="appStore.locale" :size="appStore.size">
    <router-view />
  </el-config-provider>
</template>

<script setup lang="ts">
import { useSettingsStore } from '@/store/modules/settings';
import { initAppTheme } from '@/utils/theme';
import { useAppStore } from '@/store/modules/app';

const appStore = useAppStore();

onMounted(() => {
  nextTick(() => {
    // 初始化主题：恢复主题色 / 页面圆角，并确保默认亮色模式
    initAppTheme(useSettingsStore().theme, useSettingsStore().radiusBase);
  });
});
</script>
