<template>
  <el-tooltip :content="isDark ? '切换为亮色主题' : '切换为暗色主题'" effect="dark" placement="bottom">
    <div class="dark-switch" @click="toggleTheme">
      <el-icon :size="18">
        <Sunny v-if="isDark" />
        <Moon v-else />
      </el-icon>
    </div>
  </el-tooltip>
</template>

<script setup lang="ts">
/**
 * 顶栏亮/暗主题切换按钮
 * 与「布局设置」抽屉共用同一份亮暗状态（localStorage key: useDarkKey）。
 * 默认亮色：未显式设置过时，initAppTheme() 会写入 light。
 */
import { Sunny, Moon } from '@element-plus/icons-vue';
import { useDark, useToggle } from '@vueuse/core';
import { handleThemeStyle } from '@/utils/theme';
import { useSettingsStore } from '@/store/modules/settings';
import { SideThemeEnum } from '@/enums/SideThemeEnum';

const settingsStore = useSettingsStore();
const theme = computed(() => settingsStore.theme);

const isDark = useDark({
  storageKey: 'useDarkKey',
  // 未设置过时不跟随系统（auto），默认亮色
  initialValue: 'light',
  valueDark: 'dark',
  valueLight: 'light'
});
const toggle = useToggle(isDark);

// 亮暗切换时同步菜单配色（暗色交回 CSS 默认，亮色按当前主题色恢复预设）
watch(isDark, () => {
  settingsStore.sideTheme = isDark.value || theme.value !== '#646cff' ? SideThemeEnum.DARK : SideThemeEnum.LIGHT;
  handleThemeStyle(theme.value);
});

const toggleTheme = () => {
  toggle();
};
</script>

<style scoped lang="scss">
.dark-switch {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 32px;
  padding: 0 8px;
  cursor: pointer;
  font-size: 18px;
  color: var(--el-text-color-regular);
  border-radius: var(--app-radius-md);
  transition:
    background 0.2s ease,
    color 0.2s ease;

  &:hover {
    background: var(--el-fill-color-light);
    color: var(--el-color-primary);
  }
}
</style>
