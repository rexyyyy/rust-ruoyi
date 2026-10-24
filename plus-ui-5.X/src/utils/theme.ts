import defaultSettings from '@/settings';

/**
 * 是否处于暗黑模式（不引第三方依赖，便于在任意环境复用）
 */
export const isDarkMode = (): boolean => {
  const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('useDarkKey') : null;
  if (stored === 'dark' || stored === 'light') return stored === 'dark';
  return typeof document !== 'undefined' && document.documentElement.classList.contains('dark');
};

// 计算颜色亮度
const getLuminance = (hex: string): number => {
  const rgb = hexToRgb(hex);
  if (rgb.length < 3) return 0;
  const [r, g, b] = rgb.map((c) => {
    const v = Number(c) / 255;
    return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
  });
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
};

// 处理主题样式
export const handleThemeStyle = (theme: string) => {
  document.documentElement.style.setProperty('--el-color-primary', theme);
  for (let i = 1; i <= 9; i++) {
    document.documentElement.style.setProperty(`--el-color-primary-light-${i}`, `${getLightColor(theme, i / 10)}`);
  }
  for (let i = 1; i <= 9; i++) {
    document.documentElement.style.setProperty(`--el-color-primary-dark-${i}`, `${getDarkColor(theme, i / 10)}`);
  }
  updateSidebarTheme(theme);
};

// 各主题的侧边栏预设颜色
const sidebarPresets: Record<string, Record<string, string>> = {
  '#409EFF': {
    menuBg: '#1f2d3d', menuColor: '#bfcbd9', menuActiveText: '#f4f4f5',
    menuHover: '#263445', subMenuBg: '#1f2d3d', subMenuActiveText: '#f4f4f5',
    subMenuHover: '#001528', subMenuTitleHover: '#293444'
  },
  '#009688': {
    menuBg: '#1f2d3d', menuColor: '#bfcbd9', menuActiveText: '#f4f4f5',
    menuHover: '#263445', subMenuBg: '#1f2d3d', subMenuActiveText: '#f4f4f5',
    subMenuHover: '#001528', subMenuTitleHover: '#293444'
  },
  '#1b2a47': {
    menuBg: '#1b2a47', menuColor: '#ffffff', menuActiveText: '#ffffff',
    menuHover: '#263850', subMenuBg: '#1b2a47', subMenuActiveText: '#ffffff',
    subMenuHover: '#263850', subMenuTitleHover: '#263850'
  },
  '#646cff': {
    menuBg: '#ffffff', menuColor: '#333333', menuActiveText: '#333333',
    menuHover: '#f0f1f5', subMenuBg: '#fafafa', subMenuActiveText: '#333333',
    subMenuHover: '#f0f1f5', subMenuTitleHover: '#f0f1f5'
  }
};

// 菜单变量清单（用于清理内联值，回落到 variables.module.scss 的默认值）
const sidebarVars = [
  'menuBg', 'menuColor', 'menuActiveText', 'menuHover',
  'subMenuBg', 'subMenuActiveText', 'subMenuHover', 'subMenuTitleHover'
];

/**
 * 清除内联侧边栏变量，交还给 CSS 默认值
 * （html.dark 下由 variables.module.scss 提供暗色菜单配色）
 */
export const clearSidebarTheme = () => {
  const root = document.documentElement.style;
  sidebarVars.forEach((key) => root.removeProperty('--' + key));
};

// 根据主题色更新侧边栏CSS变量
const updateSidebarTheme = (bgColor: string) => {
  // 暗黑模式下侧边栏配色由 html.dark 定义，不覆盖
  if (isDarkMode()) {
    clearSidebarTheme();
    return;
  }
  const root = document.documentElement.style;
  const preset = sidebarPresets[bgColor];
  if (preset) {
    Object.entries(preset).forEach(([key, value]) => root.setProperty('--' + key, value));
    return;
  }
  // 无预设时，根据亮度计算（自由取色回退）
  const isDark = getLuminance(bgColor) <= 0.179;
  const textColor = isDark ? '#ffffff' : '#333333';
  const hoverBg = isDark ? getLightColor(bgColor, 0.15) : getDarkColor(bgColor, 0.08);
  const subBg = isDark ? getDarkColor(bgColor, 0.15) : getLightColor(bgColor, 0.05);
  const subHover = isDark ? getLightColor(bgColor, 0.1) : getDarkColor(bgColor, 0.05);
  root.setProperty('--menuBg', bgColor);
  root.setProperty('--menuColor', textColor);
  root.setProperty('--menuActiveText', isDark ? '#ffffff' : '#333333');
  root.setProperty('--menuHover', hoverBg);
  root.setProperty('--subMenuBg', subBg);
  root.setProperty('--subMenuActiveText', isDark ? '#ffffff' : '#333333');
  root.setProperty('--subMenuHover', subHover);
  root.setProperty('--subMenuTitleHover', hoverBg);
};

// 页面圆角
export const applyRadiusBase = (val: number) => {
  const el = document.documentElement;
  el.style.setProperty('--app-radius-base', `${val}px`);
  el.style.setProperty('--app-radius-sm', `${Math.round(val * 0.6)}px`);
  el.style.setProperty('--app-radius-md', `${val}px`);
  el.style.setProperty('--app-radius-lg', `${Math.round(val * 1.4)}px`);
  el.style.setProperty('--el-border-radius-base', `${val}px`);
  el.style.setProperty('--el-border-radius-small', `${Math.round(val * 0.6)}px`);
};

// 默认亮色落位标记：主题功能升级后首次加载时，把历史遗留（跟随系统 auto / 旧值）的暗色状态归位为亮色
const THEME_DEFAULT_FLAG = 'theme-default-light-v1';

/**
 * 亮暗模式默认值归一（本项目默认「亮色」）
 * - 第一次进入本版主题功能：一律重置为亮色（清掉历史遗留的 auto/dark）；
 * - 之后：只有本地显式存过 'dark' 才保持暗色，'auto' / 未设置 / 其它值一律归一为 'light'，
 *   避免 vueuse useDark 读到 'auto' 后跟随系统（Windows 深色）偏好而默认变黑；
 * - 想「每次刷新都强制亮色、完全忽略本地选择」时，把下面 const dark 直接改成 false。
 * 需要在 Vue 挂载前调用一次，保证组件内 useDark(useStorage) 读到的是 light。
 */
export const initDefaultColorMode = (): boolean => {
  try {
    const ls = typeof localStorage !== 'undefined' ? localStorage : undefined;
    if (!ls) return false;
    const migrated = ls.getItem(THEME_DEFAULT_FLAG) === '1';
    const stored = ls.getItem('useDarkKey');
    const dark = migrated ? stored === 'dark' : false;
    ls.setItem(THEME_DEFAULT_FLAG, '1');
    ls.setItem('useDarkKey', dark ? 'dark' : 'light');
    document.documentElement.classList.toggle('dark', dark);
    return dark;
  } catch (e) {
    return false;
  }
};

/**
 * 应用启动时恢复主题配置：
 * - 亮暗模式归一为默认亮色（已显式选过暗色的除外）
 * - 恢复主题色、页面圆角
 */
export const initAppTheme = (theme?: string, radiusBase?: number) => {
  initDefaultColorMode();
  if (typeof radiusBase === 'number') applyRadiusBase(radiusBase);
  handleThemeStyle(theme || defaultSettings.theme);
};

// hex颜色转rgb颜色
export const hexToRgb = (str: string): string[] => {
  str = str.replace('#', '');
  const hexs = str.match(/../g);
  for (let i = 0; i < 3; i++) {
    if (hexs) {
      hexs[i] = String(parseInt(hexs[i], 16));
    }
  }
  return hexs ? hexs : [];
};

// rgb颜色转Hex颜色
export const rgbToHex = (r: string, g: string, b: string) => {
  const hexs = [Number(r).toString(16), Number(g).toString(16), Number(b).toString(16)];
  for (let i = 0; i < 3; i++) {
    if (hexs[i].length == 1) {
      hexs[i] = `0${hexs[i]}`;
    }
  }
  return `#${hexs.join('')}`;
};

// 变浅颜色值
export const getLightColor = (color: string, level: number) => {
  const rgb = hexToRgb(color);
  for (let i = 0; i < 3; i++) {
    const s = (255 - Number(rgb[i])) * level + Number(rgb[i]);
    rgb[i] = String(Math.floor(s));
  }
  return rgbToHex(rgb[0], rgb[1], rgb[2]);
};

// 变深颜色值
export const getDarkColor = (color: string, level: number) => {
  const rgb = hexToRgb(color);
  for (let i = 0; i < 3; i++) {
    rgb[i] = String(Math.floor(Number(rgb[i]) * (1 - level)));
  }
  return rgbToHex(rgb[0], rgb[1], rgb[2]);
};
