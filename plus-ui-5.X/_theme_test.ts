// 主题逻辑验证：用最小 DOM 桩在 Node 里真跑 handleThemeStyle / initAppTheme
class StyleStub {
  props: Record<string, string> = {};
  setProperty(k: string, v: string) {
    this.props[k] = v;
  }
  removeProperty(k: string) {
    delete this.props[k];
  }
}
const styleStub = new StyleStub();
(globalThis as any).document = {
  documentElement: {
    style: styleStub,
    classList: {
      _c: new Set<string>(),
      toggle(c: string, on: boolean) {
        if (on) this._c.add(c);
        else this._c.delete(c);
      },
      contains(c: string) {
        return this._c.has(c);
      }
    }
  }
};
const ls: Record<string, string> = {};
(globalThis as any).localStorage = {
  getItem: (k: string) => (k in ls ? ls[k] : null),
  setItem: (k: string, v: string) => {
    ls[k] = v;
  },
  removeItem: (k: string) => {
    delete ls[k];
  }
};

import { handleThemeStyle, initAppTheme, isDarkMode } from '@/utils/theme';
import defaultSettings from '@/settings';

const lines: string[] = [];
const ok = (name: string, cond: boolean, extra = '') => {
  lines.push(`${cond ? 'PASS' : 'FAIL'}  ${name}${extra ? ' | ' + extra : ''}`);
  if (!cond) process.exitCode = 1;
};

ok('默认主题为亮色 #646cff', defaultSettings.theme === '#646cff', defaultSettings.theme);
ok('默认侧栏风格 theme-light', defaultSettings.sideTheme === 'theme-light', defaultSettings.sideTheme);

// 1) 亮色主题 -> 白色菜单
initAppTheme('#646cff', 8);
ok('默认亮色模式（非 dark）', isDarkMode() === false);
ok('html 未挂 dark 类', (globalThis as any).document.documentElement.classList.contains('dark') === false);
ok('亮色主题菜单背景白', styleStub.props['--menuBg'] === '#ffffff', styleStub.props['--menuBg']);
ok('亮色主题菜单文字深色', styleStub.props['--menuColor'] === '#333333', styleStub.props['--menuColor']);
ok('主色写入 #646cff', styleStub.props['--el-color-primary'] === '#646cff', styleStub.props['--el-color-primary']);
ok('圆角写入 8px', styleStub.props['--app-radius-base'] === '8px', styleStub.props['--app-radius-base']);

// 2) 切黑色主题 -> 深色菜单
handleThemeStyle('#1b2a47');
ok('黑色主题菜单背景 #1b2a47', styleStub.props['--menuBg'] === '#1b2a47', styleStub.props['--menuBg']);
ok('黑色主题菜单文字白色', styleStub.props['--menuColor'] === '#ffffff', styleStub.props['--menuColor']);

// 3) 切默认蓝 -> 经典深色菜单
handleThemeStyle('#409EFF');
ok('默认蓝菜单背景 #1f2d3d', styleStub.props['--menuBg'] === '#1f2d3d', styleStub.props['--menuBg']);

// 4) 绿色主题
handleThemeStyle('#009688');
ok('绿色主题菜单背景 #1f2d3d', styleStub.props['--menuBg'] === '#1f2d3d', styleStub.props['--menuBg']);

// 5) 自由取色（无预设）回退：亮色 -> 黑字，暗色 -> 白字
handleThemeStyle('#ffd700');
ok('自由亮色回退文字 #333333', styleStub.props['--menuColor'] === '#333333', styleStub.props['--menuColor']);
handleThemeStyle('#101010');
ok('自由暗色回退文字 #ffffff', styleStub.props['--menuColor'] === '#ffffff', styleStub.props['--menuColor']);

// 6) 暗黑模式：菜单变量交回 CSS 默认
localStorage.setItem('useDarkKey', 'dark');
handleThemeStyle('#646cff');
ok('暗黑模式清空 --menuBg', styleStub.props['--menuBg'] === undefined, String(styleStub.props['--menuBg']));
ok('暗黑模式主色仍生效', styleStub.props['--el-color-primary'] === '#646cff');

// 7) 回到亮色：恢复预设
localStorage.setItem('useDarkKey', 'light');
handleThemeStyle('#646cff');
ok('回到亮色恢复白色菜单', styleStub.props['--menuBg'] === '#ffffff', styleStub.props['--menuBg']);

// 8) 未设置过 useDarkKey 时默认亮色
delete ls['useDarkKey'];
initAppTheme();
ok('未设置偏好时默认 light', localStorage.getItem('useDarkKey') === 'light', String(localStorage.getItem('useDarkKey')));

console.log(lines.join('\n'));
