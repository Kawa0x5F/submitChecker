#!/bin/bash

# docker/python-runner イメージをビルド
docker build -t python-runner ./docker/python-runner

# コンテナを起動する例
# 実際のアプリケーションでは、Tauriアプリケーションから動的にコンテナを起動・管理することになります。
# 例えば、以下のように実行コードと入力をマウントしてコンテナを起動します。

# docker run --rm -v /path/to/user_code.py:/app/user_code.py -v /path/to/input.txt:/app/input.txt -v /path/to/output.txt:/app/output.txt python-runner python script_runner.py /app/user_code.py /app/input.txt /app/output.txt

echo "Docker containers launched (example)."