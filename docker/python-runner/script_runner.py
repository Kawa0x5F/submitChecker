import sys
import subprocess

def run_code(code_filepath, input_filepath, output_filepath):
    """
    指定されたコードファイルを実行し、入力ファイルの内容を標準入力に渡し、
    標準出力を指定された出力ファイルにリダイレクトします。
    """
    try:
        with open(input_filepath, 'r') as infile:
            process = subprocess.run(
                ['python', code_filepath],  # 実行するコードの言語に合わせて変更 (例: g++ for C++, javac for Java)
                stdin=infile,
                capture_output=True,
                text=True,
                check=True
            )
        with open(output_filepath, 'w') as outfile:
            outfile.write(process.stdout)
        print("Code executed successfully.")
    except subprocess.CalledProcessError as e:
        print(f"Error executing code: {e}")
        print(f"Stderr: {e.stderr}")
    except FileNotFoundError:
        print(f"Error: Code file or input file not found.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

if __name__ == "__main__":
    if len(sys.argv) != 4:
        print("Usage: python script_runner.py <code_filepath> <input_filepath> <output_filepath>")
        sys.exit(1)

    code_file = sys.argv[1]
    input_file = sys.argv[2]
    output_file = sys.argv[3]

    run_code(code_file, input_file, output_file)