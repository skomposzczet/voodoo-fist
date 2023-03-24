<template>
    <div>
        <button :class="['btn', (is_disabled ? 'dis' : 'enb')]" :disabled="is_disabled" :title="reason">
            {{ text }}
        </button>
    </div>
</template>
<script lang="ts">
import { defineComponent } from 'vue';
export default defineComponent({
    name: "DisablableButton",
    props: {
        text: String,
        is_disabled: Boolean,
        reason: String,
    },
});
</script>
<style>
.dis {
    background-color: grey;
}
.dis:active {
    transform: none;
}
.enb {
    background-color: green;
}
.enb:hover {
    background-color: rgb(0, 100, 0);
}
</style>