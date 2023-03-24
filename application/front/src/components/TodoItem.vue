
<template>
    <div :class="['item', item.is_done ? 'done' : '']" @contextmenu="evt => toggle_item(evt)">
        <h3>
            <FancyForm :text="item.text" :font_size="25" @change-text="rename"/>
            <Icon @click="$emit('delete-item', item._id)" icon="ep:close-bold" class="delete"/>
        </h3>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue'
import {Icon} from "@iconify/vue"
import FancyForm from "./FancyForm.vue"
import {TodoItem, TodoItemPatch} from '../api-types'

export default defineComponent({
    name: "TodoItem",
    components: {
        Icon,
        FancyForm,
    },
    props: {
        item: {type: Object as PropType<TodoItem>, required: true},
    },
    methods: {
        toggle_item(e: Event) {
            e.preventDefault();
            const patch: TodoItemPatch = {_id: this.item._id, is_done: !this.item.is_done};
            this.$emit('patch-item', patch);
        },
        rename(new_text: string) {
            const patch: TodoItemPatch = {_id: this.item._id, text: new_text};
            this.$emit('patch-item', patch);
        },
    },
});
</script>
<style scoped>
.item {
    padding: 5px;
    border: none;
    border-radius: 5px;
    background-color: #272727;
    color: whitesmoke;
    display: grid;
    align-items: center;
}
.item.done {
    text-decoration: line-through;
    color: grey
}
.item h3 {
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.delete {
    color: whitesmoke;
}
.delete:hover {
    color: red;
    animation-name: wiggle;
    animation-duration: 0.4s;
    animation-iteration-count: infinite;
    animation-timing-function: linear;
}
@keyframes wiggle {
    0% {transform: rotate( 0deg );}
    25% {transform: rotate( -15deg );}
    50% {transform: rotate( 10deg );}
    75% {transform: rotate( -10deg );}
    100% {transform: rotate( 15deg );}
}
</style>