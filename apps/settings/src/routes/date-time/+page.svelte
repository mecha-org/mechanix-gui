<script>
	import Icons from "$lib/components/icons.svelte";
import Layout from "$lib/components/layout.svelte";
	import ListItem from "$lib/components/list-item.svelte";
	import Switch from "$lib/components/ui/switch/switch.svelte";

</script>
<Layout title="Date & time">
    <div class="flex flex-col gap-4">
        <ListItem title="Enable Automatic Time" isLink>
            <Switch/>
        </ListItem>
        <ListItem title="Set time" isLink href="/date-time/set-time">
           <div class="flex flex-row gap-2 items-center">
                <p class="text-lg text-misty-slate">UST</p>
                <Icons name="right_arrow" height="30px" width="30px"/>
           </div>
        </ListItem>
        <ListItem title="Set date" isLink href="/date-time/set-date">
           <div class="flex flex-row gap-2 items-center">
                <p class="text-lg text-misty-slate">January 1,2024</p>
                <Icons name="right_arrow" height="30px" width="30px"/>
           </div>
        </ListItem>
    </div>
</Layout>