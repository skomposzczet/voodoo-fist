<template >
    <div class="listitem" @mouseover="toggle_hovered(true)" @mouseout="toggle_hovered(false)">
        <div class="title">
            <ColorPicker shape="circle" :pureColor="list.color" :disableAlpha="true" @update:pureColor="set_color"/>
            <router-link :to="{path: '/list/' + list._id.$oid}">
                <h2>{{ list.title }}</h2>
            </router-link><br>
        </div>
        <Icon v-show="hovered" icon="ion:trash-sharp" class="delete" @click="delete_list" />
    </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import {Icon} from "@iconify/vue"
import swal from 'sweetalert';
import {List, ListPatch, Color} from "@/api-types"

export default defineComponent({
    name: "ListItem",
    components: {
        Icon,
    },
    props: {
        list: {
            type: Object as PropType<List>,
            required: true
        },
    },
    data() {
        return {
            hovered: false,
            timer: undefined as | number | undefined,
        }
    },
    methods: {
        set_color(color: string) {
            const vs = color.slice(4, -1).split(',').map(v => parseInt(v));
            const new_color: Color = {r: vs[0], g: vs[1], b: vs[2]};
            const patch: ListPatch = {_id: this.list._id, color: new_color, title: null};
            clearTimeout(this.timer);
            this.timer = setTimeout(() => (
                this.$emit('change-color', patch)
            ), 2000);
        },
        async delete_list() {
            const accepted = await swal('You sure?', { buttons: ['nah', 'sure']});
            if (accepted) {
                this.$emit('delete-list', this.list._id);
            }
        },
        toggle_hovered(value: boolean) {
            this.hovered = value;
        },
    },
});
</script>

<style scoped>
.listitem {
    border-radius: 6px;
    background-color: rgba(245, 245, 245, 0);
    color: whitesmoke;
    padding-left: 10px;
    display: flex;
    align-items: center;
    justify-content: space-between;
}
.listitem:hover {
    background-color: rgba(245, 245, 245, 0.1);
}
.title {
    display: flex;
    align-items: center;
    justify-content: left;
}
.delete {
    font-size: 20px;
}
.delete:hover { 
    color: red;
}
</style>