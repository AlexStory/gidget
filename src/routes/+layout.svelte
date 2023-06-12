<script lang="ts">
    import Drawer, { AppContent, Content, Header, Title, Subtitle } from '@smui/drawer';
    import NavList from '../lib/NavList.svelte';
</script>

<svelte:head>
    <link rel="stylesheet" href="/smui.css" />
    <link rel="stylesheet" href="/styles.css" />
</svelte:head>

<div class="drawer-container">
    <Drawer>
        <Header>
            <Title>Gidget</Title>
            <Subtitle>Your New Toolbox</Subtitle>
        </Header>
        <Content>
            <NavList />
        </Content>
    </Drawer>

    <AppContent class="app-content">
        <div class="wrapper">
            <slot />
        </div>
    </AppContent>
</div>

<style>
    .drawer-container {
        display: flex;
        height: 100vh;
        width: 100vw;
    }

    .wrapper {
        display: flex;
        width: 100%;
        flex-direction: column;
        align-items: center;
    }

    :global(.app-content) {
        display: flex;
        width: 100%;
    }
</style>