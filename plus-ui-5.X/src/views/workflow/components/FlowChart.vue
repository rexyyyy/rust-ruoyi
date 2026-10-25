<template>
  <div class="flow-chart">
    <svg v-if="nodes.length" :viewBox="viewBox" class="fc-svg" xmlns="http://www.w3.org/2000/svg">
      <defs>
        <marker :id="arrowId" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
          <path d="M 0 0 L 10 5 L 0 10 z" :fill="colors[0]" />
        </marker>
      </defs>

      <!-- 连线先画，保证节点压在连线之上 -->
      <g v-for="(link, idx) in links" :key="'s' + idx">
        <path :d="link.d" class="fc-line" :stroke="link.color" :marker-end="`url(#${arrowId})`" />
        <text v-if="link.label" :x="link.labelX" :y="link.labelY" class="fc-skip-label" text-anchor="middle">
          {{ link.label }}
        </text>
      </g>

      <g v-for="node in nodes" :key="node.nodeCode" class="fc-node" :transform="`translate(${node.cx}, ${node.cy})`">
        <!-- 开始 / 结束 -->
        <circle v-if="node.shape === 'circle'" :r="CIRCLE_R" :fill="node.fill" :stroke="node.stroke" stroke-width="1.5" />
        <!-- 网关 -->
        <polygon
          v-else-if="node.shape === 'diamond'"
          :points="`0,-${GATEWAY_HH} ${GATEWAY_HW},0 0,${GATEWAY_HH} -${GATEWAY_HW},0`"
          :fill="node.fill"
          :stroke="node.stroke"
          stroke-width="1.5"
        />
        <!-- 普通节点 -->
        <rect
          v-else
          :x="-NODE_W / 2"
          :y="-NODE_H / 2"
          :width="NODE_W"
          :height="NODE_H"
          rx="6"
          :fill="node.fill"
          :stroke="node.stroke"
          stroke-width="1.5"
        />
        <text
          v-for="(line, li) in node.lines"
          :key="li"
          :x="0"
          :y="node.textY(li, node.lines.length)"
          class="fc-node-label"
          text-anchor="middle"
        >
          {{ line }}
        </text>
      </g>
    </svg>
    <el-empty v-else description="暂无流程图数据" />
  </div>
</template>

<script setup name="FlowChart" lang="ts">
import { DesignJson, DesignNode } from '@/api/workflow/definition/types';
import { CIRCLE_R, GATEWAY_HH, GATEWAY_HW, NODE_H, NODE_W, elbowPath, nodeMeta, parseNodeCoord, parseSkipCoord } from '../design/flow';

interface Props {
  /** 后端 /workflow/instance/flowChart/{id} 或设计器数据 */
  def?: DesignJson | null;
}

const props = defineProps<Props>();

/** 节点办理状态：0 未办理 1 待办理 2 已办理 */
const DEFAULT_RGB = ['107,114,128', '245,158,11', '56,161,105'];
const uid = Math.random().toString(36).slice(2, 8);
const arrowId = `fc-arrow-${uid}`;

const colors = computed(() => {
  const raw = props.def?.chartStatusColor;
  return raw && raw.length >= 3 ? raw : DEFAULT_RGB;
});

const toRgb = (c: string) => (c.startsWith('rgb') ? c : `rgb(${c})`);

interface DrawnNode {
  nodeCode: string;
  nodeName: string;
  cx: number;
  cy: number;
  shape: 'circle' | 'rect' | 'diamond';
  fill: string;
  stroke: string;
  lines: string[];
  textY: (idx: number, total: number) => number;
}

const nodes = computed<DrawnNode[]>(() => {
  const list = props.def?.nodeList ?? [];
  return list.map((node: DesignNode) => {
    const c = parseNodeCoord(node.coordinate);
    const meta = nodeMeta(node.nodeType);
    // 状态色优先，没有状态时按节点类型上色（与设计器观感一致）
    const status = node.status === undefined || node.status === null ? -1 : Number(node.status);
    const stroke = status >= 0 && status <= 2 ? toRgb(colors.value[status]) : meta.color;
    const fill = status >= 0 && status <= 2 ? mix(stroke) : '#fff';
    const limit = meta.shape === 'rect' ? 6 : 4;
    const lines = wrap(node.nodeName || node.nodeCode, limit);
    return {
      nodeCode: node.nodeCode,
      nodeName: node.nodeName ?? '',
      cx: c.x,
      cy: c.y,
      shape: meta.shape,
      fill,
      stroke,
      lines,
      textY: (idx: number, total: number) => (idx - (total - 1) / 2) * 14
    };
  });
});

interface DrawnSkip {
  d: string;
  color: string;
  label: string;
  labelX: number;
  labelY: number;
}

const links = computed<DrawnSkip[]>(() => {
  const byCode = new Map((props.def?.nodeList ?? []).map((n) => [n.nodeCode, n]));
  const out: DrawnSkip[] = [];
  for (const node of props.def?.nodeList ?? []) {
    for (const skip of node.skipList ?? []) {
      const to = byCode.get(skip.nextNodeCode);
      if (!to) continue;
      const from = { ...node } as DesignNode;
      const coord = parseSkipCoord(skip.coordinate);
      const start = coord?.start ?? parseNodeCoord(from.coordinate);
      const end = coord?.end ?? parseNodeCoord(to.coordinate);
      const status = skip.status === undefined || skip.status === null ? -1 : Number(skip.status);
      const color = status >= 0 && status <= 2 ? toRgb(colors.value[status]) : '#98a2b3';
      out.push({
        d: elbowPath(start, end),
        color,
        label: skip.skipName ?? '',
        labelX: (start.x + end.x) / 2,
        labelY: (start.y + end.y) / 2 - 6
      });
    }
  }
  return out;
});

const viewBox = computed(() => {
  const xs = nodes.value.map((n) => n.cx);
  const ys = nodes.value.map((n) => n.cy);
  if (!xs.length) return '0 0 400 200';
  const pad = 90;
  const minX = Math.min(...xs) - pad;
  const minY = Math.min(...ys) - pad;
  const width = Math.max(...xs) + pad - minX;
  const height = Math.max(...ys) + pad - minY;
  return `${minX} ${minY} ${width} ${height}`;
});

/** 状态色作为描边，填充用同色系浅色 */
const mix = (rgb: string) => {
  const parts = rgb
    .replace('rgb(', '')
    .replace(')', '')
    .split(',')
    .map((v) => Number(v.trim()));
  if (parts.length < 3 || parts.some((v) => !Number.isFinite(v))) return '#f5f7fa';
  const light = parts.map((v) => Math.round(v + (255 - v) * 0.82));
  return `rgb(${light.join(',')})`;
};

const wrap = (text: string, limit: number): string[] => {
  const chars = Array.from(text ?? '');
  if (chars.length <= limit) return chars.length ? [chars.join('')] : [''];
  const lines: string[] = [];
  for (let i = 0; i < chars.length && lines.length < 3; i += limit) {
    lines.push(chars.slice(i, i + limit).join(''));
  }
  return lines;
};

watch(
  () => props.def,
  () => {
    /* 依赖 computed 自动刷新，这里仅保留观察点便于调试 */
  }
);
</script>

<style scoped lang="scss">
.flow-chart {
  width: 100%;
  min-height: 260px;
  overflow: auto;
  background: #fafbfc;
  background-image: radial-gradient(#dcdfe6 1px, transparent 1px);
  background-size: 20px 20px;
  border: 1px solid var(--el-border-color-light);
  border-radius: 6px;

  .fc-svg {
    display: block;
    width: 100%;
    height: 100%;
    min-height: 260px;
  }

  .fc-line {
    fill: none;
    stroke-width: 1.6;
  }

  .fc-skip-label {
    font-size: 11px;
    fill: #606266;
  }

  .fc-node-label {
    font-size: 12px;
    fill: #303133;
    dominant-baseline: middle;
  }

  .fc-node {
    cursor: default;
  }
}
</style>
