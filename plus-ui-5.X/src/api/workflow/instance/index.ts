import request from '@/utils/request';
import { AxiosPromise } from 'axios';
import { FlowInstanceQuery, FlowInstanceVO } from '@/api/workflow/instance/types';
import { DesignJson } from '@/api/workflow/definition/types';

/**
 * 流程实例分页
 */
export const listInstance = (query?: FlowInstanceQuery): AxiosPromise<FlowInstanceVO[]> => {
  return request({
    url: '/workflow/instance/list',
    method: 'get',
    params: query
  });
};

/**
 * 流程实例详情
 */
export const getInstance = (id: string | number): AxiosPromise<FlowInstanceVO> => {
  return request({
    url: '/workflow/instance/' + id,
    method: 'get'
  });
};

/**
 * 流程图（节点与连线带办理状态，用于轨迹查看）
 */
export const flowChart = (id: string | number): AxiosPromise<DesignJson> => {
  return request({
    url: '/workflow/instance/flowChart/' + id,
    method: 'get'
  });
};

/**
 * 流程变量
 */
export const instanceVariable = (id: string | number): AxiosPromise<Record<string, any>> => {
  return request({
    url: '/workflow/instance/variable/' + id,
    method: 'get'
  });
};

/**
 * 按业务主键查实例（业务列表页判断是否已发起）
 */
export const instanceByBusinessId = (businessId: string | number): AxiosPromise<FlowInstanceVO> => {
  return request({
    url: '/workflow/instance/business/' + businessId,
    method: 'get'
  });
};

/** 激活实例 */
export const activeInstance = (id: string | number) => {
  return request({
    url: '/workflow/instance/active/' + id,
    method: 'put'
  });
};

/** 挂起实例 */
export const unActiveInstance = (id: string | number) => {
  return request({
    url: '/workflow/instance/unActive/' + id,
    method: 'put'
  });
};

/** 删除实例（多个用逗号分隔） */
export const delInstance = (ids: string | number | Array<string | number>) => {
  return request({
    url: '/workflow/instance/' + ids,
    method: 'delete'
  });
};
