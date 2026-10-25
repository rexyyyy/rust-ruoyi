import request from '@/utils/request';
import { AxiosPromise } from 'axios';
import {
  CompleteTaskBo,
  FlowHisTaskVO,
  FlowTaskQuery,
  FlowTaskVO,
  RevokeBo,
  StartProcessBo,
  StartReturnVO,
  TaskOperationBo,
  TerminationBo
} from '@/api/workflow/task/types';

/**
 * 我的待办
 */
export const listTodo = (query?: FlowTaskQuery): AxiosPromise<FlowTaskVO[]> => {
  return request({
    url: '/workflow/task/todoList',
    method: 'get',
    params: query
  });
};

/**
 * 我的已办
 */
export const listDone = (query?: FlowTaskQuery): AxiosPromise<FlowHisTaskVO[]> => {
  return request({
    url: '/workflow/task/doneList',
    method: 'get',
    params: query
  });
};

/**
 * 全部待办（管理员视角）
 */
export const listAllTodo = (query?: FlowTaskQuery): AxiosPromise<FlowTaskVO[]> => {
  return request({
    url: '/workflow/task/allTodoList',
    method: 'get',
    params: query
  });
};

/**
 * 全部已办（管理员视角）
 */
export const listAllDone = (query?: FlowTaskQuery): AxiosPromise<FlowHisTaskVO[]> => {
  return request({
    url: '/workflow/task/allDoneList',
    method: 'get',
    params: query
  });
};

/**
 * 发起流程
 */
export const startWorkFlow = (data: StartProcessBo): AxiosPromise<StartReturnVO> => {
  return request({
    url: '/workflow/task/startWorkFlow',
    method: 'post',
    data: data
  });
};

/**
 * 办理任务（通过 / 退回）
 */
export const completeTask = (data: CompleteTaskBo) => {
  return request({
    url: '/workflow/task/completeTask',
    method: 'post',
    data: data
  });
};

/**
 * 终止流程
 */
export const terminationTask = (data: TerminationBo) => {
  return request({
    url: '/workflow/task/terminationTask',
    method: 'post',
    data: data
  });
};

/**
 * 撤销流程（发起人撤回）
 */
export const revokeProcess = (data: RevokeBo) => {
  return request({
    url: '/workflow/task/revokeProcess',
    method: 'post',
    data: data
  });
};

/**
 * 转办 / 委派 / 加签 / 减签
 *
 * op：transfer | depute | addSignature | reductionSignature
 */
export const taskOperation = (op: string, data: TaskOperationBo) => {
  return request({
    url: '/workflow/task/taskOperation/' + op,
    method: 'post',
    data: data
  });
};

/**
 * 取回已办任务
 */
export const taskBack = (taskId: string | number) => {
  return request({
    url: '/workflow/task/taskBack/' + taskId,
    method: 'post'
  });
};

/**
 * 实例的办理历史（详情页时间线）
 */
export const flowHisTaskList = (instanceId: string | number): AxiosPromise<FlowHisTaskVO[]> => {
  return request({
    url: '/workflow/task/flowHisTaskList/' + instanceId,
    method: 'get'
  });
};

/**
 * 任务的办理人（用户 id 列表）
 */
export const taskUserList = (taskId: string | number): AxiosPromise<string[]> => {
  return request({
    url: '/workflow/task/taskUserList/' + taskId,
    method: 'get'
  });
};
