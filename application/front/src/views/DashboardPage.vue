<template>
    <h2 v-if="username !== ''">{{ username }}'s dashboard</h2>
    <UsersLists :lists="lists" @change-color="patch_list"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import authHeader from "../services/auth-header";
import DataService from "../services/data-service"
import {List, ListPatch} from "@/api-types"
import UsersLists from "../components/UserLists.vue"
import { AxiosError } from 'axios';

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
    emits: ['change-color'],
    methods: {
        handle_err(error: any) {
            console.log(error)
            if (error instanceof AxiosError && error.status === 401) {
                this.$store.dispatch('auth/logout');
                this.$router.push('/auth');
            }
        },
        async fetch_set_username() {
            try {
                const response = await this.axios.get('api/dashboard', { headers: authHeader() });
                this.username = response.data.data.username;
            } catch(err) {
                this.handle_err(err);
            }
        },
        async fetch_set_lists() {
            try {
                const response = await DataService.get_lists();
                this.lists = response.data.data.lists;
                console.log(response);
            } catch(err) {
                this.handle_err(err);
            }
        },
        async patch_list(patch: ListPatch) {
            console.log(patch)
            try {
                const res = await DataService.patch_list(patch);
            } catch(err) {
                this.handle_err(err);
            }
        }
    },
    async created() {
        await this.fetch_set_username();
        await this.fetch_set_lists();
    },
});
</script>