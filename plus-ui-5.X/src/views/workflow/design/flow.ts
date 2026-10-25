/**
 * 流程设计器的几何与校验工具。
 *
 * 坐标约定与 warm-flow（Java 版设计器导出格式）保持一致：
 * - 节点 coordinate = "x,y|x,y"，两段都是节点中心点（后段 warm-flow 用于折叠布局，这里原样写回）；
 * - 连线 coordinate = "sx,sy;ex,ey"，分别是起点锚点与终点锚点。
 * 只要遵守这套约定，本设计器保存的流程可以被 warm-flow 官方查看器直接渲染。
 */
import type { DesignJson, DesignNode, DesignSkip } from '@/api/workflow/definition/types';

/** 节点尺寸 */
export const NODE_W = 110;
export const NODE_H = 46;
/** 开始 / 结束节点是圆 */
export const CIRCLE_R = 22;
/** 网关节点菱形的半宽 / 半高 */
export const GATEWAY_HW = 30;
export const GATEWAY_HH = 24;

/** 自动布局的列间距与行间距 */
export const LAYOUT_GAP_X = 180;
export const LAYOUT_GAP_Y = 110;

/** 设计器画布的逻辑尺寸（配合 viewBox，缩放不影响坐标换算） */
export const CANVAS_SIZE = { w: 1600, h: 900 };

export type Point = { x: number; y: number };

/**
 * 节点类型（与 warm-flow `NodeType` 完全一致，勿自创编号）：
 * 0 开始、1 审批（中间）、2 结束、3 互斥网关、4 并行网关、5 包容网关。
 * warm-flow 没有「服务节点」，nodeType=6 之类引擎会当普通节点处理。
 */
export const NodeType = {
  START: 0,
  BETWEEN: 1,
  END: 2,
  MUTEX: 3,
  PARALLEL: 4,
  INCLUSIVE: 5
} as const;

/** 网关类型集合（判定菱形绘制、分支校验用） */
export const GATEWAY_TYPES: number[] = [NodeType.MUTEX, NodeType.PARALLEL, NodeType.INCLUSIVE];

export interface NodeMeta {
  value: number;
  label: string;
  /** 绘图形状 */
  shape: 'circle' | 'rect' | 'diamond';
  color: string;
}

/** 设计器左侧工具栏可用的节点类型 */
export const NODE_METAS: NodeMeta[] = [
  { value: NodeType.START, label: '开始', shape: 'circle', color: '#67c23a' },
  { value: NodeType.BETWEEN, label: '审批节点', shape: 'rect', color: '#409eff' },
  { value: NodeType.MUTEX, label: '互斥网关', shape: 'diamond', color: '#e6a23c' },
  { value: NodeType.PARALLEL, label: '并行网关', shape: 'diamond', color: '#e6a23c' },
  { value: NodeType.INCLUSIVE, label: '包容网关', shape: 'diamond', color: '#e6a23c' },
  { value: NodeType.END, label: '结束', shape: 'circle', color: '#f56c6c' }
];

export const nodeMeta = (nodeType: number): NodeMeta =>
  NODE_METAS.find((m) => m.value === nodeType) ?? { value: nodeType, label: '节点', shape: 'rect', color: '#409eff' };

/** 办理人类型（写入 permissionFlag 前缀） */
export const HandlerPrefix = {
  ROLE: 'role:',
  DEPT: 'dept:',
  USER: 'user:'
} as const;

/** 连线类型 */
export const SkipTypes = [
  { value: 'PASS', label: '通过' },
  { value: 'REJECT', label: '退回' },
  { value: 'NONE', label: '无条件' }
] as const;

/**
 * 条件表达式（warm-flow `ExpressionUtil` 格式）：`操作符@@变量名|值`。
 * 变量名取自发起流程时传入的流程变量（请假流程为 leaveDays / leaveType / businessCode）。
 */
export const ConditionOps = [
  { value: 'eq', label: '等于' },
  { value: 'ne', label: '不等于' },
  { value: 'gt', label: '大于' },
  { value: 'ge', label: '大于等于' },
  { value: 'lt', label: '小于' },
  { value: 'le', label: '小于等于' },
  { value: 'like', label: '包含' },
  { value: 'unlike', label: '不包含' }
] as const;

/** 把「操作符 + 变量 + 值」拼成 warm-flow 条件表达式 */
export const buildCondition = (op: string, variable: string, value: string | number): string => {
  const v = String(variable ?? '').trim();
  if (!v) return '';
  return `${op}@@${v}|${String(value ?? '').trim()}`;
};

/** 反解析已有条件，用于属性面板回填 */
export const parseCondition = (raw?: string | null): { op: string; variable: string; value: string } | null => {
  const m = /^([a-z]+)@@([^|]+)\|(.*)$/i.exec((raw ?? '').trim());
  if (!m) return null;
  return { op: m[1].toLowerCase(), variable: m[2].trim(), value: m[3].trim() };
};

// ---------------------------------------------------------------------------
// 坐标解析
// ---------------------------------------------------------------------------

export const parseNodeCoord = (raw?: string | null): Point => {
  const first = (raw ?? '').split('|')[0] ?? '';
  const [x, y] = first.split(',').map((v) => Number(v));
  return Number.isFinite(x) && Number.isFinite(y) ? { x, y } : { x: 120, y: 120 };
};

export const formatNodeCoord = (p: Point): string => `${Math.round(p.x)},${Math.round(p.y)}|${Math.round(p.x)},${Math.round(p.y)}`;

/** 连线坐标可能是 "x,y;x,y" 或多段折线，只取首尾 */
export const parseSkipCoord = (raw?: string | null): { start: Point; end: Point } | null => {
  const parts = (raw ?? '').split(';').filter(Boolean);
  if (parts.length < 2) return null;
  const read = (s: string): Point | null => {
    const [x, y] = s.split(',').map((v) => Number(v));
    return Number.isFinite(x) && Number.isFinite(y) ? { x, y } : null;
  };
  const start = read(parts[0]);
  const end = read(parts[parts.length - 1]);
  return start && end ? { start, end } : null;
};

// ---------------------------------------------------------------------------
// 锚点计算
// ---------------------------------------------------------------------------

/** 节点四边中心点 */
export const anchors = (node: DesignNode): Record<'left' | 'right' | 'top' | 'bottom', Point> => {
  const c = parseNodeCoord(node.coordinate);
  const meta = nodeMeta(node.nodeType);
  if (meta.shape === 'circle') {
    return {
      left: { x: c.x - CIRCLE_R, y: c.y },
      right: { x: c.x + CIRCLE_R, y: c.y },
      top: { x: c.x, y: c.y - CIRCLE_R },
      bottom: { x: c.x, y: c.y + CIRCLE_R }
    };
  }
  if (meta.shape === 'diamond') {
    return {
      left: { x: c.x - GATEWAY_HW, y: c.y },
      right: { x: c.x + GATEWAY_HW, y: c.y },
      top: { x: c.x, y: c.y - GATEWAY_HH },
      bottom: { x: c.x, y: c.y + GATEWAY_HH }
    };
  }
  return {
    left: { x: c.x - NODE_W / 2, y: c.y },
    right: { x: c.x + NODE_W / 2, y: c.y },
    top: { x: c.x, y: c.y - NODE_H / 2 },
    bottom: { x: c.x, y: c.y + NODE_H / 2 }
  };
};

/**
 * 根据两节点的相对位置挑一组锚点：
 * 水平占优走左右，垂直占优走上下，源在目标右侧时走「源右 → 目标左」的回环。
 */
export const pickAnchors = (from: DesignNode, to: DesignNode): { start: Point; end: Point } => {
  const a = anchors(from);
  const b = anchors(to);
  const fc = parseNodeCoord(from.coordinate);
  const tc = parseNodeCoord(to.coordinate);
  const dx = tc.x - fc.x;
  const dy = tc.y - fc.y;
  if (Math.abs(dx) >= Math.abs(dy)) {
    return dx >= 0 ? { start: a.right, end: b.left } : { start: a.left, end: b.right };
  }
  return dy >= 0 ? { start: a.bottom, end: b.top } : { start: a.top, end: b.bottom };
};

/** 正交折线：源锚点 → 中间水平/垂直过渡 → 目标锚点 */
export const elbowPath = (start: Point, end: Point): string => {
  if (Math.abs(start.x - end.x) < 1 || Math.abs(start.y - end.y) < 1) {
    return `M ${start.x} ${start.y} L ${end.x} ${end.y}`;
  }
  // 优先横向出线：先补一段水平，再垂直，再水平
  const midX = start.x + (end.x - start.x) / 2;
  if (Math.abs(end.x - start.x) >= Math.abs(end.y - start.y)) {
    return `M ${start.x} ${start.y} L ${midX} ${start.y} L ${midX} ${end.y} L ${end.x} ${end.y}`;
  }
  const midY = start.y + (end.y - start.y) / 2;
  return `M ${start.x} ${start.y} L ${start.x} ${midY} L ${end.x} ${midY} L ${end.x} ${end.y}`;
};

/** 按最新的节点位置，重算所有连线的 coordinate */
export const refreshSkipCoords = (def: DesignJson) => {
  const byCode = new Map(def.nodeList.map((n) => [n.nodeCode, n]));
  for (const node of def.nodeList) {
    for (const skip of node.skipList ?? []) {
      const from = byCode.get(skip.nowNodeCode);
      const to = byCode.get(skip.nextNodeCode);
      if (!from || !to) continue;
      const { start, end } = pickAnchors(from, to);
      skip.coordinate = `${Math.round(start.x)},${Math.round(start.y)};${Math.round(end.x)},${Math.round(end.y)}`;
    }
  }
};

// ---------------------------------------------------------------------------
// 其它
// ---------------------------------------------------------------------------

/** 生成节点编码，与 Java 版一致使用 32 位十六进制 UUID */
export const genNodeCode = (): string => {
  const hex = (bytes: number): string => {
    const buf = new Uint8Array(bytes);
    crypto.getRandomValues(buf);
    return Array.from(buf)
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');
  };
  // 优先用原生 randomUUID（去掉连字符），环境不支持时退回到 getRandomValues
  return crypto.randomUUID?.().replace(/-/g, '') ?? hex(16);
};

/** 从开始节点做广度优先，给没有坐标（或坐标重复）的节点排一个从左到右的位置 */
export const autoLayout = (def: DesignJson) => {
  const nodes = def.nodeList;
  if (!nodes.length) return;
  const byCode = new Map(nodes.map((n) => [n.nodeCode, n]));
  const incoming = new Map<string, number>();
  nodes.forEach((n) => incoming.set(n.nodeCode, 0));
  for (const n of nodes) {
    for (const s of n.skipList ?? []) {
      incoming.set(s.nextNodeCode, (incoming.get(s.nextNodeCode) ?? 0) + 1);
    }
  }
  const starts = nodes.filter((n) => n.nodeType === NodeType.START);
  const roots = starts.length ? starts : [nodes[0]];
  const depth = new Map<string, number>();
  const queue: DesignNode[] = [];
  roots.forEach((r) => {
    depth.set(r.nodeCode, 0);
    queue.push(r);
  });
  while (queue.length) {
    const cur = queue.shift() as DesignNode;
    const d = depth.get(cur.nodeCode) ?? 0;
    for (const s of cur.skipList ?? []) {
      const next = byCode.get(s.nextNodeCode);
      if (!next) continue;
      const seen = depth.get(next.nodeCode);
      if (seen === undefined || seen < d + 1) {
        depth.set(next.nodeCode, d + 1);
        queue.push(next);
      }
    }
  }
  // 每层内按 y 顺序均分，避免叠在一起
  const buckets = new Map<number, DesignNode[]>();
  nodes.forEach((n) => {
    const d = depth.get(n.nodeCode) ?? 0;
    if (!buckets.has(d)) buckets.set(d, []);
    buckets.get(d)!.push(n);
  });
  for (const [d, list] of buckets) {
    list.forEach((n, idx) => {
      const old = parseNodeCoord(n.coordinate);
      // 已手工摆放过的节点（同层只有一个）保留原 y
      const y = list.length === 1 && Number.isFinite(old.y) ? old.y : 120 + idx * LAYOUT_GAP_Y;
      n.coordinate = formatNodeCoord({ x: 120 + d * LAYOUT_GAP_X, y });
    });
  }
  refreshSkipCoords(def);
};

/** 保存前的结构校验，返回错误文案（空数组表示通过） */
export const validateFlow = (def: DesignJson): string[] => {
  const errors: string[] = [];
  if (!def.flowCode?.trim()) errors.push('流程编码不能为空');
  if (!def.flowName?.trim()) errors.push('流程名称不能为空');
  const nodes = def.nodeList ?? [];
  if (!nodes.length) errors.push('画布为空，请至少拖入开始和结束节点');
  const starts = nodes.filter((n) => n.nodeType === NodeType.START);
  if (nodes.length && starts.length !== 1) errors.push(`开始节点必须恰好 1 个，当前 ${starts.length} 个`);
  if (nodes.length && !nodes.some((n) => n.nodeType === NodeType.END)) errors.push('缺少结束节点');

  const codes = new Set<string>();
  for (const n of nodes) {
    if (!n.nodeCode) errors.push(`节点「${n.nodeName || '未命名'}」缺少编码`);
    else if (codes.has(n.nodeCode)) errors.push(`节点编码重复：${n.nodeCode}`);
    codes.add(n.nodeCode);
    if (!n.nodeName?.trim()) errors.push(`节点 ${n.nodeCode} 缺少名称`);
  }
  for (const n of nodes) {
    for (const s of n.skipList ?? []) {
      if (!s.nextNodeCode) errors.push(`节点「${n.nodeName}」存在未连接完成的连线`);
      else if (!codes.has(s.nextNodeCode)) errors.push(`连线指向了不存在的节点 ${s.nextNodeCode}`);
    }
    // 除结束节点外都要有出线
    if (n.nodeType !== NodeType.END && !(n.skipList ?? []).length) {
      errors.push(`节点「${n.nodeName}」没有出线，流程无法走到结束`);
    }
  }
  // 网关节点的分支语义：互斥取第一条命中条件的出线，并行全走，包容走所有命中条件的出线
  for (const n of nodes) {
    if (!GATEWAY_TYPES.includes(n.nodeType)) continue;
    const outs = n.skipList ?? [];
    const title = `网关「${n.nodeName || n.nodeCode}」`;
    if (outs.length < 2) {
      errors.push(`${title} 至少需要 2 条出线，否则不需要网关`);
      continue;
    }
    const withCond = outs.filter((s) => (s.skipCondition ?? '').trim()).length;
    if (n.nodeType === NodeType.PARALLEL && withCond > 0) {
      errors.push(`${title} 是并行网关，出线会全部执行，条件表达式无效，请去掉条件或改用互斥/包容网关`);
    }
    if (n.nodeType !== NodeType.PARALLEL && withCond === 0) {
      errors.push(`${title} 的出线都没有条件表达式，分支形同虚设（如「大于 2 天」填 gt@@leaveDays|2）`);
    }
  }
  // 孤立节点
  const reached = new Set<string>();
  const byCode = new Map(nodes.map((n) => [n.nodeCode, n]));
  const walk = (code: string) => {
    if (reached.has(code)) return;
    reached.add(code);
    const n = byCode.get(code);
    for (const s of n?.skipList ?? []) if (s.nextNodeCode) walk(s.nextNodeCode);
  };
  if (starts.length) walk(starts[0].nodeCode);
  for (const n of nodes) {
    if (!reached.has(n.nodeCode)) errors.push(`节点「${n.nodeName}」从开始节点不可达`);
  }
  return Array.from(new Set(errors));
};
