import request from '@/utils/request';
import { AxiosPromise } from 'axios';
import { CategoryVO, CategoryForm, CategoryQuery, CategoryTreeVO } from '@/api/workflow/category/types';

/**
 * 查询流程分类列表（平铺，前端用 handleTree 组树）
 */
export const listCategory = (query?: CategoryQuery): AxiosPromise<CategoryVO[]> => {
  return request({
    url: '/workflow/category/list',
    method: 'get',
    params: query
  });
};

/**
 * 查询流程分类详细
 */
export const getCategory = (categoryId: string | number): AxiosPromise<CategoryVO> => {
  return request({
    url: '/workflow/category/' + categoryId,
    method: 'get'
  });
};

/**
 * 新增流程分类
 */
export const addCategory = (data: CategoryForm) => {
  return request({
    url: '/workflow/category',
    method: 'post',
    data: data
  });
};

/**
 * 修改流程分类
 */
export const updateCategory = (data: CategoryForm) => {
  return request({
    url: '/workflow/category',
    method: 'put',
    data: data
  });
};

/**
 * 删除流程分类
 */
export const delCategory = (categoryId: string | number) => {
  return request({
    url: '/workflow/category/' + categoryId,
    method: 'delete'
  });
};

/**
 * 流程分类树（设计器里的分类下拉，后端已组好树）
 */
export const categoryTree = (): AxiosPromise<CategoryTreeVO[]> => {
  return request({
    url: '/workflow/category/tree',
    method: 'get'
  });
};
