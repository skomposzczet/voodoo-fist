<template>
    <div :key="list._id.$oid" v-for="list in lists">
        <TodoList :list="list" @change-color="patch => $emit('change-color', patch)"/>
    </div>
</template>
<script lang="ts">
import { defineComponent, PropType } from "vue"
import {List} from "@/api-types"
import TodoList from "./TodoList.vue"

export default defineComponent({
    name: "UsersLists",
    components: {
        TodoList,
    },
    props: {
        lists: {type: Object as PropType<Array<List>>}
    },
    emits: ['change-color'],
});
</script>