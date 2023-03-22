<template>
    <header>
        <router-link to="/" >
            <h1>{{ title }}</h1>
        </router-link>
    </header>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: "MyHeader",
    props: {
        title: String,
    },
});
</script>

<style scoped>
    header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
    }

</style>