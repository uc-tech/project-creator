<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let serial = '20000';
	let name = '';
	let path = 'C:\\UC';
	let message = 'N/A';

	const create = async () => {
		message = await invoke('create_project', { serial, name, path });
	};
</script>

<div class="container mx-auto mt-16">
	<form class="w-full">
		<div class="flex flex-wrap mx-3 mb-6">
			<div class="w-full sm:w-1/2 px-3 mb-6 sm:mb-0">
				<label
					class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
					for="grid-project-serial"
				>
					Project Serial
				</label>
				<input
					class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white"
					id="grid-project-serial"
					type="number"
					min="20000"
					max="99999"
					placeholder="20000"
					bind:value={serial}
				/>
			</div>
			<div class="w-full sm:w-1/2 px-3">
				<label
					class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
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
					class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
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
				<p class="text-gray-600 text-xs italic">
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
		<p class="mx-6 mt-3 text-gray-600 text-xs">{message}</p>
	</form>
</div>
