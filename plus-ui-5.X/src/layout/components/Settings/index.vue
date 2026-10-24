<template>
  <el-drawer v-model="showSettings" :with-header="false" direction="rtl" size="300px" close-on-click-modal>
    <h3 class="drawer-title">菜单导航设置</h3>
    <div class="nav-wrap">
      <el-tooltip content="左侧菜单" placement="bottom">
        <div
          class="item left"
          @click="handleNavType(NavTypeEnum.LEFT)"
          :style="{ '--theme': theme }"
          :class="{ activeItem: navType == NavTypeEnum.LEFT }"
        >
          <b></b><b></b>
        </div>
      </el-tooltip>

      <el-tooltip content="混合菜单" placement="bottom">
        <div
          class="item mix"
          @click="handleNavType(NavTypeEnum.MIX)"
          :style="{ '--theme': theme }"
          :class="{ activeItem: navType == NavTypeEnum.MIX }"
        >
          <b></b><b></b>
        </div>
      </el-tooltip>
      <el-tooltip content="顶部菜单" placement="bottom">
        <div
          class="item top"
          @click="handleNavType(NavTypeEnum.TOP)"
          :style="{ '--theme': theme }"
          :class="{ activeItem: navType == NavTypeEnum.TOP }"
        >
          <b></b><b></b>
        </div>
      </el-tooltip>
    </div>

    <template v-if="isDefaultTheme">
    <h3 class="drawer-title">主题风格设置</h3>

    <div class="setting-drawer-block-checbox">
      <div class="setting-drawer-block-checbox-item" @click="handleTheme(SideThemeEnum.DARK)">
        <img src="@/assets/images/dark.svg" alt="dark" />
        <div v-if="sideTheme === 'theme-dark'" class="setting-drawer-block-checbox-selectIcon" style="display: block">
          <i aria-label="图标: check" class="anticon anticon-check">
            <svg viewBox="64 64 896 896" data-icon="check" width="1em" height="1em" :fill="theme" aria-hidden="true" focusable="false" class>
              <path
                d="M912 190h-69.9c-9.8 0-19.1 4.5-25.1 12.2L404.7 724.5 207 474a32 32 0 0 0-25.1-12.2H112c-6.7 0-10.4 7.7-6.3 12.9l273.9 347c12.8 16.2 37.4 16.2 50.3 0l488.4-618.9c4.1-5.1.4-12.8-6.3-12.8z"
              />
            </svg>
          </i>
        </div>
      </div>
      <div class="setting-drawer-block-checbox-item" @click="handleTheme(SideThemeEnum.LIGHT)">
        <img src="@/assets/images/light.svg" alt="light" />
        <div v-if="sideTheme === 'theme-light'" class="setting-drawer-block-checbox-selectIcon" style="display: block">
          <i aria-label="图标: check" class="anticon anticon-check">
            <svg viewBox="64 64 896 896" data-icon="check" width="1em" height="1em" :fill="theme" aria-hidden="true" focusable="false" class>
              <path
                d="M912 190h-69.9c-9.8 0-19.1 4.5-25.1 12.2L404.7 724.5 207 474a32 32 0 0 0-25.1-12.2H112c-6.7 0-10.4 7.7-6.3 12.9l273.9 347c12.8 16.2 37.4 16.2 50.3 0l488.4-618.9c4.1-5.1.4-12.8-6.3-12.8z"
              />
            </svg>
          </i>
        </div>
      </div>
    </div>
    </template>
    <div class="drawer-item">
      <span>主题颜色</span>
      <span class="comp-style">
        <el-select v-model="theme" placeholder="选择主题色" style="width: 120px" @change="themeChange">
          <el-option v-for="item in themeOptions" :key="item.value" :label="item.label" :value="item.value">
            <div style="display: flex; align-items: center; gap: 8px">
              <span
                :style="{
                  display: 'inline-block',
                  width: '16px',
                  height: '16px',
                  borderRadius: '3px',
                  backgroundColor: item.value
                }"
              />
              <span>{{ item.label }}</span>
            </div>
          </el-option>
        </el-select>
      </span>
    </div>
    <div class="drawer-item">
      <span>深色模式</span>
      <span class="comp-style">
        <!-- 只用 @change + toggle：再叠加 v-model 会翻转两次导致点不动 -->
        <el-switch :model-value="isDark" class="drawer-switch" @change="toggleDark" />
      </span>
    </div>
    <div class="drawer-item">
      <span>页面圆角</span>
      <span class="comp-style">
        <el-slider v-model="radiusBase" :min="0" :max="32" :step="2" style="width: 120px" @change="radiusBaseChange" />
      </span>
    </div>

    <el-divider />

    <h3 class="drawer-title">系统布局配置</h3>

    <div class="drawer-item">
      <span>开启 Tags-Views</span>
      <span class="comp-style">
        <el-switch v-model="settingsStore.tagsView" class="drawer-switch" />
      </span>
    </div>

    <div class="drawer-item">
      <span>显示页签图标</span>
      <span class="comp-style">
        <el-switch v-model="settingsStore.tagsIcon" :disabled="!settingsStore.tagsView" class="drawer-switch" />
      </span>
    </div>

    <div class="drawer-item">
      <span>固定 Header</span>
      <span class="comp-style">
        <el-switch v-model="settingsStore.fixedHeader" class="drawer-switch" />
      </span>
    </div>

    <div class="drawer-item">
      <span>显示 Logo</span>
      <span class="comp-style">
        <el-switch v-model="settingsStore.sidebarLogo" class="drawer-switch" />
      </span>
    </div>

    <div class="drawer-item">
      <span>动态标题</span>
      <span class="comp-style">
        <el-switch v-model="settingsStore.dynamicTitle" class="drawer-switch" @change="dynamicTitleChange" />
      </span>
    </div>

    <el-divider />

    <el-button type="primary" plain icon="DocumentAdd" @click="saveSetting">保存配置</el-button>
    <el-button plain icon="Refresh" @click="resetSetting">重置配置</el-button>
  </el-drawer>
</template>

<script setup lang="ts">
import { useDynamicTitle } from '@/utils/dynamicTitle';
import { useAppStore } from '@/store/modules/app';
import { useSettingsStore } from '@/store/modules/settings';
import { usePermissionStore } from '@/store/modules/permission';
import { handleThemeStyle, applyRadiusBase } from '@/utils/theme';
import { SideThemeEnum } from '@/enums/SideThemeEnum';
import { NavTypeEnum } from '@/enums/NavTypeEnum';

const { proxy } = getCurrentInstance() as ComponentInternalInstance;
const appStore = useAppStore();
const settingsStore = useSettingsStore();
const permissionStore = usePermissionStore();

const showSettings = ref(false);
const theme = ref(settingsStore.theme);
const sideTheme = ref(settingsStore.sideTheme);
const storeSettings = computed(() => settingsStore);
// 四套预设主题色（与 utils/theme.ts 的 sidebarPresets 一一对应）
const themeOptions = ref([
  { label: '默认', value: '#409EFF' },
  { label: '绿色', value: '#009688' },
  { label: '黑色', value: '#1b2a47' },
  { label: '亮色', value: '#646cff' }
]);
const navType = ref(settingsStore.navType);
const radiusBase = ref(settingsStore.radiusBase);
// 是否暗黑模式
const isDark = useDark({
  storageKey: 'useDarkKey',
  // 未设置过时不跟随系统（auto），默认亮色
  initialValue: 'light',
  valueDark: 'dark',
  valueLight: 'light'
});
// 匹配菜单颜色
watch(isDark, () => {
  if (isDark.value) {
    settingsStore.sideTheme = SideThemeEnum.DARK;
  } else {
    settingsStore.sideTheme = sideTheme.value;
  }
  // 暗黑模式交回 CSS 默认菜单配色，亮色模式按主题色恢复预设
  handleThemeStyle(theme.value);
});
const toggleDark = () => useToggle(isDark);

/** 菜单导航设置 */
watch(
  () => navType,
  (val: string) => {
    if (val.value === NavTypeEnum.TOP) {
      appStore.toggleSideBarHide(true);
      permissionStore.setSidebarRouters(permissionStore.defaultRoutes as any);
    } else if (val.value === NavTypeEnum.LEFT) {
      appStore.toggleSideBarHide(false);
      permissionStore.setSidebarRouters(permissionStore.defaultRoutes as any);
    } else if (val.value === NavTypeEnum.MIX) {
      appStore.toggleSideBarHide(false);
    }
  },
  { immediate: true, deep: true }
);

const handleNavType = (val: NavTypeEnum) => {
  settingsStore.navType = val;
  navType.value = val;
};

const dynamicTitleChange = () => {
  // 动态设置网页标题
  useDynamicTitle();
};

// 是否为默认主题色（亮色主题自带白底菜单，不允许再手动切换侧边栏风格）
const isDefaultTheme = computed(() => theme.value !== '#646cff');

const themeChange = (val: string) => {
  settingsStore.theme = val;
  handleThemeStyle(val);
  // 根据主题色自动设定侧边栏风格
  if (val === '#646cff') {
    settingsStore.sideTheme = SideThemeEnum.LIGHT;
    sideTheme.value = SideThemeEnum.LIGHT;
  } else {
    settingsStore.sideTheme = SideThemeEnum.DARK;
    sideTheme.value = SideThemeEnum.DARK;
  }
};
const radiusBaseChange = (val: number) => {
  settingsStore.radiusBase = val;
  applyRadiusBase(val);
};
const handleTheme = (val: string) => {
  sideTheme.value = val;
  if (isDark.value && val === SideThemeEnum.LIGHT) {
    // 暗黑模式颜色不变
    settingsStore.sideTheme = SideThemeEnum.DARK;
    return;
  }
  settingsStore.sideTheme = val;
};
const saveSetting = () => {
  proxy?.$modal.loading('正在保存到本地，请稍候...');
  // 一次性赋值整个对象，触发 useStorage 响应式写入 localStorage
  // 逐个属性赋值不会触发 shallowRef 的响应式更新，导致配置没能真正落盘
  storeSettings.value.storageSetting = {
    tagsView: storeSettings.value.tagsView,
    tagsIcon: storeSettings.value.tagsIcon,
    fixedHeader: storeSettings.value.fixedHeader,
    sidebarLogo: storeSettings.value.sidebarLogo,
    dynamicTitle: storeSettings.value.dynamicTitle,
    sideTheme: storeSettings.value.sideTheme,
    theme: storeSettings.value.theme,
    navType: storeSettings.value.navType,
    radiusBase: storeSettings.value.radiusBase
  };
  setTimeout(() => {
    proxy?.$modal.closeLoading();
  }, 1000);
};
const resetSetting = () => {
  proxy?.$modal.loading('正在清除设置缓存并刷新，请稍候...');
  useStorage<any>('layout-setting', null).value = null;
  // 同时把亮暗模式恢复为默认亮色
  try {
    localStorage.setItem('useDarkKey', 'light');
    document.documentElement.classList.remove('dark');
  } catch (e) {
    /* ignore */
  }
  setTimeout('window.location.reload()', 1000);
};
const openSetting = () => {
  showSettings.value = true;
};

onMounted(() => {
  radiusBaseChange(storeSettings.value.radiusBase);
});

defineExpose({
  openSetting
});
</script>

<style lang="scss" scoped>
.setting-drawer-title {
  margin-bottom: 12px;
  color: rgba(0, 0, 0, 0.85);
  line-height: 22px;
  font-weight: bold;
  .drawer-title {
    font-size: 14px;
  }
}
.setting-drawer-block-checbox {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  margin-top: 10px;
  margin-bottom: 20px;

  .setting-drawer-block-checbox-item {
    position: relative;
    margin-right: 16px;
    border-radius: 2px;
    cursor: pointer;

    img {
      width: 48px;
      height: 48px;
    }

    .custom-img {
      width: 48px;
      height: 38px;
      border-radius: 5px;
      box-shadow: 1px 1px 2px #898484;
    }

    .setting-drawer-block-checbox-selectIcon {
      position: absolute;
      top: 0;
      right: 0;
      width: 100%;
      height: 100%;
      padding-top: 15px;
      padding-left: 24px;
      color: #1890ff;
      font-weight: 700;
      font-size: 14px;
    }
  }
}

.drawer-item {
  padding: 12px 0;
  font-size: 14px;

  .comp-style {
    float: right;
    margin: -3px 8px 0px 0px;
  }
}

// 导航模式
.nav-wrap {
  display: flex;
  justify-content: flex-start;
  align-items: center;
  margin-top: 10px;
  margin-bottom: 20px;

  .activeItem {
    border: 2px solid #{'var(--theme)'} !important;
  }

  .item {
    position: relative;
    margin-right: 16px;
    cursor: pointer;
    width: 56px;
    height: 48px;
    border-radius: 4px;
    background: #f0f2f5;
    border: 2px solid transparent;
  }

  .left {
    b:first-child {
      display: block;
      height: 30%;
      background: #fff;
    }
    b:last-child {
      width: 30%;
      background: #1b2a47;
      position: absolute;
      height: 100%;
      top: 0;
      border-radius: 4px 0 0 4px;
    }
  }
  .mix {
    b:first-child {
      border-radius: 4px 4px 0 0;
      display: block;
      height: 30%;
      background: #1b2a47;
    }
    b:last-child {
      width: 30%;
      background: #1b2a47;
      position: absolute;
      height: 70%;
      border-radius: 0 0 0 4px;
    }
  }
  .top {
    b:first-child {
      display: block;
      height: 30%;
      background: #1b2a47;
      border-radius: 4px 4px 0 0;
    }
  }
}
</style>
