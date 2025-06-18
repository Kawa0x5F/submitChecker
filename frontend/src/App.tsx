import { useState } from "react";
// import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog'
import "./App.css";

// 文字列を表示する
function App() {
	const [folderPath, setFolderPath] = useState<string>();
    const [errorMessage, setErrorMessage] = useState<string>('');

	async function select_file_paht() {
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

	return(
		<div className="container">
			<h1>Welcome Tauri!</h1>

			<form
				className="row"
				onSubmit={(e)=> {
					e.preventDefault();
					select_file_paht();
				}}
			>
				 <button type="submit">Select Folder</button>
			</form>

            {folderPath && (
                <p style={{ marginTop: '1rem' }}>
                    選択されたフォルダー: <code>{folderPath}</code>
                </p>
            )}

            {errorMessage && (
                <p style={{ marginTop: '1rem', color: 'red' }}>
                    {errorMessage}
                </p>
            )}	</div>
    )
}

export default App;