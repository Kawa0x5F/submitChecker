.PHONY: all setup build_docker build_tauri build_frontend dev_tauri dev_frontend clean

all: setup build_docker build_tauri build_frontend

setup:
	@echo "Setting up project..."
	@echo "Installing frontend dependencies..."
	npm install --prefix frontend
	@echo "Installing tauri dependencies (cargo check to ensure everything is set up)..."
	# src-tauri ディレクトリに移動して cargo check を実行
	# または --manifest-path を cargo に渡す
	# ここは既に cargo check --manifest-path src-tauri/Cargo.toml で成功しているので、そのまま
	cargo check --manifest-path src-tauri/Cargo.toml

build_docker:
	@echo "Building Docker images..."
	docker build -t python-runner ./docker/python-runner

build_tauri:
	@echo "Building Tauri application (release)..."
	# src-tauri ディレクトリに移動して cargo tauri build を実行
	cd src-tauri && cargo tauri build

build_frontend:
	@echo "Building frontend application..."
	npm run build --prefix frontend

dev_tauri:
	@echo "Starting Tauri development server..."
	cd src-tauri && cargo tauri dev

dev_frontend:
	@echo "Starting frontend development server..."
	npm run dev --prefix frontend

clean:
	@echo "Cleaning up..."
	rm -rf target/
	rm -rf src-tauri/target/
	rm -rf frontend/node_modules/
	rm -rf node_modules/
	rm -rf frontend/dist/
	docker rmi python-runner || true