# 系统邮件服务

澜灵账号邮件统一使用 `lanmind@lansuan.cc` 作为发件地址。

SMTP 密码只能配置在未来的 `apps/cloud-api` 服务端部署环境中，禁止写入桌面应用、前端代码、Git 历史或公开日志。

## 服务端环境变量

```text
SMTP_HOST=smtp.exmail.qq.com
SMTP_PORT=465
SMTP_SECURE=true
SMTP_USERNAME=lanmind@lansuan.cc
SMTP_PASSWORD=<deployment secret>
SMTP_FROM=澜灵 <lanmind@lansuan.cc>
```

桌面端只调用云端的注册、登录和验证码 API，不直接连接 SMTP。
