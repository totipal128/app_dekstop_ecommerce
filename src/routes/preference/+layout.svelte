<script>
  import Table from "$lib/component/table/Table.svelte";
  import { page } from "$app/stores";
  let is_active = true;

  let sub_mnu = [
    { label: "Users", link: "/preference/users" },
    { label: "Roles", link: "/preference/roles" },
    { label: "Permissions", link: "/preference/permissions" },
    { label: "Settings", link: "/preference/settings" },
  ];
</script>

<!-- <a href="{$page.url.pathname}/users">asdas</a> -->

<div class="button-sub-menu">
  {#each sub_mnu as item}
    <a
      href={item.link}
      style="--bg-color-sub-menu:{$page.url.pathname === item.link
        ? 'white'
        : ''}"
    >
      {item.label}
    </a>
  {/each}
</div>
<div class="content">
  <slot />
</div>

<style>
  .button-sub-menu {
    margin-top: 20px;
    margin-bottom: 20px;
  }

  .button-sub-menu a {
    background-color: var(--bg-color-sub-menu);
    height: 25px;
    border-radius: 10px;
    padding: 8px 15px;
    padding-bottom: 20px;
    border: none;
    cursor: pointer;
  }

  .button-sub-menu a:hover {
    background-color: white;
  }

  .content {
    background-color: white;
    border-radius: 10px;
    margin-top: -10px;
    padding: 10px;
    width: 100%;
  }
</style>
