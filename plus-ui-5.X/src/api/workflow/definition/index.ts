import request from '@/utils/request';
import { AxiosPromise } from 'axios';
import { DesignJson, FlowDefinitionQuery, FlowDefinitionVO } from '@/api/workflow/definition/types';

/**
 * 流程定义分页
 */
export const listDefinition = (query?: FlowDefinitionQuery): AxiosPromise<FlowDefinitionVO[]> => {
  return request({
    url: '/workflow/definition/list',
    method: 'get',
    params: query
  });
};

/**
 * 已发布的流程定义（发起流程时选择）
 */
export const publishedList = (): AxiosPromise<FlowDefinitionVO[]> => {
  return request({
    url: '/workflow/definition/publishedList',
    method: 'get'
  });
};

/**
 * 流程定义详情
 */
export const getDefinition = (id: string | number): AxiosPromise<FlowDefinitionVO> => {
  return request({
    url: '/workflow/definition/' + id,
    method: 'get'
  });
};

/**
 * 设计器数据：不传 id 返回新建壳子
 */
export const getDesign = (id?: string | number): AxiosPromise<DesignJson> => {
  return request({
    url: '/workflow/definition/design',
    method: 'get',
    params: { id }
  });
};

/**
 * 保存设计器数据，返回带真实 id 的流程定义
 */
export const saveDesign = (data: DesignJson, onlyNodeSkip = false): AxiosPromise<FlowDefinitionVO> => {
  return request({
    url: '/workflow/definition/design',
    method: 'post',
    params: { onlyNodeSkip },
    data: data
  });
};

/**
 * 导入流程定义 JSON（总是生成新版本）
 */
export const importDefinition = (json: string): AxiosPromise<FlowDefinitionVO> => {
  return request({
    url: '/workflow/definition/import',
    method: 'post',
    data: { json }
  });
};

/**
 * 导出流程定义 JSON（后端直接返回 DefJson，前端转成文件下载）
 */
export const exportDefinition = (id: string | number): AxiosPromise<DesignJson> => {
  return request({
    url: '/workflow/definition/export/' + id,
    method: 'get'
  });
};

/** 发布 */
export const publishDefinition = (id: string | number) => {
  return request({
    url: '/workflow/definition/publish/' + id,
    method: 'put'
  });
};

/** 取消发布 */
export const unPublishDefinition = (id: string | number) => {
  return request({
    url: '/workflow/definition/unPublish/' + id,
    method: 'put'
  });
};

/** 激活 */
export const activeDefinition = (id: string | number) => {
  return request({
    url: '/workflow/definition/active/' + id,
    method: 'put'
  });
};

/** 挂起 */
export const unActiveDefinition = (id: string | number) => {
  return request({
    url: '/workflow/definition/unActive/' + id,
    method: 'put'
  });
};

/** 复制为新版本 */
export const copyDefinition = (id: string | number) => {
  return request({
    url: '/workflow/definition/copy/' + id,
    method: 'post'
  });
};

/** 删除流程定义（多个用逗号分隔） */
export const delDefinition = (ids: string | number | Array<string | number>) => {
  return request({
    url: '/workflow/definition/' + ids,
    method: 'delete'
  });
};
