<template>
    <h2 v-if="username !== ''">{{ username }}'s dashboard</h2>
    <UsersLists :lists="lists"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import authHeader from "../services/auth-header";
import DataService from "../services/data-service"
import {List} from "@/api-types"
import UsersLists from "../components/UserLists.vue"

export default defineComponent({
    name: 'DashboardPage',
    components: {
        UsersLists,
    },
    data() {
        return {
            username: '',
            lists: [] as List[],
        }
    },
    methods: {
        async fetch_set_username() {
            try {
                const response = await this.axios.get('api/dashboard', { headers: authHeader() });
                this.username = response.data.data.username;
            } catch(err) {
                this.$store.dispatch('auth/logout');
                this.$router.push('/auth');
            }
        },
        async fetch_set_lists() {
            try {
                const response = await DataService.get_lists();
                this.lists = response.data.data.lists;
                console.log(response);
            } catch(err) {
                console.log(err)
            }
        },
    },
    async created() {
        await this.fetch_set_username();
        await this.fetch_set_lists();
    },
});
</script>