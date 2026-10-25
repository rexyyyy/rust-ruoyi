/**
 * 流程分类 视图对象
 */
export interface CategoryVO {
  categoryId: string;
  parentId: string;
  ancestors?: string;
  categoryName: string;
  orderNum?: number;
  createTime?: string;
  children?: CategoryVO[];
}

/**
 * 流程分类 表单（新增 / 修改共用）
 */
export interface CategoryForm {
  categoryId?: string | number;
  parentId?: string | number;
  categoryName?: string;
  orderNum?: number;
}

/**
 * 流程分类 查询参数
 */
export interface CategoryQuery {
  categoryName?: string;
}

/**
 * 下拉树节点（后端 Tree 结构）
 */
export interface CategoryTreeVO {
  id: string;
  name?: string;
  label?: string;
  value?: string;
  children?: CategoryTreeVO[];
}
