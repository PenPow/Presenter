import type { PageLoad } from "./$types";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';
import { readTextFile } from '@tauri-apps/api/fs';

export const load: PageLoad = async () => {
	const path = await open({ multiple: false, title: 'Select Presentation', filters: [{ name: 'Presentation', extensions: ['md'] }] }) as string

	const md = await readTextFile(path);

	return { out: invoke<string>("parse", { md }) }; 
};