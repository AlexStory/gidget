<script lang="ts">
    import TextField from '@smui/textfield';
    import SegmentedButton, { Segment } from '@smui/segmented-button';
    import { Label } from '@smui/common';
    import Button, { Label as BLabel } from '@smui/button';
    import { invoke } from '@tauri-apps/api/tauri';

    let choices = ['Convert To Celcius', 'Convert To Fahrenheit'];
    let temp: string  = "100";
    let selected = "Convert To Celcius";
    let converted: number | null = null;

    async function handleClick() {
        switch (selected) {
            case "Convert To Celcius":
                const cel: number = await invoke("to_celcius", { fahrenheit: parseFloat(temp) });
                converted = cel;
                break;
            case "Convert To Fahrenheit":
                const far: number = await invoke("to_fahrenheit", { celcius: parseFloat(temp) });
                converted = far;
                break;
        }
    }
</script>

<h1>Temperature Converter</h1>
<TextField 
    label="Temperature"
    bind:value={temp}>
</TextField>
<br />
<SegmentedButton segments={choices} let:segment singleSelect bind:selected>
    <Segment {segment}>
        <Label>{segment}</Label>
    </Segment>
</SegmentedButton>
<br />
<Button variant="raised" on:click={() => handleClick()}>
    <BLabel>Convert</BLabel>
</Button>

{#if converted != null}
    <p>Result: {converted}</p>
{/if}
