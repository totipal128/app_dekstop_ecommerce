<script lang="ts">
    import {Avatar, Button, Dropdown, DropdownGroup, DropdownHeader, DropdownItem} from "flowbite-svelte";
    import {BarsFromLeftOutline, BellSolid, EyeSolid} from "flowbite-svelte-icons"
    import {page} from "$app/state";
    import {createEventDispatcher} from 'svelte'
    const dispatch = createEventDispatcher();
    let {isMobile = false} = $props()

    function handleClick(){
        dispatch('toggle');
    }

    let activeUrl = $derived(page.url.pathname);

    $effect.root(()=>{
        console.log(isMobile)
    })

    console.log(isMobile)


</script>

<nav class="fixed top-0 z-50 w-full bg-neutral-primary-soft border-b border-default">
    <div class="px-3 py-3 lg:px-5 lg:pl-3">
        <div class="flex items-center justify-between">
            <div class="flex items-center justify-start rtl:justify-end">
                <button onclick="{handleClick}">
                    <BarsFromLeftOutline/>
                </button>
                <br>
                <a href="/" class="flex ms-2 md:me-24">
                    <img src="https://flowbite.com/docs/images/logo.svg" class="h-6 me-3" alt="FlowBite Logo"/>
                    <span class="self-center text-lg font-semibold whitespace-nowrap dark:text-white">Flowbitee</span>
                </a>
            </div>
            <div class="flex items-center">


                <div id="bell"
                     class="inline-flex items-center text-center text-sm font-medium text-gray-500 hover:text-gray-900 focus:outline-hidden dark:text-gray-400 dark:hover:text-white">
                    <BellSolid class="h-8 w-8"/>
                    <div class="relative flex">
                        <div class="relative end-4 -top-2 inline-flex h-3 w-3 rounded-full border-2 border-white bg-red-500 dark:border-gray-900"></div>
                    </div>
                </div>
                <Dropdown triggeredBy="#bell"
                          class="w-full max-w-sm divide-y divide-gray-100 rounded-sm shadow-sm dark:divide-gray-700 dark:bg-gray-800">
                    <div class="py-2 text-center font-bold">Notifications</div>
                    <DropdownGroup>
                        <DropdownItem class="flex space-x-4 rtl:space-x-reverse">
                            <Avatar src="/images/profile-picture-1.webp" dot={{ color: "bg-gray-300" }}/>
                            <div class="w-full ps-3">
                                <div class="mb-1.5 text-sm text-gray-500 dark:text-gray-400">
                                    New message from <span
                                        class="font-semibold text-gray-900 dark:text-white">Jese Leos</span>
                                    : "Hey, what's up? All set for the presentation?"
                                </div>
                                <div class="text-primary-600 dark:text-primary-500 text-xs">a few moments ago</div>
                            </div>
                        </DropdownItem>
                        <DropdownItem class="flex space-x-4 rtl:space-x-reverse">
                            <Avatar src="/images/profile-picture-2.webp" dot={{ color: "bg-red-400" }}/>
                            <div class="w-full ps-3">
                                <div class="mb-1.5 text-sm text-gray-500 dark:text-gray-400">
                                    <span class="font-semibold text-gray-900 dark:text-white">Joseph Mcfall</span>
                                    and
                                    <span class="font-medium text-gray-900 dark:text-white">5 others</span>
                                    started following you.
                                </div>
                                <div class="text-primary-600 dark:text-primary-500 text-xs">10 minutes ago</div>
                            </div>
                        </DropdownItem>
                        <DropdownItem class="flex space-x-4 rtl:space-x-reverse">
                            <Avatar src="/images/profile-picture-3.webp" dot={{ color: "bg-green-400" }}/>
                            <div class="w-full ps-3">
                                <div class="mb-1.5 text-sm text-gray-500 dark:text-gray-400">
                                    <span class="font-semibold text-gray-900 dark:text-white">Bonnie Green</span>
                                    and
                                    <span class="font-medium text-gray-900 dark:text-white">141 others</span>
                                    love your story. See it and view more stories.
                                </div>
                                <div class="text-primary-600 dark:text-primary-500 text-xs">44 minutes ago</div>
                            </div>
                        </DropdownItem>
                    </DropdownGroup>
                    <a href="/"
                       class="-my-1 block bg-gray-50 py-2 text-center text-sm font-medium text-gray-900 hover:bg-gray-100 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-700">
                        <div class="inline-flex items-center">
                            <EyeSolid class="me-2 h-4 w-4 text-gray-500 dark:text-gray-400"/>
                            View all
                        </div>
                    </a>
                </Dropdown>

                <Button pill color="light" id="avatar_with_name" class="p-1!">
                    <Avatar src="https://raw.githubusercontent.com/themesberg/flowbite/refs/heads/main/static/android-chrome-512x512.png"
                            class="me-2"/>
                    Bonnie Green
                </Button>
                <Dropdown triggeredBy="#avatar_with_name">
                    <DropdownHeader>
                        <span class="block text-sm text-gray-900 dark:text-white">Bonnie asdsad asadasdsad</span>
                        <span class="block truncate text-sm font-medium">name@flowbite.com</span>
                    </DropdownHeader>
                    <DropdownGroup>
                        <DropdownItem>Dashboard</DropdownItem>
                        <DropdownItem>Settings</DropdownItem>
                        <DropdownItem>Earnings</DropdownItem>
                    </DropdownGroup>
                    <DropdownHeader>Sign out</DropdownHeader>
                </Dropdown>

            </div>
        </div>
    </div>
</nav>