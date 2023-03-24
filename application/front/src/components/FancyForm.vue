<template>
    <div @dblclick="show_form" >
        <p v-show="!edit_mode">{{text_}}</p>
        <form v-show="edit_mode" @submit.prevent>
            <input type="text" @blur="submit_form" v-model="text_" ref="name"/>
        </form>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    name: "FancyForm",
    props: {
        text: String,
        font_size: Number,
    },
    data() {
        return {
            text_: this.text,
            edit_mode: false,
            fontsize: this.font_size+'px',
        }
    },
    methods: {
        show_form() {
            this.edit_mode = true;
            setTimeout(() => {
                (this.$refs.name as HTMLElement).focus();
            }, 50);
        },
        submit_form() {
            this.edit_mode = false;
            this.$emit('change-text', this.text_);
        },
    },
});
</script>

<style scoped>
div {
    font-size: v-bind(fontsize);
    width: 100%;
}
</style>