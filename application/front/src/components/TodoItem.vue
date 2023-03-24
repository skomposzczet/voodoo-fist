
<template>
    <div :class="['item', item.is_done ? '' : 'done']" @contextmenu="evt => toggle_item(evt, item._id)">
        <h3>
            <FancyForm :text="item.text" :font_size="25"/>
            <Icon @click="$emit('delete-item', item._id)" icon="ep:close-bold" class="delete"/>
        </h3>
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from 'vue'
import {Icon} from "@iconify/vue"
import FancyForm from "./FancyForm.vue"
import {MongoID, TodoItem} from '../api-types'

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
        toggle_item(e: Event, id: MongoID) {
            e.preventDefault();
            this.$emit('togle-item', id);
        }
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