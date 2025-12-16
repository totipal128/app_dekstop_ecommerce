<script lang="ts">
    import { Laptop404NotFoundError, Laptop404NotFoundErrorDark } from "flowbite-svelte-illustrations";
</script>


<div class="flex">

    <!-- Sidebar -->
    <aside class="w-64 h-screen bg-gray-100 p-4 shadow">
        <h2 class="text-lg font-bold mb-4">Menu</h2>
        <ul class="space-y-2">
            <li><a class="block px-3 py-2 hover:bg-gray-200" href="#">Dashboard</a></li>
            <li><a class="block px-3 py-2 hover:bg-gray-200" href="#">Users</a></li>
            <li><a class="block px-3 py-2 hover:bg-gray-200" href="#">Settings</a></li>
        </ul>
    </aside>

    <!-- Main Content Column -->
    <div class="flex-1 flex flex-col">

        <!-- Navbar -->
        <nav class="w-full bg-white shadow px-4 py-3 flex justify-between items-center">
            <div class="text-xl font-semibold">My App</div>
            <ul class="flex gap-6 items-center">
                <li><a href="#" class="hover:text-blue-600">Home</a></li>
                <li><a href="#" class="hover:text-blue-600">Profile</a></li>
                <li><a href="#" class="hover:text-blue-600">Logout</a></li>
            </ul>
        </nav>

        <!-- Content -->
        <main class="p-6">
            <h1 class="text-2xl font-bold">
                <div class="dark:hidden">
                    <Laptop404NotFoundError size="500"/>
                </div>
                <div class="hidden dark:block">
                    <Laptop404NotFoundErrorDark size="500" />
                </div>



            </h1>
        </main>
    </div>

</div>
