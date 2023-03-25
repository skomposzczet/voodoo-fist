<template>
    <div class="topline">
        <router-link to="/">
            <Icon icon="ion:chevron-left" class="goback"/>
        </router-link>
        <FancyForm v-if="title !== ''" :text=title :font_size="25"/>
        <p style="font-size: 10px; color: grey">{{ $route.params.id }}</p>
    </div>
    <div :key="item._id.$oid" v-for="item in items">
            <TodoItem :item="item" @delete-item="delete_item" @patch-item="patch_item"/>
    </div>
    <NewItem @add-item="add_item"/>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {TodoItem as TodoItem_t, TodoItemPatch, MongoID, TodoItemNew, Color} from '../api-types'
import DataService from '../services/data-service'
import FancyForm from '@/components/FancyForm.vue';
import TodoItem from '@/components/TodoItem.vue';
import {Icon} from "@iconify/vue"
import { AxiosError } from 'axios';
import NewItem from '@/components/NewItem.vue';

export default defineComponent({
    name: 'ListPage',
    components: {
        FancyForm,
        TodoItem,
        Icon,
        NewItem,
    },
    data() {
        return {
            items: [] as TodoItem_t[], 
            title: '',
        }
    },
    emits: ['change-color', 'reset-color'],
    methods: {
        handle_err(error: AxiosError) {
            console.log(error);
            if (!(error instanceof AxiosError)) {
                return;
            } 
            if (error.response?.status === 401) {
                this.$store.dispatch('auth/logout');
                this.$router.push('/auth');
            }
            this.$router.push('/404');
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
        async fetch_set_current_list() {
            try {
                const res = await DataService.get_list(this.$route.params.id as string);
                if (res.status === 200) {
                    this.title = res.data.data.list.title;
                    return res.data.data.list.color;
                }
            } catch(err) {
                this.handle_err(err as AxiosError);
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
                if (patch.is_done !== undefined && patch.is_done !== null) {
                    this.items[idx].is_done = patch.is_done;
                }
            } catch(err) {
                this.handle_err(err as AxiosError)
            }
        },
        async add_item(text: string) {
            try {
                const id: MongoID = {$oid: this.$route.params.id as string};
                const item: TodoItemNew = {list_oid: id, text: text};
                const res = await DataService.new_item(item);
                if (res.status === 200) {
                    await this.fetch_set_items();
                } else {
                    console.log(res.status);
                }
            } catch(err) {
                this.handle_err(err as AxiosError);
            }
        },
        change_bgcolor(rgb: Color) {
            const color = 'rgb(' + rgb.r + ', ' + rgb.g + ', ' + rgb.b + ')';
            this.$emit('change-color', color);
        },
    },
    async created() {
        await this.fetch_set_items();
        let color: Color = await this.fetch_set_current_list();
        this.change_bgcolor(color)
    },
    beforeUnmount() {
        this.$emit('reset-color');
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