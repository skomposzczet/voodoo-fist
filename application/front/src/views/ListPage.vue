<template>
    <div class="topline">
        <router-link to="/">
            <Icon icon="ion:chevron-left" class="goback"/>
        </router-link>
        <FancyForm text="todo, api get list" :font_size="25"/>
        <p style="font-size: 10px; color: grey">{{ $route.params.id }}</p>
    </div>
    <div :key="item._id.$oid" v-for="item in items">
            <TodoItem :item="item" @delete-item="delete_item" @patch-item="patch_item"/>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {TodoItem as TodoItem_t, TodoItemPatch, MongoID} from '../api-types'
import DataService from '../services/data-service'
import FancyForm from '@/components/FancyForm.vue';
import TodoItem from '@/components/TodoItem.vue';
import {Icon} from "@iconify/vue"
import { AxiosError } from 'axios';

export default defineComponent({
    name: 'ListPage',
    components: {
        FancyForm,
        TodoItem,
        Icon,
    },
    data() {
        return {
            items: [] as TodoItem_t[], 
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
        async fetch_set_items() {
            try {
                const id: MongoID = {$oid: this.$route.params.id as string};
                const res = await DataService.get_items(id);
                this.items = res.data.data.items;
            } catch(err) {
                this.handle_err(err as AxiosError)
            }
        },
        async delete_item(id: MongoID) {
            try {
                const res = await DataService.delete('item', id);
                if (res.status === 200) {
                    this.items = this.items.filter(item => (item._id !== id));
                }
            } catch(err) {
                this.handle_err(err as AxiosError)
            }
        },
        async patch_item(patch: TodoItemPatch) {
            try {
                const idx = this.items.findIndex(item => (item._id === patch._id));
                const res = await DataService.patch_item(patch);
                if (res.status !== 200) {
                    console.log(res.status);
                    return;
                }
                if (patch.is_done) {
                    this.items[idx].is_done = patch.is_done;
                }
            } catch(err) {
                this.handle_err(err as AxiosError)
            }
        }
    },
    async created() {
        await this.fetch_set_items();
    },
});
</script>
<style scoped>
div {
    padding: 3px;
    padding-left: 10px;
    padding-right: 10px;
    transition: 2s;
}
.topline {
    display: flex;
    align-items: center;
} 
.goback {
    color: whitesmoke;
    font-size: 20px;
    margin-top: 7px;
}
.goback:hover {
    color: #9F838C;
}
</style>