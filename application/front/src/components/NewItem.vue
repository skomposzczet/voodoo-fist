<template>
    <div class="adding">
        <Icon @click="submit" icon="ion:add-circle" :class="['add', (text.length === 0 ? 'forbid' : 'allow')]"/>
        <form @submit="submit">
            <input v-model="text" :placeholder="random_proposition()"/>
        </form>
    </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue'
import {Icon} from "@iconify/vue";
import swal from 'sweetalert';
import RandomProposition from '../services/propositions'

export default defineComponent({
    name: "NewItem",
    components: {
        Icon,
    },
    data() {
        return {
            text: '',
        }
    },
    methods: {
        random_proposition() {
            return RandomProposition.get();
        },
        submit(e: Event | null = null) {
            if (e)
                e.preventDefault();
            if (this.text.length === 0) {
                swal("Cannot add item!", "Item title cannot be empty!", "error");
                return;
            }
            this.$emit('add-item', this.text);
            this.text = '';
        }
    },
});
</script>
<style scoped>
.adding {
    display: flex;
    align-items: center;
    padding: 5px;
    padding-right: 50px;
    border: none;
    border-radius: 5px;
    border-color: whitesmoke;
    background-color: #272727;
}
form {
    width: 100%;
}
input {
    border: none;
    border-radius: 4px;
    width: 100%;
    font-size: 25px;
    background-color: #272727;
    color: whitesmoke;
}
input:focus {
    outline: none;
}
.add {
    margin-right: 5px;
    font-size: 30px;
    transition: color 1s ease-in-out;
    transition: transform .3s ease-in-out;
    color: whitesmoke;
}
.add.allow:hover {
    color: rgb(0, 228, 0);
    transform: rotate( 90deg );
}
.add.forbid:hover {
    color: red;
    transform: rotate( -90deg );
}
</style>