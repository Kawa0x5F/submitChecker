import { useState } from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/core';

function App() {
  const [code, setCode] = useState<string>('');
  const [input, setInput] = useState<string>('');
  const [result, setResult] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleRunCode = async () => {
    setLoading(true);
    setResult('');
    setError(null);

    try {
      // Tauri バックエンドの `run_submission` コマンドを呼び出す
      // src-tauri/src/main.rs に `invoke_handler` で登録する必要があります
      const response = await invoke<string>('run_submission', {
        codeContent: code, // ここで送信するキー名はRust側の関数引数名と合わせる
        inputContent: input,
      });
      setResult(response);
    } catch (err: any) {
      console.error('Error running code:', err);
      setError(`Execution failed: ${err.message || String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container">
      <h1>Code Submission Checker</h1>

      <div className="input-section">
        <h2>Your Code</h2>
        <textarea
          className="code-input"
          rows={15}
          value={code}
          onChange={(e) => setCode(e.target.value)}
          placeholder={`Write your Python code here, e.g.:

print("Hello, world!")

a = int(input())
b = int(input())
print(a + b)
`}
        ></textarea>
      </div>

      <div className="input-section">
        <h2>Input for Your Code</h2>
        <textarea
          className="input-data"
          rows={5}
          value={input}
          onChange={(e) => setInput(e.target.value)}
          placeholder={`Enter input data here, e.g.:

10
20
`}
        ></textarea>
      </div>

      <button onClick={handleRunCode} disabled={loading}>
        {loading ? 'Running...' : 'Run Code'}
      </button>

      {error && <div className="error-message">Error: {error}</div>}

      <div className="result-section">
        <h2>Execution Result</h2>
        <pre className="output-display">
          {result || (loading ? 'Waiting for result...' : 'No result yet.')}
        </pre>
      </div>
    </div>
  );
}

export default App;