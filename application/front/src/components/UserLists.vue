<template>
    <NewList @new-list="title => $emit('new-list', title)"/>
    <div :key="list._id.$oid" v-for="list in lists">
        <TodoList :list="list" @change-color="patch => $emit('change-color', patch)" @delete-list="id => $emit('delete-list', id)"/>
    </div>
</template>
<script lang="ts">
import { defineComponent, PropType } from "vue"
import {List} from "@/api-types"
import TodoList from "./TodoList.vue"
import NewList from "./NewList.vue"

export default defineComponent({
    name: "UsersLists",
    components: {
    TodoList,
    NewList,
},
    props: {
        lists: {type: Object as PropType<Array<List>>}
    },
    emits: ['change-color', 'delete-list', 'new-list'],
});
</script>