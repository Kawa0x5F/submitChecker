// frontend/src/App.tsx
import { useState } from 'react';
import './App.css';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

function App() {
  const [selectedFolders, setSelectedFolders] = useState<string[]>([]);
  const [inputFilePath, setInputFilePath] = useState<string>('');
  const [result, setResult] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const handleSelectFolders = async () => {
    try {
      const selected = await open({
        multiple: true,
        directory: true,
        title: 'Select submission folders (each containing a Python file)',
      });
      if (Array.isArray(selected)) {
        setSelectedFolders(selected);
      } else if (selected) {
        setSelectedFolders([selected]);
      } else {
        setSelectedFolders([]);
      }
    } catch (err) {
      console.error('Error selecting folders:', err);
      setError(`Failed to select folders: ${String(err)}`);
    }
  };

  const handleSelectInputFile = async () => {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [{
          name: 'Text File',
          extensions: ['txt']
        }],
        title: 'Select input.txt file',
      });
      if (typeof selected === 'string') {
        setInputFilePath(selected);
      } else {
        setInputFilePath('');
      }
    } catch (err) {
      console.error('Error selecting input file:', err);
      setError(`Failed to select input file: ${String(err)}`);
    }
  };

  const handleRunSubmissions = async () => {
    if (selectedFolders.length === 0) {
      setError('Please select at least one folder.');
      return;
    }
    if (!inputFilePath) {
      setError('Please select an input.txt file.');
      return;
    }

    setLoading(true);
    setResult('');
    setError(null);

    try {
      const response = await invoke<string>('run_multiple_submissions', {
        folderPaths: selectedFolders,
        inputFilePath: inputFilePath,
      });
      setResult(response);
    } catch (err: any) {
      console.error('Error running submissions:', err);
      setError(`Execution failed: ${err.message || String(err)}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container">
      <h1>Code Submission Checker</h1>

      <div className="input-section">
        <h2>Submission Folders (each with a Python file)</h2>
        <button onClick={handleSelectFolders} disabled={loading}>
          {loading ? 'Processing...' : 'Select Folders'}
        </button>
        {selectedFolders.length > 0 && (
          <div className="selected-paths">
            <p>Selected:</p>
            <ul>
              {selectedFolders.map((path, index) => (
                <li key={index}>{path}</li>
              ))}
            </ul>
          </div>
        )}
      </div>

      <div className="input-section">
        <h2>Input File (input.txt)</h2>
        <button onClick={handleSelectInputFile} disabled={loading}>
          {loading ? 'Processing...' : 'Select input.txt'}
        </button>
        {inputFilePath && (
          <p className="selected-paths">Selected: {inputFilePath}</p>
        )}
      </div>

      <button onClick={handleRunSubmissions} disabled={loading}>
        {loading ? 'Running...' : 'Run Submissions'}
      </button>

      {error && <div className="error-message">Error: {error}</div>}

      <div className="result-section">
        <h2>Overall Results</h2>
        <pre className="output-display">
          {result || (loading ? 'Waiting for results...' : 'No results yet.')}
        </pre>
      </div>
    </div>
  );
}

export default App;