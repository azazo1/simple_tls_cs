# 简单 tls 客户端/服务端对

- 使用 rcgen 创建自签名证书(及其密钥对)和服务器证书(及其密钥对).
- 使用 axum 和 axum_server 框架构建 https 服务器(使用服务器证书和服务器私钥), 并使用 rustls(ring) 作为 CryptoProvider.
- 使用 reqwest(rustls 做加密后端) 添加根证书来访问服务.

将自签名证书 `ca_cert.crt` 添加到系统信任的证书中, 然后使用浏览器访问网址, 就不会报不安全连接错误.

## 启动方式

1. 进入项目目录, 执行:

   ```shell
   cargo run --bin pemgen # 生成证书和密钥对
   ```

2. 启动两个终端, 都进入项目目录.
3. 第一个终端中执行:

   ```shell
   cargo run --bin server
   ```

4. 在第二个终端中执行:

   ```shell
   cargo run --bin client
   ```

5. 观察到客户端中返回了 html 页面.
