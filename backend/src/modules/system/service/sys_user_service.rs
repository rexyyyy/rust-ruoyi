use std::time::Duration;

use actix_web::HttpRequest;
use uuid::Uuid;

use crate::config::cache_variables::{REDIS_UUID_CAPTCHA, LOGIN_TOKEN_KEY};
use crate::config::global_variables::{ADMIN_NAME, ALL_PERMISSIONS, LOGIN_FAIL, LOGIN_SUC, STATUS_FORBIDDEN};
use crate::modules::system::domain::bo::user::{UserPageDTO, UserUpdateDTO};
use crate::modules::system::domain::bo::sign_in::SignInDTO;
use crate::modules::system::domain::{
    Page, PageRequest, SysUser, SysUserActiveModel, SysUserColumn, SysUserEntity, SysUserRoleColumn,
    SysUserRoleEntity, SysDeptColumn, SysDeptEntity,
};
use crate::modules::system::domain::vo::{JWTToken, UserCache};
use crate::modules::system::domain::vo::user::SysUserVO;
use crate::error::Error;
use crate::error::Result;
use crate::pool;
use crate::modules::system::service::CONTEXT;
use crate::token_auth::get_token;
use crate::util::password_encoder::PasswordEncoder;

use sea_orm::entity::*;
use sea_orm::query::*;
// ConnectionTrait / TransactionTrait：把复合写包进单个事务，保证原子性
use sea_orm::{ConnectionTrait, TransactionTrait};
use chrono::Local;
use crate::modules::system::dao::*;

const REDIS_KEY_RETRY: &'static str = "login:login_retry";

///Background User Service
pub struct SysUserService {}

impl SysUserService {
    pub async fn page(&self, arg: &UserPageDTO) -> Result<Page<SysUserVO>> {
        let page_req = &PageRequest::from(arg);
        let mut select = SysUserEntity::find();
        if let Some(user_name) = &arg.user_name {
            if !user_name.is_empty() {
                select = select.filter(SysUserColumn::UserName.contains(user_name.as_str()));
            }
        }
        if let Some(phonenumber) = &arg.phonenumber {
            if !phonenumber.is_empty() {
                select = select.filter(SysUserColumn::Phonenumber.contains(phonenumber.as_str()));
            }
        }
        if let Some(status) = &arg.status {
            if !status.is_empty() {
                select = select.filter(SysUserColumn::Status.eq(status.clone()));
            }
        }
        if let Some(dept_id) = &arg.dept_id {
            if !dept_id.is_empty() {
                // (dept_id = $1 OR dept_id IN (SELECT dept_id FROM sys_dept WHERE find_in_set($1, ancestors)))
                let sub = SysDeptEntity::find()
                    .filter(
                        Condition::any()
                            .add(SysDeptColumn::DeptId.eq(dept_id.parse::<i64>().unwrap_or(0)))
                            .add(SysDeptColumn::Ancestors.eq(dept_id.clone()))
                            .add(SysDeptColumn::Ancestors.like(format!("{},%", dept_id).as_str()))
                            .add(SysDeptColumn::Ancestors.like(format!("%,{},%", dept_id).as_str()))
                            .add(SysDeptColumn::Ancestors.like(format!("%,{}", dept_id).as_str())),
                    )
                    .select_only()
                    .column(SysDeptColumn::DeptId)
                    .into_query();
                select = select.filter(
                    Condition::any()
                        .add(SysUserColumn::DeptId.eq(dept_id.parse::<i64>().unwrap_or(0)))
                        .add(SysUserColumn::DeptId.in_subquery(sub)),
                );
            }
        }
        select = select.filter(SysUserColumn::DelFlag.eq("0"));
        select = select.order_by_asc(SysUserColumn::CreateTime);
        let page = paginate(pool!(), select, page_req).await?;
        Ok(page.convert::<SysUserVO>())
    }

    ///user details
    pub async fn detail(&self, user_id: &String) -> Result<SysUserVO> {
        let user = self
            .find(&user_id)
            .await?
            .ok_or_else(|| Error::from(format!("用户:{:?} 不存在！", user_id)))?;
        let user_vo = SysUserVO::from(user);
        //  let all_menus = CONTEXT.sys_menu_service.finds_all_map().await?;
        return Ok(user_vo);
    }

    /// 按主键查单个用户。
    ///
    /// 用 `one()` 而非 `all().into_iter().next()`：前者给 SQL 加 `LIMIT 1`，
    /// 由数据库侧截断结果集，也省去中间的 Vec 分配。
    pub async fn find(&self, user_id: &str) -> Result<Option<SysUser>> {
        Ok(SysUserEntity::find()
            .filter(SysUserColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
            .one(pool!())
            .await?)
    }

    /// 按账号查单个用户（登录路径高频）。同 [`Self::find`]，走 `LIMIT 1`。
    pub async fn find_by_account(&self, user_name: &str) -> Result<Option<SysUser>> {
        Ok(SysUserEntity::find()
            .filter(SysUserColumn::UserName.eq(user_name.to_string()))
            .one(pool!())
            .await?)
    }

    pub async fn add(&self, mut user: SysUser) -> Result<u64> {
        if user.user_name.is_none() || user.user_name.as_ref().unwrap().is_empty() {
            return Err(Error::from("用户名id不能为空!"));
        }
        let old_user = self
            .find_by_account(user.user_name.as_ref().unwrap())
            .await?;
        if old_user.is_some() {
            return Err(Error::from(format!(
                "用户账户:{}已存在!",
                user.user_name.as_ref().unwrap()
            )));
        }
        let mut password = user.password.as_deref().unwrap_or_default().to_string();
        if password.is_empty() {
            //默认密码
            password = "AsAPig!".to_string();
        }
        user.password = Some(password);

        Ok(insert_model(pool!(), user).await?)
    }

    /// 新增用户 + 写角色关联（事务版）。
    ///
    /// 旧实现由 controller 先 `add` 再 `add_user_roles`，且调用处漏了 `.await`，
    /// future 从未被轮询——新用户/注册用户实际一个角色都没有（菜单为空）。
    /// 这里把两步放进同一事务：用户与角色要么都写入，要么都不写入。
    pub async fn add_with_roles(&self, user: SysUser, role_ids: &[String]) -> Result<u64> {
        let txn = pool!().begin().await.map_err(Error::from)?;
        let user_id = user.user_id.to_string();
        let r = insert_model(&txn, user).await?;
        if r > 0 && !role_ids.is_empty() {
            CONTEXT
                .sys_user_role_service
                .add_user_roles_in_txn(&txn, &user_id, role_ids)
                .await?;
        }
        txn.commit().await.map_err(Error::from)?;
        Ok(r)
    }

    //返回token
    pub async fn login(&self, arg: &SignInDTO, req: &HttpRequest) -> Result<String> {
        let start = Local::now().timestamp_millis();
        // 失败计数按 账号 + IP 维度，避免全局单键导致任一账号失败即锁死全站登录
        let retry_key = Self::login_retry_key(&arg.username, req);
        self.is_need_wait_login_ex(&retry_key).await?;
        if CONTEXT.config.captcha_enabled {
            if arg.code.is_none() {
                return Err(Error::from("请输入验证码！"));
            }
            let code = arg.code.as_deref().unwrap();
            // captcha crate 实际可能生成 3~4 位字符，长度不硬编码，安全性由 Redis 值比对保证
            if code.is_empty() || code.len() > 8 {
                return Err(Error::from("验证码输入不正确！"));
            }
            let uuid = arg.uuid.as_deref().unwrap_or_default();
            let code_in_cache = CONTEXT
                .cache_service.get_string(&format!("{}{}", REDIS_UUID_CAPTCHA, &uuid)).await.unwrap_or_default();
            if code_in_cache != code { return Err(Error::from("验证码输入不正确！")); }
        }
       // println!("验证码{}",Local::now().timestamp_millis()-start);
        let user: Option<SysUser> = SysUserEntity::find()
            .filter(SysUserColumn::UserName.eq(arg.username.to_string()))
            .all(pool!())
            .await?
            .into_iter()
            .next();
        if user.is_none() { return Err(Error::from(format!("账号:{} 不存在!", arg.username))); }
     //   println!("查找是否有这个账号{}",Local::now().timestamp_millis()-start);
        let user = user.unwrap();
        if user.status.clone() == Some(STATUS_FORBIDDEN.to_string()) {
            return Err(Error::from("账户被禁用!"));
        }
        let mut error = None;

        // check pwd（bcrypt 是 200ms+ 的纯 CPU 操作，下沉到阻塞线程池，避免阻塞 worker）
        if !PasswordEncoder::verify_async(
            user.password
                .as_ref()
                .ok_or_else(|| Error::from("错误的用户数据，密码为空!"))?,
            &arg.password,
        )
        .await
        {
            error = Some(Error::from("密码不正确!"));
        }
        //todo 加密时间过长，需要换一个，初步定为 https://github.com/RustCrypto/hashes
     //   println!("密码验证{}",Local::now().timestamp_millis()-start);

        if error.is_some() {
            CONTEXT.sys_logininfor_service.add_async(&crate::util::web_utils::build_logininfor(req, arg.username.clone(), LOGIN_FAIL, error.clone().unwrap().to_string())).await;
            self.add_retry_login_limit_num(&retry_key).await?;
            return Err(error.unwrap());
        }
     //   println!("密码验证后{}",Local::now().timestamp_millis()-start);

        // 登录成功：清除该账号+IP 的失败计数
        let _ = CONTEXT.cache_service.del(&retry_key).await;
        let token = self.get_user_info(&user).await;
      //  println!("Token{}",Local::now().timestamp_millis()-start);
        CONTEXT.sys_logininfor_service.add_async(&crate::util::web_utils::build_logininfor(req, arg.username.clone(), LOGIN_SUC, "成功".to_string())).await;
     //   println!("写入日志{}",Local::now().timestamp_millis()-start);
        return token;
    }

    /// 登录失败计数键：按 账号 + 客户端 IP 隔离
    fn login_retry_key(username: &str, req: &HttpRequest) -> String {
        let ip = crate::util::web_utils::get_ip_addr(req);
        format!("{}:{}:{}", REDIS_KEY_RETRY, username, ip)
    }

    ///is need to wait
    pub async fn is_need_wait_login_ex(&self, key: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(key).await?;
            if num.unwrap_or(0) >= CONTEXT.config.login_fail_retry {
                let wait_sec: i64 = CONTEXT.cache_service.ttl(key).await?;
                if wait_sec > 0 {
                    return Err(Error::from(format!(
                        "操作过于频繁，请等待{}秒后重试!",
                        wait_sec
                    )));
                }
            }
        }
        return Ok(());
    }

    ///Add redis retry record
    pub async fn add_retry_login_limit_num(&self, key: &str) -> Result<()> {
        if CONTEXT.config.login_fail_retry > 0 {
            let num: Option<u64> = CONTEXT.cache_service.get_json(key).await?;
            let num = num.unwrap_or(0).saturating_add(1);
            CONTEXT
                .cache_service
                .set_string_ex(
                    key,
                    &num.to_string(),
                    Some(Duration::from_secs(
                        CONTEXT.config.login_fail_retry_wait_sec as u64,
                    )),
                )
                .await?;
        }
        return Ok(());
    }

    pub async fn get_user_info_by_token(&self, user_cache: &UserCache) -> Result<String> {
        let user = SysUserEntity::find()
            .filter(SysUserColumn::UserId.eq(user_cache.id.clone()))
            .all(pool!())
            .await?
            .into_iter()
            .next();
        let user = user.ok_or_else(|| Error::from(format!("账号:{} 不存在!", user_cache.user_name)))?;
        return self.get_user_info(&user).await;
    }
    //返回token
    pub async fn get_user_info(&self, user: &SysUser) -> Result<String> {
        //去除密码，增加安全性
        let mut user = user.clone();
        user.password = None;
        let user_id = user.user_id.clone();


        //提前查找所有权限，避免在各个函数方法中重复查找
        let all_menus = CONTEXT.sys_menu_service.finds_all_map().await?;

        let uuid = Uuid::new_v4();

        let user_roles = SysUserRoleEntity::find()
            .filter(SysUserRoleColumn::UserId.eq(user_id))
            .all(pool!())
            .await?;
        let role_ids: Vec<i64> = user_roles.iter().map(|x| x.role_id).collect();
        let role_menu = CONTEXT.sys_role_service
            .find_role_menu(&role_ids)
            .await?;
        let menu_ids: Vec<i64> = role_menu.iter().map(|x| x.menu_id).collect();
        let menus = CONTEXT
            .sys_menu_service.finds_menu(&menu_ids, &all_menus);
        let permissions: Vec<String> = if user.user_name.as_ref().unwrap().to_string() == ADMIN_NAME {
            vec![ALL_PERMISSIONS.to_string()]
        } else {
            menus.iter().filter_map(|x| x.perms.clone()).collect()
        };


        let user_cache = UserCache {
            id: user_id.clone(),
            user_name: user.user_name.as_ref().unwrap().to_string(),
            user: Some(user.clone().into()),
            permissions,
            menu_ids,
            roles: CONTEXT.sys_user_role_service.find_roles_by_user_id(
                user.user_id,
                &all_menus,
            ).await?.unwrap(),
            login_time: chrono::Local::now().naive_local(),


            token_key: format!("{}{}", LOGIN_TOKEN_KEY, &uuid.to_string()),
        };
        let jwt_token = JWTToken {
            login_user_key: uuid.to_string(),
            // JWT `exp` 按「秒」校验（见 jwt.rs 的 Validation::default），
            // 必须为 当前秒 + 有效期，否则会被判为已过期（或永不过期）。
            exp: (chrono::Local::now().timestamp()
                + (CONTEXT.config.token_expired_min * 60) as i64) as usize,
        };
        let access_token = jwt_token.create_token(&CONTEXT.config.jwt_secret)?;

        CONTEXT.cache_service.set_string_ex(
            &user_cache.token_key,
            &user_cache.to_string(),
            Some(Duration::from_secs(
                CONTEXT.config.token_expired_min * 60
            )),
        ).await;
        return Ok(access_token);
    }

    pub async fn update(&self, arg: UserUpdateDTO) -> Result<u64> {
        let role_ids = arg.role_ids.clone();
        let user = SysUser::from(arg);
        //do not update user_name
        let userid = user.user_id.clone();

        // 「重置用户角色 + 更新用户资料」必须原子：旧实现先删插角色、再改用户，
        // 若改用户失败，角色已经变了（反之亦然），前端看到的却是失败提示。
        let txn = pool!().begin().await.map_err(Error::from)?;

        if let Some(ids) = role_ids {
            CONTEXT
                .sys_user_role_service
                .reset_through_user_id_in_txn(&txn, &userid.to_string(), &ids)
                .await?;
        }
        // 仅更新 DTO 中提供的字段（None 表示不更新，保持原值），等价于 rbatis update_by_map 的跳过 null 语义。
        let mut am = SysUserActiveModel {
            user_id: Set(userid.clone()),
            ..Default::default()
        };
        if let Some(v) = user.dept_id { am.dept_id = Set(Some(v)); }
        if let Some(v) = user.nick_name { am.nick_name = Set(Some(v)); }
        if let Some(v) = user.email { am.email = Set(Some(v)); }
        if let Some(v) = user.phonenumber { am.phonenumber = Set(Some(v)); }
        if let Some(v) = user.sex { am.sex = Set(Some(v)); }
        if let Some(v) = user.avatar { am.avatar = Set(Some(v)); }
        if let Some(v) = user.password { am.password = Set(Some(v)); }
        if let Some(v) = user.status { am.status = Set(Some(v)); }
        if let Some(v) = user.del_flag { am.del_flag = Set(Some(v)); }
        if let Some(v) = user.login_ip { am.login_ip = Set(Some(v)); }
        if let Some(v) = user.login_date { am.login_date = Set(Some(v)); }
        if let Some(v) = user.create_by { am.create_by = Set(Some(v)); }
        if let Some(v) = user.create_time { am.create_time = Set(Some(v)); }
        if let Some(v) = user.update_by { am.update_by = Set(Some(v)); }
        if let Some(v) = user.update_time { am.update_time = Set(Some(v)); }
        if let Some(v) = user.remark { am.remark = Set(Some(v)); }
        // user_name 保持 NotSet：不参与本次更新
        let r = SysUserEntity::update_many()
            .set(am)
            .filter(SysUserColumn::UserId.eq(userid))
            .exec(&txn)
            .await?;
        txn.commit().await.map_err(Error::from)?;
        Ok(r.rows_affected)
    }

    pub async fn remove(&self, id: &str) -> Result<u64> {
        if id.is_empty() {
            return Err(Error::from("id 不能为空！"));
        }

        // user_id 在库中为 bigint，按字符串比较会导致 PostgreSQL 类型不匹配而删不掉
        let user_id_i64 = id.parse::<i64>().unwrap_or(0);

        // 「删除用户 + 删除角色关联 + 写回收站」是一个原子操作，必须包在同一事务里：
        // 否则中途失败会留下孤儿角色关联行，或删除成功却丢了回收站快照。
        // 任一步 `?` 提前返回时，`txn` 被 drop 会自动 rollback。
        let txn = pool!().begin().await.map_err(Error::from)?;

        // 快照必须在事务内读取，保证与后续删除读到一致的数据
        let trash = SysUserEntity::find()
            .filter(SysUserColumn::UserId.eq(user_id_i64))
            .all(&txn)
            .await?;
        let r = SysUserEntity::delete_many()
            .filter(SysUserColumn::UserId.eq(user_id_i64))
            .exec(&txn)
            .await?;
        CONTEXT
            .sys_trash_service
            .add_in_txn(&txn, "sys_user", &trash)
            .await?;
        CONTEXT
            .sys_user_role_service
            .remove_by_user_id_in_txn(&txn, id)
            .await?;

        txn.commit().await.map_err(Error::from)?;
        return Ok(r.rows_affected);
    }

    ///Find user-authority hierarchy permissions
    // pub async fn load_level_permission(
    //     &self,
    //     user_id: &str,
    //     all_menus: &BTreeMap<String, SysMenuVO>,
    // ) -> Result<Vec<String>> {
    //     return CONTEXT
    //         .sys_role_service
    //         .find_user_permission(user_id, all_menus)
    //         .await;
    // }
    pub async fn get_user_cache(&self, req: &HttpRequest) -> Result<UserCache> {
        // 命中增量：Auth 中间件鉴权成功时已把 UserCache 注入请求作用域，
        // 直接复用，避免每个请求重复 JWT 校验 + 一次 Redis GET。
        if let Some(c) = actix_web::HttpMessage::extensions(req).get::<UserCache>() {
            return Ok(c.clone());
        }
        let token = get_token(req);
        self.get_user_cache_by_token(&token).await
    }
    pub async fn get_user_cache_by_token(&self, token: &str) -> Result<UserCache> {
        let token = JWTToken::verify(&CONTEXT.config.jwt_secret, &token);
        if token.is_err() {
            return Err(Error::from("Token失败，请重新！"));
        }
        CONTEXT.cache_service.get_json::<UserCache>(&format!("{}{}", LOGIN_TOKEN_KEY, &token.unwrap().login_user_key)).await
    }

    pub async fn update_password(&self, new_password: &str, user_id: &str) -> Result<u64> {
        let res = pool!()
            .execute(crate::db::raw_stmt(
                                "update sys_user set password = $1 where user_id = $2",
                vec![Value::from(new_password), Value::from(user_id)],
            ))
            .await?;
        Ok(res.rows_affected())
    }

    pub async fn update_status(&self, status: &str, user_id: &str) -> Result<u64> {
        let res = pool!()
            .execute(crate::db::raw_stmt(
                                "update sys_user set status = $1 where user_id = $2",
                vec![Value::from(status.to_string()), Value::from(user_id)],
            ))
            .await?;
        Ok(res.rows_affected())
    }
}

impl SysUserService {
    /// 查询用户已分配的岗位ID（对应前端 userForm.postIds）
    pub async fn finds_post_ids_by_user_id(&self, user_id: &str) -> Result<Vec<String>> {
        let rows = pool!()
            .query_all(crate::db::raw_stmt(
                                "select post_id from sys_user_post where user_id = $1",
                vec![Value::from(user_id.parse::<i64>().unwrap_or(0))],
            ))
            .await?;
        let mut ids = vec![];
        for r in rows.iter() {
            if let Ok(v) = r.try_get::<i64>("", "post_id") {
                ids.push(v.to_string());
            }
        }
        Ok(ids)
    }

    /// 查询用户已分配的岗位名称（对应前端 profile 的 postGroup）
    pub async fn finds_post_names_by_user_id(&self, user_id: &str) -> Result<Vec<String>> {
        let rows = pool!()
            .query_all(crate::db::raw_stmt(
                                "select p.post_name from sys_user_post up \
                 left join sys_post p on p.post_id = up.post_id \
                 where up.user_id = $1 order by p.post_sort",
                vec![Value::from(user_id.parse::<i64>().unwrap_or(0))],
            ))
            .await?;
        let mut names = vec![];
        for r in rows.iter() {
            if let Ok(v) = r.try_get::<Option<String>>("", "post_name") {
                if let Some(v) = v {
                    names.push(v);
                }
            }
        }
        Ok(names)
    }

    /// 根据用户ID列表查询用户（对应Spring /system/user/optionselect）
    pub async fn finds_by_ids(&self, user_ids: &Vec<String>) -> Result<Vec<SysUserVO>> {
        if user_ids.is_empty() {
            return Ok(vec![]);
        }
        let placeholders = (1..=user_ids.len())
            .map(|i| format!("${}", i))
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!("select * from sys_user where user_id in ({})", placeholders);
        // user_id 是 bigint：必须用 i64 绑定，传字符串会触发
        // "操作符不存在: bigint = text"，使 UserSelect 组件带 ids 查询 500。
        let values: Vec<Value> = user_ids
            .iter()
            .map(|s| Value::from(s.parse::<i64>().unwrap_or(0)))
            .collect();
        let rows = pool!()
            .query_all(crate::db::raw_stmt(&sql, values))
            .await?;
        let data: Vec<SysUser> = rows
            .iter()
            .filter_map(|r| SysUser::from_query_result(r, "").ok())
            .collect();
        Ok(data.into_iter().map(SysUserVO::from).collect())
    }

    /// 根据部门ID查询用户（对应Spring /system/user/list/dept/{deptId}）
    pub async fn finds_by_dept_id(&self, dept_id: &str) -> Result<Vec<SysUserVO>> {
        let data = SysUserEntity::find()
            .filter(SysUserColumn::DeptId.eq(dept_id.parse::<i64>().unwrap_or(0)))
            .all(pool!())
            .await?;
        Ok(data.into_iter().map(SysUserVO::from).collect())
    }

    /// 更新用户头像
    pub async fn update_avatar(&self, user_id: &str, avatar: &str) -> Result<u64> {
        let am = SysUserActiveModel {
            user_id: Set(user_id.parse::<i64>().unwrap_or(0)),
            avatar: Set(avatar.parse::<i64>().ok()),
            update_time: Set(Some(chrono::Local::now().naive_local())),
            ..Default::default()
        };
        let r = SysUserEntity::update_many()
            .set(am)
            .filter(SysUserColumn::UserId.eq(user_id.parse::<i64>().unwrap_or(0)))
            .exec(pool!())
            .await?;
        Ok(r.rows_affected)
    }
}
