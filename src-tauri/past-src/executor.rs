// src-tauri/src/executor.rs
use std::path::Path;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncReadExt;
use tokio::process::{Child, Command};
use tokio::time::{timeout, Duration};

pub struct ExecutorManager {}

impl ExecutorManager {
    pub fn new() -> Self {
        ExecutorManager {}
    }

    pub async fn run_code_in_container(
        &self,
        code_path: String,
        input_path: String,
        output_file_path: String,
    ) -> Result<String, String> {
        // 出力ファイルが既存のディレクトリでないことを確認し、ファイルを準備
        if Path::new(&output_file_path).is_dir() {
            eprintln!(
                "Warning: Output path {} is a directory. Attempting to remove it.",
                output_file_path
            );
            tokio::fs::remove_dir_all(&output_file_path)
                .await
                .map_err(|e| {
                    format!(
                        "Failed to remove existing directory at {}: {}",
                        output_file_path, e
                    )
                })?;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true) // 常に新規作成または上書きして空にする
            .open(&output_file_path)
            .await
            .map_err(|e| {
                format!(
                    "Failed to create or open output file {}: {}",
                    output_file_path, e
                )
            })?;

        let container_code_path = "/app/user_code.py";
        let container_input_path = "/app/input.txt";
        let container_output_file_path = "/app/output.txt";

        println!("Output file path on host: {}", output_file_path);
        println!(
            "Output file target in container: {}",
            container_output_file_path
        );

        // Dockerコマンドの構築全体を単一の文字列としてシェルに渡す
        let docker_command_str = format!(
            "docker run --rm --network=none --memory=256m --cpus=0.5 \
     -v \"{}\":{} -v \"{}\":{} -v \"{}\":{} \
     python-runner python /app/script_runner.py {} {} {}",
            code_path,
            container_code_path,
            input_path,
            container_input_path,
            output_file_path,
            container_output_file_path, // ここまでが -v オプション
            container_code_path,
            container_input_path,
            container_output_file_path // script_runner.py への引数
        );

        println!("Executing command via sh: {}", docker_command_str);

        let mut command = Command::new("sh"); // <-- sh を起動
        command.arg("-c"); // <-- -c オプションで次の文字列をコマンドとして実行
        command.arg(&docker_command_str); // <-- 構築したDockerコマンド文字列を渡す

        // Child インスタンスを Option でラップして保持
        let mut child_option: Option<Child> = Some(
            command
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to start docker process: {}", e))?,
        );

        // Option::take() で Child の所有権を移動させて Future を取得
        // ここが重要な変更点です。
        let output_future = child_option.take().unwrap().wait_with_output(); // <-- ここを修正

        let output = match timeout(Duration::from_secs(10), output_future).await {
            Ok(Ok(output)) => output,
            Ok(Err(e)) => {
                // エラーが発生した場合、child_option はすでに None なので kill 処理は不要
                // (wait_with_output() が所有権を消費済みのため)
                return Err(format!("Docker command failed: {}", e));
            }
            Err(_) => {
                // タイムアウトした場合
                eprintln!("Docker process timed out after 10 seconds.");
                // タイムアウト時には wait_with_output() は終了していないため、child_option はまだ Some である。
                // ここで kill() を呼ぶためには、child_option から所有権を再度取得する必要がある。
                // ただし、wait_with_output() の Future がキャンセルされた場合、
                // child_option はそのまま Some の状態である。
                // なので、take() で所有権を取得して kill を試みる。
                if let Some(mut c) = child_option.take() {
                    // <-- ここは以前のまま (変更なし)
                    eprintln!("Attempting to kill timed-out child process...");
                    if let Err(kill_err) = c.kill().await {
                        eprintln!("Failed to kill timed-out child process: {}", kill_err);
                    }
                }
                return Err("Execution timed out.".to_string());
            }
        };

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            println!("Attempting to read output from: {}", output_file_path);

            let file_output = match File::open(&output_file_path).await {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).await.map_err(|e| {
                        format!("Failed to read output file {}: {}", output_file_path, e)
                    })?;
                    contents
                }
                Err(e) => {
                    let metadata = tokio::fs::metadata(&output_file_path).await;
                    if let Ok(meta) = metadata {
                        if meta.is_dir() {
                            return Err(format!(
                                "Failed to open output file: {} (path is unexpectedly a directory)",
                                output_file_path
                            ));
                        } else if meta.is_file() {
                            return Err(format!(
                                "Failed to open output file: {} (path is a file, but another error: {})",
                                output_file_path, e
                            ));
                        }
                    }
                    return Err(format!(
                        "Failed to open output file: {}: {}",
                        output_file_path, e
                    ));
                }
            };

            Ok(format!("Execution Output:\n{}\n\nConsole Output (from script_runner.py):\n{}\n\nConsole Errors (from script_runner.py):\n{}", file_output, stdout, stderr))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Err(format!(
                "Docker execution failed with status: {:?}\nStderr: {}",
                output.status, stderr
            ))
        }
    }
}
