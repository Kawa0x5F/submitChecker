# submitChecker

DockerコンテナでPythonコードを実行し、その結果をRustのGUIツール、Tauri上で表示する。

# Usage

Dockerコンテナのビルド
```bash
make docker_build
```

フロントエンドの立ち上げ
```bash
make dev_frontend
```

tauriのビルド
```bash
make setup
```

tauriの立ち上げ
```bash
make dev_tauri
```