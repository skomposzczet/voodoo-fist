<template>
    <div class="topline">
        <router-link to="/">
            <Icon icon="ion:chevron-left" class="goback"/>
        </router-link>
        <FancyForm text="todo, api get list" :font_size="25"/>
        <p style="font-size: 10px; color: grey">{{ $route.params.id }}</p>
    </div>
    <div :key="item._id.$oid" v-for="item in items">
            <TodoItem :item="item"/>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import {TodoItem as TodoItem_t, MongoID} from '../api-types'
import DataService from '../services/data-service'
import FancyForm from '@/components/FancyForm.vue';
import TodoItem from '@/components/TodoItem.vue';
import {Icon} from "@iconify/vue"

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
        async fetch_set_items() {
            try {
                const id: MongoID = {$oid: this.$route.params.id as string};
                const res = await DataService.get_items(id);
                this.items = res.data.data.items;
            } catch(err) {
                console.log(err);
            }
        },
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