<script>
	import Layout from "$lib/components/layout.svelte";
	import ListHeading from "$lib/components/list-heading.svelte";
	import Input from "$lib/components/ui/input/input.svelte";

</script>
<Layout title="Set time">
    <ListHeading title="Time set according to the standard time (UST)"/>
    <div class="w-full grid grid-cols-2 gap-4">
        <div><Input/></div>
        <div><Input/></div>
    </div>
</Layout>