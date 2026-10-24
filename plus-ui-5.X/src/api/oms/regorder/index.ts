import request from '@/utils/request';
import { OmsRegorderForm, OmsRegorderQuery, OmsRegorderVO } from './types';
import { AxiosPromise } from 'axios';

// 查询注册码登记列表
export const listOmsRegorder = (query: OmsRegorderQuery): AxiosPromise<OmsRegorderVO[]> => {
  return request({
    url: '/oms/regorder/list',
    method: 'get',
    params: query
  });
};

// 查询注册码登记详细
export const getOmsRegorder = (regid: string | number): AxiosPromise<OmsRegorderVO> => {
  return request({
    url: '/oms/regorder/' + regid,
    method: 'get'
  });
};

// 新增注册码登记
export const addOmsRegorder = (data: OmsRegorderForm) => {
  return request({
    url: '/oms/regorder',
    method: 'post',
    data: data
  });
};

// 修改注册码登记
export const updateOmsRegorder = (data: OmsRegorderForm) => {
  return request({
    url: '/oms/regorder',
    method: 'put',
    data: data
  });
};

// 删除注册码登记
export const delOmsRegorder = (regid: string | number | Array<string | number>) => {
  return request({
    url: '/oms/regorder/' + regid,
    method: 'delete'
  });
};
