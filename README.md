## **Ad-player**

-----

### **项目简介**

用于学习的 **Tauri/Rust/Vue** 项目，使用 **Vue 3** 构建的音乐播放器。

### **项目进度**

#### 1.0-本地的音乐播放器
1. [ ] 播放指定路径的声音文件
2. [ ] 按需引入 ElementUI 中的组件
3. [ ] 持久化文件路径
4. [ ] 在多平台上实现项目构建


### **项目设置**

#### **1. 环境要求**

在运行项目之前，请确保你的系统已安装以下环境：

  * **Node.js**: v22.15.1
  * **Rust**: rustc 1.87.0 (17067e9ac 2025-05-09)
  * **Tauri Prerequisites**: 访问 [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) 页面，安装你操作系统所需的依赖。

#### **2. 安装依赖**

```bash
npm install
```

#### **3. 运行项目**

```bash
npm run tauri dev
```

#### **4. 构建发布版**

```bash
# 构建相关内容待更新
```
-----

### **项目结构**

```
tauri-music-player/
├── src-tauri/             # Tauri 项目后端（Rust）
│   ├── Cargo.toml         # Rust 依赖和配置
│   └── src/main.rs        # 主要的 Rust 代码
├── src/                   # Vue 前端代码
│   ├── components/        # Vue 组件
│   └── App.vue            # 主要的 Vue 根组件
├── public/                # 静态资源
├── vite.config.js         # Vite 配置
└── package.json           # 前端依赖和脚本
```

-----

### **贡献指南**

如果有任何 bug 或者 学习记录上的意见或建议，请提交 **Issue** 告诉我。
