<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let serial = `${new Date().getFullYear() - 2000}000`;
	let name = '';
	let path = 'C:\\UC';
	let message = ' ';
	let isError = false;

	const create = async () => {
		const gitignore = await getGitIgnore();
		const gitattributes = await getGitAttribute();
		await invoke('create_project', { serial, name, path, gitignore, gitattributes })
			.then((msg) => {
				isError = false;
				message = msg;
			})
			.catch((err) => {
				isError = true;
				message = err;
			});
	};

	const getGitIgnore = async () => {
		try {
			const response = await fetch(
				'https://api.github.com/repos/github/gitignore/contents/Unity.gitignore'
			);
			const json = await response.json();
			return atob(json.content);
		} catch (e) {
			return '';
		}
	};

	const getGitAttribute = async () => {
		try {
			const response = await fetch(
				'https://api.github.com/repos/gitattributes/gitattributes/contents/Unity.gitattributes'
			);
			const json = await response.json();
			return atob(json.content);
		} catch (e) {
			return '';
		}
	};
</script>

<div class="fixed w-full h-full dark:bg-slate-800">
	<div class="absolute bottom-0 right-0 p-6 text-gray-400">
		v0.1.5
	</div>
	<div class="container mx-auto mt-16">
		<form class="w-full">
			<div class="flex flex-wrap mx-3 mb-6">
				<div class="w-full sm:w-1/2 px-3 mb-6 sm:mb-0">
					<label
						class="block uppercase tracking-wide text-gray-700 dark:text-gray-300 text-xs font-bold mb-2"
						for="grid-project-serial"
					>
						Project Serial
					</label>
					<input
						class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white"
						id="grid-project-serial"
						type="text"
						placeholder="20000"
						bind:value={serial}
					/>
				</div>
				<div class="w-full sm:w-1/2 px-3">
					<label
						class="block uppercase tracking-wide text-gray-700 dark:text-gray-300 text-xs font-bold mb-2"
						for="grid-project-name"
					>
						Project Name
					</label>
					<input
						class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
						id="grid-project-name"
						type="text"
						placeholder="e.g. The Bold"
						bind:value={name}
					/>
				</div>
			</div>
			<div class="flex flex-wrap mx-3 mb-6">
				<div class="w-full px-3">
					<label
						class="block uppercase tracking-wide text-gray-700 dark:text-gray-300 text-xs font-bold mb-2"
						for="grid-path"
					>
						Path
					</label>
					<input
						class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
						id="grid-path"
						type="text"
						placeholder="e.g. C:\UC"
						bind:value={path}
					/>
					<p class="text-gray-600 dark:text-gray-400 text-xs italic">
						Location of the folder where the project data will be generated.
					</p>
				</div>
			</div>
			<button
				class="flex flex-wrap mx-6 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded"
				type="button"
				on:click={create}
			>
				Create
			</button>
			<p
				class="mx-6 mt-3 text-xs {isError
					? 'text-red-600 dark:text-red-400'
					: 'text-green-600 dark:text-green-400'}"
			>
				{message}
			</p>
		</form>
	</div>
</div>
