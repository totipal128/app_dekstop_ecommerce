<script lang="ts">
    import { AccordionItem, useMediaQuery, P } from "flowbite-svelte";
    const isMdAndUp = useMediaQuery("(min-width: 200px)");
    let title ="Home"
</script>

<AccordionItem open={isMdAndUp()}>
    {#snippet header()}{title}{/snippet}
    <P>This content is visible on medium screens and larger.</P>
</AccordionItem>