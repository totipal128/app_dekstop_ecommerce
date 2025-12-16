<script>
  import { goto } from "$app/navigation";
  import { user } from "$lib/auth";
  import Table from "$lib/component/table/Table.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Button from "$lib/component/button/Button.svelte";
  import AddModal from "./AddModal.svelte";

  let users = [];
  let loading = true;
  let idx = 0;
  let error = null;
  let user_data = [];

  async function fetchUsers() {
    loading = true;
    try {
      users = await invoke("get_all_users");
      error = null;

      user_data = users.map((u) => [u.id, u.name, u.email, u.username]);
    } catch (err) {
      error = err.message || "Failed to load users";
      console.error("Error fetching users:", err);
    } finally {
      loading = false;
    }
  }

  onMount(fetchUsers);

  let addModalOpen = false;
  function addUser() {
    // Logic to add a new user
    addModalOpen = !addModalOpen;
  }

  function handleEdit(id) {
    console.log(id);
  }
</script>

<Button text="Add User" type="primary" onclick={addUser} />

{#if addModalOpen}
  <AddModal on:toggle={addUser} {fetchUsers} />
{/if}

{#if loading}
  <p>Loading users...</p>
{:else if error}
  <p class="error">Error: {error}</p>
{:else}
  <Table
    headers={["id", "Name", "email", "username", "action"]}
    data={user_data}
    onEdit={handleEdit}
    is_action="true"
  />
{/if}
