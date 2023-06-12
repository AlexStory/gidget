<script lang="ts">
    import List, { Item, Text, Subheader } from '@smui/list';
    import { goto } from '$app/navigation';
    import { page } from '$app/stores';
</script>

<div>
    <List>
        <Subheader tag="h6">Converters</Subheader>
        <Item href="/temp"
            activated={$page.url.pathname === '/temp'}>
            <Text>Temperature</Text>
        </Item>
        <Item href="/image-converter"
            activated={$page.url.pathname === '/image-converter'}>
            <Text>Image Converter</Text>
        </Item>
    </List>
</div>