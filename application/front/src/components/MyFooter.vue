<template>
    <footer>
        <p>&copy; 2023 Copyright: <a href="https://github.com/skomposzczet" target="_blank">Skomposzczet</a></p>
        <router-link to="/about">About</router-link>
    </footer>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: "MyFooter",
});
</script>

<style scoped>
footer {
    margin-top: 30px;
    text-align: center;
}
a {
    font-weight: bold;
}
</style>