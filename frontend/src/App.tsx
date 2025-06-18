import { useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog'
import "./App.css";

// 文字列を表示する
function App() {
	const [folderPath, setFolderPath] = useState<string>();
	const [submitFolders, setSubmitFolders] = useState<string[]>();
    const [errorMessage, setErrorMessage] = useState<string>('');

	async function select_file_path() {
		try{
			const selectedPath = await open({
			multiple: false,
			directory: true,
			})
			
			if (typeof selectedPath === 'string') {
				/* フォルダが選択された時 */
				setFolderPath(selectedPath ?? "");
				setErrorMessage('');
			} else {
				/* フォルダが選択されなかった時 */
				setErrorMessage('フォルダが選択されませんでした');
			}
        } catch (error) {
            // エラーハンドリング
            console.error('フォルダ選択中にエラーが発生しました:', error);
            setErrorMessage('エラーが発生しました。');
        }
	}

	async function find_folders() {
		if(!folderPath){
			setErrorMessage('フォルダを選択してください');
			return;
		}
		try{
			const folder: string[] = await invoke("find_folders", { parentFolderPath: folderPath });
			setSubmitFolders(folder);
			setErrorMessage('');
		} catch(error) {
			setErrorMessage(String(error));
			setSubmitFolders(undefined);
		}
	}

	return(
		<div className="container">
			<h1>Welcome Tauri!</h1>

			<form
				className="row"
				onSubmit={(e)=> {
					e.preventDefault();
					select_file_path();
				}}
			>
				 <button type="submit">Select Folder</button>
			</form>

			<form
				className="row"
				onSubmit={(e)=> {
					e.preventDefault();
					find_folders();
				}}
			>
				 <button type="submit">Run Code</button>
			</form>

            {folderPath && (
                <p style={{ marginTop: '1rem' }}>
                    選択されたフォルダー: <code>{folderPath}</code>
                </p>
            )}

            {submitFolders && (
                <div style={{ marginTop: '1rem' }}>
					<p>フォルダ内:</p>
					<ul>{
						submitFolders.map((folder, index) => (
							<li key={index}>
								<code>{folder}</code>
							</li>
						))
						}
					</ul>
				</div>
            )}

            {errorMessage && (
                <p style={{ marginTop: '1rem', color: 'red' }}>
                    {errorMessage}
                </p>
            )}
		</div>
    )
}

export default App;