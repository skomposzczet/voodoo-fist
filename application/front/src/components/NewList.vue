<template >
    <div class="adding">
        <form ref="listForm" :style="[is_active ? { height: computed_height } : {}]" class="appearing formed" @submit="on_submit">
            <input type="text" placeholder="New List Title" ref="inp" v-model="text" @input="on_input"/>
            <DisablableButton text="OK" :is_disabled="!addable" :reason="reason"/>
            <div v-if="loading" class="lds-ring"><div></div><div></div><div></div><div></div></div>
        </form>
        <GreatButton @click="on_click" :style="{backgroundColor: colors[index]}" :text="texts[index]"/>
    </div>
</template>
<script lang="ts">
import { defineComponent } from "vue"
import GreatButton from "./GreatButton.vue"
import DisablableButton from "./DisablableButton.vue"
import DataService from "../services/data-service";
import { AxiosError } from "axios";

export default defineComponent({
    name: "NewList",
    components: {
        GreatButton,
        DisablableButton,
    },
    data() {
        return {
            is_active: false,
            computed_height: '',
            index: 0,
            texts: ['New list', 'Cancel'],
            colors: ['green', 'red'],
            text: '',
            addable: false,
            reason: '',
            loading: false,
            timer: undefined as | number | undefined,
        }
    },
    methods: {
        on_click() {
            this.toggle_active();
            this.toggle_index();
        },
        toggle_active() {
            this.is_active = !this.is_active;
            if (this.is_active)
                (this.$refs.inp as HTMLElement).focus();
        },
        toggle_index() {
            this.index = (this.index + 1) % 2;
        },
        init_height() {
            (this.$refs['listForm'] as HTMLElement).style.height = 'auto';
            (this.$refs['listForm'] as HTMLElement).style.position = 'absolute';
            (this.$refs['listForm'] as HTMLElement).style.visibility = 'hidden';
            (this.$refs['listForm'] as HTMLElement).style.display = 'block';

            this.computed_height = getComputedStyle((this.$refs['listForm'] as HTMLElement)).height;      

            (this.$refs['listForm'] as HTMLElement).style.position = '';
            (this.$refs['listForm'] as HTMLElement).style.visibility = '';
            (this.$refs['listForm'] as HTMLElement).style.display = '';
            (this.$refs['listForm'] as HTMLElement).style.height = '';
        },
        on_submit(e: Event) {
            e.preventDefault();
            if (!this.addable) {
                return;
            }
            this.$emit('new-list', this.text);
            setTimeout(this.on_click, 200);
            setTimeout(() => {this.text = ''; this.on_input()}, 600);
        },
        on_input() {
            clearTimeout(this.timer);
            if (this.text.length === 0) {
                this.addable = false;
                this.loading = false;
                this.reason = 'Title cannot be empty';
                return;
            }

            this.loading = true;
            this.timer = setTimeout(async () => (
                await this.allowed()
            ), 500);
        },
        async allowed() {
            this.loading = false;
            const unique = await this.is_unique(this.text);
            if (!unique) {
                this.addable = false;
                this.reason = 'Not unique';
            }
            else {
                this.addable = true;
                this.reason = '';
            }
        },
        async is_unique(title: string) {
            try {
                const json = JSON.parse('{"title": "' + title + '"}');
                await DataService.search('list', json);
                return false;
            } catch(err) {
                const error = err as AxiosError;
                if (error.response?.status === 404) {
                    return true;
                } else if (error.status === 401) {
                    this.$store.dispatch('auth/logout');
                    this.$router.push('/auth');
                }
            }
        }
    },
    mounted() {
        this.init_height();
        this.on_input();
    }
});
</script>
<style scoped>
.adding {
    padding: 5px;
}
.appearing {
    height: 0;
    overflow: hidden;
    transition: 0.5s;
}
.formed {
    display: flex;
    flex-direction: row;
    align-items: center;
}
</style>
