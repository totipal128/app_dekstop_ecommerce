<script>
    import { Sidebar } from "flowbite-svelte";
    import { ChartOutline, ShoppingBagSolid, UserSolid, GridSolid, EditSolid } from "flowbite-svelte-icons";
    import { page } from "$app/state";

    // --- sidebar sliding ---
    export let size = 220; // width sidebar
    let isOpen = true;

    const toggleSidebar = () => {
        isOpen = !isOpen;
    };

    $: activeUrl = page.url.pathname;
</script>

<!-- NAVBAR -->
<nav class="fixed top-0 left-0 right-0 h-16 bg-white border-b border-default flex items-center justify-between px-4 z-50">
    <div class="flex items-center gap-4">
        <button
                class="p-2 rounded-lg bg-gray-100 hover:bg-gray-200"
                on:click={toggleSidebar}
        >
            <!-- icon hamburger -->
            <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
        </button>

        <h1 class="text-lg font-semibold">Admin Dashboard</h1>
    </div>

    <div class="flex items-center">
        <!-- user avatar -->
        <div class="w-8 h-8 rounded-full bg-gray-300"></div>
    </div>
</nav>

<!-- SIDEBAR -->
<aside
        class="fixed top-16 h-screen bg-neutral-primary-soft border-e border-default transition-all duration-300 z-40 overflow-y-auto"
        style="width: {size}px; left: {isOpen ? '0px' : `-${size}px`}"
>

    <Sidebar {activeUrl} isSingle={false} class="h-full">
        <Sidebar.Group>
            <Sidebar.Item label="Dashboard">
                {#snippet icon()}
                    <ChartOutline class="h-5 w-5" />
                {/snippet}
            </Sidebar.Item>

            <Sidebar.DropdownWrapper label="Shop">
                {#snippet icon()}
                    <ShoppingBagSolid class="h-5 w-5" />
                {/snippet}
                <Sidebar.Item label="Products" href="/products" />
            </Sidebar.DropdownWrapper>

            <Sidebar.DropdownWrapper label="Profile">
                {#snippet icon()}
                    <UserSolid class="h-5 w-5" />
                {/snippet}
                <Sidebar.Item label="Projects" href="/projects" />
            </Sidebar.DropdownWrapper>

            <Sidebar.Item label="Components" href="/components">
                {#snippet icon()}
                    <GridSolid class="h-5 w-5" />
                {/snippet}
            </Sidebar.Item>
        </Sidebar.Group>

        <Sidebar.Group border>
            <Sidebar.DropdownWrapper label="Settings">
                {#snippet icon()}
                    <EditSolid class="h-5 w-5" />
                {/snippet}
                <Sidebar.Item label="Account" href="/account" />
            </Sidebar.DropdownWrapper>
        </Sidebar.Group>
    </Sidebar>
</aside>

<!-- PAGE CONTENT -->
<div class="pt-20 transition-all duration-300"
     style="margin-left: {isOpen ? `${size}px` : '0px'}">
    <slot />
</div>
