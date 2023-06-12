<script lang="ts">
    import TextField from '@smui/textfield';
    import Select, { Option } from '@smui/select';
    import Button from '@smui/button';
    import Snackbar, { Label } from '@smui/snackbar';
    import { save } from '@tauri-apps/api/dialog';
    import { invoke } from '@tauri-apps/api/tauri';

    let files: FileList | null = null;
    let format: string = 'png';
    let snackbar: Snackbar;
    let failSnack: Snackbar;

    async function handleClick() {
        if (!files || files.length === 0) {
            return;
        }

        const buff = await files[0].arrayBuffer();
        const bytes = new Uint8Array(buff);
        const arr = Array.from(bytes);

        const filePath = await save({
            filters: [{
                name: 'Image',
                extensions: [format],
            }]
        });
        try {
            const _success = await invoke('convert_image', {
                image: arr,
                path: filePath,
                format,
            });

            snackbar && snackbar.open();
        } catch {
            failSnack && failSnack.open();
        }
    }
</script>

<h1>Image Converter</h1>
<div class="hide-file-ui">
    <TextField 
        type="file" 
        label="Choose a file" 
        bind:files={files}
        input$accept="image/png, image/jpeg, .ico, .webp"/>

    <Select label="Format" bind:value={format}>
        <Option value="png">PNG</Option>
        <Option value="jpeg">JPEG</Option>
        <Option value="ico">ICO</Option>
    </Select>
</div>
<br />
<Button on:click={() => handleClick()} variant="raised">Convert</Button>
<Snackbar bind:this={snackbar}>
    <Label>Image converted successfully!</Label>
</Snackbar>
<Snackbar bind:this={failSnack}>
    <Label>Image conversion failed!</Label>
</Snackbar>
<style>
    .hide-file-ui :global(input[type='file']::file-selector-button) {
        display: none;
    }
 
    .hide-file-ui
        :global(:not(.mdc-text-field--label-floating) input[type='file']) {
        color: transparent;
    }
</style>