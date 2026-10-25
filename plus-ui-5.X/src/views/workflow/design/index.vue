<template>
  <div class="app-container design-page">
    <!-- 顶部工具栏 -->
    <div class="dp-toolbar">
      <el-space wrap>
        <el-button icon="Back" @click="goBack">返回</el-button>
        <el-button type="primary" icon="Check" :loading="saving" @click="save(false)">保存</el-button>
        <el-button type="success" icon="Promotion" :loading="saving" @click="save(true)">保存并发布</el-button>
        <el-button icon="Guide" @click="runValidate">校验</el-button>
        <el-button icon="Sort" @click="doAutoLayout">自动布局</el-button>
        <el-dropdown @command="onImportExport">
          <el-button>
            JSON<el-icon class="el-icon--right"><arrow-down /></el-icon>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="export">导出 JSON</el-dropdown-item>
              <el-dropdown-item command="import">导入 JSON</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
        <el-button-group>
          <el-button icon="ZoomOut" @click="stepZoom(-0.1)" />
          <el-button disabled>{{ Math.round(zoom * 100) }}%</el-button>
          <el-button icon="ZoomIn" @click="stepZoom(0.1)" />
        </el-button-group>
      </el-space>
      <div class="dp-tip">
        <el-text size="small" type="info">拖动节点改变位置 · 悬停节点后拖动小圆点连线 · 点击连线编辑属性 · Delete 删除选中</el-text>
      </div>
    </div>

    <div class="dp-body">
      <!-- 左侧节点面板 -->
      <div class="dp-palette">
        <div class="dp-section-title">节点类型</div>
        <div v-for="meta in NODE_METAS" :key="meta.value" class="dp-palette-item" @click="addNode(meta)">
          <span class="dp-dot" :style="{ background: meta.color }" />
          <span>{{ meta.label }}</span>
        </div>
        <el-divider />
        <div class="dp-section-title">流程信息</div>
        <el-form label-width="70px" size="small" class="dp-base-form">
          <el-form-item label="编码">
            <el-input v-model="def.flowCode" placeholder="flowCode" />
          </el-form-item>
          <el-form-item label="名称">
            <el-input v-model="def.flowName" placeholder="流程名称" />
          </el-form-item>
          <el-form-item label="分类">
            <el-tree-select
              v-model="def.category"
              :data="def.categoryList ?? []"
              :props="{ label: 'label', children: 'children' }"
              node-key="id"
              check-strictly
              clearable
              placeholder="请选择分类"
              class="w-full"
            />
          </el-form-item>
          <el-form-item label="表单">
            <el-select v-model="def.formPath" clearable placeholder="请选择表单" class="w-full">
              <el-option v-for="item in def.formPathList ?? []" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
        </el-form>
      </div>

      <!-- 中间画布 -->
      <div class="dp-canvas" @mouseleave="cancelLinking">
        <svg
          ref="svgEl"
          class="dp-svg"
          :style="{ transform: `scale(${zoom})` }"
          :width="CANVAS.w"
          :height="CANVAS.h"
          :viewBox="`0 0 ${CANVAS.w} ${CANVAS.h}`"
          @mousemove="onMouseMove"
          @mouseup="onMouseUp"
          @click.self="clearSelection"
        >
          <defs>
            <marker id="dp-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path d="M 0 0 L 10 5 L 0 10 z" fill="#98a2b3" />
            </marker>
            <marker id="dp-arrow-active" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path d="M 0 0 L 10 5 L 0 10 z" fill="#409eff" />
            </marker>
          </defs>

          <!-- 连线 -->
          <g v-for="link in links" :key="link.key">
            <path :d="link.d" class="dp-line-hit" @click.stop="selectSkip(link.node, link.skip, link.key)" />
            <path
              :d="link.d"
              class="dp-line"
              :class="{ active: link.active }"
              :marker-end="link.active ? 'url(#dp-arrow-active)' : 'url(#dp-arrow)'"
            />
            <text v-if="link.label" :x="link.labelX" :y="link.labelY" class="dp-skip-label" text-anchor="middle">{{ link.label }}</text>
            <text
              v-if="link.condition"
              :x="link.labelX"
              :y="link.label ? link.labelY + 13 : link.labelY"
              class="dp-cond-label"
              text-anchor="middle"
            >
              {{ link.condition }}
            </text>
          </g>

          <!-- 正在绘制的连线 -->
          <path v-if="linking" :d="tempLinkPath" class="dp-line dp-line-temp" marker-end="url(#dp-arrow-active)" />

          <!-- 节点 -->
          <g
            v-for="node in def.nodeList"
            :key="node.nodeCode"
            class="dp-node"
            :class="{
              active: node.nodeCode === selectedNode?.nodeCode,
              'as-source': node.nodeCode === selectedSkip?.node.nodeCode
            }"
            :transform="`translate(${center(node).x}, ${center(node).y})`"
            @mousedown.left="startDrag(node, $event)"
            @mouseup="finishLink(node)"
            @click.stop="selectNode(node)"
          >
            <circle v-if="shapeOf(node) === 'circle'" :r="CIRCLE_R" :fill="fillOf()" :stroke="strokeOf(node)" stroke-width="1.5" />
            <polygon
              v-else-if="shapeOf(node) === 'diamond'"
              :points="`0,-${GATEWAY_HH} ${GATEWAY_HW},0 0,${GATEWAY_HH} -${GATEWAY_HW},0`"
              :fill="fillOf()"
              :stroke="strokeOf(node)"
              stroke-width="1.5"
            />
            <rect
              v-else
              :x="-NODE_W / 2"
              :y="-NODE_H / 2"
              :width="NODE_W"
              :height="NODE_H"
              rx="6"
              :fill="fillOf()"
              :stroke="strokeOf(node)"
              stroke-width="1.5"
            />
            <text
              v-for="(line, li) in labelLines(node)"
              :key="li"
              :x="0"
              :y="(li - (labelLines(node).length - 1) / 2) * 14"
              class="dp-node-label"
              text-anchor="middle"
              dominant-baseline="middle"
            >
              {{ line }}
            </text>

            <!-- 四个锚点，悬停时出现，用于拉出连线 -->
            <circle
              v-for="a in anchorList(node)"
              :key="a.name"
              :cx="a.x - center(node).x"
              :cy="a.y - center(node).y"
              r="5"
              class="dp-anchor"
              @mousedown.left.stop="startLink(node, $event)"
            />
          </g>
        </svg>
      </div>

      <!-- 右侧属性面板 -->
      <div class="dp-props">
        <div class="dp-section-title">{{ panelTitle }}</div>

        <el-empty v-if="!selectedNode && !selectedSkip" description="选中节点或连线后编辑属性" :image-size="70" />

        <!-- 节点属性：选中连线时让位给连线面板，否则两者同时有值会抢渲染 -->
        <el-form v-else-if="selectedNode && !selectedSkip" label-width="80px" size="small">
          <el-form-item label="节点名称">
            <el-input v-model="selectedNode.nodeName" placeholder="如：部门主管" maxlength="30" />
          </el-form-item>
          <el-form-item label="节点编码">
            <el-input v-model="selectedNode.nodeCode" />
          </el-form-item>
          <el-form-item label="节点类型">
            <el-tag size="small">{{ nodeMeta(selectedNode.nodeType).label }}</el-tag>
          </el-form-item>
          <el-form-item v-if="selectedNode.nodeType !== NodeType.START && selectedNode.nodeType !== NodeType.END" label="办理人">
            <div class="dp-handler">
              <el-input v-model="selectedNode.permissionFlag" placeholder="role:1@@dept:103@@user:5" />
              <el-button class="mt1" icon="User" @click="openHandler">选择办理人</el-button>
            </div>
          </el-form-item>
          <el-form-item v-if="selectedNode.nodeType === NodeType.BETWEEN" label="节点表单">
            <el-select v-model="selectedNode.formPath" clearable placeholder="不填沿用流程表单" class="w-full">
              <el-option v-for="item in def.formPathList ?? []" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item v-if="isGateway(selectedNode)" label="监听器">
            <el-input v-model="selectedNode.listenerPath" placeholder="条件表达式类路径（可留空）" />
          </el-form-item>
          <el-form-item label="出线">
            <div class="dp-skips">
              <div
                v-for="(skip, idx) in selectedNode.skipList"
                :key="idx"
                class="dp-skip-item"
                :class="{ active: selectedSkip?.skip === skip }"
                @click="selectSkip(selectedNode, skip, selectedNode.nodeCode + ':' + idx)"
              >
                <span>{{ skip.skipName || '（未命名）' }}</span>
                <span class="dp-skip-to">→ {{ nameOf(skip.nextNodeCode) }}</span>
                <el-icon class="dp-skip-del" @click.stop="removeSkip(selectedNode, skip)"><delete /></el-icon>
              </div>
              <el-text v-if="!selectedNode.skipList.length" size="small" type="info">无，拖拽节点边缘圆点连线</el-text>
            </div>
          </el-form-item>
          <el-form-item>
            <el-button type="danger" plain icon="Delete" :disabled="selectedNode.nodeType === NodeType.START" @click="removeNode(selectedNode)">
              删除节点
            </el-button>
          </el-form-item>
        </el-form>

        <!-- 连线属性 -->
        <el-form v-else-if="selectedSkip" label-width="80px" size="small">
          <el-form-item label="起点">
            <el-input :model-value="nameOf(selectedSkip.node.nodeCode)" disabled />
          </el-form-item>
          <el-form-item label="终点">
            <el-input :model-value="nameOf(selectedSkip.skip.nextNodeCode)" disabled />
          </el-form-item>
          <el-form-item label="连线名称">
            <el-input v-model="selectedSkip.skip.skipName" placeholder="如：同意" maxlength="20" />
          </el-form-item>
          <el-form-item label="流转类型">
            <el-select v-model="selectedSkip.skip.skipType" class="w-full">
              <el-option v-for="t in SkipTypes" :key="t.value" :label="t.label" :value="t.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="条件">
            <div class="dp-cond">
              <el-select v-model="cond.op" class="cond-op">
                <el-option v-for="o in ConditionOps" :key="o.value" :label="o.label" :value="o.value" />
              </el-select>
              <el-input v-model="cond.variable" placeholder="变量名 如 leaveDays" />
              <el-input v-model="cond.value" placeholder="值 如 2" class="cond-val" />
              <el-button link type="primary" @click="applyCondition">生成</el-button>
              <el-button link type="info" @click="clearCondition">清空</el-button>
            </div>
            <el-input v-model="selectedSkip.skip.skipCondition" placeholder="gt@@leaveDays|2（留空即无条件出线）" />
            <div class="dp-tip">格式「操作符@@变量名|值」，变量取发起时传入的流程变量；互斥网关按出线顺序取第一条命中</div>
          </el-form-item>
          <el-form-item>
            <el-button type="danger" plain icon="Delete" @click="removeSelectedSkip">删除连线</el-button>
          </el-form-item>
        </el-form>
      </div>
    </div>

    <!-- 办理人选择 -->
    <el-dialog v-model="handlerVisible" title="选择办理人" width="720px" append-to-body>
      <el-tabs v-model="handlerTab">
        <el-tab-pane label="角色" name="role">
          <el-checkbox-group v-model="handlerModel.roles">
            <el-checkbox v-for="r in roleOptions" :key="r.roleId" :value="String(r.roleId)" class="dp-hb">
              {{ r.roleName }}
            </el-checkbox>
          </el-checkbox-group>
        </el-tab-pane>
        <el-tab-pane label="部门" name="dept">
          <el-tree
            ref="deptTreeRef"
            :data="deptOptions"
            :props="{ label: 'label', children: 'children' }"
            node-key="id"
            show-checkbox
          />
        </el-tab-pane>
        <el-tab-pane label="用户" name="user">
          <el-checkbox-group v-model="handlerModel.users">
            <el-checkbox v-for="u in userOptions" :key="u.userId" :value="String(u.userId)" class="dp-hb">
              {{ u.nickName }}
            </el-checkbox>
          </el-checkbox-group>
        </el-tab-pane>
      </el-tabs>
      <template #footer>
        <el-button @click="handlerVisible = false">取消</el-button>
        <el-button type="primary" @click="confirmHandler">确定</el-button>
      </template>
    </el-dialog>

    <!-- 导入 JSON -->
    <el-dialog v-model="importVisible" title="导入流程定义 JSON" width="640px" append-to-body>
      <el-input v-model="importText" type="textarea" :rows="16" placeholder="粘贴设计器 / warm-flow 导出的流程 JSON" />
      <template #footer>
        <el-button @click="importVisible = false">取消</el-button>
        <el-button type="primary" @click="applyImport">导入到画布</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup name="WorkflowDesign" lang="ts">
import { ElMessage, ElMessageBox } from 'element-plus';
import { DesignJson, DesignNode, DesignSkip } from '@/api/workflow/definition/types';
import { getDesign, publishDefinition, saveDesign } from '@/api/workflow/definition';
import { listDept } from '@/api/system/dept';
import { listRole } from '@/api/system/role';
import { listUser } from '@/api/system/user';
import {
  CANVAS_SIZE,
  CIRCLE_R,
  ConditionOps,
  GATEWAY_HH,
  GATEWAY_HW,
  GATEWAY_TYPES,
  NODE_H,
  NODE_METAS,
  NODE_W,
  NodeType,
  SkipTypes,
  anchors,
  autoLayout,
  buildCondition,
  elbowPath,
  formatNodeCoord,
  genNodeCode,
  nodeMeta,
  parseCondition,
  parseNodeCoord,
  pickAnchors,
  refreshSkipCoords,
  validateFlow,
  type Point
} from './flow';

const route = useRoute();
const router = useRouter();

/** 画布固定尺寸，配合 viewBox + CSS scale 做缩放，鼠标坐标换算不受缩放影响 */
const CANVAS = CANVAS_SIZE;

const def = reactive<DesignJson>({ flowCode: '', flowName: '', modelValue: 'CLASSICS', category: '', nodeList: [] });
const svgEl = ref<SVGSVGElement>();
const zoom = ref(1);
const saving = ref(false);

const selectedNode = ref<DesignNode | null>(null);
const selectedSkip = ref<{ node: DesignNode; skip: DesignSkip; key: string } | null>(null);

const drag = ref<{ node: DesignNode; offset: Point } | null>(null);
const linking = ref<{ from: DesignNode; start: Point; end: Point } | null>(null);

const isGateway = (node: DesignNode) => GATEWAY_TYPES.includes(node.nodeType);

/** 条件表达式的结构化编辑区（生成后仍可在下方输入框手工微调） */
const cond = reactive({ op: 'gt', variable: '', value: '' });
watch(
  () => selectedSkip.value,
  (sel) => {
    const p = parseCondition(sel?.skip.skipCondition);
    cond.op = p?.op ?? 'gt';
    cond.variable = p?.variable ?? '';
    cond.value = p?.value ?? '';
  },
  { immediate: true }
);
const applyCondition = () => {
  const expr = buildCondition(cond.op, cond.variable, cond.value);
  if (!expr) {
    ElMessage.warning('请先填写变量名');
    return;
  }
  if (selectedSkip.value) selectedSkip.value.skip.skipCondition = expr;
};
const clearCondition = () => {
  cond.variable = '';
  cond.value = '';
  if (selectedSkip.value) selectedSkip.value.skip.skipCondition = '';
};

// ---------------------------------------------------------------------------
// 载入
// ---------------------------------------------------------------------------

const definitionId = computed(() => (route.params.id ? String(route.params.id) : ''));

const loadDesign = async () => {
  const res = await getDesign(definitionId.value || undefined);
  const data = res.data ?? ({ nodeList: [] } as unknown as DesignJson);
  data.nodeList = data.nodeList ?? [];
  Object.assign(def, data);
  if (!def.modelValue) def.modelValue = 'CLASSICS';
  // 新建时给一个「开始 → 结束」的起手式
  if (!def.nodeList.length) {
    const start = mkNode(NodeType.START, '开始', { x: 160, y: 200 });
    const end = mkNode(NodeType.END, '结束', { x: 420, y: 200 });
    start.skipList.push(mkSkip(start, end));
    def.nodeList.push(start, end);
  }
  clearSelection();
};

/** 设计器 JSON 与后端一致：nodeList 里每条 skip 挂在起点节点上 */
const mkNode = (nodeType: number, nodeName: string, p: Point): DesignNode => ({
  nodeType,
  nodeCode: genNodeCode(),
  nodeName,
  permissionFlag: nodeType === NodeType.BETWEEN ? '' : null,
  coordinate: formatNodeCoord(p),
  skipList: []
});

const mkSkip = (from: DesignNode, to: DesignNode): DesignSkip => ({
  nowNodeCode: from.nodeCode,
  nextNodeCode: to.nodeCode,
  skipName: '',
  skipType: 'PASS',
  skipCondition: '',
  coordinate: skipCoord(from, to)
});

/** 按当前两端节点位置算出连线坐标串 */
const skipCoord = (from: DesignNode, to: DesignNode): string => {
  const { start, end } = pickAnchors(from, to);
  return `${Math.round(start.x)},${Math.round(start.y)};${Math.round(end.x)},${Math.round(end.y)}`;
};

onMounted(() => {
  loadDesign();
  loadHandlers();
  window.addEventListener('keydown', onKeydown);
});

onBeforeUnmount(() => window.removeEventListener('keydown', onKeydown));

// ---------------------------------------------------------------------------
// 画布渲染
// ---------------------------------------------------------------------------

const center = (node: DesignNode): Point => parseNodeCoord(node.coordinate);

const shapeOf = (node: DesignNode) => nodeMeta(node.nodeType).shape;
const strokeOf = (node: DesignNode) => nodeMeta(node.nodeType).color;
const fillOf = () => '#ffffff';

const labelLines = (node: DesignNode): string[] => {
  const text = node.nodeName || node.nodeCode || '';
  const limit = shapeOf(node) === 'rect' ? 6 : 4;
  const chars = Array.from(text);
  if (chars.length <= limit) return chars.length ? [text] : [''];
  const out: string[] = [];
  for (let i = 0; i < chars.length && out.length < 3; i += limit) {
    out.push(chars.slice(i, i + limit).join(''));
  }
  return out;
};

const anchorList = (node: DesignNode) => {
  const a = anchors(node);
  return [
    { name: 'left', ...a.left },
    { name: 'right', ...a.right },
    { name: 'top', ...a.top },
    { name: 'bottom', ...a.bottom }
  ];
};

interface DrawnLink {
  key: string;
  node: DesignNode;
  skip: DesignSkip;
  d: string;
  label: string;
  condition: string;
  labelX: number;
  labelY: number;
  active: boolean;
}

const links = computed<DrawnLink[]>(() => {
  const byCode = new Map(def.nodeList.map((n) => [n.nodeCode, n]));
  const out: DrawnLink[] = [];
  def.nodeList.forEach((node) => {
    (node.skipList ?? []).forEach((skip, idx) => {
      const to = byCode.get(skip.nextNodeCode);
      if (!to) return;
      const { start, end } = pickAnchors(node, to);
      out.push({
        key: `${node.nodeCode}:${idx}`,
        node,
        skip,
        d: elbowPath(start, end),
        label: skip.skipName ?? '',
        condition: (skip.skipCondition ?? '').trim(),
        labelX: (start.x + end.x) / 2,
        labelY: (start.y + end.y) / 2 - 7,
        active: selectedSkip.value?.skip === skip
      });
    });
  });
  return out;
});

const tempLinkPath = computed(() => {
  const l = linking.value;
  return l ? elbowPath(l.start, l.end) : '';
});

const nameOf = (code?: string) => def.nodeList.find((n) => n.nodeCode === code)?.nodeName || code || '';

const toPoint = (e: MouseEvent): Point => {
  const rect = (svgEl.value as SVGSVGElement).getBoundingClientRect();
  return { x: ((e.clientX - rect.left) / rect.width) * CANVAS.w, y: ((e.clientY - rect.top) / rect.height) * CANVAS.h };
};

const snap = (v: number) => Math.round(v / 5) * 5;

// ---------------------------------------------------------------------------
// 交互
// ---------------------------------------------------------------------------

const startDrag = (node: DesignNode, e: MouseEvent) => {
  if (linking.value) return;
  const p = toPoint(e);
  const c = center(node);
  drag.value = { node, offset: { x: p.x - c.x, y: p.y - c.y } };
};

const startLink = (node: DesignNode, e: MouseEvent) => {
  const p = toPoint(e);
  const a = anchors(node);
  const pick = [a.left, a.right, a.top, a.bottom].reduce((best, cur) =>
    Math.hypot(cur.x - p.x, cur.y - p.y) < Math.hypot(best.x - p.x, best.y - p.y) ? cur : best
  );
  linking.value = { from: node, start: { ...pick }, end: p };
  drag.value = null;
};

const onMouseMove = (e: MouseEvent) => {
  const p = toPoint(e);
  if (linking.value) {
    linking.value.end = p;
    return;
  }
  if (drag.value) {
    const { node, offset } = drag.value;
    node.coordinate = formatNodeCoord({ x: snap(p.x - offset.x), y: snap(p.y - offset.y) });
  }
};

const onMouseUp = () => {
  drag.value = null;
  if (linking.value) cancelLinking();
};

const finishLink = (node: DesignNode) => {
  const l = linking.value;
  if (!l) return;
  if (l.from.nodeCode === node.nodeCode) {
    ElMessage.warning('不能连接到自身');
    cancelLinking();
    return;
  }
  const exists = (l.from.skipList ?? []).some((s) => s.nextNodeCode === node.nodeCode);
  if (exists) {
    ElMessage.warning('这两个节点之间已存在连线');
  } else {
    l.from.skipList.push(mkSkip(l.from, node));
  }
  cancelLinking();
};

const cancelLinking = () => {
  linking.value = null;
};

const selectNode = (node: DesignNode) => {
  selectedNode.value = node;
  selectedSkip.value = null;
};

const selectSkip = (node: DesignNode | null, skip: DesignSkip, key: string) => {
  selectedSkip.value = { node: node as DesignNode, skip, key };
  // 不能同时置位 selectedNode，否则节点面板会遮住连线面板，条件就没法编辑了
  selectedNode.value = null;
};

const clearSelection = () => {
  selectedNode.value = null;
  selectedSkip.value = null;
};

const panelTitle = computed(() => {
  if (selectedSkip.value) return '连线属性';
  if (selectedNode.value) return `节点属性 · ${nodeMeta(selectedNode.value.nodeType).label}`;
  return '属性';
});

const addNode = (meta: (typeof NODE_METAS)[number]) => {
  if (meta.value === NodeType.START && def.nodeList.some((n) => n.nodeType === NodeType.START)) {
    ElMessage.warning('流程只能有一个开始节点');
    return;
  }
  const taken = def.nodeList.map((n) => center(n));
  const y = 120 + (taken.length % 5) * 110;
  const x = 120 + Math.floor(taken.length / 5) * 180;
  const node = mkNode(meta.value, meta.label, { x, y });
  def.nodeList.push(node);
  selectNode(node);
};

const removeNode = (node: DesignNode) => {
  const idx = def.nodeList.findIndex((n) => n.nodeCode === node.nodeCode);
  if (idx < 0) return;
  def.nodeList.splice(idx, 1);
  for (const n of def.nodeList) {
    n.skipList = (n.skipList ?? []).filter((s) => s.nextNodeCode !== node.nodeCode);
  }
  clearSelection();
};

const removeSkip = (node: DesignNode, skip: DesignSkip) => {
  node.skipList = (node.skipList ?? []).filter((s) => s !== skip);
  if (selectedSkip.value?.skip === skip) selectedSkip.value = null;
};

const removeSelectedSkip = () => {
  const cur = selectedSkip.value;
  if (cur) removeSkip(cur.node, cur.skip);
};

const onKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    cancelLinking();
    return;
  }
  if (e.key !== 'Delete') return;
  const tag = (document.activeElement?.tagName ?? '').toLowerCase();
  if (['input', 'textarea'].includes(tag)) return;
  if (selectedSkip.value) removeSelectedSkip();
  else if (selectedNode.value && selectedNode.value.nodeType !== NodeType.START) removeNode(selectedNode.value);
};

const stepZoom = (delta: number) => {
  zoom.value = Math.min(2, Math.max(0.5, Number((zoom.value + delta).toFixed(2))));
};

const doAutoLayout = () => {
  autoLayout(def);
  clearSelection();
};

const runValidate = async () => {
  refreshSkipCoords(def);
  const errors = validateFlow(def);
  if (!errors.length) {
    ElMessage.success('校验通过');
    return;
  }
  await ElMessageBox.alert(errors.map((e) => `<div>· ${e}</div>`).join(''), '流程校验未通过', {
    dangerouslyUseHTMLString: true,
    confirmButtonText: '知道了'
  });
};

// ---------------------------------------------------------------------------
// 保存 / 导入导出
// ---------------------------------------------------------------------------

const save = async (publish: boolean) => {
  refreshSkipCoords(def);
  const errors = validateFlow(def);
  if (errors.length) {
    ElMessage.error(errors[0]);
    return;
  }
  saving.value = true;
  try {
    const payload = JSON.parse(JSON.stringify({ ...def, nodeList: def.nodeList })) as DesignJson;
    delete (payload as any).categoryList;
    delete (payload as any).formPathList;
    const res = await saveDesign(payload);
    const id = String(res.data?.id ?? '');
    if (id) {
      def.id = id;
      def.version = res.data?.version ?? def.version;
    }
    if (publish && id) {
      await publishDefinition(id);
      ElMessage.success('已保存并发布');
    } else {
      ElMessage.success('保存成功');
    }
    if (id && !definitionId.value) {
      router.replace(`/workflow/design/${id}`);
    }
  } finally {
    saving.value = false;
  }
};

const onImportExport = (command: string) => {
  if (command === 'export') {
    refreshSkipCoords(def);
    const blob = new Blob([JSON.stringify(def, null, 2)], { type: 'application/json' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = `${def.flowCode || 'flow'}.json`;
    a.click();
    URL.revokeObjectURL(a.href);
    return;
  }
  importText.value = '';
  importVisible.value = true;
};

const importVisible = ref(false);
const importText = ref('');

const applyImport = () => {
  let parsed: DesignJson;
  try {
    parsed = JSON.parse(importText.value) as DesignJson;
  } catch {
    ElMessage.error('JSON 解析失败，请检查内容');
    return;
  }
  if (!parsed?.nodeList?.length) {
    ElMessage.error('缺少 nodeList，不是有效的流程定义 JSON');
    return;
  }
  // 导入的是「另一份定义」，id / version 必须丢掉，避免覆盖库里原流程
  const keepName = parsed.flowName;
  Object.assign(def, parsed, { id: null, version: null, nodeList: parsed.nodeList });
  def.nodeList.forEach((n) => (n.skipList = n.skipList ?? []));
  clearSelection();
  importVisible.value = false;
  ElMessage.success(`已载入：${keepName ?? def.flowCode ?? ''}，保存后生成新版本`);
};

// ---------------------------------------------------------------------------
// 办理人
// ---------------------------------------------------------------------------

interface DeptNode {
  id: string | number;
  label: string;
  children?: DeptNode[];
}

const roleOptions = ref<any[]>([]);
const userOptions = ref<any[]>([]);
const deptOptions = ref<DeptNode[]>([]);

const handlerVisible = ref(false);
const handlerTab = ref('role');
const deptTreeRef = ref();
const handlerModel = reactive({ roles: [] as string[], depts: [] as string[], users: [] as string[] });

const loadHandlers = async () => {
  // 部门用扁平列表拼成树，接口不可用时退化为空列表，不阻塞设计器
  const [roleRes, userRes, deptRes] = await Promise.allSettled([
    listRole({ pageNum: 1, pageSize: 500 }),
    listUser({ pageNum: 1, pageSize: 500 }),
    listDept()
  ]);
  if (roleRes.status === 'fulfilled') roleOptions.value = roleRes.value.rows ?? roleRes.value.data ?? [];
  if (userRes.status === 'fulfilled') userOptions.value = userRes.value.rows ?? userRes.value.data ?? [];
  if (deptRes.status === 'fulfilled') deptOptions.value = buildDeptTree(deptRes.value.data ?? []);
};

const buildDeptTree = (list: any[]): DeptNode[] => {
  const nodes = new Map<string | number, DeptNode & { parentId?: string | number }>();
  list.forEach((d) => nodes.set(d.deptId, { id: d.deptId, label: d.deptName, children: [], parentId: d.parentId }));
  const roots: DeptNode[] = [];
  nodes.forEach((node) => {
    const parent = node.parentId === undefined ? undefined : nodes.get(node.parentId);
    if (parent) parent.children!.push(node);
    else roots.push(node);
  });
  return roots;
};

const openHandler = () => {
  const parts = (selectedNode.value?.permissionFlag ?? '').split('@@').filter(Boolean);
  handlerModel.roles = parts.filter((p) => p.startsWith('role:')).map((p) => p.slice(5));
  handlerModel.depts = parts.filter((p) => p.startsWith('dept:')).map((p) => p.slice(5));
  handlerModel.users = parts.filter((p) => p.startsWith('user:')).map((p) => p.slice(5));
  handlerTab.value = 'role';
  handlerVisible.value = true;
  nextTick(() => deptTreeRef.value?.setCheckedKeys(handlerModel.depts.map((v) => String(v))));
};

const confirmHandler = () => {
  const checked = (deptTreeRef.value?.getCheckedKeys() ?? []).map((v: any) => String(v));
  const parts = [
    ...handlerModel.roles.map((v) => `role:${v}`),
    ...checked.map((v) => `dept:${v}`),
    ...handlerModel.users.map((v) => `user:${v}`)
  ];
  if (selectedNode.value) selectedNode.value.permissionFlag = parts.join('@@');
  handlerVisible.value = false;
};

const goBack = () => {
  router.push('/workflow/definition');
};

watch(definitionId, () => loadDesign());
</script>

<style scoped lang="scss">
.design-page {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 84px);
  padding: 0;
}

.dp-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--el-bg-color);
  border-bottom: 1px solid var(--el-border-color-light);

  .dp-tip {
    white-space: nowrap;
  }
}

.dp-body {
  flex: 1;
  display: flex;
  min-height: 0;
}

.dp-palette,
.dp-props {
  width: 260px;
  flex: none;
  overflow: auto;
  padding: 10px;
  background: var(--el-bg-color);
}

.dp-palette {
  border-right: 1px solid var(--el-border-color-light);
}

.dp-props {
  width: 300px;
  border-left: 1px solid var(--el-border-color-light);
}

.dp-section-title {
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--el-text-color-primary);
}

.dp-palette-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 10px;
  margin-bottom: 6px;
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;

  &:hover {
    border-color: var(--el-color-primary);
    background: var(--el-color-primary-light-9);
  }

  .dp-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
  }
}

.dp-canvas {
  flex: 1;
  overflow: auto;
  background: #fafbfc;
  background-image: radial-gradient(#dcdfe6 1px, transparent 1px);
  background-size: 20px 20px;
}

.dp-svg {
  transform-origin: 0 0;
  display: block;
}

.dp-line {
  fill: none;
  stroke: #98a2b3;
  stroke-width: 1.6;

  &.active {
    stroke: var(--el-color-primary);
    stroke-width: 2.4;
  }
}

.dp-line-hit {
  fill: none;
  stroke: transparent;
  stroke-width: 12;
  cursor: pointer;
}

.dp-line-temp {
  stroke: var(--el-color-primary);
  stroke-dasharray: 4 3;
}

.dp-skip-label {
  font-size: 11px;
  fill: #606266;
  pointer-events: none;
}

/* 分支上另起一行显示条件原文，网关分流一眼可读 */
.dp-cond-label {
  font-size: 10px;
  fill: #e6a23c;
  pointer-events: none;
}

.dp-node {
  cursor: move;

  /* 自身被选中：描边转主色 */
  &.active rect,
  &.active circle,
  &.active polygon {
    stroke: var(--el-color-primary);
    stroke-width: 2.5;
  }

  /* 仅作为选中连线的端点：只加粗，不与“选中节点”混淆 */
  &.as-source rect,
  &.as-source circle,
  &.as-source polygon {
    stroke-width: 2.5;
  }

  &:hover .dp-anchor {
    opacity: 1;
    pointer-events: all;
  }
}

.dp-node-label {
  font-size: 12px;
  fill: #303133;
  pointer-events: none;
  user-select: none;
}

.dp-anchor {
  fill: var(--el-color-primary);
  stroke: #fff;
  stroke-width: 1.5;
  opacity: 0;
  pointer-events: none;
  cursor: crosshair;
}

.dp-handler {
  width: 100%;
}

.dp-cond {
  display: flex;
  align-items: center;
  gap: 6px;
  width: 100%;
  margin-bottom: 6px;

  .cond-op {
    width: 96px;
    flex: none;
  }

  .cond-val {
    width: 76px;
    flex: none;
  }
}

.dp-tip {
  width: 100%;
  font-size: 12px;
  line-height: 1.5;
  color: #909399;
}

.dp-skips {
  width: 100%;

  .dp-skip-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 8px;
    margin-bottom: 4px;
    font-size: 12px;
    border: 1px solid var(--el-border-color-lighter);
    border-radius: 4px;
    cursor: pointer;

    &.active {
      border-color: var(--el-color-primary);
      background: var(--el-color-primary-light-9);
    }

    .dp-skip-to {
      flex: 1;
      color: var(--el-text-color-secondary);
    }

    .dp-skip-del {
      color: var(--el-color-danger);
    }
  }
}

.w-full {
  width: 100%;
}

.mt1 {
  margin-top: 4px;
}

.dp-hb {
  display: block;
  margin-right: 0;
}
</style>
