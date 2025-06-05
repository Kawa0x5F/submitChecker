import { useState } from "react";
import { invoke } from '@tauri-apps/api/core';

// 文字列を表示する
function App() {
	const [greetMsg, setGreetMsg] = useState<string>();
	const [name, setName] = useState<string>();

	async function greet() {
		setGreetMsg(await invoke("greet", { name }));
	}

	return(
		<div className="container">
			<h1>Welcome Tauri!</h1>

			<form
				className="row"
				onSubmit={(e)=> {
					e.preventDefault();
					greet();
				}}
			>
				<input
					id="greet-input"
					onChange={(e) => setName(e.currentTarget.value)}
					placeholder="Enter a name..."
				 />
				 <button type="submit">Greet</button>
			</form>

			<p>{greetMsg}</p>
	</div>
    )
}

export default App;