import request from '@/utils/request';
import { DemoCourseForm, DemoCourseQuery, DemoCourseVO } from './types';
import { AxiosPromise } from 'axios';

// 查询演示课程表列表
export const listDemoCourse = (query: DemoCourseQuery): AxiosPromise<DemoCourseVO[]> => {
  return request({
    url: '/demo/course/list',
    method: 'get',
    params: query
  });
};

// 查询演示课程表详细
export const getDemoCourse = (courseId: string | number): AxiosPromise<DemoCourseVO> => {
  return request({
    url: '/demo/course/' + courseId,
    method: 'get'
  });
};

// 新增演示课程表
export const addDemoCourse = (data: DemoCourseForm) => {
  return request({
    url: '/demo/course',
    method: 'post',
    data: data
  });
};

// 修改演示课程表
export const updateDemoCourse = (data: DemoCourseForm) => {
  return request({
    url: '/demo/course',
    method: 'put',
    data: data
  });
};

// 删除演示课程表
export const delDemoCourse = (courseId: string | number | Array<string | number>) => {
  return request({
    url: '/demo/course/' + courseId,
    method: 'delete'
  });
};
