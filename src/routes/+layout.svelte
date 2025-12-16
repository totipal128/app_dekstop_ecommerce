<script>
  import "./layout.css";
  import "@tailwindplus/elements";
  import "../app.css";
  import "flowbite";

  import { page } from "$app/stores";

  import { Avatar, Breadcrumb, BreadcrumbItem } from "flowbite-svelte";
  import {
    ChartPieOutline,
    ClipboardListOutline,
    StoreOutline,
  } from "flowbite-svelte-icons";
  import Navbar from "$lib/component/Navbar.svelte";
  import Sidebar from "$lib/component/Sidebar.svelte";
  import { onMount } from "svelte";
  import { loadUser } from "$lib/auth";

  import menu from "$lib/data/list_menu.json";

  // color
  let size = $state(6);
  let bgColorMain = "#F55C47";
  let bgColorSidebar = "#FFFFFF";
  let bgColorFooter = "#FFFFFF";
  let bgColorContent = "#EEEEEE";

  onMount(() => {
    loadUser(); // Load user state from localStorage
  });

  // sidebar open or close
  let isOpen = $state(true);

  function toggleSideBar() {
    isOpen = !isOpen;
  }

  // screen width
  let width = $state(window.innerWidth);
  let isMobile = $state(width < 480);
  $effect.root(() => {
    function handleResize() {
      width = window.innerWidth;

      if (width <= 480) {
        isMobile = true;
      } else if (width <= 650) {
        isOpen = false;
      } else {
        isMobile = false;
      }
      // console.log(width);
    }

    window.addEventListener("resize", handleResize);

    // cleanup saat komponen dilepas
    return () => window.removeEventListener("resize", handleResize);
  });

  //     other
  let broadcramb = [];
</script>

<!-- <Menu listmenu=listmenu /> -->

<Navbar on:toggle={toggleSideBar} {isMobile} />

{#if !isMobile}
  <div
    class="flex w-full h-screen bg-[var(--bg-color-main)]"
    style="(--bg-color-main{bgColorMain}"
  >
    <div
      class="h-screen duration-200 shadow-lg rounded-e-xl pt-13 bg-[var(--bg-color)]"
      style="--bg-color:{bgColorSidebar}"
      class:w-80={isOpen}
      class:w-20={!isOpen}
    >
      <Sidebar {isOpen} class="" {menu} />
    </div>
    <div
      class="w-full h-[90vh] pt-22 pl-5 pr-5 rounded-[10px]"
      style="--bg-color-content:{bgColorContent}"
    >
      <div class="mb-3">
        <Breadcrumb aria-label="Default breadcrumb example">
          <BreadcrumbItem href="/" home>Dashbord <br /></BreadcrumbItem>
          {#each $page.url.pathname.split("/") as url, k}
            {#if url != "" && url != "dashboard"}
              {#if $page.url.pathname.split("/").length - 1 === k}
                <BreadcrumbItem><p class="capitalize">{url}</p></BreadcrumbItem>
              {:else}
                <BreadcrumbItem
                  href={$page.url.pathname
                    .split("/")
                    .slice(0, k + 1)
                    .join("/")}
                  ><p class="capitalize">{url}</p>
                </BreadcrumbItem>
              {/if}
            {/if}
          {/each}
        </Breadcrumb>
      </div>
      <div
        class="w-full h-[100%] overflow-auto rounded-[10px] p-3 bg-[var(--bg-color-content)]"
        style="--bg-color-content:{bgColorContent}"
      >
        <slot />
      </div>
    </div>
    <div
      class="bg-[var(--bg-color-footer)] absolute bottom-0 right-0 h-10 w-full border-t border-black/20 rounded-t-[2px]"
      style="--bg-color-footer:{bgColorFooter}"
    >
      <p class="text-center text-sm pt-2">© 2025 All Rights Reserved.</p>
    </div>
  </div>
{:else}
  <div class="h-full mt-19">
    <div class="p-5 h-full mx-auto break-words mb-17">
      <slot />
    </div>

    <div class="fixed bottom-0 w-full size-16 bg-[#F9F8F6] flex justify-around">
      <div
        class="m-3 rounded-sm"
        class:bg-[#9CC6DB]={$page.url.pathname === "/" ||
          $page.url.pathname === ""}
      >
        <a href="/" class="hover:bg-sky-700">
          <ChartPieOutline class="h-10 w-10 shrink-0" />
        </a>
      </div>

      <div
        class="m-3 rounded-sm"
        class:bg-[#9CC6DB]={$page.url.pathname === "/sale/" ||
          $page.url.pathname === "/sale"}
      >
        <a href="/sale" class="hover:bg-sky-700">
          <StoreOutline class="h-10 w-10 shrink-0" />
        </a>
      </div>

      <div
        class="m-3 rounded-sm"
        class:bg-[#9CC6DB]={$page.url.pathname === "/item/" ||
          $page.url.pathname === "/item"}
      >
        <a href="/item" class="hover:bg-sky-700">
          <ClipboardListOutline class="shrink-0 h-10 w-10" />
        </a>
      </div>

      <div class="m-3 rounded-sm">
        <a href="/user" class="hover:bg-sky-700">
          {#if $page.url.pathname === "/user/" || $page.url.pathname === "/user"}
            <Avatar border class="border-5 border-[#9CC6DB]" />
          {:else}
            <Avatar border />
          {/if}
        </a>
      </div>
    </div>
  </div>
{/if}

<style>
</style>
