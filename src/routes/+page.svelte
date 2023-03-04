<script lang="ts">
	import type { PageData } from "./$types";
	export let data: PageData;

	let slideNumber = 0;

	const slides = data.out.split('<hr />');
	let slide = slides[slideNumber];

	const KEYBINDS: Record<string, string[]> = {
		left: ['KeyA', 'KeyS', 'KeyP', 'KeyH', 'KeyK'],
		right: ['Enter', 'Space', 'KeyW', 'KeyD', 'KeyN', 'KeyJ', 'KeyL']
	}

	function changeSlide(event: KeyboardEvent) {
		if(KEYBINDS.left.includes(event.code)) slideNumber = (slideNumber === 0 ? 0 : slideNumber - 1)
		else if(KEYBINDS.right.includes(event.code)) slideNumber = (slideNumber === (slides.length - 1) ? slideNumber : slideNumber + 1)

		slide = slides[slideNumber];
	}
</script>

<svelte:window on:keypress|preventDefault={changeSlide}/>

{@html slide}