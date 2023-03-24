<template>
    <h3>Create your todo list with VooDoo Fist.</h3>
    <router-link to="/">Home</router-link>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: 'AboutPage',
    emits: ['change-color', 'reset-color'],
});
</script>