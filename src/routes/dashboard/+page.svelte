<script>
    import BarChart from '$lib/component/chart/BarChart.svelte'
    import { Breadcrumb, BreadcrumbItem } from "flowbite-svelte";

</script>

<!--<Breadcrumb aria-label="Default breadcrumb example">-->
<!--    <BreadcrumbItem href="/" home>Home</BreadcrumbItem>-->
<!--    <BreadcrumbItem href="/">Projects</BreadcrumbItem>-->
<!--    <BreadcrumbItem>Flowbite Svelte</BreadcrumbItem>-->
<!--</Breadcrumb>-->

<div class="flex flex-wrap justify-around">
    <div class="p-2">
        <BarChart/>
    </div>
    <div class="p-2">
        <BarChart/>
    </div>
    <div class="p-2">
        <BarChart/>
    </div>

</div>