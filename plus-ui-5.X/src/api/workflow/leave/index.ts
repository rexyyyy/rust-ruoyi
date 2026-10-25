import request from '@/utils/request';
import { AxiosPromise } from 'axios';
import { LeaveForm, LeaveQuery, LeaveVO } from '@/api/workflow/leave/types';
import { StartReturnVO } from '@/api/workflow/task/types';

/**
 * 请假单分页
 */
export const listLeave = (query?: LeaveQuery): AxiosPromise<LeaveVO[]> => {
  return request({
    url: '/workflow/leave/list',
    method: 'get',
    params: query
  });
};

/**
 * 请假单详情
 */
export const getLeave = (id: string | number): AxiosPromise<LeaveVO> => {
  return request({
    url: '/workflow/leave/' + id,
    method: 'get'
  });
};

/**
 * 新增请假单（草稿），返回主键字符串
 */
export const addLeave = (data: LeaveForm): AxiosPromise<string> => {
  return request({
    url: '/workflow/leave',
    method: 'post',
    data: data
  });
};

/**
 * 修改请假单
 */
export const updateLeave = (data: LeaveForm) => {
  return request({
    url: '/workflow/leave',
    method: 'put',
    data: data
  });
};

/**
 * 保存并提交审批（一步发起流程）
 */
export const submitLeave = (data: LeaveForm): AxiosPromise<StartReturnVO> => {
  return request({
    url: '/workflow/leave/submit',
    method: 'post',
    data: data
  });
};

/**
 * 按业务主键取请假单（待办详情页渲染业务表单）
 */
export const leaveByBusinessId = (businessId: string | number): AxiosPromise<LeaveVO> => {
  return request({
    url: '/workflow/instance/leave/' + businessId,
    method: 'get'
  });
};

/**
 * 删除请假单（多个用逗号分隔）
 */
export const delLeave = (ids: string | number | Array<string | number>) => {
  return request({
    url: '/workflow/leave/' + ids,
    method: 'delete'
  });
};
