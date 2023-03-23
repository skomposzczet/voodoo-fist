<template>
    <header>
        <router-link to="/" >
            <h1>{{ title }}</h1>
        </router-link>
        <a v-if="$store.state.auth.status" class="hvr" @click="logout">Logout</a>
    </header>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: "MyHeader",
    props: {
        title: String,
    },
    methods: {
        logout() {
            this.$store.dispatch('auth/logout');
            this.$router.push('/auth');
        }
    }
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