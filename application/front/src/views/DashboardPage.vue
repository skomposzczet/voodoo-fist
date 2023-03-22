<template>
    <h1>Home</h1>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import authHeader from "../services/auth-header";

export default defineComponent({
    name: 'DashboardPage',
    async mounted() {
        try {
            const response = await this.axios.get('api/lists', { headers: authHeader() });
            console.log(response);
        } catch(err) {
            this.$router.push('/auth')      
        }
    },
});
</script>