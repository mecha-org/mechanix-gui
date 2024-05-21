<script>
	import Layout from "$lib/components/layout.svelte";
	import ListHeading from "$lib/components/list-heading.svelte";
	import Input from "$lib/components/ui/input/input.svelte";

</script>
<Layout title="Set date">
    <div class="w-full grid grid-cols-3 gap-4">
        <div><Input/></div>
        <div><Input/></div>
        <div><Input/></div>
    </div>
</Layout>