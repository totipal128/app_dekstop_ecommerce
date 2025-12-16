<script>
  import { SidebarDropdownItem, SidebarDropdownWrapper } from "flowbite-svelte";
  import {
    ChartOutline,
    ChevronDoubleLeftOutline,
    CogOutline,
    ChevronDoubleRightOutline,
    StoreOutline,
    ClipboardListOutline,
  } from "flowbite-svelte-icons";
  import { page } from "$app/stores";

  let colorHover = "#e3dddd";
  let bgColor = "#ffffff";

  let {
    isOpen = true,
    class: classes = "",
    menu = [
      {
        label: "Dashboard",
        link: "/dashboard",
        icon: '<i class="fi fi-rr-dashboard-panel"></i>',
        is_dropdown: false,
        sub_menu: [
          {
            label: "sub 1",
            link: "",
          },
        ],
      },
      {
        label: "Master Data",
        link: "/master-data",
        icon: '<i class="fi fi-tr-dashboard-panel"></i>',
        is_dropdown: false,
        sub_menu: [
          {
            label: "sub 1",
            link: "",
          },
        ],
      },
      {
        label: "Sales",
        link: "/sale",
        icon: '<i class="fi fi-tr-dashboard-panel"></i>',
        is_dropdown: true,
        sub_menu: [
          {
            label: "Orders",
            link: "/sale/orders",
          },
          {
            label: "Customers",
            link: "/sale/customers",
          },
        ],
      },
      {
        label: "Reports",
        link: "/reports",
        icon: '<i class="fi fi-tr-dashboard-panel"></i>',
        is_dropdown: false,
        sub_menu: [
          {
            label: "sub 1",
            link: "",
          },
        ],
      },
    ],
  } = $props();

  function openSidebar() {
    isOpen = !isOpen;
  }
</script>

<!--<div-->
<!--        class="h-screen duration-200 relative shadow-lg rounded-e-xl pt-13 ${classes}"-->
<!--        style="&#45;&#45;bg-color:{bgColor}"-->
<!--        class:w-60={isOpen}-->
<!--        class:w-20={!isOpen}-->
<!--&gt;-->
<!--    -->
<!--</div>-->

<aside class="mt-9">
  <nav>
    <ul>
      {#each menu as m}
        {#if !m.is_dropdown}
          <li
            class="rounded-lg"
            class:bg-[#bde0fe]={$page.url.pathname === m.link}
          >
            <a
              href={m.link}
              class="items-center flex items-center gap-3 pl-4 hover:bg-[var(--hover-color)]/90 rounded-sm"
              style="--hover-color:{colorHover}"
            >
              <!-- <svelte:component this={m.icon} class="shrink-0 h-6 w-6 group-hover:stroke-white"/> -->
              <div class="pt-2 text-[35px]">
                {@html m.icon}
              </div>
              <p
                class="group-hover:bg-gray-50 transition-opacity"
                class:hidden={!isOpen}
              >
                {m.label}
              </p>
            </a>
          </li>
        {:else}
          <li class="rounded-lg">
            <SidebarDropdownWrapper
              label={isOpen ? m.label : ""}
              class="p-2 hover:bg-[var(--hover-color)]/90"
              style="--hover-color:{colorHover}"
            >
              {#snippet icon()}
                <!-- <svelte:component this={m.icon} class="shrink-0 h-6 w-6 group-hover:stroke-white"/> -->
                <div class="pt-2 text-[35px]">
                  {@html m.icon}
                </div>
              {/snippet}
              {#each m.sub_menu as sm}
                <SidebarDropdownItem label={sm.label} href={sm.link}
                ></SidebarDropdownItem>
              {/each}
            </SidebarDropdownWrapper>
          </li>
        {/if}
      {/each}
    </ul>
  </nav>
</aside>

<style>
  #sidebar-toggle {
    margin-left: 50px;
    padding: 5px;
  }
  li {
    margin: 10px;
  }
</style>
