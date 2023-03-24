<template>
    <h2 v-if="username !== ''">{{ username }}'s dashboard</h2>
    <UsersLists :lists="lists" @change-color="patch_list" @delete-list="delete_list" @new-list="new_list"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import authHeader from "../services/auth-header";
import DataService from "../services/data-service"
import {List, ListPatch, MongoID} from "@/api-types"
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
    methods: {
        handle_err(error: AxiosError) {
            console.log(error);
            if (error instanceof AxiosError && error.response?.status === 401) {
                this.$store.dispatch('auth/logout');
                this.$router.push('/auth');
            }
        },
        async fetch_set_username() {
            try {
                const response = await this.axios.get('api/dashboard', { headers: authHeader() });
                this.username = response.data.data.username;
            } catch(err) {
                this.handle_err(err as AxiosError);
            }
        },
        async fetch_set_lists() {
            try {
                const response = await DataService.get_lists();
                this.lists = response.data.data.lists;
            } catch(err) {
                this.handle_err(err as AxiosError);
            }
        },
        async patch_list(patch: ListPatch) {
            try {
                await DataService.patch_list(patch);
            } catch(err) {
                this.handle_err(err as AxiosError);
            }
        },
        async delete_list(id: MongoID) {
            try {
                const res = await DataService.delete('list', id);
                if (res.status === 200) {
                    this.lists = this.lists.filter(list => (list._id !== id));
                }
            } catch(err) {
                this.handle_err(err as AxiosError);
            }
        },
        async new_list(title: string) {
            try{
                const res = await DataService.new_list(title);
                if (res.status === 200) {
                    await this.fetch_set_lists();
                }
            } catch(err) {
                this.handle_err(err as AxiosError);
            }
        },
    },
    async created() {
        await this.fetch_set_username();
        await this.fetch_set_lists();
    },
});
</script>