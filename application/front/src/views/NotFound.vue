<template>
    <div>
        <img src="https://cataas.com/cat/says/404%20not%20found?fontColor=white" class="image">
    </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'NotFound',
});
</script>
<style>
.image {
    max-width: 100%;
    max-height: 500px;
    display: block;
    margin-left: auto;
    margin-right: auto;
}
</style>
